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
    CustomInvariantExecutionError, CustomInvariantPreparationError, CustomInvariantTraversalError,
    CustomInvariantVerdict,
};
pub use execution_context::{CustomInvariantExecutionContext, CustomInvariantProvenance};
pub use registration::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
};
pub use scope_planner::CustomInvariantScopePlanner;
pub use structural_views::{
    StructuralAspectStateView, StructuralRelationRecord, StructuralRelationView,
};
pub use touched_scope::{
    CustomInvariantTouchedSummary, PlannedEntityCreate, PlannedRelationCreate,
    PlannedRelationEndpointUpdate, StructuralCountView, TouchedStructuralSet,
};
pub use traversal::{
    BoundedStructuralTraversal, CustomInvariantTraversalSummary, StructuralTraversalResult,
};

pub(crate) use errors::{
    CustomInvariantFailure, CustomInvariantFailureKind, CustomInvariantRuntimePhase,
    PreparedCustomInvariantExecutionOutcome,
};
pub(crate) use registration::PreparedCustomInvariantExecution;
