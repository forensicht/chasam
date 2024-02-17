use std::path::PathBuf;

use crate::app::components::searchbar::{
    SearchBarModel,
    SearchBarOutput,
};

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
}

impl CsamModel {
    pub fn new(
        searchbar: AsyncController<SearchBarModel>,
    ) -> Self {
        Self {
            searchbar,
        }
    }
}

#[derive(Debug)]
pub enum CsamInput {
    StartSearch(PathBuf),
    SearchCompleted(usize),
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

            append = &adw::HeaderBar {
                set_hexpand: true,
                set_css_classes: &["flat"],
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: true,

                #[wrap(Some)]
                set_title_widget = model.searchbar.widget(),
            },

            append = &adw::ToastOverlay {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_css_classes: &["view"],

                    append = &gtk::Label {
                        set_label: "Content CSAM",
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

        let model = CsamModel::new(searchbar_controller);
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
            CsamInput::StartSearch(path) => {
                println!("{}", path.display());
            }
            CsamInput::SearchCompleted(count) => {
                println!("{}", count);
            }
            CsamInput::Notify(msg, timeout) => {
                println!("{} - {}", msg, timeout);
            }
        }   

        self.update_view(widgets, sender);
    }
}
