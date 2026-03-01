//! Step contract types and macro for the operation pipeline.
//!
//! DOMAIN: Compiler-enforced declarations for individual operation steps.
//! Each step declares which policies it needs and whether it is
//! precision-sensitive. The pipeline validates these before execution.
//!
//! DEPENDENCIES: forge-core (PolicyKind)

use forge_core::PolicyKind;

/// Compiler-enforced contract for an operation step.
///
/// Each step in an operation declares:
/// - Which policies must be configured (validated before execution)
/// - Whether it uses floating-point geometry (for precision escalation)
pub trait StepContract {
    /// Machine-readable step name (e.g. "classify_surface_pair").
    fn step_name(&self) -> &'static str;

    /// Policy kinds that must be configured before this step can execute.
    fn policy_queries(&self) -> &[PolicyKind];

    /// Whether this step performs precision-sensitive geometry operations.
    ///
    /// Steps marked `true` benefit from precision escalation (float →
    /// compensated → interval → rational) when the operation demands it.
    fn precision_sensitive(&self) -> bool;
}

/// Audit entry for a single step within an operation pipeline.
#[derive(Debug, Clone)]
pub struct StepAuditEntry {
    /// Step name (from `StepContract::step_name`).
    pub name: String,
    /// Number of decisions recorded during this step.
    pub decision_count: usize,
    /// Whether this step was precision-sensitive.
    pub precision_sensitive: bool,
}

/// Audit record for an entire operation pipeline.
#[derive(Debug, Clone)]
pub struct OperationAuditRecord {
    /// Audit entries for each step, in execution order.
    pub steps: Vec<StepAuditEntry>,
}

impl OperationAuditRecord {
    /// Total number of decisions across all steps.
    pub fn total_decisions(&self) -> usize {
        self.steps.iter().map(|s| s.decision_count).sum()
    }

    /// Number of steps executed.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// Implement `StepContract` for a step struct with declarative syntax.
///
/// # Example
///
/// ```ignore
/// declare_step!(ClassifySurfacePair,
///     name: "classify_surface_pair",
///     policies: [PolicyKind::CoincidentGeometry, PolicyKind::NearTangency],
///     precision_sensitive: true,
/// );
/// ```
#[macro_export]
macro_rules! declare_step {
    ($step_ty:ty,
        name: $name:expr,
        policies: [$($policy:expr),* $(,)?],
        precision_sensitive: $prec:expr $(,)?
    ) => {
        impl $crate::operations::pipeline::step_contract::StepContract for $step_ty {
            fn step_name(&self) -> &'static str {
                $name
            }

            fn policy_queries(&self) -> &[forge_core::PolicyKind] {
                &[$($policy),*]
            }

            fn precision_sensitive(&self) -> bool {
                $prec
            }
        }
    };
}
