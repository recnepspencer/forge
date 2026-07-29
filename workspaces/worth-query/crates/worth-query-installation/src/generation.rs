use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_RUNTIME_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryInstallationRuntimeIdentity(NonZeroU64);

impl WorthQueryInstallationRuntimeIdentity {
    pub fn fresh() -> Self {
        let candidate = NEXT_RUNTIME_IDENTITY.fetch_add(1, Ordering::Relaxed);
        Self(NonZeroU64::new(candidate).expect("runtime identity must remain non-zero"))
    }

    pub(crate) fn retained(&self) -> Self {
        Self(self.0)
    }

    /// Retains the installation identity inside the execution composition
    /// authority that consumed this installer.
    ///
    /// This does not mint a new runtime identity. Execution keeps the
    /// retained value private and uses it only while installing providers
    /// into the same operating world.
    #[doc(hidden)]
    pub fn retain_for_execution_installation(&self) -> Self {
        self.retained()
    }

    pub fn ordinal(&self) -> u64 {
        self.0.get()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryInstallationGeneration(NonZeroU64);

impl WorthQueryInstallationGeneration {
    pub fn initial() -> Self {
        Self(NonZeroU64::new(1).expect("initial generation is non-zero"))
    }

    #[doc(hidden)]
    pub fn from_ordinal(ordinal: u64) -> Self {
        Self(NonZeroU64::new(ordinal).expect("installation generation must be non-zero"))
    }

    pub fn ordinal(self) -> u64 {
        self.0.get()
    }

    pub fn successor(self) -> Self {
        Self::from_ordinal(
            self.ordinal()
                .checked_add(1)
                .expect("installation generation must not overflow"),
        )
    }
}
