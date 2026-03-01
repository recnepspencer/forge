//! Public API for the feature engine subsystem.
//!
//! External code should prefer these exports over deep engine internals.

pub use super::contract::{
    AuditLevel, EntityOriginKind, EulerOpKind, FeatureContract, FeatureInputs, InvariantKind,
    SurfaceKind,
};
pub use super::errors::PipelineError;
pub use super::invariants::validate_invariant;
pub use super::tree::{Feature, FeatureOutput, FeatureTree, NativeFeature};
