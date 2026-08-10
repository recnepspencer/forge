mod analysis;
mod chain;
mod resolver;
mod types;

pub use chain::dependency_chain_to;
pub(crate) use resolver::explain_reconstructing_with_policy_resolver;
pub use resolver::{explain, explain_with_policy_resolver};
#[cfg(test)]
pub use types::ConditionDecision;
pub use types::{
    CausalDisposition, CausalLink, CausalLinkKind, NodeExplanation, RewiringDependency,
    RewiringSummary, ScopeProvenance, ScopeProvenanceKind, UpstreamCause,
};
