pub mod keyword_database;
pub mod md5_database;
pub mod media_details;
pub mod phash_database;
pub mod statusbar;
pub mod toolbar;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use num_format::ToFormattedString;
use relm4::{
    adw,
    component::{
        AsyncComponent, AsyncComponentParts, AsyncComponentSender, ComponentController, Controller,
    },
    gtk::{
        self,
        glib::{self, object::ObjectExt, value::ToValue},
        prelude::{BoxExt, Cast, FrameExt, GtkWindowExt, ListModelExt, OrientableExt, WidgetExt},
    },
    typed_view::grid::TypedGridView,
    Component, RelmWidgetExt,
};
use relm4_components::open_dialog::*;

use super::dialogs;
use crate::app::{
    components::progress_dialog::{ProgressDialog, ProgressDialogOutput, ProgressSettings},
    components::searchbar::{SearchBarInput, SearchBarModel, SearchBarOutput},
    config::info,
    factories::media_item::MediaItem,
    models,
};
use crate::{context::AppContext, fl};
use core_chasam::csam::StateMedia;
use media_details::{MediaDetailsInput, MediaDetailsModel, MediaDetailsOutput};
use statusbar::{StatusbarInput, StatusbarModel};
use toolbar::{ToolbarModel, ToolbarOutput};

pub struct CsamModel {
    ctx: AppContext,
    save_dialog: Controller<OpenDialog>,
    progress_dialog: Controller<ProgressDialog>,
    searchbar: Controller<SearchBarModel>,
    toolbar: Controller<ToolbarModel>,
    statusbar: Controller<StatusbarModel>,
    media_list_wrapper: TypedGridView<MediaItem, gtk::NoSelection>,
    media_filter: Rc<RefCell<models::MediaFilter>>,
    media_details: Controller<MediaDetailsModel>,
    thumbnail_size: i32,
}

#[derive(Debug)]
pub enum CsamInput {
    StartSearch(PathBuf),
    StopSearch,
    ZoomIn,
    ZoomOut,
    HammingDistanceFilter(u32),
    ImageFilter(bool),
    VideoFilter(bool),
    CSAMFilter(bool),
    SelectAllMedias(bool),
    SizeFilter0KB(bool),
    SizeFilter30KB(bool),
    SizeFilter100KB(bool),
    SizeFilter500KB(bool),
    SizeFilterA500KB(bool),
    SearchEntry(String),
    SaveSelectedMedia,
    CancelMediaExport,
    SaveFileResponse(PathBuf),
    MediaListSelect(u32),
    ShowInfoDialog(String),
    ShowProgressDialog(bool),
    Notify(String, u32),
    Ignore,
}

#[derive(Debug)]
pub enum CsamCommandOutput {
    SearchCompleted,
    AddMedia(anyhow::Result<Vec<models::Media>>),
    MediaFound(usize),
    ShowProgressDialog(bool),
    Notify(String, u32),
}

#[relm4::component(pub async)]
impl AsyncComponent for CsamModel {
    type Init = AppContext;
    type Input = CsamInput;
    type Output = ();
    type CommandOutput = CsamCommandOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_vexpand: true,

