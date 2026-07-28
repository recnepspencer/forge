use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};
use std::thread;
use std::time::{Duration, Instant};

const ACQUISITION_POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) struct NativeDesktopLease {
    _guard: MutexGuard<'static, ()>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct NativeDesktopLeaseDeadline;

impl NativeDesktopLease {
    pub(super) fn acquire(deadline: Instant) -> Result<Self, NativeDesktopLeaseDeadline> {
        static NATIVE_DESKTOP: OnceLock<Mutex<()>> = OnceLock::new();
        let desktop = NATIVE_DESKTOP.get_or_init(|| Mutex::new(()));
        loop {
            match desktop.try_lock() {
                Ok(guard) => return Ok(Self { _guard: guard }),
                Err(TryLockError::Poisoned(poisoned)) => {
                    return Ok(Self {
                        _guard: poisoned.into_inner(),
                    });
                }
                Err(TryLockError::WouldBlock) if Instant::now() < deadline => {
                    thread::sleep(ACQUISITION_POLL_INTERVAL);
                }
                Err(TryLockError::WouldBlock) => return Err(NativeDesktopLeaseDeadline),
            }
        }
    }
}
