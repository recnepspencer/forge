use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_APPLICATION_OUTCOME_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct WorthQueryApplicationOutcomeIdentity(u64);

impl WorthQueryApplicationOutcomeIdentity {
    pub(crate) fn mint() -> Option<Self> {
        NEXT_APPLICATION_OUTCOME_IDENTITY
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .ok()
            .map(Self)
    }

    pub(crate) fn restore(value: u64) -> Option<Self> {
        let next = value.checked_add(1)?;
        if value == 0 {
            return None;
        }
        NEXT_APPLICATION_OUTCOME_IDENTITY.fetch_max(next, Ordering::Relaxed);
        Some(Self(value))
    }

    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}
