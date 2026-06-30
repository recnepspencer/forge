#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiRuntimeDiagnosticCode {
    ReloadFailurePreserved,
    InvalidCandidateDenied,
    CandidateAdmissionDenied,
    ArtifactEquivalenceDenied,
    ReplacementImpactDenied,
    ImpactNarrowingDenied,
    IdentityMatchingDenied,
    DurableStateReconciliationDenied,
    QueryLiveRebindDenied,
    QueryRecoveryPreserved,
    PlanLoweringDenied,
    LaneAdmissionDenied,
    ActivationStagingDenied,
    ActivationGateDenied,
    AtomicPlanSwapRolledBack,
    PlanInspectionDenied,
    DiagnosticsProjectionAdmitted,
    DiagnosticsSupportMaterialized,
}

impl WorthUiRuntimeDiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReloadFailurePreserved => "reload.failure_preserved",
            Self::InvalidCandidateDenied => "reload.invalid_candidate_denied",
            Self::CandidateAdmissionDenied => "candidate.admission_denied",
            Self::ArtifactEquivalenceDenied => "artifact_equivalence.denied",
            Self::ReplacementImpactDenied => "replacement_impact.denied",
            Self::ImpactNarrowingDenied => "impact_narrowing.denied",
            Self::IdentityMatchingDenied => "identity_matching.denied",
            Self::DurableStateReconciliationDenied => "durable_state.reconciliation_denied",
            Self::QueryLiveRebindDenied => "query.live_rebind_denied",
            Self::QueryRecoveryPreserved => "query.recovery_preserved",
            Self::PlanLoweringDenied => "plan.lowering_denied",
            Self::LaneAdmissionDenied => "lane.admission_denied",
            Self::ActivationStagingDenied => "activation.staging_denied",
            Self::ActivationGateDenied => "activation.gate_denied",
            Self::AtomicPlanSwapRolledBack => "plan_swap.rolled_back",
            Self::PlanInspectionDenied => "plan.inspection_denied",
            Self::DiagnosticsProjectionAdmitted => "diagnostics.projection_admitted",
            Self::DiagnosticsSupportMaterialized => "diagnostics.support_materialized",
        }
    }
}
