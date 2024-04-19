use std::sync::{atomic::AtomicPtr, MutexGuard};
use librpm::rpmts_s;
use super::global_state::GlobalState;

pub struct TransactionSet(AtomicPtr<librpm::rpmts_s>);

impl TransactionSet {
    pub fn create() -> Self {
        TransactionSet(AtomicPtr::new(unsafe { librpm::rpmtsCreate() }))
    }
    pub fn as_mut_ptr(&mut self) -> &mut *mut rpmts_s {
        self.0.get_mut()
    }
}

impl Drop for TransactionSet {
    fn drop(&mut self) {
        unsafe {
            librpm::rpmtsFree(*self.0.get_mut());
        }
    }
}

pub struct GlobalTS(MutexGuard<'static, GlobalState>);

impl GlobalTS {
    pub fn create() -> Self {
        GlobalTS(GlobalState::lock())  
    }
    pub fn as_mut_ptr(&mut self) -> *mut rpmts_s {
        *self.0.ts.as_mut_ptr()
    }
}

impl Drop for GlobalTS {
    fn drop(&mut self) {
        unsafe {
            librpm::rpmtsClean(self.as_mut_ptr());
        }
    }
    
}