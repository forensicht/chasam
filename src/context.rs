use std::sync::Arc;

use num_format::Locale;

use crate::app::{config::settings, models};
use core_chasam::csam;

#[derive(Clone)]
pub struct AppContext {
    pub csam_service: Arc<csam::Service>,
}

impl AppContext {
    pub fn new() -> Self {
        let csam_repo = Arc::new(csam::repository::InMemoryRepository::new());
        let csam_service = Arc::new(csam::Service::new(csam_repo));

        AppContext { csam_service }
    }

    pub fn get_preference(&self) -> models::Preference {
        match settings::PREFERENCES.lock() {
            Ok(preference) => preference.clone(),
            _ => models::Preference::default(),
        }
    }

    pub fn get_locale(&self) -> Locale {
        let language = self.get_preference().language.to_string();
        Locale::from_name(language).expect("Failed to loading language.")
    }
}
