use std::collections::VecDeque;

use relm4::{
    component::{Component, ComponentParts},
    gtk::gdk::gdk_pixbuf,
    gtk::{
        self,
        prelude::{BoxExt, GestureExt, GridExt, OrientableExt, WidgetExt},
    },
    ComponentSender, RelmWidgetExt,
};

use crate::app::models;
use crate::fl;

pub struct MediaDetailsModel {
    media: models::MediaDetail,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
    is_visible: bool,
}

#[derive(Debug)]
pub enum MediaDetailsInput {
    OpenMedia,
    ShowMedia(models::MediaDetail),
    Reset,
}

#[derive(Debug)]
pub enum MediaDetailsOutput {
    Notify(String, u32),
}

#[relm4::component(pub)]
impl Component for MediaDetailsModel {
    type Init = models::MediaDetail;
    type Input = MediaDetailsInput;
    type Output = MediaDetailsOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 6,

            gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_hexpand: true,
                set_vexpand: true,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    #[watch]
                    set_visible: model.is_visible,

                    gtk::Picture {
                        #[watch]
                        set_pixbuf: model.pixbuf.as_ref(),
                        set_width_request: 320,
                        set_height_request: -1,
                        set_content_fit: gtk::ContentFit::Contain,
                        set_can_shrink: true,
                        set_cursor: gtk::gdk::Cursor::from_name("pointer", None).as_ref(),
                        add_controller = gtk::GestureClick {
                            connect_released[sender] => move |gesture, _, _, _| {
                                gesture.set_state(gtk::EventSequenceState::Denied);
                                sender.input(MediaDetailsInput::OpenMedia)
                            }
                        }
                    },

                    gtk::Grid {
                        set_column_spacing: 12,
                        set_row_spacing: 6,

                        attach[0, 0, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("name")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 0, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.name,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                            set_single_line_mode: false,
                            set_wrap: true,
                            set_wrap_mode: gtk::pango::WrapMode::Char,
                        },
                        attach[0, 1, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("path")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 1, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.path,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                            set_single_line_mode: false,
                            set_wrap: true,
                            set_wrap_mode: gtk::pango::WrapMode::Char,
                        },
                        attach[0, 2, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("type")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 2, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.media_type.name(),
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },
                        attach[0, 3, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("size")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 3, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.size,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },
                        attach[0, 4, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("last-modified")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 4, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.last_modified,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },
                        attach[0, 5, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("hash")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 5, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.hash,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },
                        attach[0, 6, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("phash")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 6, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.phash,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },
                        attach[0, 7, 1, 1] = &gtk::Label {
                            set_label: &format!("{}:", fl!("match-type")),
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["key-label"],
                        },
                        attach[1, 7, 1, 1] = &gtk::Label {
                            #[watch]
                            set_label: &model.media.match_type,
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },
                        // attach[0, 8, 1, 1] = &gtk::Label {
                        //     set_label: &format!("{}:", fl!("hamming-distance")),
                        //     set_halign: gtk::Align::Start,
                        //     set_css_classes: &["key-label"],
                        // },
                        // attach[1, 8, 1, 1] = &gtk::Label {
                        //     #[watch]
                        //     set_label: &model.media.hamming,
                        //     set_halign: gtk::Align::Start,
                        //     set_hexpand: true,
                        // },
                    },
                },
            },
        },
    }

    fn init(
        media: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MediaDetailsModel {
            media,
            pixbuf: None,
            is_visible: false,
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
            MediaDetailsInput::OpenMedia => {
                if open::that(&self.media.path).is_err() {
                    let msg = format!("{} {}", fl!("open-media-error"), self.media.name.as_str());
                    sender
                        .output(MediaDetailsOutput::Notify(msg, 5))
                        .unwrap_or_default();
                }
            }
            MediaDetailsInput::ShowMedia(media) => {
                self.pixbuf = match media.media_type {
                    models::MediaType::Image => gdk_pixbuf::Pixbuf::from_file(&media.path).ok(),
                    models::MediaType::Video => {
                        if let Some(ref img_buf) = media.img_buf {
                            let bytes: VecDeque<_> = img_buf.iter().cloned().collect();
                            gdk_pixbuf::Pixbuf::from_read(bytes).ok()
                        } else {
                            None
                        }
                    }
                };
                self.media = media;
                self.is_visible = true;
            }
            MediaDetailsInput::Reset => {
                self.media = models::MediaDetail::default();
                self.pixbuf = None;
                self.is_visible = false;
            }
        }

        self.update_view(widgets, sender);
    }
}
