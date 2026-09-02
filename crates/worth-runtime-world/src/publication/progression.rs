/// Compiler-visible phase names. Values are descriptive; the phase structs
/// remain the authority-bearing transition tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeWorldPublicationPhase {
    ProductBranchIntent,
    ResolvedExpectedProductHead,
    AdmittedCompositeRuntimeWorldBasis,
    LoweredOwnerComponentPlan,
    ReservedCompositePublicationAttempt,
    OwnerExecutionSettlement,
    CompositePublicationReady,
    RuntimeWorldPublicationOutcome,
}
