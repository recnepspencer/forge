#![allow(unused_imports)]

mod aspect_field_target;
mod catalog;
mod contracts;
mod custom_rule;
mod descriptor;
mod execution;
mod groups;
mod results;
mod rule_id;
mod rules;

pub use aspect_field_target::UniqueEntityAspectField;
pub(crate) use catalog::relation_integrity_registrations_for_plan;
pub use catalog::{InvariantCatalog, InvariantRegistration};
pub use contracts::InvariantPlanContract;
pub use custom_rule::{
    BoundedStructuralTraversal, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantPreparationError, CustomInvariantProvenance, CustomInvariantRegistration,
    CustomInvariantRegistrationError, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantTouchedSummary, CustomInvariantTraversalError, CustomInvariantTraversalSummary,
    CustomInvariantVerdict, PlannedEntityCreate, PlannedRelationCreate,
    PlannedRelationEndpointUpdate, StructuralAspectStateView, StructuralCountView,
    StructuralRelationRecord, StructuralRelationView, StructuralTraversalResult,
    TouchedStructuralSet,
};
pub(crate) use custom_rule::{
    CustomInvariantFailure, CustomInvariantFailureKind, CustomInvariantRuntimePhase,
    ErasedCustomInvariantRule, PreparedCustomInvariantExecution,
    PreparedCustomInvariantExecutionOutcome,
};
pub use descriptor::{
    CustomInvariantDescriptor, CustomInvariantOperationalMetadata, InvariantRuleDescriptor,
    InvariantSemanticsClass, SupportedExecutionPoints,
};
pub use execution::{
    InvariantCheckResult, InvariantClass, InvariantDecisionKind, InvariantDecisionRecord,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantReportedRule, InvariantVerdict,
    InvariantWitnessKey,
};
pub use groups::{InvariantCostClass, InvariantGroup, InvariantGroupSet};
pub use results::{
    CustomInvariantFailureIdentity, CustomInvariantFailureKind as ResultCustomInvariantFailureKind,
    CustomInvariantFailurePhase, InvariantAdvisory, InvariantViolation, InvariantViolationFields,
    RelationCardinalityBoundary, RelationEndpointBoundary, StorageInconsistencyFailure,
    StorageInconsistencyLookup, StorageInconsistencyScan,
};
pub use rule_id::{
    CustomInvariantRuleId, CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion,
    InvariantRuleId, NativeInvariantRuleId,
};
pub use rules::{InvariantRule, RecordKindTag};
