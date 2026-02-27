//! Shared step implementations for the Forge operation pipeline.
//!
//! DOMAIN: Cross-feature callable functions — one per declared step in
//! `operations/pipeline/steps.rs`. These are the actual implementations
//! that step closures call from within `OperationPipeline::run_step`.
//!
//! CONSUMERS: operations/boolean, operations/fillet, operations/chamfer,
//! operations/shell, operations/extrude, operations/loft, operations/sweep
//!
//! INVARIANTS:
//! - Every function is pure (no side effects beyond ModelingContext logging)
//! - Every function returns `Result<_, KernelError>` — no panics (D5)
//! - Policy guards live in the step contract; implementations assume policies
//!   have already been validated by `OperationPipeline::run_step`

pub mod apply_euler_ops;
pub mod certify_boundary;
pub mod classify_edge_convexity;
pub mod classify_surface_pair;
pub mod construct_surface;
pub mod detect_slivers;
pub mod resolve_persistent_selection;
pub mod validate_manifold;
