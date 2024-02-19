mod toolbar;

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
    adw, 
    prelude::*,
    gtk::prelude::*, 
    component::{
        AsyncComponent, 
        AsyncComponentController, 
        AsyncComponentParts, 
        AsyncComponentSender, 
        AsyncController,
    }, 
    factory::AsyncFactoryVecDeque, 
};

pub struct CsamModel {
    searchbar: AsyncController<SearchBarModel>,
    toolbar: AsyncController<ToolbarModel>,
    media_list_factory: AsyncFactoryVecDeque<MediaModel>,
    thumbnail_size: i32,
}

impl CsamModel {
    pub fn new(
        searchbar: AsyncController<SearchBarModel>,
        toolbar: AsyncController<ToolbarModel>,
        media_list_factory: AsyncFactoryVecDeque<MediaModel>,
    ) -> Self {
        Self {
            searchbar,
            toolbar,
            media_list_factory,
            thumbnail_size: models::media::THUMBNAIL_SIZE,
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
    SelectAllVideos(bool),
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

#[relm4::component(pub async)]
impl AsyncComponent for CsamModel {
    type Init = ();
    type Input = CsamInput;
    type Output = ();
    type CommandOutput = ();

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
                                        set_activate_on_single_click: false,
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
                ToolbarOutput::SelectAll(is_selected) => CsamInput::SelectAllVideos(is_selected),
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

        let model = CsamModel::new(
            searchbar_controller,
            toolbar_controller,
            media_list_factory,
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

            }
            CsamInput::ZoomOut => {

            }
            CsamInput::StartSearch(path) => {
                println!("{}", path.display());
            }
            CsamInput::SearchCompleted(count) => {
                println!("{}", count);
            }
            CsamInput::SelectAllVideos(is_selected) => {

            }
            CsamInput::SearchEntry(query) => {

            }
            CsamInput::SizeFilter0KB(is_active) => {

            }
            CsamInput::SizeFilter30KB(is_active) => {

            }
            CsamInput::SizeFilter100KB(is_active) => {

            }
            CsamInput::SizeFilter500KB(is_active) => {

            }
            CsamInput::SizeFilterA500KB(is_active) => {

            }
            CsamInput::MediaListSelect(index) => {

            }
            CsamInput::SelectedMedia(is_selected) => {

            }
            CsamInput::Notify(msg, timeout) => {
                println!("{} - {}", msg, timeout);
            }
        }   

        self.update_view(widgets, sender);
    }
}
