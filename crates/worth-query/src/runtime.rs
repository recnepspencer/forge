use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, DeclarativeLiveQueryError,
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape,
};
#[allow(unused_imports)]
pub use crate::intent_admission::{
    certify_intent_admission, worth_query_intent_admission_certification_output_manifest,
    worth_query_intent_admission_closeout_extension_outputs,
    worth_query_intent_admission_compile_fail_targets,
    worth_query_intent_admission_coverage_inventory,
    worth_query_intent_admission_crate_doc_example_targets,
    worth_query_intent_admission_doc_example_report, worth_query_intent_admission_family_inventory,
    worth_query_intent_admission_golden_transcripts,
    worth_query_intent_admission_legacy_parity_report, worth_query_intent_admission_mutation_audit,
    worth_query_intent_admission_oracle_report,
    worth_query_intent_admission_representative_family_report,
    worth_query_intent_admission_representative_output_report,
    worth_query_intent_admission_required_certification_outputs,
    worth_query_intent_admission_seeded_certification_report,
    worth_query_intent_admission_slope_report, worth_query_intent_admission_support_matrix,
    worth_query_intent_admission_support_traceability_report,
    WorthQueryAdmittedIntentExecutionHandoff, WorthQueryAdmittedIntentPlan,
    WorthQueryAdmittedRuntimeEffectWriteIntent, WorthQueryAdmittedRuntimeExistingTruthProbeIntent,
    WorthQueryAdmittedRuntimeInspectionIntent, WorthQueryAdmittedRuntimeIntent,
    WorthQueryAdmittedRuntimeWriteBatchIntent, WorthQueryAdmittedRuntimeWriteIntent,
    WorthQueryAdmittedWorkspaceLiveReadIntent, WorthQueryAdmittedWorkspaceReadIntent,
    WorthQueryAuthoritativeIntentExecutionBinding, WorthQueryAuthoritativeIntentExecutionHandoff,
    WorthQueryAuthoritativeIntentExecutionPlan,
    WorthQueryAuthoritativeMutationBatchExecutionBinding,
    WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    WorthQueryAuthoritativeMutationBatchExecutionPlan,
    WorthQueryAuthoritativeMutationBatchIntentSeed,
    WorthQueryAuthoritativeMutationExecutionBinding,
    WorthQueryAuthoritativeMutationExecutionHandoff, WorthQueryAuthoritativeMutationExecutionPlan,
    WorthQueryAuthoritativeMutationIntentSeed, WorthQueryAuthoritativeMutationPreflight,
    WorthQueryEffectTriggeredIntentExecutionBinding,
    WorthQueryEffectTriggeredIntentExecutionHandoff, WorthQueryEffectTriggeredIntentExecutionPlan,
    WorthQueryExistingTruthProbeExecutionBinding, WorthQueryExistingTruthProbeExecutionHandoff,
    WorthQueryExistingTruthProbeExecutionPlan, WorthQueryExistingTruthProbeIntentSeed,
    WorthQueryExistingTruthProbeRoutingPreflight, WorthQueryGenericInspectionIntentSeed,
    WorthQueryGenericInspectionIntentTarget, WorthQueryGenericInspectionIntentTargetSeed,
    WorthQueryIntentAdmissionAuthorityLaneEligibility, WorthQueryIntentAdmissionBasisEligibility,
    WorthQueryIntentAdmissionCapabilityEligibility, WorthQueryIntentAdmissionCertificationBundle,
    WorthQueryIntentAdmissionCertificationCounterSnapshot,
    WorthQueryIntentAdmissionCertificationOutput, WorthQueryIntentAdmissionCompileFailTarget,
    WorthQueryIntentAdmissionCoverageInventory, WorthQueryIntentAdmissionCoverageRow,
    WorthQueryIntentAdmissionCoverageStatus, WorthQueryIntentAdmissionCoveredEntrypoint,
    WorthQueryIntentAdmissionCrateDocExampleTarget, WorthQueryIntentAdmissionDecision,
    WorthQueryIntentAdmissionDecisionClass, WorthQueryIntentAdmissionDocExampleReport,
    WorthQueryIntentAdmissionDocExampleRow, WorthQueryIntentAdmissionEligibility,
    WorthQueryIntentAdmissionEligibilityAuthority, WorthQueryIntentAdmissionExecutionBoundary,
    WorthQueryIntentAdmissionExecutionHandoffInventory, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentAdmissionFamilyInventory,
    WorthQueryIntentAdmissionFamilyInventoryRow, WorthQueryIntentAdmissionGoldenTranscript,
    WorthQueryIntentAdmissionInvariantEligibility, WorthQueryIntentAdmissionLegacyParityCheck,
    WorthQueryIntentAdmissionLegacyParityLane, WorthQueryIntentAdmissionLegacyParityReport,
    WorthQueryIntentAdmissionLegacyParityRow, WorthQueryIntentAdmissionMutationAudit,
    WorthQueryIntentAdmissionMutationAuditRow, WorthQueryIntentAdmissionOracleComparisonRow,
    WorthQueryIntentAdmissionOracleLane, WorthQueryIntentAdmissionOracleManifestRow,
    WorthQueryIntentAdmissionOracleReport, WorthQueryIntentAdmissionPlanKind,
    WorthQueryIntentAdmissionPolicyEligibility, WorthQueryIntentAdmissionPreDecisionPosture,
    WorthQueryIntentAdmissionProjectionSourceEligibility, WorthQueryIntentAdmissionProofShapeAudit,
    WorthQueryIntentAdmissionPublicBoundaryAudit,
    WorthQueryIntentAdmissionRepresentativeFamilyLane,
    WorthQueryIntentAdmissionRepresentativeFamilyReport,
    WorthQueryIntentAdmissionRepresentativeFamilyRow,
    WorthQueryIntentAdmissionRepresentativeOutputReport, WorthQueryIntentAdmissionResultArtifact,
    WorthQueryIntentAdmissionRoutingSupportEligibility,
    WorthQueryIntentAdmissionSeedGeneratorClass, WorthQueryIntentAdmissionSeedReplayRow,
    WorthQueryIntentAdmissionSeededCertificationReport, WorthQueryIntentAdmissionSlopeReport,
    WorthQueryIntentAdmissionSourceLaneEligibility, WorthQueryIntentAdmissionSupportDetail,
    WorthQueryIntentAdmissionSupportEligibility, WorthQueryIntentAdmissionSupportMatrix,
    WorthQueryIntentAdmissionSupportPosture, WorthQueryIntentAdmissionSupportRow,
    WorthQueryIntentAdmissionSupportTraceabilityReport,
    WorthQueryIntentAdmissionSupportTraceabilityRow, WorthQueryIntentAdmissionSurfaceDescriptor,
    WorthQueryIntentAdmissionTopologyAudit, WorthQueryIntentAdmissionTopologyAuditRow,
    WorthQueryIntentAdmissionTopologyDomain, WorthQueryIntentAdvisoryDecision,
    WorthQueryIntentAdvisoryStop, WorthQueryIntentDecisionTraceEnvelope,
    WorthQueryIntentDecisionTraceEnvelopeKind, WorthQueryIntentDecisionTraceRow,
    WorthQueryIntentDecisionTraceStage, WorthQueryIntentNonAdmittedStop,
    WorthQueryIntentViolationDecision, WorthQueryIntentViolationStop,
    WorthQueryLiveReadExecutionBinding, WorthQueryLiveReadExecutionHandoff,
    WorthQueryLiveReadExecutionPlan, WorthQueryLiveReadIntentSeed,
    WorthQueryRawIntentAdmissionRequest, WorthQueryReadExecutionBinding,
    WorthQueryReadExecutionHandoff, WorthQueryReadExecutionIntentSeed, WorthQueryReadExecutionPlan,
    WorthQueryRuntimeEffectWriteIntentAdmissionReview, WorthQueryRuntimeEffectWriteIntentAuthoring,
    WorthQueryRuntimeExistingTruthProbeIntentAdmissionReview,
    WorthQueryRuntimeExistingTruthProbeIntentAuthoring,
    WorthQueryRuntimeInspectionIntentAdmissionReview, WorthQueryRuntimeInspectionIntentAuthoring,
    WorthQueryRuntimeIntentAdmissionReview, WorthQueryRuntimeIntentAuthoring,
    WorthQueryRuntimeWriteBatchIntentAdmissionReview, WorthQueryRuntimeWriteBatchIntentAuthoring,
    WorthQueryRuntimeWriteIntentAdmissionReview, WorthQueryRuntimeWriteIntentAuthoring,
    WorthQueryUnifiedInspectionExecutionBinding, WorthQueryUnifiedInspectionExecutionHandoff,
    WorthQueryUnifiedInspectionExecutionPlan,
    WorthQueryWorkspaceDerivedInspectionIntentAdmissionReview,
    WorthQueryWorkspaceDerivedInspectionIntentAuthoring,
    WorthQueryWorkspaceDerivedMaterializationIntentAdmissionReview,
    WorthQueryWorkspaceDerivedMaterializationIntentAuthoring,
    WorthQueryWorkspaceLiveReadIntentAdmissionReview, WorthQueryWorkspaceLiveReadIntentAuthoring,
    WorthQueryWorkspaceReadIntentAdmissionReview, WorthQueryWorkspaceReadIntentAuthoring,
};
#[allow(unused_imports)]
pub use crate::lower_runtime_routing::{
    certify_lower_runtime_non_bypass, certify_lower_runtime_performance_slopes,
    certify_lower_runtime_routing, inspect_lower_runtime_boundary, inspect_lower_runtime_closeout,
    summarize_lower_runtime_boundary, worth_query_lower_runtime_acceptance_suite,
    worth_query_lower_runtime_boundary_reconciliation_report,
    worth_query_lower_runtime_certification_output_manifest,
    worth_query_lower_runtime_closeout_extension_outputs,
    worth_query_lower_runtime_closeout_registry, worth_query_lower_runtime_closeout_report,
    worth_query_lower_runtime_closeout_report_digest, worth_query_lower_runtime_closure_test,
    worth_query_lower_runtime_compile_fail_boundary_digest,
    worth_query_lower_runtime_compile_fail_boundary_target_count,
    worth_query_lower_runtime_crossing_inventory, worth_query_lower_runtime_direct_import_audit,
    worth_query_lower_runtime_gap_registry, worth_query_lower_runtime_golden_transcripts,
    worth_query_lower_runtime_phase_artifact_manifest_digest,
    worth_query_lower_runtime_phase_manifest, worth_query_lower_runtime_phase_progression_digest,
    worth_query_lower_runtime_proof_shape_audit, worth_query_lower_runtime_proof_shape_digest,
    worth_query_lower_runtime_public_surface_inventory,
    worth_query_lower_runtime_required_certification_outputs,
    worth_query_lower_runtime_support_matrix, worth_query_lower_runtime_synthetic_tail_report,
    worth_query_lower_runtime_target_dx_digest,
    worth_query_lower_runtime_typestate_transition_digest, WorthQueryLowerRuntimeAcceptanceLane,
    WorthQueryLowerRuntimeAcceptanceRow, WorthQueryLowerRuntimeAcceptanceSuite,
    WorthQueryLowerRuntimeArtifactStrength, WorthQueryLowerRuntimeAuthorityOwner,
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource,
    WorthQueryLowerRuntimeBoundaryExecutionKind, WorthQueryLowerRuntimeBoundaryExecutionReceipt,
    WorthQueryLowerRuntimeBoundaryReconciliationReport,
    WorthQueryLowerRuntimeBoundaryReconciliationRow, WorthQueryLowerRuntimeBoundarySummary,
    WorthQueryLowerRuntimeCapabilityEligibility, WorthQueryLowerRuntimeCapabilityPosture,
    WorthQueryLowerRuntimeCapabilityRequest, WorthQueryLowerRuntimeCertificationBundle,
    WorthQueryLowerRuntimeCertificationLane, WorthQueryLowerRuntimeCertificationOutputDigest,
    WorthQueryLowerRuntimeCertificationRow, WorthQueryLowerRuntimeCloseoutPosture,
    WorthQueryLowerRuntimeCloseoutRegistry, WorthQueryLowerRuntimeCloseoutReport,
    WorthQueryLowerRuntimeCloseoutRow, WorthQueryLowerRuntimeClosureTest,
    WorthQueryLowerRuntimeClosureTestLane, WorthQueryLowerRuntimeClosureTestRow,
    WorthQueryLowerRuntimeCostPosture, WorthQueryLowerRuntimeCrossingClassification,
    WorthQueryLowerRuntimeCrossingInventory, WorthQueryLowerRuntimeCrossingRow,
    WorthQueryLowerRuntimeDirectImportAudit, WorthQueryLowerRuntimeDirectImportAuditRow,
    WorthQueryLowerRuntimeDirectImportPosture, WorthQueryLowerRuntimeFailureTopology,
    WorthQueryLowerRuntimeGapRegistry, WorthQueryLowerRuntimeGapRegistryRow,
    WorthQueryLowerRuntimeGoldenTranscript, WorthQueryLowerRuntimeNonBypassAudit,
    WorthQueryLowerRuntimePerformanceFamily, WorthQueryLowerRuntimePerformanceSlopeReport,
    WorthQueryLowerRuntimePerformanceSlopeRow, WorthQueryLowerRuntimePhaseArtifact,
    WorthQueryLowerRuntimePhaseManifest, WorthQueryLowerRuntimePhaseManifestRow,
    WorthQueryLowerRuntimeProofShapeAudit, WorthQueryLowerRuntimeProofShapeAuditRow,
    WorthQueryLowerRuntimeProofShapeEnforcement, WorthQueryLowerRuntimeProofShapeViolation,
    WorthQueryLowerRuntimePublicSurfaceInventory, WorthQueryLowerRuntimePublicSurfaceKind,
    WorthQueryLowerRuntimePublicSurfaceRow, WorthQueryLowerRuntimeReadmissionReceipt,
    WorthQueryLowerRuntimeRouteKind, WorthQueryLowerRuntimeRoutePlan,
    WorthQueryLowerRuntimeRoutingInspection, WorthQueryLowerRuntimeSeamKey,
    WorthQueryLowerRuntimeSupportDetail, WorthQueryLowerRuntimeSupportMatrix,
    WorthQueryLowerRuntimeSupportPosture, WorthQueryLowerRuntimeSupportRow,
    WorthQueryLowerRuntimeSyntheticTailReport, WorthQueryLowerRuntimeSyntheticTailRow,
};
use crate::memory_workspace::{
    WorthQueryEntity, WorthQueryMutationKind, WorthQueryMutationReceipt, WorthQueryWorkspaceError,
};
use crate::program::{
    validate_inputs, WorthQueryAuthorityRequirement, WorthQueryDerivedView,
    WorthQueryOperationInput, WorthQueryOperationOutput, WorthQueryProgram,
    WorthQueryProgramEffect, WorthQueryProgramError, WorthQueryProgramTrace,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::WorthQuerySessionLabel;
use crate::subscription::{
    admit_active_subscription_lane, admit_query_subscription, attach_subscription_consumer,
    close_subscription_lifecycle, declare_query_subscription, lower_query_subscription_to_bridge,
    open_active_subscription_lane, prepare_subscription_activation,
    select_query_subscription_family, ActiveAllocationScopeWidth, ActiveFanoutWidth,
    ActiveRegistryLookupWidth, ActiveSubscriptionAllocationPosture, ActiveSubscriptionRuntime,
    ActiveSubscriptionWorkBudget, ConsumerDeliveryPacingWidth, DeliveryBackpressurePolicy,
    QuerySubscriptionAdmissionBudget, QuerySubscriptionAdmissionDimensions,
    QuerySubscriptionBridgeLoweringBudget, QuerySubscriptionSliceBudget,
    QuerySubscriptionWorkBudget, SubscriptionConsumerAttachmentBudget,
    SubscriptionConsumerAttachmentRequest, SubscriptionLifecycleCloseRequest,
};
use crate::view_shape_live::LiveViewShapeFamily;
use worth_relational::facade::runtime::RelationalRuntime;
pub use worth_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
    CustomInvariantRuleId, CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
    CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCatalog, InvariantCostClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
    InvariantRegistration, InvariantRule,
};
use worth_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeNamingMutationBundle,
    BridgeSymbolicTargetReferenceBundle, RuntimeBridge,
};

