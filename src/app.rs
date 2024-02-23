pub mod config;
pub mod models;
pub mod components;
pub mod factories;
pub mod typed_view;

use crate::fl;
use crate::app::components::{
    about_dialog::AboutDialog,
    preferences::PreferencesModel,
    sidebar::{
        SidebarModel,
        SidebarOutput,
    },
    content::{
        ContentModel,
        ContentInput,
    },
};

use relm4::{
    prelude::*,
    gtk::prelude::*,
    gtk::glib,
    adw,
    component::{
        AsyncComponent,
        AsyncComponentParts,
        AsyncController,
        AsyncComponentController,
    },
    view,
    AsyncComponentSender,
    loading_widgets::LoadingWidgets,
    actions::{
        ActionGroupName,
        RelmAction,
        RelmActionGroup,
    },
    ComponentBuilder,
    ComponentController,
    Controller,
    main_adw_application,
};
use relm4_icons::icon_name;

pub struct App {
    sidebar: AsyncController<SidebarModel>,
    content: AsyncController<ContentModel>,
    preferences: Option<AsyncController<PreferencesModel>>,
    about_dialog: Option<Controller<AboutDialog>>,
}

impl App {
    pub fn new(
        sidebar: AsyncController<SidebarModel>,
        content: AsyncController<ContentModel>,
        preferences: Option<AsyncController<PreferencesModel>>,
        about_dialog: Option<Controller<AboutDialog>>,
    ) -> Self {
        Self {
            sidebar,
            content,
            preferences,
            about_dialog,
        }
    }
}

#[derive(Debug)]
pub enum AppInput {
    SelectedSidebarOption(models::SidebarOption),
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

#[relm4::component(pub async)]
impl AsyncComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    menu! {
        primary_menu: {
            section! {
                preferences => PreferencesAction,
                about => AboutAction,
                quit => QuitAction,
            }
        }
    }

    view! {
        #[root]
        main_window = adw::ApplicationWindow {
            set_size_request: (800, 640),
            set_default_size: (1280, 968),
            set_resizable: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppInput::Quit);
                glib::Propagation::Stop
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                #[name(sidebar)]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_width_request: 50,

                    gtk::CenterBox {
                        set_visible: true,
						set_margin_top: 6,
						set_margin_bottom: 6,

						#[wrap(Some)]
						set_center_widget = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,

							gtk::MenuButton {
								set_valign: gtk::Align::Center,
								set_css_classes: &["flat"],
								set_icon_name: icon_name::MENU,
                                set_tooltip: fl!("menu"),
								set_menu_model: Some(&primary_menu),
							},
						},
                    },

                    append: model.sidebar.widget(),
                },
               
                append: model.content.widget(),
            }
        }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                set_title: Some(fl!("chasam")),
                set_default_size: (500, 350),
                set_resizable: false,

                #[name = "loading"]
                gtk::CenterBox {
                    set_margin_all: 50,
                    set_orientation: gtk::Orientation::Vertical,

                    #[wrap(Some)]
                    set_center_widget = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 3,
                        set_margin_bottom: 12,

                        gtk::Picture {
                            set_resource: Some("/com/github/forensicht/ChaSAM/icons/ChaSAM.png"),
                        },

                        gtk::Label {
                            set_label: fl!("chasam"),
                            set_css_classes: &["title-1"],
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                        },
                    },

                    #[wrap(Some)]
                    set_end_widget = &gtk::Spinner {
                        start: (),
                        set_size_request: (30, 30),
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                    },
                }
            }
        }

        Some(LoadingWidgets::new(root, loading))
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        relm4::tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let preferences: &str = fl!("preferences");
        let about: &str = fl!("about");
        let quit: &str = fl!("quit");

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let sidebar_controller = SidebarModel::builder()
            .launch(())
            .forward(sender.input_sender(), |output| match output {
                SidebarOutput::SelectedOption(option) => AppInput::SelectedSidebarOption(option),
            });

        let content_controller = ContentModel::builder()
            .launch(())
            .detach();

        let mut model = App::new(
            sidebar_controller,
            content_controller,
            None, 
            None,
        );

        let widgets = view_output!();

        let preferences_controller = PreferencesModel::builder()
            .launch(widgets.main_window.upcast_ref::<gtk::Window>().clone())
            .detach();

        model.preferences = Some(preferences_controller);

        let about_dialog = ComponentBuilder::default()
            .launch(widgets.main_window.upcast_ref::<gtk::Window>().clone())
            .detach();

        model.about_dialog = Some(about_dialog);

        let preferences_action = {
            let window = model.preferences.as_ref().unwrap().widget().clone();
            RelmAction::<PreferencesAction>::new_stateless(move |_| {
                window.present();
            })
        };

        let about_action = {
            let sender = model.about_dialog.as_ref().unwrap().sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap_or_default();
            })
        };

        let quit_action = {
            let sender = sender.clone();
            RelmAction::<QuitAction>::new_stateless(move |_| {
                sender.input_sender().send(AppInput::Quit).unwrap_or_default();
            })
        };

        actions.add_action(preferences_action);
        actions.add_action(about_action);
        actions.add_action(quit_action);

        widgets.main_window.insert_action_group(
            WindowActionGroup::NAME,
            Some(&actions.into_action_group()),
        );

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInput::SelectedSidebarOption(sidebar_option) => {
                self.content.emit(ContentInput::SelectSidebarOption(sidebar_option));
            }
            AppInput::Quit => {
                main_adw_application().quit();
            }
        }

        self.update_view(widgets, sender);
    }
}
