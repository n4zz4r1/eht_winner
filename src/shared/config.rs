use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

pub struct SharedConfig {
    pub lport_current: Mutex<u16>,
}

impl SharedConfig {
    pub fn new(lport_current: u16) -> Self {
        SharedConfig {
            lport_current: Mutex::new(lport_current),
        }
    }
}

lazy_static! {
    pub static ref SHARED_DATA: Arc<SharedConfig> = Arc::new(SharedConfig::new(4444));
}