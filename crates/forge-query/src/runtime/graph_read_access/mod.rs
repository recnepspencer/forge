mod access_requirements;
mod access_shape;
mod basis_binding;
mod boolean_expression;
mod compile_fail_boundary;
mod cost_model;
mod explanation_api;
mod operation_resolution;
mod schema_reference_admission;
mod schema_reference_evidence;
mod selectivity_normalization;
mod selectivity_shape;
mod selectivity_vocabulary;
mod shape_derivation;
mod shape_explanation;
mod vocabulary;

pub(crate) use basis_binding::bind_graph_read_basis_for_read_graph;
pub(crate) use boolean_expression::admit_boolean_predicate_expression_for_read_graph;
pub(crate) use operation_resolution::resolve_graph_read_operations_for_read_graph;
pub(crate) use schema_reference_admission::admit_query_schema_references_for_read_graph;
pub(crate) use selectivity_normalization::normalize_boolean_selectivity_for_access_shape;
pub(crate) use shape_derivation::derive_graph_read_access_shape;

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
};
pub use cost_model::{
    estimate_graph_read_access_cost, ForgeQueryGraphReadAccessCostEstimate,
    ForgeQueryGraphReadAccessCostEstimateDigest, ForgeQueryGraphReadBudget,
    ForgeQueryGraphReadBudgetCheck, ForgeQueryGraphReadBudgetClass,
    ForgeQueryGraphReadBudgetClassKind, ForgeQueryGraphReadBudgetDigest,
    ForgeQueryGraphReadComplexityContract, ForgeQueryGraphReadComplexityContractKind,
    ForgeQueryGraphReadCostEstimateCounters, ForgeQueryGraphReadCostEstimateStatus,
    ForgeQueryGraphReadCostEstimateStatusKind, ForgeQueryGraphReadCostEvidence,
    ForgeQueryGraphReadIntrinsicCostEstimate, ForgeQueryGraphReadMemoryByteEstimate,
    ForgeQueryGraphReadSupportedCostEstimate,
};
pub use explanation_api::{
    derive_graph_read_access_requirements, explain_boolean_selectivity_shape_for_family,
    explain_boolean_selectivity_shape_for_family_with_operation_registry,
    explain_graph_read_access_requirement_outcome_for_family,
    explain_graph_read_access_requirement_outcome_for_family_with_operation_registry,
    explain_graph_read_access_requirements_for_family,
    explain_graph_read_access_requirements_for_family_with_operation_registry,
    explain_graph_read_access_shape_for_family,
    explain_graph_read_access_shape_for_family_with_operation_registry,
    resolve_graph_read_operations_for_family_with_registry,
    try_derive_graph_read_access_requirements, ForgeQueryGraphReadAccessShapeExplanationError,
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
pub use vocabulary::{
    ForgeQueryGraphReadAccessShapeDigest, ForgeQueryGraphReadFanoutPosture,
    ForgeQueryGraphReadLifecycleClass, ForgeQueryGraphReadOrderingPosture,
    ForgeQueryGraphReadPredicateFamily, ForgeQueryGraphReadResultPressure,
    ForgeQueryGraphReadRootPosture, ForgeQueryGraphReadTraversalOperator,
};
