use relm4::adw::{
    self, gtk,
    prelude::{GtkWindowExt, IsA, MessageDialogExt},
};

pub fn show_info_dialog(
    root: Option<&impl IsA<gtk::Window>>,
    heading: Option<&str>,
    body: Option<&str>,
) {
    let dialog = adw::MessageDialog::new(root, heading, body);
    dialog.set_transient_for(root);
    dialog.set_modal(true);
    dialog.set_destroy_with_parent(false);
    dialog.add_response("cancel", "_OK");
    dialog.set_default_response(Some("cancel"));
    dialog.set_close_response("cancel");
    dialog.present();
}
