use relm4::{
    gtk,
    gtk::glib,
    adw::{gdk, gio, prelude::ApplicationExt},
    main_adw_application,
};
use anyhow::{Result, Ok};

use super::info::{APP_ID, APP_NAME};

pub(crate) fn init() -> Result<()> {
    glib::set_application_name(APP_NAME);
    gio::resources_register_include!("resources.gresource")?;

    if let Some(display) = gdk::Display::default() {
        gtk::IconTheme::for_display(&display)
        .add_resource_path("/com/github/forensicht/ChaSAM/icons");
    }
    gtk::Window::set_default_icon_name(APP_ID);

    let app = main_adw_application();
    app.set_resource_base_path(Some("/com/github/forensicht/ChaSAM/app"));
    
    Ok(())
}
