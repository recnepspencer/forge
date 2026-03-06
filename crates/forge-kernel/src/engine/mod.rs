//! Feature evaluation engine.
//!
//! DOMAIN: Trait definitions, pipeline infrastructure, evaluation, and
//! transactional state lifecycle for features (Primitives, Boolean,
//! Fillet, etc.). Features themselves live in their own top-level
//! directories; this module provides the shared engine that drives them.
//!
//! DEPENDENCIES: forge-core (OperationResult, KernelError),
//! forge-signal (NodeId), forge-topo (TopologyState)
//!
//! ## Structure
//!
//! ```text
//! engine/
//! ├── contracts/        ← Feature, FeatureContract, FeatureInputs, FeatureRegistry traits
//! ├── pipeline/         ← FeaturePipeline executor, post-invariant validation
//! ├── operation_space/  ← Numerical conditioning (local coordinate transforms)
//! ├── output/           ← SolidEnvelope, ArenaSnapshot, TopologyDelta
//! ├── transaction/      ← KernelState, KernelDraft, BRepWorkspace, OperationFinalizer
//! ├── feature_tree.rs   ← FeatureTree<R> signal-graph manager
//! ├── macros.rs         ← declare_feature! macro (#[macro_export])
//! ├── facade.rs         ← public API
//! └── tests.rs          ← feature tests
//! ```

mod contracts;
mod feature_tree;
mod operation_space;
mod output;
mod pipeline;
pub(crate) mod transaction;

// Re-export contract types at stable crate paths for the `declare_feature!` macro.
// The macro expands in caller scope and references `$crate::engine::contract::*`,
// so this module must remain a public re-export target.
pub mod contract {
    //! Stable re-export of contract types for macro hygiene.
    pub use super::contracts::contract::*;
}

pub mod facade;
pub mod macros;

#[cfg(test)]
mod tests;
