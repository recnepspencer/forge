mod access_admission;
mod access_authority_context;
mod access_requirements;
mod access_shape;
mod async_materialized_read;
mod basis_binding;
mod boolean_expression;
mod compile_fail_boundary;
mod cost_model;
mod ephemeral_index_provisioning;
mod explanation_api;
mod graph_index_inventory;
mod live_maintenance;
mod operation_resolution;
mod persistent_index_requirement;
mod schema_reference_admission;
mod schema_reference_evidence;
mod selectivity_normalization;
mod selectivity_shape;
mod selectivity_vocabulary;
mod shape_derivation;
mod shape_explanation;
mod streaming_frontier_execution;
mod vocabulary;

pub(crate) use access_admission::{
    admit_graph_read_access_for_family_in_authority_with_inventory,
    admit_graph_read_access_for_family_with_inventory,
};
pub(crate) use boolean_expression::admit_boolean_predicate_expression_for_read_graph;
pub(crate) use graph_index_inventory::match_graph_index_inventory_for_requirements;
pub(crate) use operation_resolution::resolve_graph_read_operations_for_read_graph;
pub(crate) use schema_reference_admission::admit_query_schema_references_for_read_graph;
pub(crate) use selectivity_normalization::normalize_boolean_selectivity_for_access_shape;
pub(crate) use shape_derivation::derive_graph_read_access_shape;

