#![allow(unused_imports)]

mod catalog;
mod contracts;
mod custom_rule;
mod descriptor;
mod execution;
mod groups;
mod results;
mod rule_id;
mod rules;

pub(crate) use catalog::{payload_schema_registration, relation_integrity_registrations_for_plan};
pub use catalog::{InvariantCatalog, InvariantRegistration};
pub use contracts::InvariantPlanContract;
pub use custom_rule::{
    BoundedStructuralTraversal, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantPreparationError, CustomInvariantProvenance, CustomInvariantRegistration,
    CustomInvariantRegistrationError, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantTouchedSummary, CustomInvariantTraversalError, CustomInvariantTraversalSummary,
    CustomInvariantVerdict, PlannedEntityCreate, PlannedRelationCreate, StructuralCountView,
    StructuralPayloadView, StructuralRelationRecord, StructuralRelationView,
    StructuralTraversalResult, TouchedStructuralSet,
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
    InvariantAdvisory, InvariantViolation, InvariantViolationFields, RelationCardinalityBoundary,
    RelationEndpointBoundary,
};
pub use rule_id::{
    CustomInvariantRuleId, CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion,
    InvariantRuleId, NativeInvariantRuleId,
};
pub use rules::{InvariantRule, RecordKindTag};