            append = &adw::HeaderBar {
                set_hexpand: true,
                set_css_classes: &["flat"],
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: true,

                #[wrap(Some)]
                set_title_widget = model.searchbar.widget(),
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,
                set_css_classes: &["view"],

                append = model.toolbar.widget(),

                #[name(overlay)]
                append = &adw::ToastOverlay {
                    #[wrap(Some)]
                    set_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,
                        set_vexpand: true,

                        append = &gtk::Paned {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_hexpand: true,
                            set_vexpand: true,
                            set_resize_start_child: true,
                            set_resize_end_child: false,
                            set_shrink_start_child: false,
                            set_shrink_end_child: false,
                            set_wide_handle: true,
                            set_margin_bottom: 5,
                            set_margin_end: 6,
                            set_margin_start: 6,

                            #[wrap(Some)]
                            set_start_child = &gtk::Frame {
                                set_width_request: 800,
                                set_hexpand: true,
                                set_vexpand: true,
                                set_margin_end: 6,

                                gtk::ScrolledWindow {
                                    set_hscrollbar_policy: gtk::PolicyType::Never,
                                    set_hexpand: true,
                                    set_vexpand: true,

                                    #[local_ref]
                                    media_list_widget -> gtk::GridView {
                                        set_vexpand: true,
                                        set_single_click_activate: true,
                                        set_enable_rubberband: false,
                                        set_max_columns: 10,
                                        connect_activate[sender] => move |_, position| {
                                            sender.input(CsamInput::MediaListSelect(position));
                                        },
                                    },
                                },
                            },

                            #[wrap(Some)]
                            set_end_child = &gtk::Frame {
                                set_width_request: 300,
                                set_vexpand: true,
                                set_margin_start: 6,

                                set_child = model.media_details.widget().downcast_ref::<gtk::Box>(),
                            },
                        },
                    },
                },
            },

            append = model.statusbar.widget(),
        }
    }

    async fn init(
        ctx: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let save_dialog_settings = OpenDialogSettings {
            folder_mode: true,
            accept_label: String::from(fl!("open")),
            cancel_label: String::from(fl!("cancel")),
            create_folders: true,
            is_modal: true,
            filters: Vec::new(),
        };

        let save_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(save_dialog_settings)
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => CsamInput::SaveFileResponse(path),
                OpenDialogResponse::Cancel => CsamInput::Ignore,
            });

        let progress_settings = ProgressSettings {
            text: fl!("wait").to_string(),
            secondary_text: Some("Exporting media...".to_string()),
            cancel_label: fl!("cancel").to_string(),
        };

        let progress_dialog = ProgressDialog::builder()
            .transient_for(&root)
            .launch(progress_settings)
            .forward(sender.input_sender(), move |response| match response {
                ProgressDialogOutput::Cancel => CsamInput::CancelMediaExport,
            });

        let searchbar_controller =
            SearchBarModel::builder()
                .launch(())
                .forward(sender.input_sender(), |output| match output {
                    SearchBarOutput::StartSearch(path) => CsamInput::StartSearch(path),
                    SearchBarOutput::StopSearch => CsamInput::StopSearch,
                    SearchBarOutput::Notify(msg, timeout) => CsamInput::Notify(msg, timeout),
                });

        let toolbar_controller = ToolbarModel::builder()
            .launch_with_broker(ctx.clone(), &toolbar::SELECT_BROKER)
            .forward(sender.input_sender(), |output| match output {
                ToolbarOutput::SelectAll(is_selected) => CsamInput::SelectAllMedias(is_selected),
                ToolbarOutput::SaveSelected => CsamInput::SaveSelectedMedia,
                ToolbarOutput::ZoomIn => CsamInput::ZoomIn,
                ToolbarOutput::ZoomOut => CsamInput::ZoomOut,
                ToolbarOutput::HammingDistanceFilter(value) => {
                    CsamInput::HammingDistanceFilter(value)
                }
                ToolbarOutput::ImageFilter(is_active) => CsamInput::ImageFilter(is_active),
                ToolbarOutput::VideoFilter(is_active) => CsamInput::VideoFilter(is_active),
                ToolbarOutput::CSAMFilter(is_active) => CsamInput::CSAMFilter(is_active),
                ToolbarOutput::SizeFilter0KB(is_active) => CsamInput::SizeFilter0KB(is_active),
                ToolbarOutput::SizeFilter30KB(is_active) => CsamInput::SizeFilter30KB(is_active),
                ToolbarOutput::SizeFilter100KB(is_active) => CsamInput::SizeFilter100KB(is_active),
                ToolbarOutput::SizeFilter500KB(is_active) => CsamInput::SizeFilter500KB(is_active),
                ToolbarOutput::SizeFilterGreater500KB(is_active) => {
                    CsamInput::SizeFilterA500KB(is_active)
                }
                ToolbarOutput::SearchEntry(query) => CsamInput::SearchEntry(query),
            });

        let statusbar_controller = StatusbarModel::builder().launch(ctx.clone()).detach();

        let media_list_wrapper: TypedGridView<MediaItem, gtk::NoSelection> = TypedGridView::new();
        media_list_wrapper
            .selection_model
            .bind_property(
                "n-items",
                &statusbar_controller.widgets().label_media_found,
                "label",
            )
            .transform_to({
                let locale = ctx.get_locale();
                move |_, n_items: u32| Some(n_items.to_formatted_string(&locale).to_value())
            })
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        let media_details_controller = MediaDetailsModel::builder()
            .launch(models::MediaDetail::default())
            .forward(sender.input_sender(), |output| match output {
                MediaDetailsOutput::Notify(msg, timeout) => CsamInput::Notify(msg, timeout),
            });

        let mut model = CsamModel {
            ctx,
            save_dialog,
            progress_dialog,
            searchbar: searchbar_controller,
            toolbar: toolbar_controller,
            statusbar: statusbar_controller,
            media_list_wrapper,
            media_filter: Rc::new(RefCell::new(models::MediaFilter::default())),
            media_details: media_details_controller,
            thumbnail_size: models::media::THUMBNAIL_SIZE,
        };

        let filter = model.media_filter.clone();
        model.media_list_wrapper.add_filter(on_filter(filter));
        model.media_list_wrapper.set_filter_status(0, false);

        let media_list_widget = &model.media_list_wrapper.view;
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            CsamInput::StartSearch(path) => {
                self.media_list_wrapper.clear();
                self.statusbar.emit(StatusbarInput::Loading(true));
                self.media_details.emit(MediaDetailsInput::Reset);
                self.on_search(path, &sender).await;
            }
            CsamInput::StopSearch => {
                self.ctx.csam_service.cancel_task();
            }
            CsamInput::MediaListSelect(position) => {
                if let Some(item) = self.media_list_wrapper.get_visible(position) {
                    let media = &item.borrow().media;
                    self.media_details.emit(MediaDetailsInput::ShowMedia(
                        models::MediaDetail::from(media),
                    ));
                }
            }
            CsamInput::SelectAllMedias(is_selected) => {
                self.on_select_all_medias(is_selected).await;
            }
            CsamInput::ZoomIn => {
                self.apply_media_zoom(true).await;
            }
            CsamInput::ZoomOut => {
                self.apply_media_zoom(false).await;
            }
            CsamInput::HammingDistanceFilter(value) => {
                self.media_filter.borrow_mut().hamming_distance = value;
                self.apply_media_filters().await;
            }
            CsamInput::CSAMFilter(is_active) => {
                self.media_filter.borrow_mut().is_csam = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::ImageFilter(is_active) => {
                self.media_filter.borrow_mut().is_image = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::VideoFilter(is_active) => {
                self.media_filter.borrow_mut().is_video = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::SizeFilter0KB(is_active) => {
                self.media_filter.borrow_mut().is_size_0 = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::SizeFilter30KB(is_active) => {
                self.media_filter.borrow_mut().is_size_30 = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::SizeFilter100KB(is_active) => {
                self.media_filter.borrow_mut().is_size_100 = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::SizeFilter500KB(is_active) => {
                self.media_filter.borrow_mut().is_size_500 = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::SizeFilterA500KB(is_active) => {
                self.media_filter.borrow_mut().is_size_greater_500 = is_active;
                self.apply_media_filters().await;
            }
            CsamInput::SearchEntry(query) => {
                self.media_filter.borrow_mut().search_entry = Some(query);
                self.apply_media_filters().await;
            }
            CsamInput::SaveSelectedMedia => {
                self.save_dialog.emit(OpenDialogMsg::Open);
            }
            CsamInput::CancelMediaExport => {
                self.ctx.csam_service.cancel_task();
            }
            CsamInput::SaveFileResponse(path) => {
                self.on_save_selected_media(&path, sender.clone()).await;
            }
            CsamInput::ShowInfoDialog(msg) => {
                let window = root.toplevel_window();
                dialogs::show_info_dialog(window.as_ref(), Some(info::APP_NAME), Some(&msg));
            }
            CsamInput::ShowProgressDialog(show) => {
                if show {
                    self.progress_dialog.widget().present();
                } else {
                    self.progress_dialog.widget().close();
                }
            }
            CsamInput::Notify(msg, timeout) => {
                let toast = adw::Toast::builder().title(msg).timeout(timeout).build();
                widgets.overlay.add_toast(toast);
            }
            CsamInput::Ignore => (),
        }

        self.update_view(widgets, sender);
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            CsamCommandOutput::SearchCompleted => {
                self.searchbar.emit(SearchBarInput::SearchCompleted);
                self.statusbar.emit(StatusbarInput::Loading(false));
            }
            CsamCommandOutput::MediaFound(found) => {
                self.statusbar.emit(StatusbarInput::TotalFound(found));
            }
            CsamCommandOutput::AddMedia(result) => match result {
                Ok(medias) => {
                    let media_items = medias
                        .into_iter()
                        .map(MediaItem::new)
                        .inspect(|item| {
                            if item.is_video() {
                                self.statusbar.emit(StatusbarInput::VideoFound(1));
                            } else {
                                self.statusbar.emit(StatusbarInput::ImageFound(1));
                            }
                            if item.is_csam() {
                                self.statusbar.emit(StatusbarInput::CSAMFound(1));
                            }
                        })
                        .collect::<Vec<MediaItem>>();

                    self.media_list_wrapper.extend_from_iter(media_items);
                }
                Err(err) => {
                    sender.input(CsamInput::Notify(
                        format!("{}: {}", fl!("generic-error"), err),
                        5,
                    ));
                    tracing::error!("{}: {}", fl!("generic-error"), err);
                }
            },
            CsamCommandOutput::ShowProgressDialog(show) => {
                sender.input(CsamInput::ShowProgressDialog(show))
            }
            CsamCommandOutput::Notify(msg, timeout) => {
                sender.input(CsamInput::Notify(msg, timeout))
            }
        }
    }
}

impl CsamModel {
    async fn on_search(&mut self, path: PathBuf, sender: &AsyncComponentSender<CsamModel>) {
        self.statusbar.emit(StatusbarInput::Calculating);

        let (tx, mut rx) = relm4::tokio::sync::mpsc::channel(100);

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    while let Some(state) = rx.recv().await {
                        match state {
                            StateMedia::Completed => {
                                out.send(CsamCommandOutput::SearchCompleted)
                                    .unwrap_or_default();
                            }
                            StateMedia::Found(count) => {
                                out.send(CsamCommandOutput::MediaFound(count))
                                    .unwrap_or_default();
                            }
                            StateMedia::Ok(medias) => {
                                let vec_medias = medias.iter().map(models::Media::from).collect();

                                out.send(CsamCommandOutput::AddMedia(Ok(vec_medias)))
                                    .unwrap_or_default();
                            }
                            StateMedia::Err(error) => {
                                out.send(CsamCommandOutput::AddMedia(Err(error)))
                                    .unwrap_or_default();
                            }
                        }
                    }
                })
                .drop_on_shutdown()
        });

        self.ctx.csam_service.search_media(path, tx);
    }

    async fn on_select_all_medias(&mut self, is_active: bool) {
        if is_active {
            for position in 0..self.media_list_wrapper.selection_model.n_items() {
                let item = self.media_list_wrapper.get_visible(position).unwrap();
                item.borrow_mut().set_active(is_active);
            }
        } else {
            for position in 0..self.media_list_wrapper.len() {
                let item = self.media_list_wrapper.get(position).unwrap();
                item.borrow_mut().set_active(is_active);
            }
        }
    }

    async fn on_save_selected_media(&mut self, path: &PathBuf, sender: AsyncComponentSender<Self>) {
        let mut selected_media = vec![];
        for position in 0..self.media_list_wrapper.selection_model.n_items() {
            let item = self.media_list_wrapper.get_visible(position).unwrap();
            let item = item.borrow();
            if item.is_active() {
                selected_media.push(item.media.path.clone());
            }
        }

        if selected_media.is_empty() {
            sender.input(CsamInput::ShowInfoDialog(fl!("select-media").to_string()));
            return;
        }

        let ctx = self.ctx.clone();
        let path = path.to_owned();

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    out.send(CsamCommandOutput::ShowProgressDialog(true))
                        .unwrap_or_default();

                    match ctx.csam_service.export_media(&path, &selected_media).await {
                        Ok(_) => {
                            out.send(CsamCommandOutput::Notify(
                                fl!("media-export-success").to_string(),
                                5,
                            ))
                            .unwrap_or_default();
                        }
                        Err(err) => {
                            tracing::error!("Media export error: {}", err);
                            out.send(CsamCommandOutput::Notify(
                                format!("{}: {}", fl!("media-export-error"), err),
                                5,
                            ))
                            .unwrap_or_default();
                        }
                    }

                    out.send(CsamCommandOutput::ShowProgressDialog(false))
                        .unwrap_or_default();
                })
                .drop_on_shutdown()
        });
    }

    async fn apply_media_filters(&mut self) {
        self.media_list_wrapper.set_filter_status(0, false);
        self.media_list_wrapper.set_filter_status(0, true);
    }

    async fn apply_media_zoom(&mut self, is_zoom_in: bool) {
        use models::media::THUMBNAIL_SIZE;
        use models::media::ZOOM_LIMIT;
        use models::media::ZOOM_SIZE;

        if is_zoom_in {
            if self.thumbnail_size < ZOOM_LIMIT {
                self.thumbnail_size += ZOOM_SIZE;
            }
        } else if self.thumbnail_size > THUMBNAIL_SIZE {
            let mut thumb_size = self.thumbnail_size - ZOOM_SIZE;
            if thumb_size < THUMBNAIL_SIZE {
                thumb_size = THUMBNAIL_SIZE;
            }
            self.thumbnail_size = thumb_size;
        }

        let len = self.media_list_wrapper.len();
        for position in 0..len {
            let item = self.media_list_wrapper.get(position).unwrap();
            item.borrow_mut().set_thumbnail_size(self.thumbnail_size);
        }
    }
}

