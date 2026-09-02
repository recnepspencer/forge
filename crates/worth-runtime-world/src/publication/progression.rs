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

impl RuntimeWorldPublicationPhase {
    /// The only legal successor relation for the serial publication grammar.
    /// The value-level vocabulary is useful for diagnostics; the consuming
    /// phase tokens remain the authority for actual transitions.
    pub const fn successor(self) -> Option<Self> {
        match self {
            Self::ProductBranchIntent => Some(Self::ResolvedExpectedProductHead),
            Self::ResolvedExpectedProductHead => Some(Self::AdmittedCompositeRuntimeWorldBasis),
            Self::AdmittedCompositeRuntimeWorldBasis => Some(Self::LoweredOwnerComponentPlan),
            Self::LoweredOwnerComponentPlan => Some(Self::ReservedCompositePublicationAttempt),
            Self::ReservedCompositePublicationAttempt => Some(Self::OwnerExecutionSettlement),
            Self::OwnerExecutionSettlement => Some(Self::CompositePublicationReady),
            Self::CompositePublicationReady => Some(Self::RuntimeWorldPublicationOutcome),
            Self::RuntimeWorldPublicationOutcome => None,
        }
    }
}
