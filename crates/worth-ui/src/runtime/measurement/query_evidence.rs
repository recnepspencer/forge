#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorthUiMeasurementQueryEvidenceKind {
    SubscriptionSelectionDiagnostics,
    SignalCompatibilityAndContinuation,
    PlannerParallelAdmissionAndScalePosture,
    AsyncResourcesAndResultState,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorthUiMeasurementQueryEvidence {
    kind: WorthUiMeasurementQueryEvidenceKind,
    evidence_digest: u64,
}

impl WorthUiMeasurementQueryEvidence {
    pub fn subscription_selection_diagnostics(evidence_digest: u64) -> Self {
        Self::new(
            WorthUiMeasurementQueryEvidenceKind::SubscriptionSelectionDiagnostics,
            evidence_digest,
        )
    }

    pub fn signal_compatibility_and_continuation(evidence_digest: u64) -> Self {
        Self::new(
            WorthUiMeasurementQueryEvidenceKind::SignalCompatibilityAndContinuation,
            evidence_digest,
        )
    }

    pub fn planner_parallel_admission_and_scale_posture(evidence_digest: u64) -> Self {
        Self::new(
            WorthUiMeasurementQueryEvidenceKind::PlannerParallelAdmissionAndScalePosture,
            evidence_digest,
        )
    }

    pub fn async_resources_and_result_state(evidence_digest: u64) -> Self {
        Self::new(
            WorthUiMeasurementQueryEvidenceKind::AsyncResourcesAndResultState,
            evidence_digest,
        )
    }

    fn new(kind: WorthUiMeasurementQueryEvidenceKind, evidence_digest: u64) -> Self {
        Self {
            kind,
            evidence_digest,
        }
    }

    pub fn kind(&self) -> WorthUiMeasurementQueryEvidenceKind {
        self.kind
    }

    pub fn evidence_digest(&self) -> u64 {
        self.evidence_digest
    }
}
