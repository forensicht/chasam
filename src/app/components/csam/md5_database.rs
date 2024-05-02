use std::path::PathBuf;

use relm4::{
    adw::{
        self,
        prelude::{
            BoxExt, ButtonExt, EditableExt, EntryRowExt, GtkWindowExt, OrientableExt,
            PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt, WidgetExt,
        },
    },
    component::{AsyncComponent, AsyncComponentParts, Component, Controller},
    gtk, AsyncComponentSender, ComponentController, RelmWidgetExt,
};
use relm4_components::open_dialog::*;
use relm4_icons::icon_names;

use crate::app::components::progress_dialog::{
    ProgressDialog, ProgressDialogOutput, ProgressSettings,
};
use crate::app::{components::dialogs, config::settings, models};
use crate::{context::AppContext, fl};

pub struct MD5DatabaseModel {
    ctx: AppContext,
    open_dialog: Controller<OpenDialog>,
    progress_dialog: Controller<ProgressDialog>,
    media_path: PathBuf,
}

#[derive(Debug)]
pub enum MD5DatabaseInput {
    OpenFileRequest,
    OpenFileResponse(PathBuf),
    GenerateDatabase,
    GoPrevious,
    Cancel,
    Ignore,
}

#[derive(Debug)]
pub enum MD5DatabaseOutput {
    GeneratedDatabase,
    GoPrevious,
}

#[relm4::component(pub async)]
impl AsyncComponent for MD5DatabaseModel {
    type Init = AppContext;
    type Input = MD5DatabaseInput;
    type Output = MD5DatabaseOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            append = &adw::HeaderBar {
                set_show_start_title_buttons: true,
                set_show_end_title_buttons: true,
                #[wrap(Some)]
                set_title_widget = &gtk::Label {
                    set_label: fl!("hash"),
                    set_css_classes: &["heading"],
                },
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",
                    set_css_classes: &["flat"],
                    set_tooltip: fl!("preferences"),
                    connect_clicked => MD5DatabaseInput::GoPrevious,
                },
            },

            append = &adw::Clamp {
                #[wrap(Some)]
                set_child = &adw::PreferencesPage {
                    set_vexpand: true,

                    add = &adw::PreferencesGroup {
                        set_title: fl!("hash"),
                        set_description: Some(fl!("add-hash-description")),

                        #[wrap(Some)]
                        set_header_suffix = &gtk::Box {
                            set_css_classes: &["linked"],
                            gtk::Button {
                                set_icon_name: icon_names::SAVE_FILLED,
                                set_css_classes: &["circular", "suggested-action"],
                                set_valign: gtk::Align::Center,
                                set_tooltip: fl!("generate-database"),
                                connect_clicked => MD5DatabaseInput::GenerateDatabase,
                            },
                        },

                        adw::EntryRow {
                            set_hexpand: true,
                            set_title: fl!("media-path"),
                            set_show_apply_button: false,
                            #[watch]
                            set_text: &model.media_path.to_str().unwrap_or_default(),
                            add_suffix = &gtk::Box {
                                set_css_classes: &["linked"],
                                gtk::Button {
                                    set_icon_name: icon_names::FOLDER_OPEN_FILLED,
                                    set_css_classes: &["circular", "accent"],
                                    set_valign: gtk::Align::Center,
                                    set_tooltip: fl!("select-directory"),
                                    connect_clicked => MD5DatabaseInput::OpenFileRequest,
                                }
                            },
                        },
                    },
                }
            },
        }
    }

    async fn init(
        ctx: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
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
                OpenDialogResponse::Accept(path) => MD5DatabaseInput::OpenFileResponse(path),
                OpenDialogResponse::Cancel => MD5DatabaseInput::Ignore,
            });

        let progress_settings = ProgressSettings {
            text: String::from(fl!("wait")),
            secondary_text: Some(String::from(fl!("generating-hash-database"))),
            cancel_label: String::from(fl!("cancel")),
        };

        let progress_dialog = ProgressDialog::builder()
            .transient_for(&root)
            .launch(progress_settings)
            .forward(sender.input_sender(), |response| match response {
                ProgressDialogOutput::Cancel => MD5DatabaseInput::Cancel,
            });

        let model = MD5DatabaseModel {
            ctx,
            open_dialog,
            progress_dialog,
            media_path: PathBuf::default(),
        };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            MD5DatabaseInput::OpenFileRequest => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            MD5DatabaseInput::OpenFileResponse(path) => {
                self.media_path = path;
            }
            MD5DatabaseInput::GenerateDatabase => {
                let window = root.toplevel_window();
                self.generate_database(window, &sender).await;
            }
            MD5DatabaseInput::Cancel => {
                self.ctx.csam_service.cancel_task();
            }
            MD5DatabaseInput::GoPrevious => {
                sender
                    .output(MD5DatabaseOutput::GoPrevious)
                    .unwrap_or_default();
            }
            MD5DatabaseInput::Ignore => {}
        }
    }
}

impl MD5DatabaseModel {
    async fn generate_database(
        &mut self,
        window: Option<gtk::Window>,
        sender: &AsyncComponentSender<Self>,
    ) {
        if !self.media_path.exists() {
            dialogs::show_info_dialog(
                window.as_ref(),
                Some(fl!("hash")),
                Some(fl!("msg-media-path")),
            );
            return;
        }

        let progress_dialog = self.progress_dialog.widget();
        progress_dialog.present();

        let db_path = {
            let preference = match settings::PREFERENCES.lock() {
                Ok(preference) => preference.clone(),
                _ => models::Preference::default(),
            };
            preference.database_path.clone()
        };
        let media_path = self.media_path.clone();

        match self
            .ctx
            .csam_service
            .create_hash_database(db_path, media_path)
            .await
        {
            Ok(count) => {
                progress_dialog.close();
                dialogs::show_info_dialog(
                    window.as_ref(),
                    Some(fl!("hash")),
                    Some(&format!("{}: {}", fl!("total-hash-generated"), count)),
                );
                sender
                    .output(MD5DatabaseOutput::GeneratedDatabase)
                    .unwrap_or_default();
            }
            Err(err) => {
                tracing::error!("Could not generate MD5 hash database. Error: {}", err);
                dialogs::show_info_dialog(
                    window.as_ref(),
                    Some(fl!("hash")),
                    Some(fl!("failed-to-generate-db")),
                );
            }
        }
    }
}
