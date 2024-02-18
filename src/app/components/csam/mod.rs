mod toolbar;

use crate::app::components::searchbar::{
    SearchBarModel,
    SearchBarOutput,
};
use toolbar::{
    ToolbarModel,
    ToolbarInput,
    ToolbarOutput,
};

use std::path::PathBuf;

use relm4::{
    adw, 
    component::{
        AsyncComponent, 
        AsyncComponentSender, 
        AsyncComponentParts, 
        AsyncController,
        AsyncComponentController,
    }, 
    gtk::prelude::*, 
    prelude::*,
};

pub struct CsamModel {
    searchbar: AsyncController<SearchBarModel>,
    toolbar: AsyncController<ToolbarModel>,
}

impl CsamModel {
    pub fn new(
        searchbar: AsyncController<SearchBarModel>,
        toolbar: AsyncController<ToolbarModel>,
    ) -> Self {
        Self {
            searchbar,
            toolbar,
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
    SelectedItem(bool),
    SizeFilter0KB(bool),
    SizeFilter30KB(bool),
    SizeFilter100KB(bool),
    SizeFilter500KB(bool),
    SizeFilterA500KB(bool),
    SearchEntry(String),

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

        let model = CsamModel::new(
            searchbar_controller,
            toolbar_controller,
        );
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
            CsamInput::SelectedItem(is_selected) => {

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
            CsamInput::Notify(msg, timeout) => {
                println!("{} - {}", msg, timeout);
            }
        }   

        self.update_view(widgets, sender);
    }
}
