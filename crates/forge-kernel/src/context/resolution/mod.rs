//! Resolution vertical slice.
//!
//! DOMAIN: The cascade engine for resolving ambiguous policy queries —
//! data types and resolution logic.

mod policy_decision;
mod resolver;

pub use policy_decision::{ResolvedPolicyDecision, ResolvedPolicySource};
