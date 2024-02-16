use crate::app::models::SidebarOption;

use relm4::{
    prelude::*,
    gtk::prelude::*,
    adw,
    component::{
        AsyncComponent,
        AsyncComponentParts,
    },
    AsyncComponentSender,
};

pub struct ContentModel {
    sidebar_option: SidebarOption,
}

#[derive(Debug)]
pub enum ContentInput {
    SelectSidebarOption(SidebarOption),
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
    type Init = ();
    type Input = ContentInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_hexpand: true,

            #[transition = "Crossfade"]
            append = match model.sidebar_option {
                SidebarOption::CSAM => {
                    adw::HeaderBar {
                        set_hexpand: true,
                        set_css_classes: &["flat"],
                        set_show_start_title_buttons: false,
                        set_show_end_title_buttons: true,

                        #[wrap(Some)]
                        set_title_widget = &gtk::Label {
                            set_hexpand: true,
                            #[watch]
                            set_label: model.sidebar_option.name().as_str(),
                        }, 
                    } 
                },
                SidebarOption::Face => { 
                    adw::HeaderBar {
                        set_hexpand: true,
                        set_css_classes: &["flat"],
                        set_show_start_title_buttons: false,
                        set_show_end_title_buttons: true,

                        #[wrap(Some)]
                        set_title_widget = &gtk::Label {
                            set_hexpand: true,
                            #[watch]
                            set_label: model.sidebar_option.name().as_str(),
                        }, 
                    }
                },
                SidebarOption::DB => {
                    adw::HeaderBar {
                        set_hexpand: true,
                        set_css_classes: &["flat"],
                        set_show_start_title_buttons: false,
                        set_show_end_title_buttons: true,

                        #[wrap(Some)]
                        set_title_widget = &gtk::Label {
                            set_hexpand: true,
                            #[watch]
                            set_label: model.sidebar_option.name().as_str(),
                        }, 
                    }
                },
            }
        } 
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = ContentModel {
            sidebar_option: SidebarOption::CSAM,
        };
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
            ContentInput::SelectSidebarOption(sidebar_option) => {
                self.sidebar_option = sidebar_option;
            }
        }

        self.update_view(widgets, sender);
    }
}
