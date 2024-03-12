use crate::app::models;
use crate::fl;

use std::path::PathBuf;

use relm4::{
    adw,
    adw::prelude::{
        BoxExt, ButtonExt, EditableExt, EntryRowExt, OrientableExt, PreferencesGroupExt,
        PreferencesPageExt, PreferencesRowExt, WidgetExt,
    },
    component::{AsyncComponent, AsyncComponentParts, Component, Controller},
    gtk, AsyncComponentSender, ComponentController, RelmWidgetExt,
};
use relm4_components::open_dialog::*;
use relm4_icons::icon_name;

pub struct PHashDatabaseModel {
    open_dialog: Controller<OpenDialog>,
    media_path: PathBuf,
    preference: models::Preference,
}

#[derive(Debug)]
pub enum PHashDatabaseInput {
    OpenFileRequest,
    OpenFileResponse(PathBuf),
    GoPrevious,
    Ignore,
}

#[derive(Debug)]
pub enum PHashDatabaseOutput {
    GoPrevious,
}

#[relm4::component(pub async)]
impl AsyncComponent for PHashDatabaseModel {
    type Init = models::Preference;
    type Input = PHashDatabaseInput;
    type Output = PHashDatabaseOutput;
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
                                set_icon_name: icon_name::SAVE_FILLED,
                                set_css_classes: &["circular", "suggested-action"],
                                set_valign: gtk::Align::Center,
                                set_tooltip: fl!("generate-database"),
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
                                    set_icon_name: icon_name::FOLDER_OPEN_FILLED,
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
        preference: Self::Init,
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

        let model = PHashDatabaseModel {
            open_dialog,
            media_path: PathBuf::default(),
            preference,
        };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            PHashDatabaseInput::OpenFileRequest => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            PHashDatabaseInput::OpenFileResponse(path) => {
                self.media_path = path;
            }
            PHashDatabaseInput::GoPrevious => {
                sender
                    .output(PHashDatabaseOutput::GoPrevious)
                    .unwrap_or_default();
            }
            PHashDatabaseInput::Ignore => {}
        }
    }
}