fn on_filter(filter: Rc<RefCell<models::MediaFilter>>) -> impl Fn(&MediaItem) -> bool {
    move |item: &MediaItem| -> bool {
        let filter = filter.borrow();
        let media = &item.media;

        // filter by keyword
        if let Some(query) = &filter.search_entry {
            if !media.name.to_lowercase().contains(&query.to_lowercase()) {
                return false;
            }
        }

        // filter by media type
        if !filter.is_image && media.media_type == models::MediaType::Image {
            return false;
        }
        if !filter.is_video && media.media_type == models::MediaType::Video {
            return false;
        }

        // filter by CSAM file
        if filter.is_csam && media.match_type.is_empty() {
            return false;
        }

        // filter by hamming distance
        // if filter.is_csam && (media.hamming > filter.hamming_distance) {
        //     return false;
        // }
        if (media.hamming > 0) && (media.hamming > filter.hamming_distance) {
            return false;
        }

        // filter by file size
        if !filter.is_size_0 && media.size == 0 {
            return false;
        }
        if !filter.is_size_30 && (media.size > 0 && media.size <= 30) {
            return false;
        }
        if !filter.is_size_100 && (media.size > 30 && media.size <= 100) {
            return false;
        }
        if !filter.is_size_500 && (media.size > 100 && media.size <= 500) {
            return false;
        }
        if !filter.is_size_greater_500 && media.size > 500 {
            return false;
        }

        true
    }
}