pub(crate) use access_admission::ForgeQueryGraphReadAccessExecutionRecorder;
pub use access_admission::{
    admit_graph_read_access_for_family, admit_graph_read_access_for_family_in_authority,
    plan_admitted_graph_read_access_for_family,
    plan_admitted_graph_read_access_for_family_in_authority, ForgeQueryAdmittedGraphReadAccessPlan,
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessCase, ForgeQueryGraphReadAccessCaseRegistry,
    ForgeQueryGraphReadAccessDenial, ForgeQueryGraphReadAccessDenialKind,
    ForgeQueryGraphReadAccessExecutionCounters, ForgeQueryGraphReadAccessInventoryMatch,
    ForgeQueryGraphReadAccessPlanConsumption, ForgeQueryGraphReadAccessPlanExplanation,
    ForgeQueryGraphReadBudgetExceededDenial, ForgeQueryGraphReadPersistentArtifactAudit,
    ForgeQueryGraphReadRequiredCapabilityOwner,
};
pub use access_authority_context::{
    admit_graph_read_access_authority,
    admit_graph_read_access_authority_from_policy_tenant_request,
    ForgeQueryGraphReadAccessAuthorityContext, ForgeQueryGraphReadAccessAuthorityCounters,
    ForgeQueryGraphReadAccessAuthorityDenial, ForgeQueryGraphReadAccessAuthorityDenialKind,
    ForgeQueryGraphReadAccessAuthorityReceipt, ForgeQueryGraphReadAccessAuthorityRequest,
    ForgeQueryGraphReadAccessBasisScope, ForgeQueryGraphReadAccessBasisScopeKind,
    ForgeQueryGraphReadPolicyTenantAuthorityRequest,
};
pub use access_requirements::{
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementCounters,
    ForgeQueryGraphReadAccessRequirementDerivationError,
    ForgeQueryGraphReadAccessRequirementExplanationOutcome,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow,
    ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadAccessRequirementSetDigest,
    ForgeQueryGraphReadOrderingFieldAuthority, ForgeQueryGraphReadPredicateFieldAuthority,
    ForgeQueryGraphReadRelationAuthority,
};
pub use access_shape::ForgeQueryGraphReadAccessShape;
pub use async_materialized_read::{
    ForgeQueryGraphReadCheckpointInterval, ForgeQueryGraphReadMaterializationAdmittedJob,
    ForgeQueryGraphReadMaterializationAdmittedLimits,
    ForgeQueryGraphReadMaterializationCancellationReceipt,
    ForgeQueryGraphReadMaterializationCheckpoint, ForgeQueryGraphReadMaterializationCounters,
    ForgeQueryGraphReadMaterializationJob, ForgeQueryGraphReadMaterializationJobState,
    ForgeQueryGraphReadMaterializationPolicy, ForgeQueryGraphReadMaterializationProgress,
    ForgeQueryGraphReadMaterializationReceipt, ForgeQueryGraphReadMaterializationRecoveryHandle,
    ForgeQueryGraphReadMaterializationRequest, ForgeQueryGraphReadMaterializationRequestError,
    ForgeQueryGraphReadMaterializationResourceLimitReceipt,
    ForgeQueryGraphReadMaterializationRuntime, ForgeQueryGraphReadMaterializedArtifact,
    ForgeQueryGraphReadMaterializedRowProof,
};
pub use basis_binding::{
    ForgeQueryGraphReadBasisBinding, ForgeQueryGraphReadBasisPosture,
    ForgeQueryGraphReadPolicyTenantPosture, ForgeQueryGraphReadPolicyTenantProofBinding,
    ForgeQueryGraphReadRelationshipProofBindingPosture,
};
pub use boolean_expression::{
    ForgeQueryAdmittedBooleanExpressionBranch, ForgeQueryAdmittedBooleanExpressionBranchKind,
    ForgeQueryAdmittedBooleanExpressionCounters, ForgeQueryAdmittedBooleanExpressionTopology,
    ForgeQueryAdmittedBooleanPredicateExpression, ForgeQueryAdmittedBooleanPredicateLeaf,
    ForgeQueryBooleanExpressionAdmissionError, ForgeQueryBooleanExpressionAdmissionErrorKind,
};
pub use compile_fail_boundary::{
    forge_query_graph_read_access_compile_fail_boundary_digest,
    forge_query_graph_read_access_compile_fail_target_count,
    forge_query_graph_read_access_compile_fail_targets,
    forge_query_graph_read_proof_transition_manifest,
    forge_query_graph_read_proof_transition_manifest_count,
    forge_query_graph_read_proof_transition_manifest_digest,
    ForgeQueryGraphReadProofBoundaryEvidenceKind, ForgeQueryGraphReadProofTransitionManifestRow,
};
pub use cost_model::{
    derive_graph_read_cost_evidence, estimate_graph_read_access_cost,
    estimate_graph_read_access_cost_with_planning_observation,
    ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadAccessCostEstimateDigest,
    ForgeQueryGraphReadBudget, ForgeQueryGraphReadBudgetCheck, ForgeQueryGraphReadBudgetClass,
    ForgeQueryGraphReadBudgetClassKind, ForgeQueryGraphReadBudgetDigest,
    ForgeQueryGraphReadComplexityContract, ForgeQueryGraphReadComplexityContractKind,
    ForgeQueryGraphReadCostAttributionRow, ForgeQueryGraphReadCostEstimateCounters,
    ForgeQueryGraphReadCostEstimateStatus, ForgeQueryGraphReadCostEstimateStatusKind,
    ForgeQueryGraphReadCostEvidence, ForgeQueryGraphReadInlineEphemeralAllowance,
    ForgeQueryGraphReadInlineEphemeralAllowanceKind, ForgeQueryGraphReadIntrinsicCostContribution,
    ForgeQueryGraphReadIntrinsicCostEstimate, ForgeQueryGraphReadMemoryByteEstimate,
    ForgeQueryGraphReadObservedCostEstimate, ForgeQueryGraphReadPlanningObservation,
    ForgeQueryGraphReadSupportedCostContribution, ForgeQueryGraphReadSupportedCostEstimate,
};
pub(crate) use ephemeral_index_provisioning::provision_ephemeral_graph_indexes_for_read_execution;
pub use ephemeral_index_provisioning::{
    ForgeQueryEphemeralGraphIndex, ForgeQueryEphemeralGraphIndexAllocationRow,
    ForgeQueryEphemeralGraphIndexCounters, ForgeQueryEphemeralGraphIndexLifecycleRegistry,
    ForgeQueryEphemeralGraphIndexPlan, ForgeQueryEphemeralGraphIndexProvisioningError,
    ForgeQueryEphemeralGraphIndexReceipt, ForgeQueryEphemeralGraphIndexScope,
    ForgeQueryEphemeralGraphIndexScopeKind,
};
pub use explanation_api::{
    derive_graph_read_access_requirements, explain_boolean_selectivity_shape_for_family,
    explain_boolean_selectivity_shape_for_family_with_operation_registry,
    explain_graph_read_access_requirement_outcome_for_family,
    explain_graph_read_access_requirement_outcome_for_family_in_authority,
    explain_graph_read_access_requirement_outcome_for_family_with_operation_registry,
    explain_graph_read_access_requirements_for_family,
    explain_graph_read_access_requirements_for_family_in_authority,
    explain_graph_read_access_requirements_for_family_with_operation_registry,
    explain_graph_read_access_shape_for_family,
    explain_graph_read_access_shape_for_family_in_authority,
    explain_graph_read_access_shape_for_family_with_operation_registry,
    resolve_graph_read_operations_for_family_in_authority_with_registry,
    resolve_graph_read_operations_for_family_with_registry,
    try_derive_graph_read_access_requirements, ForgeQueryGraphReadAccessShapeExplanationError,
};
pub use graph_index_inventory::{
    forge_query_graph_index_inventory, match_current_graph_index_inventory_for_requirements,
    ForgeQueryGraphIndexInventory, ForgeQueryGraphIndexInventoryCounters,
    ForgeQueryGraphIndexInventoryMatch, ForgeQueryGraphIndexInventoryMatchOutcome,
    ForgeQueryGraphIndexInventoryMatchReport, ForgeQueryGraphIndexLifecycleClass,
    ForgeQueryGraphIndexLifecycleOwner, ForgeQueryGraphIndexPosture,
    ForgeQueryGraphIndexSupportRow, ForgeQueryGraphIndexSupportState,
};
pub use live_maintenance::{
    ForgeQueryLiveGraphReadAccessDenial, ForgeQueryLiveGraphReadAccessPlan,
    ForgeQueryLiveGraphReadAccessPosture, ForgeQueryLiveGraphReadAccessReceipt,
    ForgeQueryLiveGraphReadMaintenanceBudget, ForgeQueryLiveGraphReadMaintenanceCounters,
    ForgeQueryLiveGraphReadMaintenanceReceipt, ForgeQueryLiveGraphReadMutationDeltaScope,
};
pub use operation_resolution::{
    ForgeQueryBuiltInGraphReadOperation, ForgeQueryDomainRegisteredGraphReadOperation,
    ForgeQueryGraphReadOperationCapabilityRequirement,
    ForgeQueryGraphReadOperationCapabilityRequirementDeclaration,
    ForgeQueryGraphReadOperationCapabilityRequirementKind, ForgeQueryGraphReadOperationOutcome,
    ForgeQueryGraphReadOperationRegistration, ForgeQueryGraphReadOperationRegistry,
    ForgeQueryGraphReadOperationResolution, ForgeQueryGraphReadOperationUnsupportedDenial,
    ForgeQueryGraphReadOperationUnsupportedDenialKind,
    ForgeQueryGraphReadOperationUnsupportedShapeDeclaration,
    ForgeQueryGraphReadRegistryAdmissionError, ForgeQueryGraphReadResolvedOperation,
    ForgeQueryGraphReadResolvedOperationFamily, ForgeQueryGraphReadResolvedOperationKind,
};
pub use persistent_index_requirement::{
    ForgeQueryGraphReadFamilyIndexContract, ForgeQueryPersistentGraphIndexRequirementCounters,
    ForgeQueryPersistentGraphIndexRequirementDeclaration,
    ForgeQueryPersistentGraphIndexRequirementReceipt, ForgeQueryPersistentGraphIndexRequirementRow,
};
pub use schema_reference_evidence::{
    ForgeQueryAdmittedGraphReadOrderingField, ForgeQueryAdmittedGraphReadPredicateField,
    ForgeQueryAdmittedGraphReadProjectionField, ForgeQueryAdmittedGraphReadRelation,
    ForgeQueryAdmittedGraphReadRelationDirection, ForgeQueryAdmittedQuerySchemaReferences,
    ForgeQueryGraphReadAdmittedSchemaFieldKind, ForgeQueryGraphReadSchemaReferenceAdmissionError,
    ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind,
};
pub use selectivity_shape::{
    ForgeQueryBooleanPredicateSelectivityRow, ForgeQueryBooleanSelectivityBranch,
    ForgeQueryBooleanSelectivityCounters, ForgeQueryBooleanSelectivityShape,
};
pub use selectivity_vocabulary::{
    ForgeQueryBooleanPredicateTopology, ForgeQueryBooleanSelectivityAdmissionPosture,
    ForgeQueryBooleanSelectivityBranchKind, ForgeQueryBooleanSelectivityShapeDigest,
    ForgeQueryPredicateAnchorPosture, ForgeQueryPredicateOperandOperator,
    ForgeQueryPredicateSelectivityClass, ForgeQueryTraversalPredicateOrderingPosture,
};
pub use shape_explanation::{
    ForgeQueryGraphReadAccessShapeDerivationCounters, ForgeQueryGraphReadAccessShapeExplanation,
};
pub(crate) use streaming_frontier_execution::{
    streaming_frontier_is_admissible, streaming_receipt_for_admitted_read_result,
};
pub use streaming_frontier_execution::{
    ForgeQueryGraphReadFrontierCursor, ForgeQueryGraphReadStreamingCounters,
    ForgeQueryGraphReadStreamingCursorDenial, ForgeQueryGraphReadStreamingCursorDenialKind,
    ForgeQueryGraphReadStreamingCursorSession, ForgeQueryGraphReadStreamingPageBudget,
    ForgeQueryGraphReadStreamingPageReceipt, ForgeQueryGraphReadStreamingPlan,
    ForgeQueryGraphReadStreamingReceipt,
};
pub use vocabulary::{
    ForgeQueryGraphReadAccessShapeDigest, ForgeQueryGraphReadFanoutPosture,
    ForgeQueryGraphReadLifecycleClass, ForgeQueryGraphReadOrderingPosture,
    ForgeQueryGraphReadPredicateFamily, ForgeQueryGraphReadResultPressure,
    ForgeQueryGraphReadRootPosture, ForgeQueryGraphReadTraversalOperator,
};