mod aspect_api_closeout;
mod async_result_state;
mod authoritative_mutation_evidence_bridge_alignment;
mod authoritative_mutation_evidence_closeout;
mod authoritative_mutation_evidence_support;
mod authoritative_mutation_evidence_support_bridge;
mod authority;
mod backend;
mod branch;
mod bridge_mutation_lowering;
mod builder;
mod computed;
mod concurrent_hostile_matrix;
mod delivery;
mod downstream_delivery_contract;
mod downstream_delivery_resume;
mod effect;
mod error;
mod evidence_identities;
#[cfg(test)]
pub(crate) use evidence_identities::{
    runtime_state_snapshot_basis_label_identity, runtime_state_snapshot_result_shape_label_identity,
};
#[cfg(test)]
mod fallback_seam_counters;
mod graph_obligation_registration;
mod graph_read_access;
mod handle_contract;
mod inspection;
mod intent;
mod journal_position;
mod journal_replay;
mod live_subscription;
mod live_subscription_accessors;
mod live_subscription_delivery_routing;
mod managed_live_resource;
mod materialized_fact_posture;
mod mixed_cause_delivery;
mod mixed_cause_emission;
mod mutation;
mod mutation_surface;
mod ordinary_runtime_posture;
mod preview;
mod public_api;
mod published_artifacts;
mod read_composition;
mod read_composition_builder_shared;
mod read_composition_builder_walks;
mod read_composition_current_execution;
mod read_composition_frontier;
mod read_composition_frontier_search;
mod read_composition_hooks;
mod read_composition_lowering;
mod read_composition_materialization;
mod read_composition_operator_builders;
mod read_composition_phase_gate;
mod read_composition_phase_one_closeout;
mod read_composition_relationship_proof;
mod read_composition_row_selection;
mod read_composition_runtime;
mod read_composition_shared;
mod read_composition_successor;
mod read_composition_support_report;
mod read_composition_walks;
mod remask_posture;
mod runtime_api_contract;
mod runtime_authoritative_mutation_obligation_dispatch;
mod runtime_authoritative_mutation_routing;
mod runtime_batch_write_entrypoints;
mod runtime_batch_write_intents;
mod runtime_batch_writes;
mod runtime_batching;
mod runtime_declarations;
mod runtime_helpers;
mod runtime_inspection;
mod runtime_inspection_materialization_identity;
mod runtime_inspection_materialization_intents;
mod runtime_intent_phase_four_execution;
mod runtime_intent_phase_three_resolution;
mod runtime_intents;
mod runtime_journal_replay;
mod runtime_live_read_intents;
mod runtime_non_authoritative_obligation_dispatch;
mod runtime_probe_routing_intents;
mod runtime_read_access_plan;
mod runtime_read_execution_receipts;
mod runtime_read_intents;
mod runtime_read_obligation_dispatch;
mod runtime_reads_programs;
mod runtime_session_lowering;

