//! Feature pipeline infrastructure.
//!
//! DOMAIN: Declarative contracts, typed inputs, step sequencing,
//! and auto-injection for the feature evaluation pipeline.
//!
//! DEPENDENCIES: forge-schema (Command types), forge-signal (NodeId),
//! features/tree (FeatureTree, NativeFeature), features/wrappers
//!
//! ## Structure
//!
//! ```text
//! pipeline/
//! ├── mod.rs        ← this file (Table of Contents)
//! ├── dispatch.rs   ← Tier 0: Command → FeatureTree bridge
//! └── tests.rs      ← acceptance tests
//! ```
//!
//! Future tiers will add:
//! - `contract.rs`  — Tier 1: FeatureContract, AuditLevel, InvariantKind
//! - `executor.rs`  — Tier 1: FeaturePipeline::execute
//! - `macros.rs`    — Tier 1: declare_feature! macro

pub mod dispatch;
pub mod contract;
pub mod macros;
pub mod executor;

#[cfg(test)]
mod tests;
