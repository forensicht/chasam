mod toolbar;

use crate::fl;
use core_chasam as service;
use core_chasam::csam::StateMedia;
use crate::app::{
    models,
    components::searchbar::{
        SearchBarModel,
        SearchBarOutput,
    },
    factories::media::{
        MediaInput, 
        MediaModel, 
        MediaOutput
    },
};
use toolbar::{
    ToolbarModel,
    ToolbarInput,
    ToolbarOutput,
};

use std::path::PathBuf;

use relm4::{
    adw, component::{
        AsyncComponent, 
        AsyncComponentController, 
        AsyncComponentParts, 
        AsyncComponentSender, 
        AsyncController,
    }, factory::AsyncFactoryVecDeque, gtk::prelude::*, prelude::*, RelmIterChildrenExt
};
use anyhow::Result;

pub struct CsamModel {
    searchbar: AsyncController<SearchBarModel>,
    toolbar: AsyncController<ToolbarModel>,
    media_list_factory: AsyncFactoryVecDeque<MediaModel>,
    media_list_filter: models::MediaFilter,
    thumbnail_size: i32,
    service: service::csam::SearchMedia,
}

impl CsamModel {
    pub fn new(
        searchbar: AsyncController<SearchBarModel>,
        toolbar: AsyncController<ToolbarModel>,
        media_list_factory: AsyncFactoryVecDeque<MediaModel>,
        service: service::csam::SearchMedia,
    ) -> Self {
        Self {
            searchbar,
            toolbar,
            media_list_factory,
            media_list_filter: models::MediaFilter::default(),
            thumbnail_size: models::media::THUMBNAIL_SIZE,
            service,
        }
    }
}

#[derive(Debug)]
pub enum CsamInput {
    // Searchbar
    StartSearch(PathBuf),
    SearchCompleted(usize),

    // Toolbar
    ZoomIn,
    ZoomOut,
    SelectAllMedias(bool),
    SizeFilter0KB(bool),
    SizeFilter30KB(bool),
    SizeFilter100KB(bool),
    SizeFilter500KB(bool),
    SizeFilterA500KB(bool),
    SearchEntry(String),

    MediaListSelect(usize),
    SelectedMedia(bool),
    Notify(String, u32),
}

#[derive(Debug)]
pub enum CsamCommandOutput {
    SearchCompleted,
    AddMedia(Result<models::Media>),
    MediaFound(usize),
}

#[relm4::component(pub async)]
impl AsyncComponent for CsamModel {
    type Init = ();
    type Input = CsamInput;
    type Output = ();
    type CommandOutput = CsamCommandOutput;

    view! {
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
                            set_resize_end_child: true,
                            set_shrink_start_child: false,
                            set_shrink_end_child: false,
                            set_margin_bottom: 6,
                            set_margin_end: 6,
                            set_margin_start: 6,

                            #[wrap(Some)]
                            set_start_child = &gtk::Frame {
                                set_width_request: 800,
                                set_vexpand: true,
                                set_margin_end: 6,

                                gtk::ScrolledWindow {
                                    set_hscrollbar_policy: gtk::PolicyType::Never,
                                    set_hexpand: true,
                                    set_vexpand: true,

                                    #[local_ref]
                                    media_list_widget -> gtk::FlowBox {
                                        set_valign: gtk::Align::Start,
                                        set_max_children_per_line: 12,
                                        set_selection_mode: gtk::SelectionMode::None,
                                        set_activate_on_single_click: true,
                                        connect_child_activated[sender] => move |_, child| {
                                            let index = child.index() as usize;
                                            sender.input(CsamInput::MediaListSelect(index));
                                        },
                                    },
                                },
                            },

                            #[wrap(Some)]
                            set_end_child = &gtk::Frame {
                                set_width_request: 300,
                                set_vexpand: true,
                                set_margin_start: 6,
                            },
                        },
                    },
                },
            },
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let searchbar_controller = SearchBarModel::builder()
            .launch(())
            .forward(sender.input_sender(), |output| match output {
                SearchBarOutput::StartSearch(path) => CsamInput::StartSearch(path),
                SearchBarOutput::Notify(msg, timeout) => CsamInput::Notify(msg, timeout),
            });

        let toolbar_controller = ToolbarModel::builder()
            .launch(())
            .forward(sender.input_sender(), |output| match output {
                ToolbarOutput::ZoomIn => CsamInput::ZoomIn,
                ToolbarOutput::ZoomOut => CsamInput::ZoomOut,
                ToolbarOutput::SelectAll(is_selected) => CsamInput::SelectAllMedias(is_selected),
                ToolbarOutput::SearchEntry(query) => CsamInput::SearchEntry(query),
                ToolbarOutput::SizeFilter0KB(is_active) => CsamInput::SizeFilter0KB(is_active),
                ToolbarOutput::SizeFilter30KB(is_active) => CsamInput::SizeFilter30KB(is_active),
                ToolbarOutput::SizeFilter100KB(is_active) => CsamInput::SizeFilter100KB(is_active),
                ToolbarOutput::SizeFilter500KB(is_active) => CsamInput::SizeFilter500KB(is_active),
                ToolbarOutput::SizeFilterGreater500KB(is_active) => CsamInput::SizeFilterA500KB(is_active),
            });

        let media_list_factory = AsyncFactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |output| match output {
                MediaOutput::Selected(is_selected) => CsamInput::SelectedMedia(is_selected),
            });

        let service = service::csam::SearchMedia::new();
        let model = CsamModel::new(
            searchbar_controller,
            toolbar_controller,
            media_list_factory,
            service,
        );
        let media_list_widget = model.media_list_factory.widget();
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            CsamInput::ZoomIn => {
                self.apply_media_zoom(true).await;
            }
            CsamInput::ZoomOut => {
                self.apply_media_zoom(false).await;
            }
            CsamInput::StartSearch(path) => {
                self.on_search(path, &sender).await;
            }
            CsamInput::SearchCompleted(count) => {
                println!("{}", count);
            }
            CsamInput::SelectAllMedias(is_selected) => {
                self.on_select_all_medias(is_selected).await;
            }
            CsamInput::SearchEntry(query) => {
                self.media_list_filter.search_entry = Some(query);
                let _affected = self.apply_media_filters().await;
            }
            CsamInput::SizeFilter0KB(is_active) => {
                self.media_list_filter.size_0 = is_active;
                let _affected = self.apply_media_filters().await;
            }
            CsamInput::SizeFilter30KB(is_active) => {
                self.media_list_filter.size_30 = is_active;
                let _affected = self.apply_media_filters().await;
            }
            CsamInput::SizeFilter100KB(is_active) => {
                self.media_list_filter.size_100 = is_active;
                let _affected = self.apply_media_filters().await;
            }
            CsamInput::SizeFilter500KB(is_active) => {
                self.media_list_filter.size_500 = is_active;
                let _affected = self.apply_media_filters().await;
            }
            CsamInput::SizeFilterA500KB(is_active) => {
                self.media_list_filter.size_greater_500 = is_active;
                let _affected = self.apply_media_filters().await;
            }
            CsamInput::MediaListSelect(index) => {
                println!("Select item: {}", index);
            }
            CsamInput::SelectedMedia(is_selected) => {
                self.toolbar.emit(ToolbarInput::SelectedItem(is_selected));
            }
            CsamInput::Notify(msg, timeout) => {
                println!("{} - {}", msg, timeout);
            }
        }   

        self.update_view(widgets, sender);
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            CsamCommandOutput::SearchCompleted => {
                println!("Search Completed");
            }
            CsamCommandOutput::MediaFound(count) => {
                println!("Media Found: {}", count);
            }
            CsamCommandOutput::AddMedia(result) => {
                match result {
                    Ok(media) => {
                        let mut guard = self.media_list_factory.guard();
                        guard.push_back(media);
                    }
                    Err(error) => tracing::error!("{}: {}", fl!("generic-error"), error),
                }
            }
        }
    }
}

