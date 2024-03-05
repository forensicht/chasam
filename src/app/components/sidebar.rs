use crate::app::{
    factories::sidebar_option::{SidebarOptionModel, SidebarOptionOutput},
    models,
};

use relm4::{
    component::{AsyncComponentParts, SimpleAsyncComponent},
    factory::AsyncFactoryVecDeque,
    gtk::prelude::{OrientableExt, WidgetExt},
    prelude::*,
};

pub struct SidebarModel {
    sidebar_option_factory: AsyncFactoryVecDeque<SidebarOptionModel>,
}

#[derive(Debug)]
pub enum SidebarInput {
    SelectedOption(models::SidebarOption),
}

#[derive(Debug)]
pub enum SidebarOutput {
    SelectedOption(models::SidebarOption),
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for SidebarModel {
    type Init = ();
    type Input = SidebarInput;
    type Output = SidebarOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::ScrolledWindow {
                set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),
                set_vexpand: true,

                #[local_ref]
                option_list_widget -> gtk::ListBox {
                    set_css_classes: &["navigation-sidebar"],
                    connect_row_selected => move |_, listbox_row| {
                        if let Some(row) = listbox_row {
                            row.activate();
                        }
                    },
                }
            },
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut sidebar_option_factory = AsyncFactoryVecDeque::builder().launch_default().forward(
            sender.input_sender(),
            |output| match output {
                SidebarOptionOutput::Selected(option) => SidebarInput::SelectedOption(option),
            },
        );

        {
            let mut guard = sidebar_option_factory.guard();
            for option in models::SidebarOption::list() {
                guard.push_back(option);
            }
        }

        let model = SidebarModel {
            sidebar_option_factory,
        };
        let option_list_widget = model.sidebar_option_factory.widget();
        if let Some(widget) = option_list_widget.first_child() {
            widget.activate();
        }
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            SidebarInput::SelectedOption(option) => {
                sender
                    .output(SidebarOutput::SelectedOption(option))
                    .unwrap_or_default();
            }
        }
    }
}
