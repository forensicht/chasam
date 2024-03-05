use crate::fl;
use relm4::{
    component::{ComponentParts, SimpleComponent},
    gtk::{
        self,
        prelude::{BoxExt, OrientableExt, WidgetExt},
    },
    ComponentSender,
};

pub struct StatusbarModel {
    image_found: usize,
    video_found: usize,
    suspects_found: usize,
    total_found: usize,
}

#[derive(Debug)]
pub enum StatusbarInput {
    Reset,
    ImageFound(usize),
    CSAMFound(usize),
    VideoFound(usize),
    TotalFound(usize),
}

#[relm4::component(pub)]
impl SimpleComponent for StatusbarModel {
    type Init = ();
    type Input = StatusbarInput;
    type Output = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_hexpand: true,
            set_margin_bottom: 4,
            set_margin_end: 6,
            set_margin_start: 6,
            set_margin_top: 4,
            set_halign: gtk::Align::Start,
            set_spacing: 6,

            gtk::Label {
                set_label: fl!("processed-files"),
            },

            #[name(label_media_found)]
            gtk::Label {
                set_label: "0",
            },

            gtk::Label {
                set_label: fl!("out-of"),
            },

            gtk::Label {
                #[watch]
                set_label: &if model.total_found > 0 {
                    model.total_found.to_string()
                } else {
                    fl!("calculating").to_string()
                },
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{}: {}", fl!("images"), model.image_found),
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{}: {}", fl!("videos"), model.video_found),
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{}: {}", fl!("suspects-files"), model.suspects_found),
                set_css_classes: &["color-red"],
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = StatusbarModel {
            image_found: 0,
            video_found: 0,
            suspects_found: 0,
            total_found: 0,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            StatusbarInput::Reset => {
                self.image_found = 0;
                self.suspects_found = 0;
                self.video_found = 0;
                self.total_found = 0;
            }
            StatusbarInput::ImageFound(found) => self.image_found += found,
            StatusbarInput::CSAMFound(found) => self.suspects_found += found,
            StatusbarInput::VideoFound(found) => self.video_found = found,
            StatusbarInput::TotalFound(found) => self.total_found = found,
        }
    }
}
