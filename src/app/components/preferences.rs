use crate::app::{config::settings, models};
use crate::fl;

use std::path::PathBuf;
use std::str::FromStr;

use relm4::{
    adw,
    adw::prelude::{
        ActionRowExt, AdwWindowExt, BoxExt, ButtonExt, CheckButtonExt, ComboRowExt, EditableExt,
        EntryRowExt, GtkWindowExt, IsA, MessageDialogExt, OrientableExt, PreferencesGroupExt,
        PreferencesPageExt, PreferencesRowExt, WidgetExt,
    },
    component::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController, Component,
        ComponentController, Controller,
    },
    gtk, AsyncComponentSender, RelmWidgetExt,
};
use relm4_components::open_dialog::*;
use relm4_icons::icon_name;

use super::csam::database_component::CsamDatabaseComponent;

pub struct PreferencesModel {
    open_dialog: Controller<OpenDialog>,
    csam_database: AsyncController<CsamDatabaseComponent>,
    preference: models::Preference,
}

#[derive(Debug)]
pub enum PreferencesInput {
    Ignore,
    OpenFileRequest,
    OpenFileResponse(PathBuf),
    SetColorScheme(models::ColorScheme),
    SetLanguage(models::Language),
}

#[relm4::component(pub async)]
impl AsyncComponent for PreferencesModel {
    type Init = gtk::Window;
    type Input = PreferencesInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::PreferencesWindow {
            set_title: Some(fl!("preferences")),
            set_hide_on_close: true,
            set_default_size: (400, 600),
            set_resizable: false,
            set_transient_for: Some(&main_window),

            #[wrap(Some)]
            #[name(overlay)]
            set_content = &adw::ToastOverlay {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    append = &adw::HeaderBar {
                        set_show_end_title_buttons: true,
                        #[wrap(Some)]
                        #[name(view_switch_title)]
                        set_title_widget = &adw::ViewSwitcherTitle::new(),
                    },

                    #[name(view_stack)]
                    append = &adw::ViewStack {

                        add = &adw::Clamp {
                            #[wrap(Some)]
                            set_child = &adw::PreferencesPage {
                                set_vexpand: true,

                                add = &adw::PreferencesGroup {
                                    set_title: fl!("appearance"),
                                    adw::ComboRow {
                                        set_title: fl!("color-scheme"),
                                        set_model: Some(&gtk::StringList::new(&[
                                            fl!("color-scheme-light"),
                                            fl!("color-scheme-dark"),
                                            fl!("color-scheme-default"),
                                        ])),
                                        set_selected: match model.preference.color_scheme {
                                            models::ColorScheme::Light => 0,
                                            models::ColorScheme::Dark => 1,
                                            models::ColorScheme::Default => 2,
                                        },
                                        connect_selected_notify[sender] => move |combo_row| {
                                            match combo_row.selected() {
                                                0 => sender.input_sender().send(
                                                    PreferencesInput::SetColorScheme(models::ColorScheme::Light)
                                                ).unwrap_or_default(),
                                                1 => sender.input_sender().send(
                                                    PreferencesInput::SetColorScheme(models::ColorScheme::Dark)
                                                ).unwrap_or_default(),
                                                _ => sender.input_sender().send(
                                                    PreferencesInput::SetColorScheme(models::ColorScheme::Default)
                                                ).unwrap_or_default(),
                                            }
                                        },
                                    }
                                },

                                add = &adw::PreferencesGroup {
                                    set_title: fl!("language"),
                                    adw::ActionRow {
                                        set_title: fl!("englis"),
                                        add_prefix = &gtk::Box {
                                            set_halign: gtk::Align::Center,
                                            set_valign: gtk::Align::Center,
                                            append = &gtk::Image {
                                                set_width_request: 64,
                                                set_height_request: 44,
                                                set_halign: gtk::Align::Center,
                                                set_valign: gtk::Align::Center,
                                                set_resource: Some("/com/github/forensicht/ChaSAM/icons/en.png"),
                                            }
                                        },
                                        add_suffix = &gtk::Box {
                                            set_halign: gtk::Align::Center,
                                            set_valign: gtk::Align::Center,
                                            #[name = "chk_language"]
                                            append = &gtk::CheckButton {
                                                set_active: match model.preference.language {
                                                    models::Language::English => true,
                                                    _ => false,
                                                },
                                                connect_toggled[sender] => move |chk_button| {
                                                    if chk_button.is_active() {
                                                        sender
                                                            .input_sender()
                                                            .send(PreferencesInput::SetLanguage(models::Language::English))
                                                            .unwrap_or_default();
                                                    }
                                                },
                                            }
                                        },
                                    },
                                    adw::ActionRow {
                                        set_title: fl!("portuguese"),
                                        add_prefix = &gtk::Box {
                                            set_halign: gtk::Align::Center,
                                            set_valign: gtk::Align::Center,
                                            append = &gtk::Image {
                                                set_width_request: 64,
                                                set_height_request: 44,
                                                set_halign: gtk::Align::Center,
                                                set_valign: gtk::Align::Center,
                                                set_resource: Some("/com/github/forensicht/ChaSAM/icons/pt.png"),
                                            }
                                        },
                                        add_suffix = &gtk::Box {
                                            set_halign: gtk::Align::Center,
                                            set_valign: gtk::Align::Center,
                                            append = &gtk::CheckButton {
                                                set_group: Some(&chk_language),
                                                set_active: match model.preference.language {
                                                    models::Language::Portuguese => true,
                                                    _ => false,
                                                },
                                                connect_toggled[sender] => move |chk_button| {
                                                    if chk_button.is_active() {
                                                        sender
                                                            .input_sender()
                                                            .send(PreferencesInput::SetLanguage(models::Language::Portuguese))
                                                            .unwrap_or_default();
                                                    }
                                                },
                                            }
                                        },
                                    },
                                    adw::ActionRow {
                                        set_title: fl!("spanish"),
                                        add_prefix = &gtk::Box {
                                            set_halign: gtk::Align::Center,
                                            set_valign: gtk::Align::Center,
                                            append = &gtk::Image {
                                                set_width_request: 64,
                                                set_height_request: 44,
                                                set_halign: gtk::Align::Center,
                                                set_valign: gtk::Align::Center,
                                                set_resource: Some("/com/github/forensicht/ChaSAM/icons/es.png"),
                                            }
                                        },
                                        add_suffix = &gtk::Box {
                                            set_halign: gtk::Align::Center,
                                            set_valign: gtk::Align::Center,
                                            append = &gtk::CheckButton {
                                                set_group: Some(&chk_language),
                                                set_active: match model.preference.language {
                                                    models::Language::Spanish => true,
                                                    _ => false,
                                                },
                                                connect_toggled[sender] => move |chk_button| {
                                                    if chk_button.is_active() {
                                                        sender
                                                            .input_sender()
                                                            .send(PreferencesInput::SetLanguage(models::Language::Spanish))
                                                            .unwrap_or_default();
                                                    }
                                                },
                                            }
                                        },
                                    },
                                },

                                add = &adw::PreferencesGroup {
                                    set_title: fl!("database"),
                                    adw::EntryRow {
                                        set_hexpand: true,
                                        set_title: fl!("database-path"),
                                        set_show_apply_button: false,
                                        #[watch]
                                        set_text: &model.preference.database_path.to_str().unwrap_or_default(),
                                        add_suffix = &gtk::Box {
                                            set_css_classes: &["linked"],
                                            gtk::Button {
                                                set_icon_name: icon_name::FOLDER_OPEN_FILLED,
                                                set_css_classes: &["circular", "accent"],
                                                add_css_class: "circular",
                                                set_valign: gtk::Align::Center,
                                                set_tooltip: fl!("select-directory"),
                                                connect_clicked => PreferencesInput::OpenFileRequest,
                                            }
                                        },
                                    },
                                },
                            }
                        } -> {
                            set_name: Some("app"),
                            set_title: Some(fl!("application")),
                            set_icon_name: Some(icon_name::CONFIGURE),
                        },

                        add = &adw::Clamp {
                            set_child = Some(model.csam_database.widget()),
                        } -> {
                            set_name: Some("csam"),
                            set_title: Some(fl!("csam")),
                            set_icon_name: Some(icon_name::PARENT),
                        },

                    }, // adw::ViewStack

                }
            }
        }
    }

    async fn init(
        main_window: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut preference = models::Preference::default();

        if let Ok(settings_toml) = settings::get_settings() {
            let color_scheme = settings_toml.theme;
            let language = models::Language::from_str(settings_toml.language.as_str()).unwrap();
            let database_path = settings_toml.database_path.as_str();
            preference = models::Preference::new(color_scheme, language, database_path);
        }

        let open_dialog_settings = OpenDialogSettings {
            folder_mode: true,
            accept_label: String::from(fl!("open")),
            cancel_label: String::from(fl!("cancel")),
            create_folders: false,
            is_modal: true,
            filters: Vec::new(),
        };

        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(open_dialog_settings)
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => PreferencesInput::OpenFileResponse(path),
                OpenDialogResponse::Cancel => PreferencesInput::Ignore,
            });

        let csam_database_controller = CsamDatabaseComponent::builder()
            .launch(preference.clone())
            .detach();

        let model = PreferencesModel {
            open_dialog,
            csam_database: csam_database_controller,
            preference,
        };

        let widgets = view_output!();
        let view_stack = Some(&widgets.view_stack);
        widgets.view_switch_title.set_stack(view_stack);

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            PreferencesInput::OpenFileRequest => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            PreferencesInput::OpenFileResponse(path) => {
                self.preference.database_path = path;
            }
            PreferencesInput::SetColorScheme(color_scheme) => {
                settings::set_color_scheme(color_scheme);
                self.preference.color_scheme = color_scheme;
            }
            PreferencesInput::SetLanguage(language) => {
                self.preference.language = language;
                self.show_dialog(root);
            }
            PreferencesInput::Ignore => {}
        }

        match settings::save_preferences(&self.preference).await {
            Err(error) => tracing::error!("{error}"),
            _ => {}
        }

        self.update_view(widgets, sender);
    }
}

impl PreferencesModel {
    fn show_dialog(&self, root: &impl IsA<gtk::Window>) {
        let dialog = adw::MessageDialog::new(
            Some(root),
            Some(fl!("preferences")),
            Some(fl!("message-dialog")),
        );
        dialog.set_transient_for(Some(root));
        dialog.set_modal(true);
        dialog.set_destroy_with_parent(false);
        dialog.add_response("cancel", "_OK");
        dialog.set_default_response(Some("cancel"));
        dialog.set_close_response("cancel");
        dialog.present();
    }
}