pub use read_composition_row_selection::worth_query_materialized_relation_field_key;
mod runtime_sessions;
mod runtime_unified_inspection_intents;
mod runtime_write_intents;
mod runtime_writes;
mod shared_read;
mod shared_read_pins;
mod state;
mod state_basis;
mod state_snapshot;
mod support;
mod support_matrix;
mod surface;
mod time_only_delivery;
mod workspace;
mod workspace_contracts;
mod workspace_declaration;
mod workspace_graph;
mod workspace_live_queries;
mod workspace_mutations;
mod workspace_queries;
mod workspace_read_composition_support;
mod workspace_shared_read;
mod workspace_submission;

const RUNTIME_SUBSCRIPTION_FAMILY_BUDGET_POLICY: &str =
    "runtime-live-subscription-family:scratch_buffer_only:canonical=64:relationship=64:policy=64:projection=512:tenant=1";
const RUNTIME_SUBSCRIPTION_SLICE_BUDGET_POLICY: &str =
    "runtime-live-subscription-slice:scratch_buffer_only:all-widths=64";
const RUNTIME_SUBSCRIPTION_BRIDGE_BUDGET_POLICY: &str =
    "runtime-live-subscription-bridge:admitted:bridge=1:slice=64:policy=64:basis=64:signal=64";
const RUNTIME_SUBSCRIPTION_ADMISSION_BUDGET_POLICY: &str =
    "runtime-live-subscription-admission:admitted:all-widths=64";
const RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY: &str =
    "runtime-live-active-lifecycle:registry=1:fanout=1:allocation=1:lifecycle_arena";
const RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY: &str =
    "runtime-live-consumer-attachment:fanout=1:pacing=1:allocation=1:retain_within_window";

