use crate::app::{
    components::{csam::CsamModel, csam_db::CsamDBModel, face::FaceModel},
    models::SidebarOption,
};

use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk::prelude::*,
    prelude::*,
    AsyncComponentSender,
};

pub struct ContentModel {
    csam: AsyncController<CsamModel>,
    face: AsyncController<FaceModel>,
    csam_db: AsyncController<CsamDBModel>,
    sidebar_option: Option<SidebarOption>,
}

impl ContentModel {
    pub fn new(
        csam: AsyncController<CsamModel>,
        face: AsyncController<FaceModel>,
        csam_db: AsyncController<CsamDBModel>,
        sidebar_option: Option<SidebarOption>,
    ) -> Self {
        Self {
            csam,
            face,
            csam_db,
            sidebar_option,
        }
    }
}

#[derive(Debug)]
pub enum ContentInput {
    SelectSidebarOption(SidebarOption),
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
    type Init = ();
    type Input = ContentInput;
    type Output = ();
    type CommandOutput = ();

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
                Some(SidebarOption::DB) => {
                    gtk::Box {
                        #[watch]
                        set_visible: model.sidebar_option.is_some(),
                        set_orientation: gtk::Orientation::Vertical,
                        append: model.csam_db.widget(),
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
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let csam_controller = CsamModel::builder().launch(()).detach();
        let face_controller = FaceModel::builder().launch(()).detach();
        let csam_db_controller = CsamDBModel::builder().launch(()).detach();

        let model = ContentModel::new(
            csam_controller,
            face_controller,
            csam_db_controller,
            Some(SidebarOption::CSAM),
        );
        let widgets = view_output!();

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
            ContentInput::SelectSidebarOption(sidebar_option) => {
                self.sidebar_option.replace(sidebar_option);
            }
        }

        self.update_view(widgets, sender);
    }
}
