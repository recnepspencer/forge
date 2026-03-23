mod analysis;
mod chain;
mod resolver;
mod types;

pub use chain::dependency_chain_to;
#[allow(unused_imports)]
pub(crate) use resolver::derive_rewiring_summary;
pub(crate) use resolver::explain_reconstructing_with_policy_resolver;
pub use resolver::{explain, explain_with_policy_resolver};
pub use types::{
    CausalDisposition, CausalLink, CausalLinkKind, ConditionDecision, MeaningfulChangeReason,
    NodeExplanation, RewiringDependency, RewiringSummary, ScopeProvenance,
    ScopeProvenanceKind, UpstreamCause,
};
