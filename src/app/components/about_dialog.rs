use crate::app::config::info::{APP_NAME, VERSION};
use crate::fl;

use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct AboutDialog {}

pub struct Widgets {
    main_window: gtk::Window,
}

impl SimpleComponent for AboutDialog {
    type Init = gtk::Window;
    type Input = ();
    type Output = ();
    type Root = ();
    type Widgets = Widgets;

    fn init_root() -> Self::Root {}

    fn init(
        main_window: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = Widgets { main_window };

        ComponentParts { model, widgets }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        let dialog = adw::AboutWindow::builder()
            .icon_name(APP_NAME)
            .application_icon(APP_NAME)
            .application_name(APP_NAME)
            .developer_name("Tiago Martins\nHericson dos Santos")
            .copyright("© 2024 Tiago Martins")
            .license_type(gtk::License::Mpl20)
            .website("https://github.com/forensicht/chasam")
            .issue_url("https://github.com/forensicht/chasam/issues")
            .version(VERSION)
            .modal(true)
            .transient_for(&widgets.main_window)
            .developers(vec![
                "Tiago Martins <tiago.tsmweb@gmail.com>",
                "Hericson dos Santos <hericson.cipol@gmail.com>",
            ])
            .artists(vec![
                "Tiago Martins <tiago.tsmweb@gmail.com>",
                "Hericson dos Santos <hericson.cipol@gmail.com>",
            ])
            .translator_credits(fl!("translators"))
            .comments(fl!("comments"))
            .build();
        dialog.present();
    }
}
