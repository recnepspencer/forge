use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

use forge_relational::facade::runtime::RelationalRuntime;
pub use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
    CustomInvariantRuleId, CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
    CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCatalog, InvariantCostClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
    InvariantRegistration, InvariantRule,
};
use forge_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeNamingMutationBundle,
    BridgeSymbolicTargetReferenceBundle, RuntimeBridge,
};
use serde_json::Value;

use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, DeclarativeLiveQueryError,
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape,
};
#[allow(unused_imports)]
pub use crate::intent_admission::{
    admit_runtime_intent_request, certify_intent_admission,
    forge_query_intent_admission_certification_output_manifest,
    forge_query_intent_admission_closeout_extension_outputs,
    forge_query_intent_admission_compile_fail_targets,
    forge_query_intent_admission_coverage_inventory,
    forge_query_intent_admission_crate_doc_example_targets,
    forge_query_intent_admission_doc_example_report, forge_query_intent_admission_family_inventory,
    forge_query_intent_admission_golden_transcripts,
    forge_query_intent_admission_legacy_parity_report, forge_query_intent_admission_mutation_audit,
    forge_query_intent_admission_oracle_report,
    forge_query_intent_admission_representative_family_report,
    forge_query_intent_admission_representative_output_report,
    forge_query_intent_admission_required_certification_outputs,
    forge_query_intent_admission_seeded_certification_report,
    forge_query_intent_admission_slope_report, forge_query_intent_admission_support_matrix,
    forge_query_intent_admission_support_traceability_report,
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryAdmittedIntentPlan,
    ForgeQueryAdmittedRuntimeEffectWriteIntent, ForgeQueryAdmittedRuntimeExistingTruthProbeIntent,
    ForgeQueryAdmittedRuntimeInspectionIntent, ForgeQueryAdmittedRuntimeIntent,
    ForgeQueryAdmittedRuntimeWriteBatchIntent, ForgeQueryAdmittedRuntimeWriteIntent,
    ForgeQueryAdmittedWorkspaceLiveReadIntent, ForgeQueryAdmittedWorkspaceReadIntent,
    ForgeQueryAuthoritativeIntentExecutionBinding, ForgeQueryAuthoritativeIntentExecutionHandoff,
    ForgeQueryAuthoritativeIntentExecutionPlan,
    ForgeQueryAuthoritativeMutationBatchExecutionBinding,
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ForgeQueryAuthoritativeMutationBatchExecutionPlan,
    ForgeQueryAuthoritativeMutationBatchIntentSeed,
    ForgeQueryAuthoritativeMutationExecutionBinding,
    ForgeQueryAuthoritativeMutationExecutionHandoff, ForgeQueryAuthoritativeMutationExecutionPlan,
    ForgeQueryAuthoritativeMutationIntentSeed, ForgeQueryAuthoritativeMutationPreflight,
    ForgeQueryEffectTriggeredIntentExecutionBinding,
    ForgeQueryEffectTriggeredIntentExecutionHandoff, ForgeQueryEffectTriggeredIntentExecutionPlan,
    ForgeQueryExistingTruthProbeExecutionBinding, ForgeQueryExistingTruthProbeExecutionHandoff,
    ForgeQueryExistingTruthProbeExecutionPlan, ForgeQueryExistingTruthProbeIntentSeed,
    ForgeQueryExistingTruthProbeRoutingPreflight, ForgeQueryGenericInspectionIntentSeed,
    ForgeQueryGenericInspectionIntentTarget, ForgeQueryGenericInspectionIntentTargetSeed,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionCertificationBundle,
    ForgeQueryIntentAdmissionCertificationCounterSnapshot,
    ForgeQueryIntentAdmissionCertificationOutput, ForgeQueryIntentAdmissionCompileFailTarget,
    ForgeQueryIntentAdmissionCoverageInventory, ForgeQueryIntentAdmissionCoverageRow,
    ForgeQueryIntentAdmissionCoverageStatus, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionCrateDocExampleTarget, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionDecisionClass, ForgeQueryIntentAdmissionDocExampleReport,
    ForgeQueryIntentAdmissionDocExampleRow, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionEligibilityAuthority, ForgeQueryIntentAdmissionExecutionBoundary,
    ForgeQueryIntentAdmissionExecutionHandoffInventory, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdmissionFamilyInventory,
    ForgeQueryIntentAdmissionFamilyInventoryRow, ForgeQueryIntentAdmissionGoldenTranscript,
    ForgeQueryIntentAdmissionInvariantEligibility, ForgeQueryIntentAdmissionLegacyParityCheck,
    ForgeQueryIntentAdmissionLegacyParityLane, ForgeQueryIntentAdmissionLegacyParityReport,
    ForgeQueryIntentAdmissionLegacyParityRow, ForgeQueryIntentAdmissionMutationAudit,
    ForgeQueryIntentAdmissionMutationAuditRow, ForgeQueryIntentAdmissionOracleComparisonRow,
    ForgeQueryIntentAdmissionOracleLane, ForgeQueryIntentAdmissionOracleManifestRow,
    ForgeQueryIntentAdmissionOracleReport, ForgeQueryIntentAdmissionPlanKind,
    ForgeQueryIntentAdmissionPolicyEligibility, ForgeQueryIntentAdmissionPreDecisionPosture,
    ForgeQueryIntentAdmissionProjectionSourceEligibility, ForgeQueryIntentAdmissionProofShapeAudit,
    ForgeQueryIntentAdmissionPublicBoundaryAudit,
    ForgeQueryIntentAdmissionRepresentativeFamilyLane,
    ForgeQueryIntentAdmissionRepresentativeFamilyReport,
    ForgeQueryIntentAdmissionRepresentativeFamilyRow,
    ForgeQueryIntentAdmissionRepresentativeOutputReport, ForgeQueryIntentAdmissionResultArtifact,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSeedGeneratorClass, ForgeQueryIntentAdmissionSeedReplayRow,
    ForgeQueryIntentAdmissionSeededCertificationReport, ForgeQueryIntentAdmissionSlopeReport,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportDetail,
    ForgeQueryIntentAdmissionSupportEligibility, ForgeQueryIntentAdmissionSupportMatrix,
    ForgeQueryIntentAdmissionSupportPosture, ForgeQueryIntentAdmissionSupportRow,
    ForgeQueryIntentAdmissionSupportTraceabilityReport,
    ForgeQueryIntentAdmissionSupportTraceabilityRow, ForgeQueryIntentAdmissionSurfaceDescriptor,
    ForgeQueryIntentAdmissionTopologyAudit, ForgeQueryIntentAdmissionTopologyAuditRow,
    ForgeQueryIntentAdmissionTopologyDomain, ForgeQueryIntentAdvisoryDecision,
    ForgeQueryIntentAdvisoryStop, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceRow,
    ForgeQueryIntentDecisionTraceStage, ForgeQueryIntentNonAdmittedStop,
    ForgeQueryIntentViolationDecision, ForgeQueryIntentViolationStop,
    ForgeQueryLiveReadExecutionBinding, ForgeQueryLiveReadExecutionHandoff,
    ForgeQueryLiveReadExecutionPlan, ForgeQueryLiveReadIntentSeed,
    ForgeQueryRawIntentAdmissionRequest, ForgeQueryReadExecutionBinding,
    ForgeQueryReadExecutionHandoff, ForgeQueryReadExecutionIntentSeed, ForgeQueryReadExecutionPlan,
    ForgeQueryRuntimeEffectWriteIntentAdmissionReview, ForgeQueryRuntimeEffectWriteIntentAuthoring,
    ForgeQueryRuntimeExistingTruthProbeIntentAdmissionReview,
    ForgeQueryRuntimeExistingTruthProbeIntentAuthoring,
    ForgeQueryRuntimeInspectionIntentAdmissionReview, ForgeQueryRuntimeInspectionIntentAuthoring,
    ForgeQueryRuntimeIntentAdmissionReview, ForgeQueryRuntimeIntentAuthoring,
    ForgeQueryRuntimeWriteBatchIntentAdmissionReview, ForgeQueryRuntimeWriteBatchIntentAuthoring,
    ForgeQueryRuntimeWriteIntentAdmissionReview, ForgeQueryRuntimeWriteIntentAuthoring,
    ForgeQueryUnifiedInspectionExecutionBinding, ForgeQueryUnifiedInspectionExecutionHandoff,
    ForgeQueryUnifiedInspectionExecutionPlan,
    ForgeQueryWorkspaceDerivedInspectionIntentAdmissionReview,
    ForgeQueryWorkspaceDerivedInspectionIntentAuthoring,
    ForgeQueryWorkspaceDerivedMaterializationIntentAdmissionReview,
    ForgeQueryWorkspaceDerivedMaterializationIntentAuthoring,
    ForgeQueryWorkspaceLiveReadIntentAdmissionReview, ForgeQueryWorkspaceLiveReadIntentAuthoring,
    ForgeQueryWorkspaceReadIntentAdmissionReview, ForgeQueryWorkspaceReadIntentAuthoring,
};
#[allow(unused_imports)]
pub use crate::lower_runtime_routing::{
    certify_lower_runtime_non_bypass, certify_lower_runtime_performance_slopes,
    certify_lower_runtime_routing, forge_query_lower_runtime_acceptance_suite,
    forge_query_lower_runtime_boundary_reconciliation_report,
    forge_query_lower_runtime_certification_output_manifest,
    forge_query_lower_runtime_closeout_extension_outputs,
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_closeout_report,
    forge_query_lower_runtime_closeout_report_digest, forge_query_lower_runtime_closure_test,
    forge_query_lower_runtime_compile_fail_boundary_digest,
    forge_query_lower_runtime_compile_fail_boundary_target_count,
    forge_query_lower_runtime_crossing_inventory, forge_query_lower_runtime_direct_import_audit,
    forge_query_lower_runtime_gap_registry, forge_query_lower_runtime_golden_transcripts,
    forge_query_lower_runtime_phase_artifact_manifest_digest,
    forge_query_lower_runtime_phase_manifest, forge_query_lower_runtime_phase_progression_digest,
    forge_query_lower_runtime_proof_shape_audit, forge_query_lower_runtime_proof_shape_digest,
    forge_query_lower_runtime_public_surface_inventory,
    forge_query_lower_runtime_required_certification_outputs,
    forge_query_lower_runtime_support_matrix, forge_query_lower_runtime_synthetic_tail_report,
    forge_query_lower_runtime_target_dx_digest,
    forge_query_lower_runtime_typestate_transition_digest, inspect_lower_runtime_boundary,
    inspect_lower_runtime_closeout, summarize_lower_runtime_boundary,
    ForgeQueryLowerRuntimeAcceptanceLane, ForgeQueryLowerRuntimeAcceptanceRow,
    ForgeQueryLowerRuntimeAcceptanceSuite, ForgeQueryLowerRuntimeArtifactStrength,
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryEnvelopeSource, ForgeQueryLowerRuntimeBoundaryExecutionKind,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    ForgeQueryLowerRuntimeBoundaryReconciliationReport,
    ForgeQueryLowerRuntimeBoundaryReconciliationRow, ForgeQueryLowerRuntimeBoundarySummary,
    ForgeQueryLowerRuntimeCapabilityEligibility, ForgeQueryLowerRuntimeCapabilityPosture,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeCertificationBundle,
    ForgeQueryLowerRuntimeCertificationLane, ForgeQueryLowerRuntimeCertificationOutputDigest,
    ForgeQueryLowerRuntimeCertificationRow, ForgeQueryLowerRuntimeCloseoutPosture,
    ForgeQueryLowerRuntimeCloseoutRegistry, ForgeQueryLowerRuntimeCloseoutReport,
    ForgeQueryLowerRuntimeCloseoutRow, ForgeQueryLowerRuntimeClosureTest,
    ForgeQueryLowerRuntimeClosureTestLane, ForgeQueryLowerRuntimeClosureTestRow,
    ForgeQueryLowerRuntimeCostPosture, ForgeQueryLowerRuntimeCrossingClassification,
    ForgeQueryLowerRuntimeCrossingInventory, ForgeQueryLowerRuntimeCrossingRow,
    ForgeQueryLowerRuntimeDirectImportAudit, ForgeQueryLowerRuntimeDirectImportAuditRow,
    ForgeQueryLowerRuntimeDirectImportPosture, ForgeQueryLowerRuntimeFailureTopology,
    ForgeQueryLowerRuntimeGapRegistry, ForgeQueryLowerRuntimeGapRegistryRow,
    ForgeQueryLowerRuntimeGoldenTranscript, ForgeQueryLowerRuntimeNonBypassAudit,
    ForgeQueryLowerRuntimePerformanceFamily, ForgeQueryLowerRuntimePerformanceSlopeReport,
    ForgeQueryLowerRuntimePerformanceSlopeRow, ForgeQueryLowerRuntimePhaseArtifact,
    ForgeQueryLowerRuntimePhaseManifest, ForgeQueryLowerRuntimePhaseManifestRow,
    ForgeQueryLowerRuntimeProofShapeAudit, ForgeQueryLowerRuntimeProofShapeAuditRow,
    ForgeQueryLowerRuntimeProofShapeEnforcement, ForgeQueryLowerRuntimeProofShapeViolation,
    ForgeQueryLowerRuntimePublicSurfaceInventory, ForgeQueryLowerRuntimePublicSurfaceKind,
    ForgeQueryLowerRuntimePublicSurfaceRow, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRoutePlan,
    ForgeQueryLowerRuntimeRoutingInspection, ForgeQueryLowerRuntimeSeamKey,
    ForgeQueryLowerRuntimeSupportDetail, ForgeQueryLowerRuntimeSupportMatrix,
    ForgeQueryLowerRuntimeSupportPosture, ForgeQueryLowerRuntimeSupportRow,
    ForgeQueryLowerRuntimeSyntheticTailReport, ForgeQueryLowerRuntimeSyntheticTailRow,
};
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQueryWorkspaceError,
};
use crate::program::{
    validate_inputs, ForgeQueryAuthorityRequirement, ForgeQueryDerivedView,
    ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryProgram,
    ForgeQueryProgramEffect, ForgeQueryProgramError, ForgeQueryProgramTrace,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::ForgeQuerySessionLabel;
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
mod handle_contract;
mod inspection;
mod intent;
mod journal_position;
mod journal_replay;
mod live_subscription;
mod live_subscription_accessors;
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
mod read_composition_frontier;
mod read_composition_frontier_search;
mod read_composition_hooks;
mod read_composition_lowering;
mod read_composition_materialization;
mod read_composition_operator_builders;
mod read_composition_phase_gate;
mod read_composition_phase_one_closeout;
mod read_composition_relationship_proof;
mod read_composition_runtime;
mod read_composition_shared;
mod read_composition_successor;
mod read_composition_support_report;
mod read_composition_walks;
mod remask_posture;
mod retained_rows;
mod runtime_api_contract;
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
mod runtime_probe_routing_intents;
mod runtime_read_intents;
mod runtime_reads_programs;
mod runtime_session_lowering;
mod runtime_sessions;
mod runtime_unified_inspection_intents;
mod runtime_write_intents;
mod runtime_writes;
mod shared_read;
mod shared_read_pins;
mod state;
mod state_basis;
mod state_basis_classification;
mod state_snapshot;
mod support;
mod support_matrix;
mod surface;
mod time_only_delivery;
mod workspace;
mod workspace_contracts;
mod workspace_declaration;
mod workspace_graph;
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

pub use aspect_api_closeout::ForgeQueryAspectApiFinalizationCloseout;
pub use async_result_state::{
    ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeAsyncResultStateKind,
};
pub use authoritative_mutation_evidence_closeout::ForgeQueryAuthoritativeMutationEvidenceCloseout;
#[allow(unused_imports)]
pub use authoritative_mutation_evidence_support::{
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryBridgeBackedVerificationSupportRow,
    ForgeQueryBridgeBackedVerificationSupportStatus,
};
pub use authority::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryEffectAction,
    ForgeQueryEffectAdmission, ForgeQueryEffectPolicy, ForgeQueryEffectPolicyDenial,
    ForgeQueryPreviewOptions,
};
pub(crate) use backend::build_bridge_authority_bundle;
pub use backend::{
    runtime_subscription_support_evidence_identity, ForgeQueryBridgeBackedRuntimeBackend,
    ForgeQueryIntentAuthorityAdapter, ForgeQueryRuntimeBackend, ForgeQueryRuntimeBackendParts,
    ForgeQueryRuntimeDeclarationInitializationAdapter,
    ForgeQueryRuntimeExistingTruthVerificationAdapter, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimeIntentAuthorityAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSnapshotIdentityAdapter, ForgeQueryRuntimeSourceAdapter,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQueryRuntimeWriteAuthorityAdapter,
    LiveViewDeclarationAdmissionBoundaryReceipt, LiveViewDeclarationAdmissionReceipt,
    SignalInvalidationBoundaryReceipt, SignalInvalidationRoutingReceipt,
    SubscriptionActivationBoundaryReceipt, SubscriptionActivationReceipt,
    WriteAuthorityExecutionReceipt,
};
pub use branch::ForgeQueryBranchSession;
use bridge_mutation_lowering::{bridge_continuity_mutation_bundle, bridge_naming_mutation_bundle};
pub use builder::ForgeQueryRuntimeBuilder;
use computed::{
    admit_derived_view_declaration, insert_derived_runtime,
    retained_live_view_names_for_candidates, route_derived_view_patches,
    ForgeQueryComputedDependencyIndex, ForgeQueryDerivedViewRuntime,
};
pub use computed::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryDerivedPatch, ForgeQueryDerivedPatchFamily,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedRefreshContext,
    ForgeQueryRetainedRefreshOrigin, ForgeQueryRetainedUpstreamInputs,
};
pub use concurrent_hostile_matrix::{
    ForgeQueryConcurrentHostileMatrixCounterSnapshot, ForgeQueryConcurrentHostileMatrixTopology,
    ForgeQueryConcurrentSubmissionIntake, ForgeQueryConcurrentSubmissionLane,
    ForgeQueryConcurrentSubmissionRecord,
};
pub use delivery::ForgeQueryRuntimeDeliveryBatch;
use delivery::{
    register_live_subscription_index, route_live_subscription_delivery,
    ForgeQueryRuntimeLiveSubscriptionActivation, ForgeQueryRuntimeLiveSubscriptionState,
};
use downstream_delivery_contract::project_downstream_delivery;
pub use downstream_delivery_contract::{
    ForgeQueryRuntimeDownstreamDelivery, ForgeQueryRuntimeDownstreamDeliveryClass,
    ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryRuntimeDownstreamSupportPosture,
};
use downstream_delivery_resume::{aggregate_support_posture, support_gate_resume_kind};
pub use downstream_delivery_resume::{
    ForgeQueryRuntimeDownstreamResumePosture, ForgeQueryRuntimeDownstreamResumePostureKind,
};
use effect::{
    admit_effect_declaration, insert_effect_runtime, route_effect_deliveries,
    ForgeQueryEffectIndex, ForgeQueryEffectRuntime,
};
pub use effect::{
    ForgeQueryEffectCondition, ForgeQueryEffectCounters, ForgeQueryEffectDeclaration,
    ForgeQueryEffectDelivery, ForgeQueryEffectDeliveryFamily, ForgeQueryEffectExpression,
    ForgeQueryEffectExpressionFailurePosture, ForgeQueryEffectHandle, ForgeQueryEffectIdempotence,
    ForgeQueryEffectInspectionEvidence, ForgeQueryEffectLoopPrevention, ForgeQueryEffectPhase,
    ForgeQueryEffectPhaseEvidence, ForgeQueryEffectSuppressionPolicy, ForgeQueryEffectTrigger,
    ForgeQueryEffectTriggerSourceKind, ForgeQueryEffectWriteAdjacentTrigger,
    ForgeQueryEffectWriteAdjacentTriggerClass,
};
#[allow(unused_imports)]
pub use error::{
    ForgeQueryRuntimeDeclarationFailureKind, ForgeQueryRuntimeError,
    ForgeQueryRuntimeLookupFailureKind, ForgeQueryRuntimeMissingArtifactKind,
    ForgeQueryRuntimeMissingComponent, ForgeQueryStopClass,
};
#[cfg(test)]
pub(crate) use fallback_seam_counters::{
    forbidden_fallback_seam_invocation_count, record_forbidden_fallback_seam_invocation,
    reset_forbidden_fallback_seam_invocations, ForgeQueryForbiddenFallbackSeam,
};
pub use handle_contract::{
    ForgeQueryHandleContract, ForgeQueryHandleContractFamily, ForgeQueryHandleContractRow,
};
pub use inspection::{
    admit_causal_inspection, anchor_causal_observation,
    build_causal_inspection_certification_scope, causal_evidence_inventory_rows,
    causal_inspection_target, certify_causal_inspection_runtime_path,
    materialize_admitted_causal_inspection, materialize_advisory_causal_inspection,
    materialize_denied_causal_inspection, request_causal_inspection,
    resolve_causal_evidence_references, resolve_indexed_causal_evidence_references,
    AdmittedCausalInspection, AdmittedQueryCausalInspectionArtifact, AdvisoryCausalInspection,
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
    CausalInspectionArtifactIntegrity, CausalInspectionArtifactKind, CausalInspectionBoundaryAudit,
    CausalInspectionBoundaryEnvelopeCategory, CausalInspectionCertificationBundle,
    CausalInspectionCertificationError, CausalInspectionCertificationErrorKind,
    CausalInspectionCertificationFailureEvidence, CausalInspectionCertificationFailureKind,
    CausalInspectionCertificationFailureSource, CausalInspectionCertificationLane,
    CausalInspectionCertificationScope, CausalInspectionEstimatedCost,
    CausalInspectionExplanationFamily, CausalInspectionMaterializationError,
    CausalInspectionMaterializationErrorKind, CausalInspectionMaterializationPolicy,
    CausalInspectionPerformanceCertificationBundle, CausalInspectionPerformanceEnvelope,
    CausalInspectionPlan, CausalInspectionPlanError, CausalInspectionPlanErrorKind,
    CausalInspectionPlanExplanation, CausalInspectionProofFlow,
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
    DeniedQueryCausalInspectionArtifact, ForgeQueryBasisLifecycleInspection,
    ForgeQueryBatchWriteComponentInspection, ForgeQueryBatchWriteReceiptInspection,
    ForgeQueryBranchIntentReceiptInspection, ForgeQueryEffectIntentReceiptInspection,
    ForgeQueryFeedbackPhaseGraphInspection, ForgeQueryFeedbackPhaseNode,
    ForgeQueryFeedbackTermination, ForgeQueryInspection, ForgeQueryInspectionTarget,
    ForgeQueryIntentConsumerInspection, ForgeQueryIntentConsumerOutcomeClass,
    ForgeQueryIntentDenialInspection, ForgeQueryIntentInspectionDeliveryCounters,
    ForgeQueryIntentReceiptInspection, ForgeQueryLiveSubscriptionInspectionCounters,
    ForgeQueryLiveViewInspection, ForgeQueryPreviewBindingInspection,
    ForgeQueryPreviewIntentReceiptInspection, ForgeQueryPreviewOutcomeInspection,
    ForgeQueryWriteReceiptInspection, QueryCausalEvidenceReferenceArtifact,
    QueryCausalInspectionArtifact, QueryCausalTemporalAsyncExplanation,
    QueryCausalTemporalAsyncExplanationKind, QueryObservationReceipt,
    QueryObservationReceiptFamily,
};
pub(crate) use intent::{
    admit_authoritative_intent_declaration, admit_authoritative_intent_execution,
    admit_effect_triggered_intent_declaration, ForgeQueryIntentAdmissionDenial,
};
pub use intent::{
    ForgeQueryBranchIntentReceipt, ForgeQueryEffectIntentReceipt, ForgeQueryIntentDeclaration,
    ForgeQueryIntentDenialEvidence, ForgeQueryIntentExecution,
    ForgeQueryIntentExecutionFailureEvidence, ForgeQueryIntentExecutionKind,
    ForgeQueryIntentExecutionProvenance, ForgeQueryIntentReceipt, ForgeQueryIntentSourceLane,
    ForgeQueryPreviewIntentReceipt,
};
#[allow(unused_imports)]
pub use journal_position::{ForgeQueryJournalPosition, ForgeQueryJournalPositionAuthority};
#[allow(unused_imports)]
pub use journal_position::{
    ForgeQueryJournalPositionAdmissionError, ForgeQueryJournalPositionSchedule,
    ForgeQueryJournalPositionScheduleViolation, ForgeQueryJournalPositionScheduleViolationKind,
};
pub(crate) use journal_replay::journal_replay_truth_reconstruction_identity;
pub use journal_replay::{
    ForgeQueryJournalReplayCounterSnapshot, ForgeQueryJournalReplayDenial,
    ForgeQueryJournalReplayDenialKind, ForgeQueryJournalReplayDiagnostics,
    ForgeQueryJournalReplayOutcome, ForgeQueryJournalReplayRequest,
    ForgeQueryJournalSegmentIdentity,
};
pub(crate) use live_subscription::{
    live_subscription_source_identity, live_subscription_view_shape_source_identity,
};
pub use live_subscription::{
    ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    ForgeQueryRuntimeLiveSubscriptionInstallation,
};
#[allow(unused_imports)]
pub use mixed_cause_delivery::{
    ForgeQueryRuntimeDeliveryCoalescingKind, ForgeQueryRuntimeMixedCauseDelivery,
    ForgeQueryRuntimeMixedCauseLaneKind, ForgeQueryRuntimeMixedCauseMemberKind,
};
use mutation::{admit_continuity_intent, admit_naming_intent};
pub(crate) use mutation::{
    command_declared_aspect_value_digest, command_declared_aspect_value_identity,
};
#[allow(unused_imports)]
pub use mutation::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectMutationOperation,
    ForgeQueryAspectMutationOperationKind, ForgeQueryAspectValue,
    ForgeQueryContinuityMutationDenial, ForgeQueryContinuityMutationDenialKind,
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityMutationIntent,
    ForgeQueryContinuityMutationOutcomeClass, ForgeQueryDeleteMutationBuilder,
    ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthAssertionDenial, ForgeQueryExistingTruthAssertionDenialKind,
    ForgeQueryExistingTruthAssertionMode, ForgeQueryExistingTruthBindingDenial,
    ForgeQueryExistingTruthBindingDenialKind, ForgeQueryExistingTruthBindingFamily,
    ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeDenialKind, ForgeQueryExistingTruthProbeField,
    ForgeQueryExistingTruthProbeMode, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryGraphCompositionBuilder,
    ForgeQueryGraphCompositionDenial, ForgeQueryGraphCompositionDenialKind,
    ForgeQueryGraphCompositionDomainInvariantDenial,
    ForgeQueryGraphCompositionInvariantPackContext,
    ForgeQueryGraphCompositionInvariantPackViolation, ForgeQueryGraphEntitySymbol,
    ForgeQueryGraphRelationMutationBuilder, ForgeQueryGraphRelationSymbol,
    ForgeQueryMutationBatchBuilder, ForgeQueryMutationMetadata, ForgeQueryNamingMutationDenial,
    ForgeQueryNamingMutationDenialKind, ForgeQueryNamingMutationFamily,
    ForgeQueryNamingMutationIntent, ForgeQuerySymbolicAspectReference,
    ForgeQuerySymbolicAspectReferenceFamily, ForgeQuerySymbolicTargetReference,
    ForgeQuerySymbolicTargetReferenceDenial, ForgeQuerySymbolicTargetReferenceDenialKind,
    ForgeQuerySymbolicTargetReferenceFamily, ForgeQueryVerifiedExistingTruthAssertion,
};
pub use mutation_surface::{
    ForgeQueryMutationSurfacePosture, ForgeQueryMutationSurfaceReport, ForgeQueryMutationSurfaceRow,
};
pub use preview::{
    ForgeQueryPreviewCloseoutEvidence, ForgeQueryPreviewCloseoutKind, ForgeQueryPreviewDiff,
    ForgeQueryPreviewEffectBindingDisposition, ForgeQueryPreviewExecutionEvidence,
    ForgeQueryPreviewExecutionKind, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewHandleBindingFamily, ForgeQueryPreviewOutcome,
    ForgeQueryPreviewPromotionDenialEvidence, ForgeQueryPreviewPromotionDenialKind,
    ForgeQueryPreviewResidueClass, ForgeQueryPreviewSession,
};
pub use public_api::{
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicApiFamilyContract,
    ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicApiNamingRow,
    ForgeQueryRuntimePublicApiTranscriptEvidence,
};
#[allow(unused_imports)]
pub use published_artifacts::{
    ForgeQueryPublishedArtifactCounterSnapshot, ForgeQueryPublishedArtifactDiagnostics,
    ForgeQueryPublishedArtifactGenerationDiagnostic,
};
pub use read_composition::ForgeQueryReadBuilder;
pub use read_composition_hooks::{
    ForgeQueryReadInvariantPackContext, ForgeQueryReadInvariantPackViolation,
};
pub use read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};
pub use read_composition_phase_gate::{
    ForgeQueryReadCompositionPhaseGate, ForgeQueryReadCompositionPhaseGateFamily,
    ForgeQueryReadCompositionPhaseGateRow, ForgeQueryReadCompositionPhaseGateStatus,
};
pub use read_composition_phase_one_closeout::ForgeQueryReadCompositionPhaseOneCloseout;
pub use read_composition_support_report::{
    ForgeQueryReadCompositionSupportClass, ForgeQueryReadCompositionSupportReport,
    ForgeQueryReadCompositionSupportRow,
};
pub use remask_posture::{
    ForgeQueryRuntimeRemaskDispositionKind, ForgeQueryRuntimeRemaskPosture,
    ForgeQueryRuntimeRemaskProjection, ForgeQueryRuntimeRemaskReasonKind,
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
    synthetic_existing_assertion_receipt,
};
#[allow(unused_imports)]
pub use shared_read::{
    ForgeQueryPublishedDerivedArtifactHandle, ForgeQueryPublishedProjectionConsumption,
    ForgeQueryPublishedProjectionInspection, ForgeQuerySharedReadBasisInspection,
    ForgeQuerySharedReadContext,
};
pub(in crate::runtime) use shared_read_pins::{
    forge_query_shared_read_stale_basis_error, ForgeQuerySharedReadGenerationLease,
};
#[allow(unused_imports)]
pub use shared_read_pins::{
    ForgeQuerySharedReadCounters, ForgeQuerySharedReadGenerationDiagnostic,
    ForgeQuerySharedReadPinningDiagnostics,
};
pub use state::ForgeQueryRuntimeStateTarget;
pub use state_snapshot::{ForgeQueryRuntimeStateKind, ForgeQueryRuntimeStateSnapshot};
#[allow(unused_imports)]
pub use support::{
    ForgeQueryBasisAdmissionEvidenceRow, ForgeQueryBranchBasisAdmission,
    ForgeQueryBridgeMutationArtifactIdentity, ForgeQueryContinuityPriorAuthorityLabel,
    ForgeQueryContinuitySuccessorAuthorityLabel, ForgeQueryExistingTruthBindingAuthorityLabel,
    ForgeQueryGraphCompositionCapabilityClass, ForgeQueryGraphCompositionCapabilitySupportRow,
    ForgeQueryGraphCompositionExtensionHookBoundary,
    ForgeQueryGraphCompositionExtensionHookSupportRow, ForgeQueryMutationAuthorityIdentity,
    ForgeQueryMutationEvidenceDigest, ForgeQueryMutationSymbolIdentity,
    ForgeQueryMutationTargetCollectionIdentity, ForgeQueryNamingAttachmentAuthorityLabel,
    ForgeQueryNamingPriorAuthorityLabel, ForgeQueryNamingTargetAuthorityLabel,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimeFamilyTeachingPosture, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportDenial, ForgeQueryRuntimeSupportProfile,
};
pub use support_matrix::{
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimePublicSupportMatrixRow,
};
#[allow(unused_imports)]
pub use surface::{
    ForgeQueryArtifactInspector, ForgeQueryBatchMutationEvidence, ForgeQueryBatchWriteReceipt,
    ForgeQueryBatchWriteRetainedArtifact, ForgeQueryContinuityClass,
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityOutcomeClass,
    ForgeQueryContinuityRejectionClass, ForgeQueryDerivedArtifactBinding,
    ForgeQueryDerivedInspectionReceipt, ForgeQueryDerivedInspectionResult,
    ForgeQueryDerivedMaterializationBundle, ForgeQueryDerivedMaterializationReceipt,
    ForgeQueryDerivedMaterializationResult, ForgeQueryDerivedMaterializationTarget,
    ForgeQueryExistingTruthAssertionEvidence, ForgeQueryExistingTruthBindingEvidence,
    ForgeQueryExistingTruthBindingOutcome, ForgeQueryExistingTruthProbeReceipt,
    ForgeQueryExistingTruthProbeResult, ForgeQueryGraphCompositionAdmissionTrace,
    ForgeQueryGraphCompositionAdmissionTraceStage, ForgeQueryGraphCompositionAssumptionSummary,
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionDomainInvariantSummary,
    ForgeQueryGraphCompositionEvidence, ForgeQueryGraphCompositionLifecycleOutcomeEntry,
    ForgeQueryGraphCompositionLifecycleOutcomeKind, ForgeQueryGraphCompositionLifecycleOutcomes,
    ForgeQueryGraphCompositionLineageEntry, ForgeQueryGraphCompositionLineageSummary,
    ForgeQueryGraphCompositionProgram, ForgeQueryGraphCompositionProgramStep,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryGraphCompositionResolutionEntry,
    ForgeQueryGraphCompositionResolutionMap, ForgeQueryInspectedArtifact,
    ForgeQueryInstalledOperation, ForgeQueryInstalledProgram, ForgeQueryLiveArtifactBinding,
    ForgeQueryLiveArtifactBundle, ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadReceipt,
    ForgeQueryLiveReadResult, ForgeQueryLiveView, ForgeQueryMutationCausalityEvidence,
    ForgeQueryMutationFamily, ForgeQueryMutationProvenanceEvidence, ForgeQueryMutationTargetClass,
    ForgeQueryMutationTargetDescriptor, ForgeQueryMutationTargetEvidence,
    ForgeQueryNamingMutationEvidence, ForgeQueryNamingMutationOutcome, ForgeQueryPatchBatch,
    ForgeQueryReadBreadth, ForgeQueryReadBuiltInOperator, ForgeQueryReadBuiltInOperatorDenial,
    ForgeQueryReadBuiltInOperatorDenialReason, ForgeQueryReadCompositionExtensionHookBoundary,
    ForgeQueryReadCompositionExtensionHookFamily, ForgeQueryReadCompositionExtensionHookSupportRow,
    ForgeQueryReadDenial, ForgeQueryReadDenialKind, ForgeQueryReadDomainInvariantDenial,
    ForgeQueryReadDomainInvariantSummary, ForgeQueryReadExecutionEngine,
    ForgeQueryReadFallbackClass, ForgeQueryReadFamily, ForgeQueryReadFamilyAdmission,
    ForgeQueryReadFamilyInvariantEvidence, ForgeQueryReadGraph, ForgeQueryReadGraphFamily,
    ForgeQueryReadOperatorFamily, ForgeQueryReadReceipt, ForgeQueryReadRelationshipProofDenial,
    ForgeQueryReadRelationshipProofDenialStage, ForgeQueryReadRelationshipProofPosture,
    ForgeQueryReadResult, ForgeQueryReadScopeClass, ForgeQueryReadScopeShapeMismatch,
    ForgeQueryRetainedScalarAlignment, ForgeQueryRetainedScalarAlignmentFact,
    ForgeQueryRetainedScalarFactSet, ForgeQueryRetainedScalarFieldFact, ForgeQueryRunReceipt,
    ForgeQuerySymbolicAspectResolutionEvidence, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQuerySymbolicTargetReferenceOutcome, ForgeQueryUnifiedInspectionReceipt,
    ForgeQueryUnifiedInspectionResult, ForgeQueryVerificationReadSetBreadth,
    ForgeQueryVerifiedAssumptionSet, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};
