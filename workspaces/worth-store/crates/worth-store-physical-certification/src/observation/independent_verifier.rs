use worth_store_offline_verifier::OfflineVerifierBoundarySeam;
use worth_store_recovery_physics::{
    RuntimeRecoveryComparisonClassification, RuntimeRecoveryComparisonReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndependentVerifierObservationKind {
    Agreement,
    Disagreement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndependentVerifierObservation {
    seam: OfflineVerifierBoundarySeam,
    comparison: RuntimeRecoveryComparisonClassification,
}

impl IndependentVerifierObservation {
    pub fn from_runtime_recovery_comparison(report: &RuntimeRecoveryComparisonReport) -> Self {
        Self {
            seam: OfflineVerifierBoundarySeam::RuntimeVerifierComparison,
            comparison: report.classification(),
        }
    }

    pub const fn seam(&self) -> OfflineVerifierBoundarySeam {
        self.seam
    }

    pub const fn kind(&self) -> IndependentVerifierObservationKind {
        if matches!(
            self.comparison,
            RuntimeRecoveryComparisonClassification::Equivalent
        ) {
            IndependentVerifierObservationKind::Agreement
        } else {
            IndependentVerifierObservationKind::Disagreement
        }
    }

    pub const fn comparison(&self) -> RuntimeRecoveryComparisonClassification {
        self.comparison
    }
}
