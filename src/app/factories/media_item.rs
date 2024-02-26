use std::collections::VecDeque;

use crate::app::models;
use crate::app::components::csam::toolbar::{
    SELECT_BROKER,
    ToolbarInput,
};

use relm4::{
    binding::{
        Binding, 
        BoolBinding, 
        I32Binding,
    }, 
    gtk::{
        pango, 
        prelude::*,
        gdk_pixbuf::Pixbuf,
    }, 
    prelude::*, 
    typed_view::grid::RelmGridItem, 
    RelmObjectExt,
};

#[derive(Debug)]
pub struct MediaItem {
    pub media: models::Media,
    pub active: BoolBinding,
    pub thumbnail_size: I32Binding,
}

impl MediaItem {
    pub fn new(media: models::Media) -> Self {
        Self {
            media,
            active: BoolBinding::new(false),
            thumbnail_size: I32Binding::new(models::media::THUMBNAIL_SIZE),
        }
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.media.is_selected = is_active;
        *self.active.guard() = is_active;
    }
}

pub struct Widgets {
    picture: gtk::Picture,
    checkbox: gtk::CheckButton,
    label: gtk::Label,
}

impl Drop for Widgets {
    fn drop(&mut self) {
        dbg!(self.label.label());
    }
}

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
                        connect_toggled => move |checkbox| {
                            SELECT_BROKER.send(ToolbarInput::SelectedItem(checkbox.is_active()));
                        },
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

        root.set_tooltip(self.media.name.as_str());
        // picture.add_write_only_binding(&self.thumbnail_size, "height-request");
        // picture.add_write_only_binding(&self.thumbnail_size, "width-request");
        // checkbox.add_binding(&self.active, "active");
        label.set_label(self.media.name.as_str());

        if let Some(data) = self.media.data.as_ref() {
            let pixbuf = Self::get_pixbuf(data);
            picture.set_pixbuf(pixbuf.as_ref());
        } else {
            picture.set_filename(Some(self.media.path.as_str()));
        }

        // *self.active.guard() = self.media.is_selected;
    }

    fn unbind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        // *self.active.guard() = self.media.is_selected;
        widgets.picture.set_pixbuf(None);
    }
}

impl MediaItem {
    fn get_pixbuf(data: &[u8]) -> Option<Pixbuf> {
        let bytes: VecDeque<u8> = data.into_iter().cloned().collect();
        let pixbuf = Pixbuf::from_read(bytes).ok()?;
        Some(pixbuf)
    }
}
