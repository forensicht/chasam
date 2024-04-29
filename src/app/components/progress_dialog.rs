use relm4::{
    adw,
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    Component, ComponentParts, ComponentSender,
};

use crate::fl;

#[derive(Debug)]
pub struct ProgressSettings {
    pub text: String,
    pub secondary_text: Option<String>,
    pub cancel_label: String,
}

impl Default for ProgressSettings {
    fn default() -> Self {
        Self {
            text: String::from(fl!("progress")),
            secondary_text: None,
            cancel_label: String::from(fl!("cancel")),
        }
    }
}

#[derive(Debug)]
pub struct ProgressDialog {
    settings: ProgressSettings,
}

#[derive(Debug)]
pub enum ProgressDialogInput {
    Cancel,
}

#[derive(Debug)]
pub enum ProgressDialogOutput {
    Cancel,
}

#[relm4::component(pub)]
impl Component for ProgressDialog {
    type Init = ProgressSettings;
    type Input = ProgressDialogInput;
    type Output = ProgressDialogOutput;
    type CommandOutput = ();

    view! {
        #[root]
        adw::Window {
            set_default_size: (300, 180),
            set_hide_on_close: true,
            set_modal: true,
            set_resizable: false,
            set_deletable: false,

            gtk::CenterBox {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_bottom: 5,
                set_margin_end: 5,
                set_margin_start: 5,
                set_margin_top: 10,

                #[wrap(Some)]
                set_start_widget = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,

                    gtk::Label {
                        set_label: &model.settings.text,
                        set_css_classes: &["heading"],
                    },

                    gtk::Label {
                        set_visible: model.settings.secondary_text.is_some(),
                        set_label?: model.settings.secondary_text.as_ref(),
                    },
                },

                #[wrap(Some)]
                set_center_widget = &gtk::Spinner {
                    set_margin_bottom: 10,
                    set_margin_top: 10,
                    start: (),
                    set_size_request: (30, 30),
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                },

                #[wrap(Some)]
                set_end_widget = &gtk::Button {
                    set_label: &model.settings.cancel_label,
                    add_css_class: "destructive-action",
                    connect_clicked => ProgressDialogInput::Cancel,
                },
            },
        }
    }

    fn init(
        settings: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProgressDialog { settings };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            ProgressDialogInput::Cancel => {
                sender
                    .output(ProgressDialogOutput::Cancel)
                    .unwrap_or_default();
                root.close();
            }
        }
    }
}
