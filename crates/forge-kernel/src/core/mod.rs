//! # Core — Policy engine and tolerance configuration
//!
//! DOMAIN: Kernel-level orchestration — tolerance policies, geometry-layer
//! thresholds, and the `check_tolerance!` macro.
//!
//! ## Modules
//!
//! - `context`   — `ModelingContext` (policy decisions + decision logging)
//! - `tolerance`  — `ToleranceConfig` + all `*Policy` structs with defaults
//! - `macros`     — `check_tolerance!` macro for doctrine D2
//!
//! INVARIANTS: Every tolerance decision is logged (D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision)

mod context;
pub mod tolerance;
mod macros;
mod brep_workspace;
mod operation_space;
mod finalization;
mod naming_resolution;

pub use context::ModelingContext;
pub use context::{ArenaSnapshot, SubOperationMetadata, compute_topology_delta};
pub use tolerance::{
    TolerancePolicy,
    TangencyPolicy,
    SliverPolicy,
    GapClosurePolicy,
    PrecisionEscalationPolicy,
    ToleranceConfig,
};
pub use brep_workspace::BRepWorkspace;

pub mod kernel_state;
pub use kernel_state::KernelState;

pub mod kernel_draft;
pub use kernel_draft::KernelDraft;
pub use operation_space::OperationSpace;
pub use finalization::{
    OperationFinalizer, FinalizationError, FinalizationStatus,
    TopologyHashBoundary, FinalizationEmitOptions, FinalizationSummary,
    CollectedFinalization,
};
pub use naming_resolution::{
    ResolutionQuery, ResolutionResult, ResolutionCandidate, ResolutionCandidates,
    ResolutionEvidence, ResolutionIncompatibility, ResolverRoute, ResolverMatchKind,
    snapshot_ref_from_entity_key, build_resolution_decision,
};
