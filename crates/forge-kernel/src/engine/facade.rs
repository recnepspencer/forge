//! Public API for the feature engine subsystem.
//!
//! External code should prefer these exports over deep engine internals.

pub use super::contracts::contract::{
    AuditLevel, EntityOriginKind, EulerOpKind, FeatureContract, FeatureInputs, InvariantKind,
    SurfaceKind,
};
pub use super::contracts::feature_trait::Feature;
pub use super::contracts::feature_registry::FeatureRegistry;
pub use super::data::feature_output::FeatureOutput;
pub use super::logic::executor::FeaturePipeline;
pub use super::logic::feature_tree::FeatureTree;
pub use super::logic::invariants::validate_invariant;
pub use super::logic::operation_space::OperationSpace;

// Transaction lifecycle types
pub use super::transaction::facade::{
    BRepWorkspace, CollectedFinalization, FinalizationError, FinalizationStatus,
    FinalizationSummary, KernelDraft, KernelState, OperationFinalizer, TopologyHashBoundary,
};
