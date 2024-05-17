use std::path::PathBuf;

use num_format::ToFormattedString;
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

use crate::app::components::dialogs;
use crate::app::components::progress_dialog::{
    ProgressDialog, ProgressDialogOutput, ProgressSettings,
};
use crate::{context::AppContext, fl};

pub struct PHashDatabaseModel {
    ctx: AppContext,
    open_dialog: Controller<OpenDialog>,
    progress_dialog: Controller<ProgressDialog>,
    media_path: PathBuf,
}

#[derive(Debug)]
pub enum PHashDatabaseInput {
    OpenFileRequest,
    OpenFileResponse(PathBuf),
    GenerateDatabase,
    ShowInfoDialog(String),
    ShowProgressDialog(bool),
    GoPrevious,
    Cancel,
    Ignore,
}

#[derive(Debug)]
pub enum PHashDatabaseOutput {
    GeneratedDatabase,
    GoPrevious,
}

#[derive(Debug)]
pub enum PHashDatabaseCommandOutput {
    GeneratedDatabase,
    ShowInfoDialog(String),
    ShowProgressDialog(bool),
}

#[relm4::component(pub async)]
impl AsyncComponent for PHashDatabaseModel {
    type Init = AppContext;
    type Input = PHashDatabaseInput;
    type Output = PHashDatabaseOutput;
    type CommandOutput = PHashDatabaseCommandOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            append = &adw::HeaderBar {
                set_show_start_title_buttons: true,
                set_show_end_title_buttons: true,
                #[wrap(Some)]
                set_title_widget = &gtk::Label {
                    set_label: fl!("phash"),
                    set_css_classes: &["heading"],
                },
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",
                    set_css_classes: &["flat"],
                    set_tooltip: fl!("preferences"),
                    connect_clicked => PHashDatabaseInput::GoPrevious,
                },
            },

            append = &adw::Clamp {
                #[wrap(Some)]
                set_child = &adw::PreferencesPage {
                    set_vexpand: true,

                    add = &adw::PreferencesGroup {
                        set_title: fl!("phash"),
                        set_description: Some(fl!("add-phash-description")),

                        #[wrap(Some)]
                        set_header_suffix = &gtk::Box {
                            set_css_classes: &["linked"],
                            gtk::Button {
                                set_icon_name: icon_names::SAVE_FILLED,
                                set_css_classes: &["circular", "suggested-action"],
                                set_valign: gtk::Align::Center,
                                set_tooltip: fl!("generate-database"),
                                connect_clicked => PHashDatabaseInput::GenerateDatabase,
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
                                    connect_clicked => PHashDatabaseInput::OpenFileRequest,
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
                OpenDialogResponse::Accept(path) => PHashDatabaseInput::OpenFileResponse(path),
                OpenDialogResponse::Cancel => PHashDatabaseInput::Ignore,
            });

        let progress_settings = ProgressSettings {
            text: fl!("wait").to_string(),
            secondary_text: Some(fl!("generating-phash-database").to_string()),
            cancel_label: fl!("cancel").to_string(),
        };

        let progress_dialog = ProgressDialog::builder()
            .transient_for(&root)
            .launch(progress_settings)
            .forward(sender.input_sender(), |response| match response {
                ProgressDialogOutput::Cancel => PHashDatabaseInput::Cancel,
            });

        let model = PHashDatabaseModel {
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
            PHashDatabaseInput::OpenFileRequest => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            PHashDatabaseInput::OpenFileResponse(path) => {
                self.media_path = path;
            }
            PHashDatabaseInput::GenerateDatabase => {
                self.generate_database(sender).await;
            }
            PHashDatabaseInput::Cancel => {
                self.ctx.csam_service.cancel_task();
            }
            PHashDatabaseInput::ShowInfoDialog(msg) => {
                let window = root.toplevel_window();
                dialogs::show_info_dialog(window.as_ref(), Some(fl!("phash")), Some(&msg));
            }
            PHashDatabaseInput::ShowProgressDialog(show) => {
                if show {
                    self.progress_dialog.widget().present();
                } else {
                    self.progress_dialog.widget().close();
                }
            }
            PHashDatabaseInput::GoPrevious => {
                sender
                    .output(PHashDatabaseOutput::GoPrevious)
                    .unwrap_or_default();
            }
            PHashDatabaseInput::Ignore => (),
        }
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            PHashDatabaseCommandOutput::GeneratedDatabase => sender
                .output(PHashDatabaseOutput::GeneratedDatabase)
                .unwrap_or_default(),
            PHashDatabaseCommandOutput::ShowInfoDialog(msg) => {
                sender.input(PHashDatabaseInput::ShowInfoDialog(msg))
            }
            PHashDatabaseCommandOutput::ShowProgressDialog(show) => {
                sender.input(PHashDatabaseInput::ShowProgressDialog(show))
            }
        }
    }
}

impl PHashDatabaseModel {
    async fn generate_database(&mut self, sender: AsyncComponentSender<Self>) {
        if !self.media_path.exists() {
            sender.input(PHashDatabaseInput::ShowInfoDialog(
                fl!("msg-media-path").to_string(),
            ));
            return;
        }

        let ctx = self.ctx.clone();
        let db_path = ctx.get_preference().database_path.clone();
        let media_path = self.media_path.clone();

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    out.send(PHashDatabaseCommandOutput::ShowProgressDialog(true))
                        .unwrap_or_default();

                    match ctx
                        .csam_service
                        .create_phash_database(db_path, media_path)
                        .await
                    {
                        Ok(count) => {
                            out.send(PHashDatabaseCommandOutput::ShowInfoDialog(
                                format!(
                                    "{}: {}",
                                    fl!("total-phash-generated"),
                                    count.to_formatted_string(&ctx.get_locale())
                                )
                                .to_string(),
                            ))
                            .unwrap_or_default();

                            out.send(PHashDatabaseCommandOutput::GeneratedDatabase)
                                .unwrap_or_default();
                        }
                        Err(err) => {
                            tracing::error!(
                                "Could not generate perceptual hash database. Error: {}",
                                err
                            );
                            out.send(PHashDatabaseCommandOutput::ShowInfoDialog(
                                fl!("failed-to-generate-db").to_string(),
                            ))
                            .unwrap_or_default();
                        }
                    }

                    out.send(PHashDatabaseCommandOutput::ShowProgressDialog(false))
                        .unwrap_or_default();
                })
                .drop_on_shutdown()
        });
    }
}
