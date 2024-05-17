use num_format::ToFormattedString;
use relm4::{
    component::{ComponentParts, SimpleComponent},
    gtk::{
        self,
        prelude::{BoxExt, OrientableExt, WidgetExt},
    },
    ComponentSender,
};

use crate::{context::AppContext, fl};

pub struct StatusbarModel {
    ctx: AppContext,
    is_loading: bool,
    image_found: usize,
    video_found: usize,
    suspects_found: usize,
    total_found: usize,
}

#[derive(Debug)]
pub enum StatusbarInput {
    Reset,
    Loading,
    ImageFound(usize),
    CSAMFound(usize),
    VideoFound(usize),
    TotalFound(usize),
}

#[relm4::component(pub)]
impl SimpleComponent for StatusbarModel {
    type Init = AppContext;
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
                    model.total_found.to_formatted_string(&model.ctx.get_locale())
                } else if model.is_loading {
                    fl!("calculating").to_string()
                } else {
                    String::from("0")
                },
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{}: {}", fl!("images"),
                    model.image_found.to_formatted_string(&model.ctx.get_locale())),
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{}: {}", fl!("videos"),
                    model.video_found.to_formatted_string(&model.ctx.get_locale())),
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{}: {}", fl!("suspects-files"),
                    model.suspects_found.to_formatted_string(&model.ctx.get_locale())),
                set_css_classes: &["color-red"],
            },
        }
    }

    fn init(
        ctx: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = StatusbarModel {
            ctx,
            is_loading: false,
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
                self.is_loading = false;
                self.image_found = 0;
                self.suspects_found = 0;
                self.video_found = 0;
                self.total_found = 0;
            }
            StatusbarInput::Loading => self.is_loading = true,
            StatusbarInput::ImageFound(found) => self.image_found += found,
            StatusbarInput::CSAMFound(found) => self.suspects_found += found,
            StatusbarInput::VideoFound(found) => self.video_found += found,
            StatusbarInput::TotalFound(found) => {
                self.total_found = found;
                self.is_loading = false;
            }
        }
    }
}
