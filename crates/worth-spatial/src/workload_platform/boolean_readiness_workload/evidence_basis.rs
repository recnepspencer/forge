use crate::planar_contracts::contract_bundle::PlanarM7ReadinessBundle;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::workload_platform::dirty_planar_clean_fail::DirtyPlanarCleanFailReceipt;
use crate::workload_platform::evidence_ledger::CompleteWorkloadEvidenceLedger;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityDenial;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityReceipt;
use crate::workload_platform::surface_support::UnsupportedSurfaceSupportReceipt;

use super::blocker_evidence::{
    PlanarBooleanReadinessBlocker, PlanarBooleanReadinessBlockerEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanReadinessEvidenceBasis {
    evidence_ledger: CompleteWorkloadEvidenceLedger,
    readiness_bundle: PlanarM7ReadinessBundle,
    parity_receipt: ProjectionFactParityReceipt,
    blocker: Option<PlanarBooleanReadinessBlockerEvidence>,
}

impl PlanarBooleanReadinessEvidenceBasis {
    pub fn from_real_workload_evidence(
        evidence_ledger: CompleteWorkloadEvidenceLedger,
        readiness_bundle: PlanarM7ReadinessBundle,
        parity_receipt: ProjectionFactParityReceipt,
    ) -> Self {
        Self {
            evidence_ledger,
            readiness_bundle,
            parity_receipt,
            blocker: None,
        }
    }

    pub fn with_policy_required_projection_parity_denial(
        mut self,
        denial: &ProjectionFactParityDenial,
    ) -> Self {
        self.blocker = Some(PlanarBooleanReadinessBlockerEvidence::new(
            PlanarBooleanReadinessBlocker::PolicyRequired,
            denial.human_reason(),
            format!(
                "projection-parity-policy:{:?}:{:?}:{}",
                denial.kind(),
                denial.failed_lane(),
                denial.human_reason()
            ),
        ));
        self
    }

    pub fn with_clean_failure(mut self, receipt: &DirtyPlanarCleanFailReceipt) -> Self {
        self.blocker = Some(PlanarBooleanReadinessBlockerEvidence::new(
            PlanarBooleanReadinessBlocker::CleanFailure,
            format!(
                "Dirty planar input is {:?}; boolean readiness has no automatic option.",
                receipt.dirty_case()
            ),
            receipt.clean_fail_digest(),
        ));
        self
    }

    pub fn with_unsupported_surface_support(
        mut self,
        receipt: &UnsupportedSurfaceSupportReceipt,
    ) -> Self {
        self.blocker = Some(PlanarBooleanReadinessBlockerEvidence::new(
            PlanarBooleanReadinessBlocker::UnsupportedWorkloadFamily,
            format!(
                "Unsupported {:?} surface support cannot enter boolean readiness.",
                receipt.family()
            ),
            receipt.stage_identity().receipt_identity(),
        ));
        self
    }

    pub fn with_predicate_uncertainty_diagnostics(
        mut self,
        receipt: &PlanarDiagnosticBundleReceipt,
    ) -> Self {
        self.blocker = Some(PlanarBooleanReadinessBlockerEvidence::new(
            PlanarBooleanReadinessBlocker::PredicateUncertainty,
            "Predicate uncertainty must be resolved before boolean readiness.",
            receipt.diagnostic_bundle_digest(),
        ));
        self
    }

    pub fn with_orientation_flip_diagnostics(
        mut self,
        receipt: &PlanarDiagnosticBundleReceipt,
    ) -> Self {
        self.blocker = Some(PlanarBooleanReadinessBlockerEvidence::new(
            PlanarBooleanReadinessBlocker::OrientationFlipLocalization,
            "Orientation flip localization must be resolved before M7 readiness.",
            receipt.diagnostic_bundle_digest(),
        ));
        self
    }

    pub fn with_rejected_kernel_summary_substitution(mut self, reason: impl Into<String>) -> Self {
        let reason = reason.into();
        self.blocker = Some(PlanarBooleanReadinessBlockerEvidence::new(
            PlanarBooleanReadinessBlocker::KernelSummarySubstitution,
            reason.clone(),
            format!("kernel-summary-substitution:{reason}"),
        ));
        self
    }

    pub(crate) fn evidence_ledger(&self) -> &CompleteWorkloadEvidenceLedger {
        &self.evidence_ledger
    }

    pub(crate) fn parity_receipt(&self) -> &ProjectionFactParityReceipt {
        &self.parity_receipt
    }

    pub(crate) fn into_certification_parts(
        self,
    ) -> (
        CompleteWorkloadEvidenceLedger,
        PlanarM7ReadinessBundle,
        ProjectionFactParityReceipt,
    ) {
        (
            self.evidence_ledger,
            self.readiness_bundle,
            self.parity_receipt,
        )
    }

    pub(crate) fn blocker_evidence(&self) -> Option<&PlanarBooleanReadinessBlockerEvidence> {
        self.blocker.as_ref()
    }
}
