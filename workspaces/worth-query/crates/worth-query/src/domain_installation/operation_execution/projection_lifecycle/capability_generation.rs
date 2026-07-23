#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryBoundCapabilityGeneration(u64);

impl WorthQueryBoundCapabilityGeneration {
    pub(crate) fn mint() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let ordinal = NEXT
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |current| current.checked_add(1),
            )
            .expect("bound capability generation space exhausted");
        Self(ordinal)
    }

    pub const fn ordinal(self) -> u64 {
        self.0
    }
}