pub use aspect_api_closeout::WorthQueryAspectApiFinalizationCloseout;
pub use async_result_state::{
    WorthQueryRuntimeAsyncResultState, WorthQueryRuntimeAsyncResultStateKind,
};
pub use authoritative_mutation_evidence_closeout::WorthQueryAuthoritativeMutationEvidenceCloseout;
#[allow(unused_imports)]
pub use authoritative_mutation_evidence_support::{
    WorthQueryAuthoritativeMutationEvidenceSupport, WorthQueryBridgeBackedVerificationSupportRow,
    WorthQueryBridgeBackedVerificationSupportStatus,
};
pub use authority::{
    WorthQueryAuthorityLane, WorthQueryBranchOptions, WorthQueryEffectAction,
    WorthQueryEffectAdmission, WorthQueryEffectPolicy, WorthQueryEffectPolicyDenial,
    WorthQueryPreviewOptions,
};
pub(crate) use backend::build_bridge_authority_bundle;
pub use backend::{
    runtime_subscription_support_evidence_identity, LiveViewDeclarationAdmissionBoundaryReceipt,
    LiveViewDeclarationAdmissionReceipt, SignalInvalidationBoundaryReceipt,
    SignalInvalidationRoutingReceipt, SubscriptionActivationBoundaryReceipt,
    SubscriptionActivationReceipt, WorthQueryBridgeBackedRuntimeBackend,
    WorthQueryIntentAuthorityAdapter, WorthQueryRuntimeBackend, WorthQueryRuntimeBackendParts,
    WorthQueryRuntimeDeclarationInitializationAdapter,
    WorthQueryRuntimeExistingTruthVerificationAdapter, WorthQueryRuntimeInspectorEvidenceAdapter,
    WorthQueryRuntimeIntentAuthorityAdapter, WorthQueryRuntimePreviewBasisAdapter,
    WorthQueryRuntimeSchemaAdapter, WorthQueryRuntimeSignalSinkAdapter,
    WorthQueryRuntimeSnapshotIdentityAdapter, WorthQueryRuntimeSourceAdapter,
    WorthQueryRuntimeSubscriptionActivationAdapter, WorthQueryRuntimeWriteAuthorityAdapter,
    WriteAuthorityExecutionReceipt,
};
pub use branch::WorthQueryBranchSession;
pub(crate) use branch::WorthQueryRuntimeBranchComparisonBasis;
use bridge_mutation_lowering::{bridge_continuity_mutation_bundle, bridge_naming_mutation_bundle};
pub use builder::WorthQueryRuntimeBuilder;
use computed::{
    admit_derived_view_declaration, insert_derived_runtime,
    retained_live_view_names_for_candidates, route_derived_view_patches,
    WorthQueryComputedDependencyIndex, WorthQueryDerivedViewRuntime,
};
pub use computed::{
    WorthQueryComputedInspectionEvidence, WorthQueryDerivedPatch, WorthQueryDerivedPatchFamily,
    WorthQueryDerivedPatchPayload, WorthQueryDerivedViewHandle, WorthQueryDerivedViewMaintainer,
    WorthQueryDerivedViewMaterialization, WorthQueryRetainedRefreshContext,
    WorthQueryRetainedRefreshOrigin, WorthQueryRetainedUpstreamInputs,
};
pub use concurrent_hostile_matrix::{
    WorthQueryConcurrentHostileMatrixCounterSnapshot, WorthQueryConcurrentHostileMatrixTopology,
    WorthQueryConcurrentSubmissionIntake, WorthQueryConcurrentSubmissionLane,
    WorthQueryConcurrentSubmissionRecord,
};
pub use delivery::WorthQueryRuntimeDeliveryBatch;
use delivery::{
    register_live_subscription_index, WorthQueryRuntimeLiveSubscriptionActivation,
    WorthQueryRuntimeLiveSubscriptionState,
};
use downstream_delivery_contract::project_downstream_delivery;
pub use downstream_delivery_contract::{
    WorthQueryRuntimeDownstreamDelivery, WorthQueryRuntimeDownstreamDeliveryClass,
    WorthQueryRuntimeDownstreamDeliveryContract, WorthQueryRuntimeDownstreamSupportPosture,
};
use downstream_delivery_resume::{aggregate_support_posture, support_gate_resume_kind};
pub use downstream_delivery_resume::{
    WorthQueryRuntimeDownstreamResumePosture, WorthQueryRuntimeDownstreamResumePostureKind,
};
use effect::{
    admit_effect_declaration, insert_effect_runtime, route_effect_deliveries,
    WorthQueryEffectIndex, WorthQueryEffectRuntime, WorthQueryEffectTarget,
};
pub use effect::{
    WorthQueryEffectCondition, WorthQueryEffectCounters, WorthQueryEffectDeclaration,
    WorthQueryEffectDelivery, WorthQueryEffectDeliveryFamily, WorthQueryEffectExpression,
    WorthQueryEffectExpressionFailurePosture, WorthQueryEffectHandle, WorthQueryEffectIdempotence,
    WorthQueryEffectInspectionEvidence, WorthQueryEffectLoopPrevention, WorthQueryEffectPayload,
    WorthQueryEffectPhase, WorthQueryEffectPhaseEvidence, WorthQueryEffectSuppressionPolicy,
    WorthQueryEffectTrigger, WorthQueryEffectTriggerSourceKind,
    WorthQueryEffectWriteAdjacentTrigger, WorthQueryEffectWriteAdjacentTriggerClass,
};
#[allow(unused_imports)]
pub use error::{
    WorthQueryGraphObligationDenial, WorthQueryRuntimeDeclarationFailureKind,
    WorthQueryRuntimeError, WorthQueryRuntimeLookupFailureKind,
    WorthQueryRuntimeMissingArtifactKind, WorthQueryRuntimeMissingComponent, WorthQueryStopClass,
};
#[cfg(test)]
pub(crate) use fallback_seam_counters::{
    forbidden_fallback_seam_invocation_count, record_forbidden_fallback_seam_invocation,
    reset_forbidden_fallback_seam_invocations, WorthQueryForbiddenFallbackSeam,
};
pub(crate) use graph_read_access::match_graph_index_inventory_for_requirements;
pub(crate) use graph_read_access::provision_ephemeral_graph_indexes_for_read_execution;
pub(crate) use graph_read_access::streaming_receipt_for_admitted_read_result;
#[allow(unused_imports)]
pub use graph_read_access::{
    admit_graph_read_access_authority,
    admit_graph_read_access_authority_from_policy_tenant_request,
    admit_graph_read_access_for_family, admit_graph_read_access_for_family_in_authority,
    derive_graph_read_access_requirements, derive_graph_read_cost_evidence,
    estimate_graph_read_access_cost, estimate_graph_read_access_cost_with_planning_observation,
    explain_boolean_selectivity_shape_for_family,
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
    match_current_graph_index_inventory_for_requirements,
    plan_admitted_graph_read_access_for_family,
    plan_admitted_graph_read_access_for_family_in_authority,
    resolve_graph_read_operations_for_family_in_authority_with_registry,
    resolve_graph_read_operations_for_family_with_registry,
    try_derive_graph_read_access_requirements, worth_query_graph_index_inventory,
    worth_query_graph_read_access_compile_fail_boundary_digest,
    worth_query_graph_read_access_compile_fail_target_count,
    worth_query_graph_read_access_compile_fail_targets,
    worth_query_graph_read_proof_transition_manifest,
    worth_query_graph_read_proof_transition_manifest_count,
    worth_query_graph_read_proof_transition_manifest_digest,
    WorthQueryAdmittedBooleanExpressionBranch, WorthQueryAdmittedBooleanExpressionBranchKind,
    WorthQueryAdmittedBooleanExpressionCounters, WorthQueryAdmittedBooleanExpressionTopology,
    WorthQueryAdmittedBooleanPredicateExpression, WorthQueryAdmittedBooleanPredicateLeaf,
    WorthQueryAdmittedGraphReadAccessPlan, WorthQueryAdmittedGraphReadOrderingField,
    WorthQueryAdmittedGraphReadPredicateField, WorthQueryAdmittedGraphReadProjectionField,
    WorthQueryAdmittedGraphReadRelation, WorthQueryAdmittedGraphReadRelationDirection,
    WorthQueryAdmittedQuerySchemaReferences, WorthQueryBooleanExpressionAdmissionError,
    WorthQueryBooleanExpressionAdmissionErrorKind, WorthQueryBooleanPredicateSelectivityRow,
    WorthQueryBooleanPredicateTopology, WorthQueryBooleanSelectivityAdmissionPosture,
    WorthQueryBooleanSelectivityBranch, WorthQueryBooleanSelectivityBranchKind,
    WorthQueryBooleanSelectivityCounters, WorthQueryBooleanSelectivityShape,
    WorthQueryBooleanSelectivityShapeDigest, WorthQueryBuiltInGraphReadOperation,
    WorthQueryDomainRegisteredGraphReadOperation, WorthQueryEphemeralGraphIndex,
    WorthQueryEphemeralGraphIndexAllocationRow, WorthQueryEphemeralGraphIndexCounters,
    WorthQueryEphemeralGraphIndexLifecycleRegistry, WorthQueryEphemeralGraphIndexPlan,
    WorthQueryEphemeralGraphIndexProvisioningError, WorthQueryEphemeralGraphIndexReceipt,
    WorthQueryEphemeralGraphIndexScope, WorthQueryEphemeralGraphIndexScopeKind,
    WorthQueryGraphIndexInventory, WorthQueryGraphIndexInventoryCounters,
    WorthQueryGraphIndexInventoryMatch, WorthQueryGraphIndexInventoryMatchOutcome,
    WorthQueryGraphIndexInventoryMatchReport, WorthQueryGraphIndexLifecycleClass,
    WorthQueryGraphIndexLifecycleOwner, WorthQueryGraphIndexPosture,
    WorthQueryGraphIndexSupportRow, WorthQueryGraphIndexSupportState,
    WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessAuthorityContext, WorthQueryGraphReadAccessAuthorityCounters,
    WorthQueryGraphReadAccessAuthorityDenial, WorthQueryGraphReadAccessAuthorityDenialKind,
    WorthQueryGraphReadAccessAuthorityReceipt, WorthQueryGraphReadAccessAuthorityRequest,
    WorthQueryGraphReadAccessBasisScope, WorthQueryGraphReadAccessBasisScopeKind,
    WorthQueryGraphReadAccessCase, WorthQueryGraphReadAccessCaseRegistry,
    WorthQueryGraphReadAccessComplexityContract, WorthQueryGraphReadAccessCostEstimate,
    WorthQueryGraphReadAccessCostEstimateDigest, WorthQueryGraphReadAccessDenial,
    WorthQueryGraphReadAccessDenialKind, WorthQueryGraphReadAccessExecutionCounters,
    WorthQueryGraphReadAccessInvalidationBasis, WorthQueryGraphReadAccessInventoryMatch,
    WorthQueryGraphReadAccessMemoryEstimateBasis, WorthQueryGraphReadAccessPlanConsumption,
    WorthQueryGraphReadAccessPlanExplanation, WorthQueryGraphReadAccessRebuildBasis,
    WorthQueryGraphReadAccessRequirementCounters,
    WorthQueryGraphReadAccessRequirementDerivationError,
    WorthQueryGraphReadAccessRequirementExplanationOutcome,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadAccessRequirementRow,
    WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessRequirementSetDigest,
    WorthQueryGraphReadAccessShape, WorthQueryGraphReadAccessShapeDerivationCounters,
    WorthQueryGraphReadAccessShapeDigest, WorthQueryGraphReadAccessShapeExplanation,
    WorthQueryGraphReadAccessShapeExplanationError, WorthQueryGraphReadAdmittedSchemaFieldKind,
    WorthQueryGraphReadBasisBinding, WorthQueryGraphReadBasisPosture, WorthQueryGraphReadBudget,
    WorthQueryGraphReadBudgetCheck, WorthQueryGraphReadBudgetClass,
    WorthQueryGraphReadBudgetClassKind, WorthQueryGraphReadBudgetDigest,
    WorthQueryGraphReadBudgetExceededDenial, WorthQueryGraphReadCheckpointInterval,
    WorthQueryGraphReadComplexityContract, WorthQueryGraphReadComplexityContractKind,
    WorthQueryGraphReadCostAttributionRow, WorthQueryGraphReadCostEstimateCounters,
    WorthQueryGraphReadCostEstimateStatus, WorthQueryGraphReadCostEstimateStatusKind,
    WorthQueryGraphReadCostEvidence, WorthQueryGraphReadFamilyIndexContract,
    WorthQueryGraphReadFanoutPosture, WorthQueryGraphReadFrontierCursor,
    WorthQueryGraphReadInlineEphemeralAllowance, WorthQueryGraphReadInlineEphemeralAllowanceKind,
    WorthQueryGraphReadIntrinsicCostContribution, WorthQueryGraphReadIntrinsicCostEstimate,
    WorthQueryGraphReadLifecycleClass, WorthQueryGraphReadMaterializationAdmittedJob,
    WorthQueryGraphReadMaterializationAdmittedLimits,
    WorthQueryGraphReadMaterializationCancellationReceipt,
    WorthQueryGraphReadMaterializationCheckpoint, WorthQueryGraphReadMaterializationCounters,
    WorthQueryGraphReadMaterializationJob, WorthQueryGraphReadMaterializationJobState,
    WorthQueryGraphReadMaterializationPolicy, WorthQueryGraphReadMaterializationProgress,
    WorthQueryGraphReadMaterializationReceipt, WorthQueryGraphReadMaterializationRecoveryHandle,
    WorthQueryGraphReadMaterializationRequest, WorthQueryGraphReadMaterializationRequestError,
    WorthQueryGraphReadMaterializationResourceLimitReceipt,
    WorthQueryGraphReadMaterializationRuntime, WorthQueryGraphReadMaterializedArtifact,
    WorthQueryGraphReadMaterializedRowProof, WorthQueryGraphReadMemoryByteEstimate,
    WorthQueryGraphReadObservedCostEstimate, WorthQueryGraphReadOperationCapabilityRequirement,
    WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    WorthQueryGraphReadOperationCapabilityRequirementKind, WorthQueryGraphReadOperationOutcome,
    WorthQueryGraphReadOperationRegistration, WorthQueryGraphReadOperationRegistry,
    WorthQueryGraphReadOperationResolution, WorthQueryGraphReadOperationUnsupportedDenial,
    WorthQueryGraphReadOperationUnsupportedDenialKind,
    WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    WorthQueryGraphReadOrderingFieldAuthority, WorthQueryGraphReadOrderingPosture,
    WorthQueryGraphReadPersistentArtifactAudit, WorthQueryGraphReadPlanningObservation,
    WorthQueryGraphReadPolicyTenantAuthorityRequest, WorthQueryGraphReadPolicyTenantPosture,
    WorthQueryGraphReadPolicyTenantProofBinding, WorthQueryGraphReadPredicateFamily,
    WorthQueryGraphReadPredicateFieldAuthority, WorthQueryGraphReadProofBoundaryEvidenceKind,
    WorthQueryGraphReadProofTransitionManifestRow, WorthQueryGraphReadRegistryAdmissionError,
    WorthQueryGraphReadRelationAuthority, WorthQueryGraphReadRelationshipProofBindingPosture,
    WorthQueryGraphReadRequiredCapabilityOwner, WorthQueryGraphReadResolvedOperation,
    WorthQueryGraphReadResolvedOperationFamily, WorthQueryGraphReadResolvedOperationKind,
    WorthQueryGraphReadResultPressure, WorthQueryGraphReadRootPosture,
    WorthQueryGraphReadSchemaReferenceAdmissionError,
    WorthQueryGraphReadSchemaReferenceAdmissionErrorKind, WorthQueryGraphReadStreamingCounters,
    WorthQueryGraphReadStreamingCursorDenial, WorthQueryGraphReadStreamingCursorDenialKind,
    WorthQueryGraphReadStreamingCursorSession, WorthQueryGraphReadStreamingPageBudget,
    WorthQueryGraphReadStreamingPageReceipt, WorthQueryGraphReadStreamingPlan,
    WorthQueryGraphReadStreamingReceipt, WorthQueryGraphReadSupportedCostContribution,
    WorthQueryGraphReadSupportedCostEstimate, WorthQueryGraphReadTraversalOperator,
    WorthQueryLiveGraphReadAccessDenial, WorthQueryLiveGraphReadAccessPlan,
    WorthQueryLiveGraphReadAccessPosture, WorthQueryLiveGraphReadAccessReceipt,
    WorthQueryLiveGraphReadMaintenanceBudget, WorthQueryLiveGraphReadMaintenanceCounters,
    WorthQueryLiveGraphReadMaintenanceReceipt, WorthQueryLiveGraphReadMutationDeltaScope,
    WorthQueryPersistentGraphIndexRequirementCounters,
    WorthQueryPersistentGraphIndexRequirementDeclaration,
    WorthQueryPersistentGraphIndexRequirementReceipt, WorthQueryPersistentGraphIndexRequirementRow,
    WorthQueryPredicateAnchorPosture, WorthQueryPredicateOperandOperator,
    WorthQueryPredicateSelectivityClass, WorthQueryTraversalPredicateOrderingPosture,
};
pub(crate) use graph_read_access::{
    admit_graph_read_access_for_family_in_authority_with_inventory,
    admit_graph_read_access_for_family_with_inventory,
};
pub use handle_contract::{
    WorthQueryHandleContract, WorthQueryHandleContractFamily, WorthQueryHandleContractRow,
};
pub(crate) use inspection::request_causal_inspection;
pub use inspection::{
    admit_causal_inspection, anchor_causal_observation,
    build_causal_inspection_certification_scope, causal_evidence_inventory_rows,
    causal_inspection_target, certify_causal_inspection_runtime_path,
    materialize_admitted_causal_inspection, materialize_advisory_causal_inspection,
    materialize_denied_causal_inspection, resolve_causal_evidence_references,
    resolve_indexed_causal_evidence_references, AdmittedCausalInspection,
    AdmittedQueryCausalInspectionArtifact, AdvisoryCausalInspection,
    AdvisoryQueryCausalInspectionArtifact, CausalDecisionTraceIndex, CausalDecisionTraceRow,
    CausalEvidenceFamily, CausalEvidenceInventoryRow, CausalEvidenceOwner, CausalEvidenceReference,
    CausalEvidenceReferenceDigest, CausalEvidenceReferenceIndex, CausalEvidenceReferenceIndexError,
    CausalEvidenceReferenceIndexErrorKind, CausalEvidenceReferenceIndexRecord,
    CausalEvidenceReferenceReceipt, CausalEvidenceReferenceResolution,
    CausalEvidenceReferenceResolutionCounters, CausalEvidenceReferenceResolutionDenial,
    CausalEvidenceReferenceSet, CausalInspection, CausalInspectionAdmissionCounters,
    CausalInspectionAdmissionDecision, CausalInspectionAdmissionDecisionKind,
    CausalInspectionAdmissionReceipt, CausalInspectionAdmissionSubject,
    CausalInspectionAdvisoryKind, CausalInspectionArtifactDecisionTrace,
    CausalInspectionArtifactIntegrity, CausalInspectionArtifactKind, CausalInspectionBasisMismatch,
    CausalInspectionBoundaryAudit, CausalInspectionBoundaryEnvelopeCategory,
    CausalInspectionCertificationBundle, CausalInspectionCertificationError,
    CausalInspectionCertificationErrorKind, CausalInspectionCertificationFailureEvidence,
    CausalInspectionCertificationFailureKind, CausalInspectionCertificationFailureSource,
    CausalInspectionCertificationLane, CausalInspectionCertificationScope,
    CausalInspectionEstimatedCost, CausalInspectionExplanationFamily,
    CausalInspectionMaterializationError, CausalInspectionMaterializationErrorKind,
    CausalInspectionMaterializationPolicy, CausalInspectionPerformanceCertificationBundle,
    CausalInspectionPerformanceEnvelope, CausalInspectionPlan, CausalInspectionPlanError,
    CausalInspectionPlanErrorKind, CausalInspectionPlanExplanation, CausalInspectionProofFlow,
    CausalInspectionProofShapeCertification, CausalInspectionReason,
    CausalInspectionRedactionPolicy, CausalInspectionRepresentativeEvidence,
    CausalInspectionRepresentativeKind, CausalInspectionRepresentativeMatrix,
    CausalInspectionRepresentativeRowDigestSet, CausalInspectionRequest,
    CausalInspectionRequestError, CausalInspectionRequestErrorKind, CausalInspectionRichness,
    CausalInspectionScaleCounterSnapshot, CausalInspectionScaleFixtureSize,
    CausalInspectionSupport, CausalInspectionSupportExplanation, CausalInspectionSupportPosture,
    CausalInspectionSupportRow, CausalInspectionSupportRowPosture, CausalInspectionTarget,
    CausalInspectionViolationKind, CausalMaterializationReceipt, CausalObservationAnchor,
    CausalObservationAnchorCounters, CausalObservationAnchorDigest, CausalObservationAnchorError,
    CausalObservationAnchorErrorKind, CausalObservationEvidenceIdentity,
    CausalObservationMissingReferencePosture, CausalObservationOutcome, DeniedCausalInspection,
    DeniedQueryCausalInspectionArtifact, QueryCausalEvidenceReferenceArtifact,
    QueryCausalInspectionArtifact, QueryCausalTemporalAsyncExplanation,
    QueryCausalTemporalAsyncExplanationKind, QueryObservationReceipt,
    QueryObservationReceiptFamily, WorthQueryBasisLifecycleInspection,
    WorthQueryBatchWriteComponentInspection, WorthQueryBatchWriteReceiptInspection,
    WorthQueryBranchIntentReceiptInspection, WorthQueryEffectIntentReceiptInspection,
    WorthQueryFeedbackPhaseGraphInspection, WorthQueryFeedbackPhaseNode,
    WorthQueryFeedbackTermination, WorthQueryInspection, WorthQueryInspectionTarget,
    WorthQueryIntentConsumerInspection, WorthQueryIntentConsumerOutcomeClass,
    WorthQueryIntentDenialInspection, WorthQueryIntentInspectionDeliveryCounters,
    WorthQueryIntentReceiptInspection, WorthQueryLiveSubscriptionInspectionCounters,
    WorthQueryLiveViewInspection, WorthQueryPreviewBindingInspection,
    WorthQueryPreviewIntentReceiptInspection, WorthQueryPreviewOutcomeInspection,
    WorthQueryWriteReceiptInspection,
};
pub(crate) use intent::{
    admit_authoritative_intent_declaration, admit_authoritative_intent_execution,
    admit_effect_triggered_intent_declaration, WorthQueryIntentAdmissionDenial,
};
pub use intent::{
    WorthQueryBranchIntentReceipt, WorthQueryEffectIntentReceipt, WorthQueryIntentDeclaration,
    WorthQueryIntentDenialEvidence, WorthQueryIntentExecution,
    WorthQueryIntentExecutionFailureEvidence, WorthQueryIntentExecutionKind,
    WorthQueryIntentExecutionProvenance, WorthQueryIntentInput, WorthQueryIntentReceipt,
    WorthQueryIntentSourceLane, WorthQueryPreviewIntentReceipt,
    WorthQueryTouchBearingIntentDeclaration,
};
#[allow(unused_imports)]
pub use journal_position::{WorthQueryJournalPosition, WorthQueryJournalPositionAuthority};
#[allow(unused_imports)]
pub use journal_position::{
    WorthQueryJournalPositionAdmissionError, WorthQueryJournalPositionSchedule,
    WorthQueryJournalPositionScheduleViolation, WorthQueryJournalPositionScheduleViolationKind,
};
pub(crate) use journal_replay::journal_replay_truth_reconstruction_identity;
pub use journal_replay::{
    WorthQueryJournalReplayCounterSnapshot, WorthQueryJournalReplayDenial,
    WorthQueryJournalReplayDenialKind, WorthQueryJournalReplayDiagnostics,
    WorthQueryJournalReplayOutcome, WorthQueryJournalReplayRequest,
    WorthQueryJournalSegmentIdentity,
};
pub(crate) use live_subscription::{
    live_subscription_source_identity, live_subscription_view_shape_source_identity,
};
pub use live_subscription::{
    WorthQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    WorthQueryRuntimeLiveSubscriptionInstallation,
};
use live_subscription_delivery_routing::route_live_subscription_delivery;
pub use managed_live_resource::{
    WorthQueryManagedLiveActivationWork, WorthQueryManagedLiveLifecycleObservation,
    WorthQueryManagedLiveLifecyclePosture, WorthQueryManagedLiveSubscriptionFamily,
};
pub(crate) use managed_live_resource::{
    WorthQueryManagedLiveRuntimeDelivery, WorthQueryManagedLiveWorkspaceCapability,
};
#[allow(unused_imports)]
pub use mixed_cause_delivery::{
    WorthQueryRuntimeDeliveryCoalescingKind, WorthQueryRuntimeMixedCauseDelivery,
    WorthQueryRuntimeMixedCauseLaneKind, WorthQueryRuntimeMixedCauseMemberKind,
};
pub(crate) use mutation::registrations_from_relational_invariant_catalog;
use mutation::{admit_continuity_intent, admit_naming_intent};
pub(crate) use mutation::{
    command_declared_aspect_value_digest, command_declared_aspect_value_identity,
};
#[allow(unused_imports)]
pub use mutation::{
    WorthQueryAdmittedAspectValue, WorthQueryAspectMutationBuilder,
    WorthQueryAspectMutationOperation, WorthQueryAspectMutationOperationKind,
    WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
    WorthQueryAuthoritativeMutationObligationDispatch,
    WorthQueryAuthoritativeMutationObligationDispatchProjection,
    WorthQueryAuthoritativeMutationObligationDispatchProjectionRow,
    WorthQueryBackendAdmissibleMutation, WorthQueryContinuityMutationDenial,
    WorthQueryContinuityMutationDenialKind, WorthQueryContinuityMutationFamily,
    WorthQueryContinuityMutationIntent, WorthQueryContinuityMutationOutcomeClass,
    WorthQueryDeleteMutationBuilder, WorthQueryExistingEntityTarget,
    WorthQueryExistingRelationTarget, WorthQueryExistingTruthAssertionDenial,
    WorthQueryExistingTruthAssertionDenialKind, WorthQueryExistingTruthAssertionMode,
    WorthQueryExistingTruthBindingDenial, WorthQueryExistingTruthBindingDenialKind,
    WorthQueryExistingTruthBindingFamily, WorthQueryExistingTruthProbe,
    WorthQueryExistingTruthProbeDenial, WorthQueryExistingTruthProbeDenialKind,
    WorthQueryExistingTruthProbeField, WorthQueryExistingTruthProbeMode,
    WorthQueryExistingTruthProbeRequest, WorthQueryExistingTruthTargetBinding,
    WorthQueryGraphCompositionBuilder, WorthQueryGraphCompositionDenial,
    WorthQueryGraphCompositionDenialKind, WorthQueryGraphCompositionDomainInvariantDenial,
    WorthQueryGraphCompositionInvariantPackContext,
    WorthQueryGraphCompositionInvariantPackViolation, WorthQueryGraphEntitySymbol,
    WorthQueryGraphMutationPolicyGateEvidence, WorthQueryGraphMutationPolicyGateVerdict,
    WorthQueryGraphObligationArtifactPolicy, WorthQueryGraphObligationAttachmentEvidence,
    WorthQueryGraphObligationBudgetExceededPolicy,
    WorthQueryGraphObligationDenialAttachmentProjection,
    WorthQueryGraphObligationDenialAttachmentProjectionRow,
    WorthQueryGraphObligationDenialProjection, WorthQueryGraphObligationDenialProjectionRow,
    WorthQueryGraphObligationDiagnosticMaterialization, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchContextKind, WorthQueryGraphObligationDispatchEnvelope,
    WorthQueryGraphObligationDispatchEnvelopeBuilder, WorthQueryGraphObligationDispatchError,
    WorthQueryGraphObligationDispatchPlan, WorthQueryGraphObligationDispatchPlanDraft,
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationExecutionContext,
    WorthQueryGraphObligationExecutionCostClass, WorthQueryGraphObligationExecutionInput,
    WorthQueryGraphObligationExecutionResultEnvelope, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationExecutorContract, WorthQueryGraphObligationIndex,
    WorthQueryGraphObligationIndexBuildCounters, WorthQueryGraphObligationIndexComplexityContract,
    WorthQueryGraphObligationIndexComplexityContractStatus, WorthQueryGraphObligationIndexEntry,
    WorthQueryGraphObligationIndexSupportRow, WorthQueryGraphObligationIndexSupportStatus,
    WorthQueryGraphObligationKind, WorthQueryGraphObligationMaterializedDispatch,
    WorthQueryGraphObligationMatrixCertificationCase,
    WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldDescriptorKind,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationPreflightWitness,
    WorthQueryGraphObligationReduction, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphObligationRegistrationDenial,
    WorthQueryGraphObligationRegistrationDenialKind, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSelection, WorthQueryGraphObligationSelectionCounters,
    WorthQueryGraphObligationSelectorPerturbationCase, WorthQueryGraphObligationStateAccessPolicy,
    WorthQueryGraphObligationStateLoadCounters, WorthQueryGraphObligationStateLoadPlan,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportMatrix,
    WorthQueryGraphObligationSupportMatrixRow, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphObligationSupportStatus, WorthQueryGraphObligationVerdict,
    WorthQueryGraphReadTouchShape, WorthQueryGraphRelationMutationBuilder,
    WorthQueryGraphRelationSymbol, WorthQueryGraphScopedCustomInvariantRegistration,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial,
    WorthQueryGraphTouchDescriptorDenialKind, WorthQueryGraphTouchDescriptorKind,
    WorthQueryGraphTouchDescriptorRow, WorthQueryGraphTouchLifecycleFamily,
    WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector, WorthQueryMutationBatchBuilder,
    WorthQueryMutationMetadata, WorthQueryMutationMetadataKey, WorthQueryMutationMetadataValue,
    WorthQueryNamingMutationDenial, WorthQueryNamingMutationDenialKind,
    WorthQueryNamingMutationFamily, WorthQueryNamingMutationIntent,
    WorthQuerySymbolicAspectReference, WorthQuerySymbolicAspectReferenceFamily,
    WorthQuerySymbolicTargetReference, WorthQuerySymbolicTargetReferenceDenial,
    WorthQuerySymbolicTargetReferenceDenialKind, WorthQuerySymbolicTargetReferenceFamily,
    WorthQueryVerifiedExistingTruthAssertion,
    WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
};
pub use mutation_surface::{
    WorthQueryMutationSurfacePosture, WorthQueryMutationSurfaceReport, WorthQueryMutationSurfaceRow,
};
pub use preview::{
    WorthQueryPreviewCloseoutEvidence, WorthQueryPreviewCloseoutKind, WorthQueryPreviewDiff,
    WorthQueryPreviewEffectBindingDisposition, WorthQueryPreviewExecutionEvidence,
    WorthQueryPreviewExecutionKind, WorthQueryPreviewHandleBindingEvidence,
    WorthQueryPreviewHandleBindingFamily, WorthQueryPreviewOutcome,
    WorthQueryPreviewPromotionDenialEvidence, WorthQueryPreviewPromotionDenialKind,
    WorthQueryPreviewResidueClass, WorthQueryPreviewSession,
};
pub use public_api::{
    WorthQueryRuntimePublicApiContract, WorthQueryRuntimePublicApiFamilyContract,
    WorthQueryRuntimePublicApiNamingContract, WorthQueryRuntimePublicApiNamingRow,
    WorthQueryRuntimePublicApiTranscriptEvidence,
};
#[allow(unused_imports)]
pub use published_artifacts::{
    WorthQueryPublishedArtifactCounterSnapshot, WorthQueryPublishedArtifactDiagnostics,
    WorthQueryPublishedArtifactGenerationDiagnostic,
};
pub use read_composition::WorthQueryReadBuilder;
pub use read_composition_hooks::{
    WorthQueryReadInvariantPackContext, WorthQueryReadInvariantPackViolation,
};
pub use read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};
pub use read_composition_phase_gate::{
    WorthQueryReadCompositionPhaseGate, WorthQueryReadCompositionPhaseGateFamily,
    WorthQueryReadCompositionPhaseGateRow, WorthQueryReadCompositionPhaseGateStatus,
};
pub use read_composition_phase_one_closeout::WorthQueryReadCompositionPhaseOneCloseout;
pub use read_composition_support_report::{
    WorthQueryReadCompositionSupportClass, WorthQueryReadCompositionSupportReport,
    WorthQueryReadCompositionSupportRow,
};
pub use remask_posture::{
    WorthQueryRuntimeRemaskDispositionKind, WorthQueryRuntimeRemaskPosture,
    WorthQueryRuntimeRemaskProjection, WorthQueryRuntimeRemaskReasonKind,
};
#[cfg(test)]
use runtime_helpers::runtime_subscription_budget_digest;
use runtime_helpers::{
    admit_authority_requirements, attach_continuity_mutation_to_receipt,
    attach_naming_mutation_to_receipt, attach_symbolic_target_reference_to_receipt,
    classify_receipt_mutation_summary, combined_batch_mutation_receipt, live_subscription_error,
    record_same_batch_symbolic_target, resolve_same_batch_symbolic_target,
    resolve_symbolic_aspect_references, runtime_active_lifecycle_budget,
    runtime_active_lifecycle_budget_policy, runtime_bridge_lowering_budget,
    runtime_consumer_attachment_budget, runtime_consumer_attachment_budget_policy,
    runtime_family_budget, runtime_slice_budget, runtime_subscription_admission_budget,
    runtime_subscription_budget_policy, subscription_dimensions_for_request,
    synthetic_existing_assertion_receipt, WorthQuerySameBatchSymbolicTarget,
    WorthQuerySameBatchSymbolicTargetKey,
};
#[allow(unused_imports)]
pub use shared_read::{
    WorthQueryPublishedDerivedArtifactHandle, WorthQueryPublishedProjectionAuthorityOutcome,
    WorthQueryPublishedProjectionInspection, WorthQuerySharedReadBasisInspection,
    WorthQuerySharedReadContext,
};
pub(in crate::runtime) use shared_read_pins::{
    worth_query_shared_read_stale_basis_error, WorthQuerySharedReadGenerationLease,
};
#[allow(unused_imports)]
pub use shared_read_pins::{
    WorthQuerySharedReadCounters, WorthQuerySharedReadGenerationDiagnostic,
    WorthQuerySharedReadPinningDiagnostics,
};
pub use state::WorthQueryRuntimeStateTarget;
pub use state_snapshot::{WorthQueryRuntimeStateKind, WorthQueryRuntimeStateSnapshot};
#[allow(unused_imports)]
pub use support::{
    WorthQueryBasisAdmissionEvidenceRow, WorthQueryBranchBasisAdmission,
    WorthQueryBridgeMutationArtifactIdentity, WorthQueryContinuityPriorAuthorityLabel,
    WorthQueryContinuitySuccessorAuthorityLabel, WorthQueryExistingTruthBindingAuthorityLabel,
    WorthQueryGraphCompositionCapabilityClass, WorthQueryGraphCompositionCapabilitySupportRow,
    WorthQueryGraphCompositionExtensionHookBoundary,
    WorthQueryGraphCompositionExtensionHookSupportRow, WorthQueryMutationAuthorityIdentity,
    WorthQueryMutationEvidenceDigest, WorthQueryMutationSymbolIdentity,
    WorthQueryMutationTargetCollectionIdentity, WorthQueryNamingAttachmentAuthorityLabel,
    WorthQueryNamingPriorAuthorityLabel, WorthQueryNamingTargetAuthorityLabel,
    WorthQueryPreviewBasisAdmission, WorthQueryRuntimeBackendPosture,
    WorthQueryRuntimeEvidenceAuthority, WorthQueryRuntimeFacadeFamily,
    WorthQueryRuntimeFamilySupport, WorthQueryRuntimeFamilySupportStatus,
    WorthQueryRuntimeFamilyTeachingPosture, WorthQueryRuntimeInspectionEvidence,
    WorthQueryRuntimeSupportDenial, WorthQueryRuntimeSupportProfile,
};
pub use support_matrix::{
    WorthQueryRuntimePublicSupportMatrix, WorthQueryRuntimePublicSupportMatrixRow,
};
pub(in crate::runtime) use surface::WorthQueryReadExecutionProduct;
#[allow(unused_imports)]
pub use surface::{
    WorthQueryArtifactInspector, WorthQueryBatchMutationEvidence, WorthQueryBatchWriteReceipt,
    WorthQueryBatchWriteRetainedArtifact, WorthQueryContinuityClass,
    WorthQueryContinuityMutationEvidence, WorthQueryContinuityOutcomeClass,
    WorthQueryContinuityRejectionClass, WorthQueryCountResult, WorthQueryDerivedArtifactBinding,
    WorthQueryDerivedInspectionReceipt, WorthQueryDerivedInspectionResult,
    WorthQueryDerivedMaterializationBundle, WorthQueryDerivedMaterializationReceipt,
    WorthQueryDerivedMaterializationResult, WorthQueryDerivedMaterializationTarget,
    WorthQueryExistingTruthAssertionEvidence, WorthQueryExistingTruthBindingEvidence,
    WorthQueryExistingTruthBindingOutcome, WorthQueryExistingTruthProbeReceipt,
    WorthQueryExistingTruthProbeResult, WorthQueryGraphCompositionAdmissionTrace,
    WorthQueryGraphCompositionAdmissionTraceStage, WorthQueryGraphCompositionAssumptionSummary,
    WorthQueryGraphCompositionBreadth, WorthQueryGraphCompositionDomainInvariantSummary,
    WorthQueryGraphCompositionEvidence, WorthQueryGraphCompositionLifecycleOutcomeEntry,
    WorthQueryGraphCompositionLifecycleOutcomeKind, WorthQueryGraphCompositionLifecycleOutcomes,
    WorthQueryGraphCompositionLineageEntry, WorthQueryGraphCompositionLineageSummary,
    WorthQueryGraphCompositionProgram, WorthQueryGraphCompositionProgramStep,
    WorthQueryGraphCompositionProgramStepKind, WorthQueryGraphCompositionResolutionEntry,
    WorthQueryGraphCompositionResolutionMap, WorthQueryGraphReadAccessComplexityCounters,
    WorthQueryGraphReadAccessReceiptSummary, WorthQueryInspectedArtifact,
    WorthQueryInstalledOperation, WorthQueryInstalledProgram, WorthQueryLiveArtifactBinding,
    WorthQueryLiveArtifactBundle, WorthQueryLiveArtifactTarget, WorthQueryLiveReadReceipt,
    WorthQueryLiveReadResult, WorthQueryLiveView, WorthQueryMutationCausalityEvidence,
    WorthQueryMutationFamily, WorthQueryMutationProvenanceEvidence, WorthQueryMutationTargetClass,
    WorthQueryMutationTargetDescriptor, WorthQueryMutationTargetEvidence,
    WorthQueryNamingMutationEvidence, WorthQueryNamingMutationOutcome, WorthQueryNativeRow,
    WorthQueryPatchBatch, WorthQueryProgramInstallationIdentity, WorthQueryProgramRunIdentity,
    WorthQueryReadAccessPlanBindingMismatch, WorthQueryReadBreadth, WorthQueryReadBuiltInOperator,
    WorthQueryReadBuiltInOperatorDenial, WorthQueryReadBuiltInOperatorDenialReason,
    WorthQueryReadCompositionExtensionHookBoundary, WorthQueryReadCompositionExtensionHookFamily,
    WorthQueryReadCompositionExtensionHookSupportRow, WorthQueryReadDenial,
    WorthQueryReadDenialKind, WorthQueryReadDomainInvariantDenial,
    WorthQueryReadDomainInvariantSummary, WorthQueryReadExecutionEngine,
    WorthQueryReadFallbackClass, WorthQueryReadFamily, WorthQueryReadFamilyAdmission,
    WorthQueryReadFamilyInvariantEvidence, WorthQueryReadGraph, WorthQueryReadGraphFamily,
    WorthQueryReadOperatorFamily, WorthQueryReadReceipt, WorthQueryReadRelationshipProofDenial,
    WorthQueryReadRelationshipProofDenialStage, WorthQueryReadRelationshipProofPosture,
    WorthQueryReadResult, WorthQueryReadScopeClass, WorthQueryReadScopeShapeMismatch,
    WorthQueryRetainedFieldPath, WorthQueryRetainedMaterializedRow,
    WorthQueryRetainedScalarAlignment, WorthQueryRetainedScalarAlignmentFact,
    WorthQueryRetainedScalarFactSet, WorthQueryRetainedScalarFieldFact, WorthQueryRunReceipt,
    WorthQuerySymbolicAspectResolutionEvidence, WorthQuerySymbolicTargetReferenceEvidence,
    WorthQuerySymbolicTargetReferenceOutcome, WorthQueryUnifiedInspectionReceipt,
    WorthQueryUnifiedInspectionResult, WorthQueryVerificationReadSetBreadth,
    WorthQueryVerifiedAssumptionSet, WorthQueryWriteCommand, WorthQueryWriteReceipt,
};
pub use workspace::WorthQueryWorkspace;
pub use workspace_declaration::{
    WorthQueryComputedBuilder, WorthQueryEffectBuilder, WorthQueryLiveViewBuilder,
    WorthQueryWorkspaceLiveViewDeclaration,
};
pub use workspace_submission::WorthQueryWorkspaceSubmissionLane;

