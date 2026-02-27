//! # Core — Policy engine and tolerance configuration
//!
//! DOMAIN: Kernel-level orchestration — tolerance policies, geometry-layer
//! thresholds, and the `check_tolerance!` macro.
//!
//! ## Modules
//!
//! - `context/`   — `ModelingContext` split into schema, accessors, decision logging,
//!                  sub-operations, counterfactual replay, policy resolution, topology delta
//! - `tolerance`  — `ToleranceConfig` + all `*Policy` structs with defaults
//! - `macros`     — `check_tolerance!` macro for doctrine D2
//!
//! INVARIANTS: Every tolerance decision is logged (D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision)

mod brep_workspace;
pub mod config;
pub(crate) mod context;
pub(crate) mod finalization;
mod macros;
mod naming_resolution;
mod operation_space;
pub mod tolerance;
pub mod tracing;

#[cfg(test)]
mod architecture_guard_tests;

pub use brep_workspace::BRepWorkspace;
pub use context::ModelingContext;
pub use context::{compute_topology_delta, ArenaSnapshot, SubOperationMetadata};
pub use context::{ResolvedPolicyDecision, ResolvedPolicySource};
pub use tolerance::{
    GapClosurePolicy, PrecisionEscalationPolicy, SliverPolicy, TangencyPolicy, ToleranceConfig,
    TolerancePolicy,
};

pub mod kernel_state;
pub use kernel_state::KernelState;

pub mod kernel_draft;
pub use finalization::{
    CollectedFinalization, FinalizationEmitOptions, FinalizationError, FinalizationStatus,
    FinalizationSummary, OperationFinalizer, TopologyHashBoundary,
};
pub use kernel_draft::KernelDraft;
pub use naming_resolution::{
    build_resolution_decision, snapshot_ref_from_entity_key, ResolutionCandidate,
    ResolutionCandidates, ResolutionEvidence, ResolutionIncompatibility, ResolutionQuery,
    ResolutionResult, ResolverMatchKind, ResolverRoute,
};
pub use operation_space::OperationSpace;
