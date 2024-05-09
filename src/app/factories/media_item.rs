use std::collections::VecDeque;

use relm4::{
    binding::{Binding, BoolBinding, I32Binding},
    gtk::glib,
    gtk::{
        self,
        gdk_pixbuf::Pixbuf,
        glib::object::ObjectExt,
        pango,
        prelude::{OrientableExt, WidgetExt},
    },
    typed_view::grid::RelmGridItem,
    RelmWidgetExt,
};

use crate::app::components::csam::toolbar::{ToolbarInput, SELECT_BROKER};
use crate::app::models;

#[derive(Debug)]
pub struct MediaItem {
    pub media: models::Media,
    active: BoolBinding,
    thumbnail_size: I32Binding,
    bindings: Vec<glib::Binding>,
}

impl MediaItem {
    pub fn new(media: models::Media) -> Self {
        let active = BoolBinding::new(false);
        active.connect_notify_local(None, |value, _| {
            let new_value = value.value();
            let old_value = unsafe { value.steal_data::<bool>("value") }.unwrap_or_default();
            unsafe { value.set_data("value", new_value) };

            if new_value != old_value {
                SELECT_BROKER.send(ToolbarInput::SelectedItem(new_value));
            }
        });

        Self {
            media,
            active,
            thumbnail_size: I32Binding::new(models::media::THUMBNAIL_SIZE),
            bindings: vec![],
        }
    }

    pub fn set_active(&mut self, is_active: bool) {
        if is_active != self.active.value() {
            *self.active.guard() = is_active;
        }
    }

    pub fn set_thumbnail_size(&mut self, size: i32) {
        *self.thumbnail_size.guard() = size;
    }

    pub fn is_video(&self) -> bool {
        match self.media.media_type {
            models::media::MediaType::Video => true,
            _ => false,
        }
    }

    pub fn is_csam(&self) -> bool {
        !self.media.match_type.is_empty()
    }

    fn get_pixbuf(data: &[u8]) -> Option<Pixbuf> {
        let bytes: VecDeque<u8> = data.into_iter().cloned().collect();
        let pixbuf = Pixbuf::from_read(bytes).ok()?;
        Some(pixbuf)
    }
}

pub struct Widgets {
    picture: gtk::Picture,
    checkbox: gtk::CheckButton,
    label: gtk::Label,
}

// impl Drop for Widgets {
//     fn drop(&mut self) {
//         dbg!(self.label.label());
//     }
// }

impl RelmGridItem for MediaItem {
    type Root = gtk::Box;
    type Widgets = Widgets;

    fn setup(_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        relm4::view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_css_classes: &["card", "activatable", "media-item-box", "border-spacing"],

                #[name(picture)]
                gtk::Picture {
                    set_size_request: (models::media::THUMBNAIL_SIZE, models::media::THUMBNAIL_SIZE),
                    set_margin_all: 3,
                    set_content_fit: gtk::ContentFit::Contain,
                    set_can_shrink: true,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    #[name(checkbox)]
                    gtk::CheckButton {
                        set_halign: gtk::Align::Start,
                        set_valign: gtk::Align::Start,
                        set_css_classes: &["border-spacing"],
                    },

                    #[name(label)]
                    gtk::Label {
                        set_margin_start: 5,
                        set_hexpand: true,
                        set_halign: gtk::Align::Start,
                        set_max_width_chars: 20,
                        set_ellipsize: pango::EllipsizeMode::End,
                    }
                },
            }
        }

        let widgets = Widgets {
            picture,
            checkbox,
            label,
        };

        (root, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, root: &mut Self::Root) {
        let Widgets {
            picture,
            checkbox,
            label,
        } = widgets;
        let media = &self.media;

        root.set_tooltip(media.name.as_str());

        let binding = self
            .thumbnail_size
            .bind_property(I32Binding::property_name(), picture, "height-request")
            .sync_create()
            .build();
        self.bindings.push(binding);

        let binding = self
            .thumbnail_size
            .bind_property(I32Binding::property_name(), picture, "width-request")
            .sync_create()
            .build();
        self.bindings.push(binding);

        let binding = self
            .active
            .bind_property(BoolBinding::property_name(), checkbox, "active")
            .bidirectional()
            .sync_create()
            .build();
        self.bindings.push(binding);

        label.set_label(media.name.as_str());

        if let Some(data) = media.data.as_ref() {
            let pixbuf = Self::get_pixbuf(data);
            picture.set_pixbuf(pixbuf.as_ref());
        } else {
            picture.set_filename(Some(media.path.as_str()));
        }

        root.set_class_active("media-highlight", media.is_csam())
    }

    fn unbind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        widgets.picture.set_pixbuf(None);

        for binding in self.bindings.drain(..) {
            binding.unbind();
        }
    }
}
