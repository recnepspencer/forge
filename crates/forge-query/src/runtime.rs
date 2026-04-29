use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeNamingMutationBundle,
    BridgeSymbolicTargetReferenceBundle, RuntimeBridge,
};
use serde_json::Value;

use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, DeclarativeLiveQueryError,
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape,
};
use crate::memory_workspace::{
    ForgeQueryCollection, ForgeQueryEntity, ForgeQueryMemoryApp, ForgeQueryMutationKind,
    ForgeQueryMutationReceipt, ForgeQueryWorkspaceError,
};
use crate::program::{
    validate_inputs, ForgeQueryAuthorityRequirement, ForgeQueryDerivedView,
    ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryProgram,
    ForgeQueryProgramEffect, ForgeQueryProgramError, ForgeQueryProgramTrace,
};
use crate::schema_view::QuerySchemaView;
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
mod authoritative_mutation_evidence_closeout;
mod authority;
mod backend;
mod branch;
mod bridge_mutation_lowering;
mod builder;
mod computed;
mod delivery;
mod effect;
mod error;
mod handle_contract;
mod inspection;
mod intent;
mod live_subscription;
mod mutation;
mod mutation_compatibility;
mod preview;
mod public_api;
mod runtime_api_contract;
mod runtime_declarations;
mod runtime_helpers;
mod runtime_inspection;
mod runtime_intents;
mod runtime_reads_programs;
mod runtime_sessions;
mod runtime_writes;
mod state;
mod support;
mod support_matrix;
mod surface;
mod workspace;
mod workspace_declaration;

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
pub use authoritative_mutation_evidence_closeout::{
    ForgeQueryAuthoritativeMutationEvidenceCloseout, ForgeQueryAuthoritativeMutationEvidenceSupport,
};
pub use authority::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryEffectAction,
    ForgeQueryEffectAdmission, ForgeQueryEffectPolicy, ForgeQueryEffectPolicyDenial,
    ForgeQueryPreviewOptions,
};
pub use backend::{
    ForgeQueryBridgeBackedRuntimeBackend, ForgeQueryRuntimeBackend, ForgeQueryRuntimeBackendParts,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimeIntentAuthorityAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSourceAdapter,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQueryRuntimeWriteAuthorityAdapter,
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
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedMutationContext,
    ForgeQueryRetainedUpstreamInputs,
};
pub use delivery::ForgeQueryRuntimeDeliveryBatch;
use delivery::{
    register_live_subscription_index, route_live_subscription_delivery,
    ForgeQueryRuntimeLiveSubscriptionActivation, ForgeQueryRuntimeLiveSubscriptionState,
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
    ForgeQueryEffectTriggerSourceKind,
};
pub use error::ForgeQueryRuntimeError;
pub use handle_contract::{
    ForgeQueryHandleContract, ForgeQueryHandleContractFamily, ForgeQueryHandleContractRow,
};
pub use inspection::{
    ForgeQueryBatchWriteComponentInspection, ForgeQueryBatchWriteReceiptInspection,
    ForgeQueryBranchIntentReceiptInspection, ForgeQueryEffectIntentReceiptInspection,
    ForgeQueryFeedbackPhaseGraphInspection, ForgeQueryFeedbackPhaseNode,
    ForgeQueryFeedbackTermination, ForgeQueryInspection, ForgeQueryInspectionTarget,
    ForgeQueryIntentDenialInspection, ForgeQueryIntentInspectionDeliveryCounters,
    ForgeQueryIntentReceiptInspection, ForgeQueryLiveSubscriptionInspectionCounters,
    ForgeQueryLiveViewInspection, ForgeQueryPreviewBindingInspection,
    ForgeQueryPreviewIntentReceiptInspection, ForgeQueryPreviewOutcomeInspection,
    ForgeQueryWriteReceiptInspection,
};
pub use intent::{
    ForgeQueryBranchIntentReceipt, ForgeQueryEffectIntentReceipt, ForgeQueryIntentAuthorityAdapter,
    ForgeQueryIntentDeclaration, ForgeQueryIntentDenialEvidence, ForgeQueryIntentExecution,
    ForgeQueryIntentExecutionKind, ForgeQueryIntentReceipt, ForgeQueryIntentSourceLane,
    ForgeQueryPreviewIntentReceipt,
};
pub use live_subscription::ForgeQueryRuntimeLiveSubscriptionInstallation;
pub(crate) use mutation::aspect_values_to_payload;
use mutation::{admit_continuity_intent, admit_naming_intent};
pub use mutation::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectMutationOperation,
    ForgeQueryAspectMutationOperationKind, ForgeQueryAspectValue,
    ForgeQueryContinuityMutationDenial, ForgeQueryContinuityMutationDenialKind,
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityMutationIntent,
    ForgeQueryContinuityMutationOutcomeClass, ForgeQueryDeleteMutationBuilder,
    ForgeQueryExistingTruthBindingDenial, ForgeQueryExistingTruthBindingDenialKind,
    ForgeQueryExistingTruthBindingFamily, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryMutationBatchBuilder, ForgeQueryMutationMetadata, ForgeQueryNamingMutationDenial,
    ForgeQueryNamingMutationDenialKind, ForgeQueryNamingMutationFamily,
    ForgeQueryNamingMutationIntent, ForgeQuerySymbolicTargetReference,
    ForgeQuerySymbolicTargetReferenceDenial, ForgeQuerySymbolicTargetReferenceDenialKind,
    ForgeQuerySymbolicTargetReferenceFamily,
};
pub use mutation_compatibility::{
    ForgeQueryMutationApiCompatibilityReport, ForgeQueryMutationCompatibilityPosture,
    ForgeQueryMutationCompatibilityRow,
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
    ForgeQueryRuntimePublicApiTranscriptEvidence, ForgeQueryRuntimeStateKind,
    ForgeQueryRuntimeStateSnapshot,
};
#[cfg(test)]
use runtime_helpers::runtime_subscription_budget_digest;
use runtime_helpers::{
    admit_authority_requirements, attach_continuity_mutation_to_receipt,
    attach_naming_mutation_to_receipt, attach_symbolic_target_reference_to_receipt,
    classify_receipt_mutation_summary, combined_batch_mutation_receipt, live_subscription_error,
    record_same_batch_symbolic_target, resolve_same_batch_symbolic_target,
    runtime_active_lifecycle_budget, runtime_bridge_lowering_budget,
    runtime_consumer_attachment_budget, runtime_family_budget, runtime_slice_budget,
    runtime_subscription_admission_budget, runtime_subscription_budget_policy,
    subscription_dimensions_for_request,
};
pub use state::ForgeQueryRuntimeStateTarget;
pub use support::{
    ForgeQueryBranchBasisAdmission, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportDenial, ForgeQueryRuntimeSupportProfile,
};
pub use support_matrix::{
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimePublicSupportMatrixRow,
};
#[allow(unused_imports)]
pub use surface::{
    ForgeQueryArtifactInspector, ForgeQueryBatchMutationEvidence, ForgeQueryBatchWriteReceipt,
    ForgeQueryContinuityClass, ForgeQueryContinuityMutationEvidence,
    ForgeQueryContinuityOutcomeClass, ForgeQueryContinuityRejectionClass,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryExistingTruthBindingOutcome,
    ForgeQueryInspectedArtifact, ForgeQueryInstalledOperation, ForgeQueryInstalledProgram,
    ForgeQueryLiveView, ForgeQueryMutationCausalityEvidence, ForgeQueryMutationFamily,
    ForgeQueryMutationProvenanceEvidence, ForgeQueryMutationTargetClass,
    ForgeQueryMutationTargetDescriptor, ForgeQueryMutationTargetEvidence,
    ForgeQueryNamingMutationEvidence, ForgeQueryNamingMutationOutcome, ForgeQueryPatchBatch,
    ForgeQueryRunReceipt, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQuerySymbolicTargetReferenceOutcome, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};
pub use workspace::ForgeQueryWorkspace;
pub use workspace_declaration::{
    ForgeQueryComputedBuilder, ForgeQueryEffectBuilder, ForgeQueryLiveViewBuilder,
    ForgeQueryWorkspaceLiveViewDeclaration,
};

pub struct ForgeQueryRuntime {
    backend: Box<dyn ForgeQueryRuntimeBackend>,
    evidence_authority: ForgeQueryRuntimeEvidenceAuthority,
    active_subscriptions: ActiveSubscriptionRuntime,
    live_subscriptions: BTreeMap<String, ForgeQueryRuntimeLiveSubscriptionState>,
    live_subscription_index: BTreeMap<String, BTreeSet<String>>,
    installed_programs: BTreeMap<String, ForgeQueryProgram>,
    run_traces: BTreeMap<String, ForgeQueryProgramTrace>,
    derived_views: BTreeMap<String, ForgeQueryDerivedViewRuntime>,
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
mod tests;
