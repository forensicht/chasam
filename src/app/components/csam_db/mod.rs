use crate::fl;

use relm4::{
    adw, 
    component::{
        AsyncComponent, 
        AsyncComponentSender, 
        AsyncComponentParts,
    }, 
    gtk::prelude::*, 
    prelude::*,
};

pub struct CsamDBModel;

#[relm4::component(pub async)]
impl AsyncComponent for CsamDBModel {
    type Init = ();
    type Input = ();
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
                set_title_widget = &gtk::Label {
                    set_label: fl!("csam-db"),
                },
            },

            append = &adw::ToastOverlay {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_css_classes: &["view"],

                    append = &gtk::Label {
                        set_label: fl!("csam-db"),
                    },
                },
            },
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = CsamDBModel{};
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        _message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        
    }
}
