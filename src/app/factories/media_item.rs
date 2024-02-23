use crate::app::models;

use crate::app::typed_view::grid::RelmGridItem;

use relm4::{
    binding::{BoolBinding, I32Binding}, 
    gtk::{
        pango, 
        prelude::*,
    }, 
    prelude::*, 
    // typed_view::grid::RelmGridItem, 
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
}

pub struct Widgets {
    overlay: gtk::Overlay,
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
                set_css_classes: &["card", "media-item-box", "border-spacing"],
    
                #[name(overlay)]
                gtk::Overlay {
                    set_size_request: (models::media::THUMBNAIL_SIZE, models::media::THUMBNAIL_SIZE),
    
                    #[name(picture)]
                    add_overlay = &gtk::Picture {
                        set_margin_all: 3,
                        set_content_fit: gtk::ContentFit::Contain,
                        set_can_shrink: true,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                    },
    
                    #[name(checkbox)]
                    add_overlay = &gtk::CheckButton {
                        set_halign: gtk::Align::Start,
                        set_valign: gtk::Align::Start,
                        set_css_classes: &["border-spacing"],
                    },
                },
    
                #[name(label)]
                gtk::Label {
                    set_margin_all: 2,
                    set_hexpand: true,
                    set_halign: gtk::Align::Fill,
                    set_max_width_chars: 25,
                    set_ellipsize: pango::EllipsizeMode::End,
                }
            }
        }

        let widgets = Widgets {
            overlay,
            picture,
            checkbox,
            label,
        };

        (root, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, root: &mut Self::Root) {
        let Widgets { 
            overlay, 
            picture, 
            checkbox, 
            label,
        } = widgets;

        root.set_tooltip(self.media.name.as_str());
        overlay.set_size_request(self.media.thumbnail_size, self.media.thumbnail_size);
        overlay.add_write_only_binding(&self.thumbnail_size, "height-request");
        overlay.add_write_only_binding(&self.thumbnail_size, "width-request");
        picture.set_filename(Some(self.media.thumb_path.as_str()));
        checkbox.add_binding(&self.active, "active");
        label.set_label(self.media.name.as_str());
    }
}
