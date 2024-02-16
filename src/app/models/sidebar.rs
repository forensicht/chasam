use crate::fl;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use relm4_icons::icon_name;

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum SidebarOption {
    CSAM,
    Face,
    DB,
}

impl SidebarOption {
    pub fn list() -> Vec<SidebarOption> {
        let list: Vec<SidebarOption> = SidebarOption::iter().collect();
        list
    }

    pub fn name(&self) -> String {
        let csam: &String = fl!("csam");
        let face: &String = fl!("face-search");
        let db: &String = fl!("csam-db");
        match self {
            Self::CSAM => csam.clone(),
            Self::Face => face.clone(),
            Self::DB => db.clone(),
        }
    }

    pub fn description(&self) -> String {
        let csam_desc: &String = fl!("csam");
        let face_desc: &String = fl!("face-search");
        let db_desc: &String = fl!("csam-db");
        match self {
            Self::CSAM => csam_desc.clone(),
            Self::Face => face_desc.clone(),
            Self::DB => db_desc.clone(),
        }
    }

    pub fn icon(&self) -> Option<&str> {
        match self {
            Self::CSAM => Some(icon_name::PARENT),
            Self::Face => Some(icon_name::STAMP),
            Self::DB => Some(icon_name::HARDDISK),
        }
    }
}
