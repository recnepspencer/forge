#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiRuntimeDiagnosticFamily {
    Reload,
    CandidateAdmission,
    ArtifactEquivalence,
    ReplacementImpact,
    ImpactNarrowing,
    IdentityMatching,
    DurableStateReconciliation,
    QueryLiveRebind,
    PlanLowering,
    LaneAdmission,
    ActivationStaging,
    ActivationGate,
    CommittedAllocationActivation,
    PlanInspection,
    DiagnosticsProjection,
}

impl WorthUiRuntimeDiagnosticFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reload => "reload",
            Self::CandidateAdmission => "replacement.candidate_admission",
            Self::ArtifactEquivalence => "replacement.artifact_equivalence",
            Self::ReplacementImpact => "replacement.impact",
            Self::ImpactNarrowing => "replacement.impact_narrowing",
            Self::IdentityMatching => "replacement.identity_matching",
            Self::DurableStateReconciliation => "replacement.durable_state_reconciliation",
            Self::QueryLiveRebind => "query.live_rebind",
            Self::PlanLowering => "plan.lowering",
            Self::LaneAdmission => "plan.lane_admission",
            Self::ActivationStaging => "activation.staging",
            Self::ActivationGate => "activation.gate",
            Self::CommittedAllocationActivation => "activation.committed_allocation",
            Self::PlanInspection => "plan.inspection",
            Self::DiagnosticsProjection => "diagnostics.projection",
        }
    }
}
