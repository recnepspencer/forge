use std::sync::{Mutex, MutexGuard, OnceLock};

pub(super) struct ExclusiveNativeCourtroomLease {
    _guard: MutexGuard<'static, ()>,
}

pub(super) fn enter_exclusive_native_courtroom() -> ExclusiveNativeCourtroomLease {
    static COURTROOM: OnceLock<Mutex<()>> = OnceLock::new();
    let courtroom = COURTROOM.get_or_init(|| Mutex::new(()));
    let guard = courtroom
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    ExclusiveNativeCourtroomLease { _guard: guard }
}