pub use workspace::ForgeQueryWorkspace;
pub use workspace_declaration::{
    ForgeQueryComputedBuilder, ForgeQueryEffectBuilder, ForgeQueryLiveViewBuilder,
    ForgeQueryWorkspaceLiveViewDeclaration,
};
pub use workspace_submission::ForgeQueryWorkspaceSubmissionLane;

pub struct ForgeQueryRuntime {
    backend: Box<dyn ForgeQueryRuntimeBackend>,
    evidence_authority: ForgeQueryRuntimeEvidenceAuthority,
    preview_session_labels: BTreeSet<ForgeQuerySessionLabel>,
    branch_session_labels: BTreeSet<ForgeQuerySessionLabel>,
    active_subscriptions: ActiveSubscriptionRuntime,
    live_subscriptions: BTreeMap<String, ForgeQueryRuntimeLiveSubscriptionState>,
    materialized_read_views: BTreeMap<String, DeclarativeLiveQueryRequest>,
    live_subscription_index: BTreeMap<String, BTreeSet<String>>,
    installed_programs: BTreeMap<String, ForgeQueryProgram>,
    run_traces: BTreeMap<String, ForgeQueryProgramTrace>,
    derived_views: BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    shared_read_pins: shared_read_pins::ForgeQuerySharedReadPinRegistry,
    published_artifacts: published_artifacts::ForgeQueryPublishedArtifactRegistry,
    journal_replay: journal_replay::ForgeQueryJournalReplayRegistry,
    derived_dependency_index: ForgeQueryComputedDependencyIndex,
    effects: BTreeMap<String, ForgeQueryEffectRuntime>,
    effect_index: ForgeQueryEffectIndex,
    next_run_id: u64,
}

struct ForgeQueryRoutedMutationSummary {
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
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
