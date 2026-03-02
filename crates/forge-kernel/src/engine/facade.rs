//! Public API for the feature engine subsystem.
//!
//! External code should prefer these exports over deep engine internals.

pub use super::contracts::contract::{
    AuditLevel, EntityOriginKind, EulerOpKind, FeatureContract, FeatureInputs, InvariantKind,
    SurfaceKind,
};
pub use super::contracts::feature_trait::Feature;
pub use super::contracts::feature_registry::FeatureRegistry;
pub use super::output::feature_output::FeatureOutput;
pub use super::pipeline::executor::FeaturePipeline;
pub use super::feature_tree::FeatureTree;
pub use super::pipeline::invariants::validate_invariant;
pub use super::operation_space::operation_space::OperationSpace;
pub use super::output::topology_delta::{ArenaSnapshot, compute_topology_delta};

// Transaction lifecycle types
pub use super::transaction::facade::{
    BRepWorkspace, CollectedFinalization, FinalizationError, FinalizationStatus,
    FinalizationSummary, KernelDraft, KernelState, OperationFinalizer, TopologyHashBoundary,
};
