use super::ts::TransactionSet;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

static RPM_GLOBAL_STATE: Lazy<Mutex<GlobalState>> =
    Lazy::new(|| Mutex::new(GlobalState::default()));

/// Tracking struct for mutable global state in RPM
pub struct GlobalState {
    /// Have any configuration functions been called? (Specifically any ones
    /// which invoke `rpmInitCrypto`, which it seems should only be called once)
    pub configured: bool,

    /// Global shared transaction set created the first time librpm's global
    /// state is accessed.
    pub ts: TransactionSet,
}

impl Default for GlobalState {
    fn default() -> GlobalState {
        GlobalState {
            configured: false,
            ts: TransactionSet::create(),
        }
    }
}

impl GlobalState {
    /// Obtain an exclusive lock to the global state
    pub fn lock() -> MutexGuard<'static, Self> {
        RPM_GLOBAL_STATE.lock().unwrap()
    }
}