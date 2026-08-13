//! Runtime-bound custom invariant planning and execution.

mod execution_context;
mod registration;
mod scope_planner;
mod structural_views;
#[cfg(test)]
mod tests;
mod touched_scope_collection;
mod traversal;

pub use execution_context::{CustomInvariantExecutionContext, CustomInvariantProvenance};
pub use registration::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
};
pub use scope_planner::CustomInvariantScopePlanner;
pub(crate) use scope_planner::PreparedCustomInvariantScope;
pub use structural_views::{StructuralRelationRecord, StructuralRelationView};
pub use traversal::CustomInvariantTraversalSummary;

pub(crate) use registration::PreparedCustomInvariantExecution;
