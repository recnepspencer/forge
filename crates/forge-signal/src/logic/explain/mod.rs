mod analysis;
mod chain;
mod resolver;
mod types;

pub use chain::dependency_chain_to;
pub use resolver::{explain, explain_with_policy_resolver};
pub use types::{
    CausalDisposition, CausalLink, ConditionDecision, MeaningfulChangeReason, NodeExplanation,
    RewiringDependency, RewiringSummary, ScopeProvenance, ScopeProvenanceKind, UpstreamCause,
};
