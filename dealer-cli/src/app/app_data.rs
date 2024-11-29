use std::sync::{Arc, OnceLock, RwLock};

static APP_DATA: OnceLock<Arc<RwLock<AppData>>> = OnceLock::new();

pub struct AppData {
    pub should_exit: bool,
}

impl AppData {
    pub fn singleton() -> Arc<RwLock<AppData>> {
        APP_DATA
            .get_or_init(|| Arc::new(RwLock::new(AppData { should_exit: false })))
            .clone()
    }
}