pub struct WorthQueryRuntime {
    backend: Box<dyn WorthQueryRuntimeBackend>,
    evidence_authority: WorthQueryRuntimeEvidenceAuthority,
    preview_session_labels: BTreeSet<WorthQuerySessionLabel>,
    branch_session_labels: BTreeSet<WorthQuerySessionLabel>,
    active_subscriptions: ActiveSubscriptionRuntime,
    live_subscriptions:
        BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryRuntimeLiveSubscriptionState>,
    materialized_read_views: BTreeMap<WorthQueryLiveArtifactTarget, DeclarativeLiveQueryRequest>,
    live_subscription_index: Vec<delivery::WorthQueryLiveSubscriptionIndexEntry>,
    installed_programs: BTreeMap<WorthQueryProgramInstallationIdentity, WorthQueryProgram>,
    run_traces: BTreeMap<WorthQueryProgramRunIdentity, WorthQueryProgramTrace>,
    derived_views: BTreeMap<WorthQueryDerivedMaterializationTarget, WorthQueryDerivedViewRuntime>,
    shared_read_pins: shared_read_pins::WorthQuerySharedReadPinRegistry,
    published_artifacts: published_artifacts::WorthQueryPublishedArtifactRegistry,
    journal_replay: journal_replay::WorthQueryJournalReplayRegistry,
    derived_dependency_index: WorthQueryComputedDependencyIndex,
    effects: BTreeMap<WorthQueryEffectTarget, WorthQueryEffectRuntime>,
    effect_index: WorthQueryEffectIndex,
    graph_obligation_registration_catalog: WorthQueryGraphObligationRegistrationCatalog,
    graph_obligation_index: WorthQueryGraphObligationIndex,
    managed_live_resource_capability: std::sync::Arc<WorthQueryManagedLiveWorkspaceCapability>,
    next_run_id: u64,
}

struct WorthQueryRoutedMutationSummary {
    affected_live_view_targets: Vec<WorthQueryLiveArtifactTarget>,
    affected_derived_view_targets: Vec<WorthQueryDerivedMaterializationTarget>,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_effect_suppression_count: usize,
    effect_expression_failure_count: usize,
    refresh_fallback: bool,
}

#[cfg(test)]
pub(crate) mod tests;
