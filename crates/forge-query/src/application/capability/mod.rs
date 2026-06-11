mod errors;
mod facade;
mod resolution;
#[cfg(test)]
mod tests;
mod witnesses;

pub use errors::{
    CapabilityAdmissionError, CapabilityAdmissionFailureClass, ForgeQueryFacadeCounters,
    ForgeQueryFacadeError, ForgeQueryFacadeFailureClass,
};
pub use facade::ForgeQueryApplicationFacade;
pub use resolution::{CapabilityAdmissionDecision, ForgeQueryCapabilityResolution};
pub use witnesses::{
    HistoricalEvaluationCapability, IdentityEvolutionCapability, LiveQueryCapability,
    PreviewSessionCapability, QueryCompositionCapability, QueryContextCapability,
    QueryReadCapability, WorkflowOrchestrationCapability,
};
