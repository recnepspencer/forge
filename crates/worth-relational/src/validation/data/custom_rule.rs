mod errors;
mod execution_context;
mod registration;
mod scope_planner;
mod structural_views;
#[cfg(test)]
mod tests;
mod touched_scope;
mod traversal;

pub use errors::{
    CustomInvariantExecutionError, CustomInvariantPreparationError, CustomInvariantVerdict,
};
pub use execution_context::{CustomInvariantExecutionContext, CustomInvariantProvenance};
pub use registration::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
};
pub use scope_planner::CustomInvariantScopePlanner;
pub use structural_views::{StructuralRelationRecord, StructuralRelationView};
#[cfg(test)]
pub use touched_scope::TouchedStructuralSet;
pub use touched_scope::{
    CustomInvariantTouchedSummary, PlannedRelationEndpointUpdate, StructuralCountView,
};
pub use traversal::CustomInvariantTraversalSummary;

pub(crate) use errors::{
    CustomInvariantFailure, CustomInvariantFailureKind, CustomInvariantRuntimePhase,
    PreparedCustomInvariantExecutionOutcome,
};
pub(crate) use registration::PreparedCustomInvariantExecution;
