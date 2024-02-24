use crate::app::models;
use crate::app::components::csam::toolbar::{
    SELECT_BROKER,
    ToolbarInput,
};

use relm4::{
    binding::{BoolBinding, I32Binding}, 
    gtk::{
        pango, 
        prelude::*,
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
                        #[block_signal(toggle_handler)]
                        set_active: false,
                        connect_toggled => move |checkbox| {
                            SELECT_BROKER.send(ToolbarInput::SelectedItem(checkbox.is_active()));
                        } @toggle_handler,
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
        picture.add_write_only_binding(&self.thumbnail_size, "height-request");
        picture.add_write_only_binding(&self.thumbnail_size, "width-request");
        picture.set_filename(Some(self.media.thumb_path.as_str()));
        checkbox.add_binding(&self.active, "active");
        label.set_label(self.media.name.as_str());
    }
}
