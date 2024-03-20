pub mod csam;
pub mod repository;
pub(crate) mod utils;

use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

pub struct ServiceProvider {
    csam_service: Arc<csam::Service>,
}

impl ServiceProvider {
    fn new() -> Self {
        ServiceProvider {
            csam_service: Arc::new(csam::Service::new()),
        }
    }

    pub fn instance() -> &'static Mutex<ServiceProvider> {
        static INSTANCE: OnceCell<Mutex<ServiceProvider>> = OnceCell::new();
        INSTANCE.get_or_init(|| Mutex::new(ServiceProvider::new()))
    }

    pub fn csam_service(&self) -> Arc<csam::Service> {
        self.csam_service.clone()
    }
}
