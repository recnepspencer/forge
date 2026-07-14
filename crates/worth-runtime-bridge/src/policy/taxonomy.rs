#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum BridgeExecutionPolicyClass {
    #[default]
    DeterministicCanonical,
    Optimized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgePolicySourceClass {
    RuntimeBaseline,
    RequestDeclared,
    TruthViewAdmitted,
    SourceCapabilityAdmitted,
    SpeculationLifecycleAdmitted,
    BridgeLowered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgePolicyFieldKind {
    ExecutionMode,
    DiagnosticsTier,
    ReplayArtifacts,
    ArtifactRetention,
    PreviewRefinement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgePolicyResolution {
    Rejected,
    Narrowed,
    Inherited,
    AcceptedAsDeclared,
}
