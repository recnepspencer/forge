#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OfflineVerifierBoundarySeam {
    LayoutWalkBeforeRuntimeRecovery,
    DamageClassificationObservation,
    RuntimeVerifierComparison,
    FutureExtensionSlot,
}

impl OfflineVerifierBoundarySeam {
    pub const fn token(self) -> &'static str {
        match self {
            Self::LayoutWalkBeforeRuntimeRecovery => "layout-walk-before-runtime-recovery",
            Self::DamageClassificationObservation => "damage-classification-observation",
            Self::RuntimeVerifierComparison => "runtime-verifier-comparison",
            Self::FutureExtensionSlot => "future-extension-slot",
        }
    }
}
