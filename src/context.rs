use std::sync::Arc;

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
}
