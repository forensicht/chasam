use crate::fl;
use relm4_icons::icon_names;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
pub enum SidebarOption {
    CSAM,
    Face,
}

impl SidebarOption {
    pub fn list() -> Vec<SidebarOption> {
        let list: Vec<SidebarOption> = SidebarOption::iter().collect();
        list
    }

    #[allow(unused)]
    pub fn name(&self) -> String {
        let csam: &String = fl!("csam");
        let face: &String = fl!("face-search");
        match self {
            Self::CSAM => csam.clone(),
            Self::Face => face.clone(),
        }
    }

    pub fn description(&self) -> String {
        let csam_desc: &String = fl!("csam");
        let face_desc: &String = fl!("face-search");
        match self {
            Self::CSAM => csam_desc.clone(),
            Self::Face => face_desc.clone(),
        }
    }

    pub fn icon(&self) -> Option<&str> {
        match self {
            Self::CSAM => Some(icon_names::PARENT),
            Self::Face => Some(icon_names::STAMP),
        }
    }
}
