#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReloadFailureStage {
    InvalidCandidate,
    CandidateAdmission,
    ImpactClassification,
    ImpactNarrowing,
    IdentityMatching,
    DurableStateReconciliation,
    QueryLiveRebind,
    ActivationStaging,
    PlanLowering,
    RuntimeHandleAllocation,
    PlanTopologyAssembly,
    ReadyActivation,
    ActivationGate,
    AtomicPlanSwap,
}