impl CsamModel {
    async fn on_search(
        &mut self, 
        path: PathBuf,
        sender: &AsyncComponentSender<CsamModel>,
    ) {
        let (tx, mut rx) = relm4::tokio::sync::mpsc::channel(100);

        sender.command(|out, shutdown| {
            shutdown.register(async move {
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
                        StateMedia::Ok(media) => {
                            let media = models::Media::from(&media);
                            out.send(CsamCommandOutput::AddMedia(Ok(media)))
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

        match self.service.search(path, tx).await {
            Err(error) => tracing::error!("{error}"),
            _ => (),
        }
    }

    async fn on_select_all_medias(
        &mut self,
        is_selected: bool,
    ) {
        self.media_list_factory
            .guard()
            .iter_mut()
            .for_each(|item| {
                item.unwrap().media.is_selected = is_selected;
            });
    }

    async fn apply_media_filters(&mut self) -> usize {
        let media_widget = self.media_list_factory.widget();
        let filter = &self.media_list_filter;

        for media_model in self.media_list_factory.iter() {
            let media_model = media_model.unwrap();
            let media = &media_model.media;
            let mut is_visible = true;

            if let Some(query) = &filter.search_entry {
                is_visible = media.name.to_lowercase().contains(&query.to_lowercase());
            }

            if !filter.size_0 && media.size == 0 {
                is_visible = false;
            } else if !filter.size_30 && (media.size > 0 && media.size <= 30) {
                is_visible = false;
            } else if !filter.size_100 && (media.size > 30 && media.size <= 100) {
                is_visible = false;
            } else if !filter.size_500 && (media.size > 100 && media.size <= 500) {
                is_visible = false;
            } else if !filter.size_greater_500 && media.size > 500 {
                is_visible = false;
            }

            let index = media_model.index.current_index() as i32;
            media_widget
                .child_at_index(index)
                .as_ref()
                .unwrap()
                .set_visible(is_visible);
        }

        media_widget
            .iter_children()
            .filter(|c| c.is_visible())
            .count()
    }

    async fn apply_media_zoom(&mut self, is_zoom_in: bool) {
        use models::media::THUMBNAIL_SIZE;
        use models::media::ZOOM_SIZE;

        if is_zoom_in {
            if self.thumbnail_size < 320 {
                self.thumbnail_size += ZOOM_SIZE;
            }
        } else {
            if self.thumbnail_size > THUMBNAIL_SIZE {
                let mut thumb_size = self.thumbnail_size - ZOOM_SIZE;
                if thumb_size < THUMBNAIL_SIZE {
                    thumb_size = THUMBNAIL_SIZE;
                }
                self.thumbnail_size = thumb_size;
            }
        }

        for media_model in self.media_list_factory.iter() {
            let index = media_model.unwrap().index.current_index();
            if is_zoom_in {
                self.media_list_factory.send(index, MediaInput::ZoomIn(self.thumbnail_size));
            } else {
                self.media_list_factory.send(index, MediaInput::ZoomOut(self.thumbnail_size));
            }
        }
    }
}
