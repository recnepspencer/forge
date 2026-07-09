mod errors;
mod facade;
mod resolution;
#[cfg(test)]
mod tests;
mod witnesses;

pub use errors::{
    CapabilityAdmissionError, CapabilityAdmissionFailureClass, WorthQueryFacadeCounters,
    WorthQueryFacadeError, WorthQueryFacadeFailureClass,
};
pub use facade::WorthQueryApplicationFacade;
pub use resolution::{CapabilityAdmissionDecision, WorthQueryCapabilityResolution};
pub use witnesses::{
    HistoricalEvaluationCapability, IdentityEvolutionCapability, LiveQueryCapability,
    PreviewSessionCapability, QueryCompositionCapability, QueryContextCapability,
    QueryReadCapability, WorkflowOrchestrationCapability,
};
