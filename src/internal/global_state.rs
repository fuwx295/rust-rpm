use std::sync::{Mutex, MutexGuard};

use once_cell::sync::Lazy;

use super::ts::TransactionSet;





static RPM_GLOBAL_STATE: Lazy<Mutex<GlobalState>> = Lazy::new(||Mutex::new(GlobalState::default()));


pub struct GlobalState {
    pub configured: bool,
    pub ts: TransactionSet,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState { configured: false, ts: TransactionSet::create() }
    }
}

impl GlobalState {
    pub fn lock()-> MutexGuard<'static, Self> {
        RPM_GLOBAL_STATE.lock().unwrap()
    }
}