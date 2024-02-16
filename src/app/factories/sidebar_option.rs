use crate::app::models;

use relm4::{
    prelude::*,
    gtk,
    gtk::prelude::*,
    view,
    factory::{
        AsyncFactoryComponent,
        AsyncFactorySender,
        DynamicIndex,
    },
    loading_widgets::LoadingWidgets,
};

#[derive(Debug)]
pub struct SidebarOptionModel {
    pub index: DynamicIndex,
    pub sidebar_option: models::SidebarOption,
}

#[derive(Debug)]
pub enum SidebarOptionInput {
    Select,
}

#[derive(Debug)]
pub enum SidebarOptionOutput {
    Selected(models::SidebarOption),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for SidebarOptionModel {
    type Init = models::SidebarOption;
    type Input = SidebarOptionInput;
    type Output = SidebarOptionOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBoxRow {
            set_tooltip: self.sidebar_option.description().as_str(),
            set_margin_bottom: 8,
            connect_activate => SidebarOptionInput::Select,

            gtk::Box {
                gtk::Image {
                    set_icon_name: self.sidebar_option.icon(),
                    set_margin_all: 0,
                    set_css_classes: &["sidebar-icon-size"]
                }
            },
        },
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 6,

                    #[name(spinner)]
                    gtk::Spinner {
                        start: (),
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                    }
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self { 
            index: index.clone(), 
            sidebar_option: init, 
        }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncFactorySender<Self>,
    ) {
       match message {
        SidebarOptionInput::Select => {
            sender.output(SidebarOptionOutput::Selected(self.sidebar_option.clone()))
                .unwrap_or_default();
        }
       } 
    }
}
