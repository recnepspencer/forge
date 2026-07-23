use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDomainInstallationGeneration(u64);

impl WorthQueryDomainInstallationGeneration {
    pub(crate) const fn initial() -> Self {
        Self(1)
    }

    pub const fn ordinal(self) -> u64 {
        self.0
    }

    pub(crate) fn successor(self) -> Self {
        Self(
            self.0
                .checked_add(1)
                .expect("installation generation must not overflow"),
        )
    }
}

#[derive(Clone)]
pub(crate) struct WorthQueryDomainInstallationGenerationLease {
    current_ordinal: Arc<AtomicU64>,
}

impl WorthQueryDomainInstallationGenerationLease {
    pub(crate) fn new(generation: WorthQueryDomainInstallationGeneration) -> Self {
        Self {
            current_ordinal: Arc::new(AtomicU64::new(generation.ordinal())),
        }
    }

    pub(crate) fn is_current(&self, generation: WorthQueryDomainInstallationGeneration) -> bool {
        self.current_ordinal.load(Ordering::Acquire) == generation.ordinal()
    }

    pub(crate) fn advance_to(&self, generation: WorthQueryDomainInstallationGeneration) {
        self.current_ordinal
            .store(generation.ordinal(), Ordering::Release);
    }
}

impl std::fmt::Debug for WorthQueryDomainInstallationGenerationLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryDomainInstallationGenerationLease")
            .field(
                "current_ordinal",
                &self.current_ordinal.load(Ordering::Relaxed),
            )
            .finish_non_exhaustive()
    }
}

impl PartialEq for WorthQueryDomainInstallationGenerationLease {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.current_ordinal, &other.current_ordinal)
    }
}

impl Eq for WorthQueryDomainInstallationGenerationLease {}
