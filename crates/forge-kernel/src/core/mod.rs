//! Core kernel infrastructure — shared across all features.
//!
//! DOMAIN: Kernel-wide modeling context and policy management.
//! INVARIANTS: None feature-specific — this is pure infrastructure.
//! DEPENDENCIES: forge-math (error types, sign types)

mod context;

pub use context::ModelingContext;
pub use context::TolerancePolicy;
pub use context::TangencyPolicy;
pub use context::SliverPolicy;
pub use context::GapClosurePolicy;
pub use context::PrecisionEscalationPolicy;
pub use context::ToleranceConfig;
pub use forge_core::ToleranceDecision;
pub use forge_core::DecisionKind;
pub use forge_core::DecisionId;
pub use forge_core::DecisionLog;
pub use forge_core::OperationResult;
pub use forge_core::KernelWarning;
pub use forge_core::OperationMetrics;
pub use forge_core::LineageDelta;
pub use forge_core::PolicyKind;
pub use forge_core::PolicyQuery;
pub use forge_core::PolicyResult;
