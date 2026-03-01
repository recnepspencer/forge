//! Feature evaluation engine.
//!
//! DOMAIN: Trait definitions, pipeline infrastructure, and evaluation for
//! features (Primitives, Boolean, Fillet, etc.). Features themselves live
//! in their own top-level directories; this module provides the shared
//! engine that drives them.
//!
//! DEPENDENCIES: forge-core (OperationResult, KernelError),
//! forge-signal (NodeId), forge-topo (TopologyState)
//!
//! ## Structure
//!
//! ```text
//! engine/
//! ├── mod.rs         ← this file (Table of Contents)
//! ├── traits.rs      ← Feature trait, FeatureOutput
//! ├── tree.rs        ← FeatureTree, NativeFeature enum
//! ├── intent.rs      ← FeatureIntent (future sketch-to-feature)
//! ├── contract.rs    ← FeatureContract, AuditLevel, InvariantKind
//! ├── executor.rs    ← FeaturePipeline::execute (7-stage lifecycle)
//! ├── macros.rs      ← declare_feature! macro
//! ├── invariants.rs  ← validate_invariant dispatch
//! ├── dispatch.rs    ← Command → FeatureTree bridge
//! ├── errors.rs      ← PipelineError taxonomy
//! └── wrappers.rs    ← Legacy feature wrappers (to be migrated)
//! ```

pub mod contract;
pub mod dispatch;
pub mod errors;
pub mod executor;
pub mod facade;
pub mod intent;
pub mod invariants;
pub mod macros;
pub mod traits;
pub mod tree;
pub mod wrappers;

#[cfg(test)]
mod tests;
