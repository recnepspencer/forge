//! Operation-level pipeline infrastructure (Tier 2 + 3).
//!
//! DOMAIN: Step contracts and typed pipeline builders for multi-step
//! operations like fillet, chamfer, and boolean. Mirrors the feature-level
//! pipeline in `engine/pipeline/` but at a finer granularity — each
//! step within an operation declares its own policies and precision needs.
//!
//! DEPENDENCIES: forge-core (PolicyKind, KernelError), core/context
//!
//! CONSUMERS: operations/boolean, operations/fillet, operations/chamfer, etc.

pub mod builder;
pub mod step_contract;
pub mod steps;

#[cfg(test)]
mod tests;
