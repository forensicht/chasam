use crate::fl;
use chrono::prelude::*;
use relm4::{
    component::{ComponentParts, SimpleComponent},
    gtk::{
        self, glib,
        prelude::{BoxExt, GestureExt, GridExt, WidgetExt},
    },
    ComponentSender,
};

use crate::app::models;

pub struct MediaDetailsModel {
    media: Option<models::MediaDetail>,
}

pub struct MediaDetailsWidgets {
    picture: gtk::Picture,
    name: gtk::Label,
    path: gtk::Label,
    media_type: gtk::Label,
    size: gtk::Label,
    last_modified: gtk::Label,
    hash: gtk::Label,
    phash: gtk::Label,
    match_type: gtk::Label,
    hamming: gtk::Label,
}

#[derive(Debug)]
pub enum MediaDetailsInput {
    OpenMedia,
    ShowMedia(models::MediaDetail),
}

#[derive(Debug)]
pub enum MediaDetailsOutput {
    Notify(String, u32),
}

impl SimpleComponent for MediaDetailsModel {
    type Init = ();
    type Input = MediaDetailsInput;
    type Output = MediaDetailsOutput;
    type Root = gtk::Box;
    type Widgets = MediaDetailsWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_bottom(6)
            .margin_end(6)
            .margin_start(6)
            .margin_top(6)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let scroll_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .hexpand(true)
            .vexpand(true)
            .build();

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .build();

        let picture = gtk::Picture::builder()
            .width_request(320)
            .height_request(-1)
            .content_fit(gtk::ContentFit::Contain)
            .can_shrink(true)
            .cursor(
                gtk::gdk::Cursor::from_name("pointer", None)
                    .as_ref()
                    .unwrap(),
            )
            .build();

        let picture_gesture = gtk::GestureClick::new();
        picture_gesture.connect_released(glib::clone!(@strong sender => move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Denied);
            sender.input(MediaDetailsInput::OpenMedia)
        }));
        picture.add_controller(picture_gesture);

        let grid = gtk::Grid::builder()
            .column_spacing(12)
            .row_spacing(6)
            .build();

        // Media::name
        let lname = gtk::Label::builder()
            .label(fl!("media-name"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let name = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .single_line_mode(false)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::Char)
            .build();

        // Media::path
        let lpath = gtk::Label::builder()
            .label(fl!("media-path"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let path = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .single_line_mode(false)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::Char)
            .build();

        // Media::media_type
        let lmedia_type = gtk::Label::builder()
            .label(fl!("media-type"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let media_type = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        // Media::size
        let lsize = gtk::Label::builder()
            .label(fl!("media-size"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let size = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        // Media::last_modified
        let llast_modified = gtk::Label::builder()
            .label(fl!("media-last-modified"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let last_modified = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        // Media::hash
        let lhash = gtk::Label::builder()
            .label(fl!("media-hash"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let hash = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        // Media::phash
        let lphash = gtk::Label::builder()
            .label(fl!("media-phash"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let phash = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        // Media::match_type
        let lmatch_type = gtk::Label::builder()
            .label(fl!("media-match-type"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let match_type = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        // Media::hamming
        let lhamming = gtk::Label::builder()
            .label(fl!("media-hamming-distance"))
            .halign(gtk::Align::Start)
            .css_classes(["key-label"])
            .build();

        let hamming = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        root.append(&scroll_window);
        scroll_window.set_child(Some(&vbox));
        vbox.append(&picture);
        vbox.append(&grid);
        grid.attach(&lname, 0, 0, 1, 1);
        grid.attach(&name, 1, 0, 1, 1);
        grid.attach(&lpath, 0, 1, 1, 1);
        grid.attach(&path, 1, 1, 1, 1);
        grid.attach(&lmedia_type, 0, 2, 1, 1);
        grid.attach(&media_type, 1, 2, 1, 1);
        grid.attach(&lsize, 0, 3, 1, 1);
        grid.attach(&size, 1, 3, 1, 1);
        grid.attach(&llast_modified, 0, 4, 1, 1);
        grid.attach(&last_modified, 1, 4, 1, 1);
        grid.attach(&lhash, 0, 5, 1, 1);
        grid.attach(&hash, 1, 5, 1, 1);
        grid.attach(&lphash, 0, 6, 1, 1);
        grid.attach(&phash, 1, 6, 1, 1);
        grid.attach(&lmatch_type, 0, 7, 1, 1);
        grid.attach(&match_type, 1, 7, 1, 1);
        grid.attach(&lhamming, 0, 8, 1, 1);
        grid.attach(&hamming, 1, 8, 1, 1);

        let model = MediaDetailsModel { media: None };
        let widgets = MediaDetailsWidgets {
            picture,
            name,
            path,
            media_type,
            size,
            last_modified,
            hash,
            phash,
            match_type,
            hamming,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            MediaDetailsInput::OpenMedia => {
                if let Some(media) = self.media.as_ref() {
                    match open::that(&media.path) {
                        Err(_) => {
                            let msg =
                                format!("{} {}", fl!("open-media-error"), media.name.as_str());
                            sender
                                .output(MediaDetailsOutput::Notify(msg, 3))
                                .unwrap_or_default();
                        }
                        _ => {}
                    }
                }
            }
            MediaDetailsInput::ShowMedia(media) => self.media = Some(media),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        let MediaDetailsWidgets {
            picture,
            name,
            path,
            media_type,
            size,
            last_modified,
            hash,
            phash,
            match_type,
            hamming,
        } = widgets;

        let media = self.media.as_ref().unwrap();
        picture.set_filename(if media.path.is_empty() {
            None
        } else {
            Some(&media.path)
        });

        name.set_label(&media.name);
        path.set_label(&media.path);
        media_type.set_label(&media.media_type);

        let media_size = if media.size > 1024 {
            format!("{:.2} MB", (media.size / 1024) as f64)
        } else {
            format!("{} KB", media.size)
        };
        size.set_label(&media_size);

        let date_time = Local.timestamp_opt(media.last_modified, 0);
        let media_last_modified = if let Some(date_time) = date_time.single() {
            date_time.format("%d/%m/%Y %H:%M:%S").to_string()
        } else {
            String::new()
        };
        last_modified.set_label(&media_last_modified);

        hash.set_label(&media.hash);
        phash.set_label(&format!("{:X}", media.phash));
        match_type.set_label(&media.match_type);

        let media_hamming = if media.hamming > 0 {
            media.hamming.to_string()
        } else {
            String::new()
        };
        hamming.set_label(&media_hamming);
    }
}
