#[path = "explain/analysis.rs"]
mod analysis;
#[path = "explain/chain.rs"]
mod chain;
#[path = "explain/resolver.rs"]
mod resolver;
#[path = "explain/types.rs"]
mod types;

pub use chain::dependency_chain_to;
pub use resolver::{explain, explain_with_policy_resolver};
pub use types::{
    ConditionDecision, MeaningfulChangeReason, NodeExplanation, UpstreamCause,
};
