use relm4::{
    component::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
        SimpleAsyncComponent,
    },
    gtk::{
        self,
        prelude::{BoxExt, OrientableExt, WidgetExt},
    },
    AsyncComponentSender,
};

use crate::app::{
    components::{csam::CsamModel, face::FaceModel},
    models::SidebarOption,
};
use crate::context::AppContext;

pub struct ContentModel {
    csam: AsyncController<CsamModel>,
    face: AsyncController<FaceModel>,
    sidebar_option: Option<SidebarOption>,
}

#[derive(Debug)]
pub enum ContentInput {
    SelectSidebarOption(SidebarOption),
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for ContentModel {
    type Init = AppContext;
    type Input = ContentInput;
    type Output = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_hexpand: true,

            #[transition = "Crossfade"]
            append = match model.sidebar_option {
                Some(SidebarOption::CSAM) => {
                    gtk::Box {
                        #[watch]
                        set_visible: model.sidebar_option.is_some(),
                        set_orientation: gtk::Orientation::Vertical,
                        append: model.csam.widget(),
                    }
                },
                Some(SidebarOption::Face) => {
                    gtk::Box {
                        #[watch]
                        set_visible: model.sidebar_option.is_some(),
                        set_orientation: gtk::Orientation::Vertical,
                        append: model.face.widget(),
                    }
                },
                None => {
                    gtk::Label {
                        set_label: "Not Found!",
                    }
                }
            }
        }
    }

    async fn init(
        ctx: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let csam_controller = CsamModel::builder().launch(ctx).detach();
        let face_controller = FaceModel::builder().launch(()).detach();

        let model = ContentModel {
            csam: csam_controller,
            face: face_controller,
            sidebar_option: Some(SidebarOption::CSAM),
        };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            ContentInput::SelectSidebarOption(sidebar_option) => {
                self.sidebar_option.replace(sidebar_option);
            }
        }
    }
}
