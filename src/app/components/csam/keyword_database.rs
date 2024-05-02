use relm4::{
    adw::{
        self,
        prelude::{
            BoxExt, ButtonExt, OrientableExt, PreferencesGroupExt, PreferencesPageExt, TextViewExt,
            WidgetExt,
        },
    },
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{
        self,
        prelude::{EntryBufferExtManual, EntryExt, TextBufferExt, TextBufferExtManual, ToValue},
    },
    AsyncComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_names;

use crate::app::{components::dialogs, config::settings, models};
use crate::{context::AppContext, fl};

pub struct KeywordDatabaseModel {
    ctx: AppContext,
    entry_buffer: gtk::EntryBuffer,
    text_buffer: gtk::TextBuffer,
}

#[derive(Debug)]
pub enum KeywordDatabaseInput {
    AddKeyword,
    LoadKeywords,
    SaveKeywords,
    GoPrevious,
}

#[derive(Debug)]
pub enum KeywordDatabaseOutput {
    SavedKeywords,
    GoPrevious,
}

#[relm4::component(pub async)]
impl AsyncComponent for KeywordDatabaseModel {
    type Init = AppContext;
    type Input = KeywordDatabaseInput;
    type Output = KeywordDatabaseOutput;
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
                    set_label: fl!("keywords"),
                    set_css_classes: &["heading"],
                },
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",
                    set_css_classes: &["flat"],
                    set_tooltip: fl!("preferences"),
                    connect_clicked => KeywordDatabaseInput::GoPrevious,
                },
            },

            append = &adw::Clamp {
                #[wrap(Some)]
                set_child = &adw::PreferencesPage {
                    set_vexpand: true,

                    add = &adw::PreferencesGroup {
                        set_title: fl!("keywords"),
                        set_description: Some(fl!("add-keyword-description")),

                        #[wrap(Some)]
                        set_header_suffix = &gtk::Box {
                            set_css_classes: &["linked"],
                            gtk::Button {
                                set_icon_name: icon_names::SAVE_FILLED,
                                set_css_classes: &["circular", "suggested-action"],
                                set_valign: gtk::Align::Center,
                                set_tooltip: fl!("save-keywords"),
                                connect_clicked => KeywordDatabaseInput::SaveKeywords,
                            },
                        },

                        gtk::Entry {
                            set_buffer: &model.entry_buffer,
                            set_placeholder_text: Some(fl!("enter-keyword")),
                            set_hexpand: true,
                            set_margin_bottom: 6,
                            set_secondary_icon_name: Some(icon_names::PLUS_LARGE),
                            set_secondary_icon_tooltip_text: Some(fl!("add-keyword")),
                            connect_icon_release[sender] => move |_, icon_position| {
                                if icon_position == gtk::EntryIconPosition::Secondary {
                                    sender.input(KeywordDatabaseInput::AddKeyword);
                                }
                            },
                            connect_activate => KeywordDatabaseInput::AddKeyword,
                        },

                        gtk::ScrolledWindow {
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                            set_hexpand: true,
                            set_vexpand: true,

                            gtk::TextView {
                                set_buffer: Some(&model.text_buffer),
                                set_editable: true,
                                set_hexpand: true,
                                set_valign: gtk::Align::Fill,
                                set_bottom_margin: 5,
                                set_left_margin: 5,
                                set_right_margin: 5,
                                set_top_margin: 5,
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
        let text_buffer = gtk::TextBuffer::default();
        text_buffer.create_tag(Some("gray_bg"), &[("background", &"lightgray".to_value())]);

        let mut model = KeywordDatabaseModel {
            ctx,
            entry_buffer: gtk::EntryBuffer::default(),
            text_buffer,
        };
        model.load_keywords().await;

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
            KeywordDatabaseInput::AddKeyword => {
                let keyword = self.entry_buffer.text().to_string();
                if keyword.is_empty() {
                    return;
                }
                if self.search_keyword(&keyword).await == 0 {
                    self.insert_keyword(&keyword).await;
                }
                self.entry_buffer.set_text("");
            }
            KeywordDatabaseInput::LoadKeywords => {
                self.load_keywords().await;
            }
            KeywordDatabaseInput::SaveKeywords => {
                let window = root.toplevel_window();
                self.save_keywords(window, &sender).await;
            }
            KeywordDatabaseInput::GoPrevious => {
                sender
                    .output(KeywordDatabaseOutput::GoPrevious)
                    .unwrap_or_default();
            }
        }
    }
}

impl KeywordDatabaseModel {
    async fn search_keyword(&mut self, keyword: &str) -> u32 {
        let mut keyword_found = 0;
        let text_buffer = &self.text_buffer;

        // We get the first and last position in the text buffer.
        let mut start_find = text_buffer.start_iter();
        let end_find = text_buffer.end_iter();

        // We remove any previous text tags.
        text_buffer.remove_tag_by_name("gray_bg", &start_find, &end_find);

        while let Some((start_match, end_match)) =
            start_find.forward_search(keyword, gtk::TextSearchFlags::TEXT_ONLY, None)
        {
            text_buffer.apply_tag_by_name("gray_bg", &start_match, &end_match);
            let offset = end_match.offset();
            start_find = text_buffer.iter_at_offset(offset);
            keyword_found += 1;
        }

        keyword_found
    }

    async fn insert_keyword(&mut self, keyword: &str) {
        let mut iter = self.text_buffer.iter_at_offset(0);
        let mut keyword = keyword.to_string();
        keyword.push('\n');
        self.text_buffer.insert(&mut iter, &keyword);
    }

    async fn load_keywords(&mut self) {
        let (mut start_iter, mut end_iter) = self.text_buffer.bounds();
        self.text_buffer.delete(&mut start_iter, &mut end_iter);

        let keywords = self.ctx.csam_service.load_keywords().await;
        for kw in keywords.iter() {
            self.insert_keyword(kw).await;
        }
    }

    async fn save_keywords(
        &mut self,
        window: Option<gtk::Window>,
        sender: &AsyncComponentSender<Self>,
    ) {
        let start_iter = self.text_buffer.start_iter();
        let end_iter = self.text_buffer.end_iter();
        let text = self.text_buffer.text(&start_iter, &end_iter, true);

        if text.is_empty() {
            dialogs::show_info_dialog(
                window.as_ref(),
                Some(fl!("keyword")),
                Some(fl!("msg-keyword-empty")),
            );
            return;
        }

        let db_path = {
            let preference = match settings::PREFERENCES.lock() {
                Ok(preference) => preference.clone(),
                _ => models::Preference::default(),
            };
            preference.database_path.clone()
        };

        let keywords = text.as_str();
        match self.ctx.csam_service.save_keywords(db_path, keywords).await {
            Ok(_) => {
                dialogs::show_info_dialog(
                    window.as_ref(),
                    Some(fl!("keyword")),
                    Some(fl!("saved-successfully")),
                );
                sender
                    .output(KeywordDatabaseOutput::SavedKeywords)
                    .unwrap_or_default();
            }
            Err(err) => {
                tracing::error!("{err}");
                dialogs::show_info_dialog(
                    window.as_ref(),
                    Some(fl!("keyword")),
                    Some(fl!("failed-to-save")),
                );
            }
        }
    }
}
