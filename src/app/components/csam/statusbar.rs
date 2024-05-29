use num_format::ToFormattedString;
use relm4::{
    component::{Component, ComponentParts},
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
    is_calculating: bool,
    image_found: usize,
    video_found: usize,
    suspects_found: usize,
    total_found: usize,
}

#[derive(Debug)]
pub enum StatusbarInput {
    Loading(bool),
    Calculating,
    ImageFound(usize),
    CSAMFound(usize),
    VideoFound(usize),
    TotalFound(usize),
}

#[relm4::component(pub)]
impl Component for StatusbarModel {
    type Init = AppContext;
    type Input = StatusbarInput;
    type Output = ();
    type CommandOutput = ();

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

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::End,
                set_spacing: 6,

                #[name(spinner)]
                gtk::Spinner {
                    stop: (),
                },

                gtk::Label {
                    #[watch]
                    set_label: &if model.is_loading {
                        fl!("loading").to_string()
                    } else {
                        fl!("done").to_string()
                    },
                },
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 6,
                set_margin_end: 6,
            },

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
                } else if model.is_calculating {
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
            is_calculating: false,
            image_found: 0,
            video_found: 0,
            suspects_found: 0,
            total_found: 0,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            StatusbarInput::Loading(is_loading) => {
                if is_loading {
                    self.is_loading = true;
                    self.is_calculating = false;
                    self.image_found = 0;
                    self.suspects_found = 0;
                    self.video_found = 0;
                    self.total_found = 0;
                    widgets.spinner.start();
                } else {
                    self.is_loading = false;
                    widgets.spinner.stop();
                }
            }
            StatusbarInput::Calculating => self.is_calculating = true,
            StatusbarInput::ImageFound(found) => self.image_found += found,
            StatusbarInput::CSAMFound(found) => self.suspects_found += found,
            StatusbarInput::VideoFound(found) => self.video_found += found,
            StatusbarInput::TotalFound(found) => {
                self.total_found = found;
                self.is_calculating = false;
            }
        }

        self.update_view(widgets, sender);
    }
}
