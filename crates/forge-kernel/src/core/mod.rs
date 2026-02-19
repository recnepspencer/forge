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

pub use context::ModelingContext;
pub use tolerance::{
    TolerancePolicy,
    TangencyPolicy,
    SliverPolicy,
    GapClosurePolicy,
    PrecisionEscalationPolicy,
    ToleranceConfig,
};
