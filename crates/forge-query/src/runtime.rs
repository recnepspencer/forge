use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;

use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, DeclarativeLiveQueryError,
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape,
};
use crate::memory_workspace::{
    ForgeQueryCollection, ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMemoryApp, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError,
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

mod authority;
mod backend;
mod branch;
mod builder;
mod computed;
mod delivery;
mod effect;
mod error;
mod inspection;
mod intent;
mod live_subscription;
mod preview;
mod support;
mod surface;

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
pub use builder::ForgeQueryRuntimeBuilder;
use computed::{
    admit_derived_view_declaration, insert_derived_runtime, route_derived_view_patches,
    ForgeQueryComputedDependencyIndex, ForgeQueryDerivedViewRuntime,
};
pub use computed::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryDerivedPatch, ForgeQueryDerivedPatchFamily,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization,
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
pub use inspection::{
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
pub use preview::{
    ForgeQueryPreviewCloseoutEvidence, ForgeQueryPreviewCloseoutKind, ForgeQueryPreviewDiff,
    ForgeQueryPreviewEffectBindingDisposition, ForgeQueryPreviewExecutionEvidence,
    ForgeQueryPreviewExecutionKind, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewHandleBindingFamily, ForgeQueryPreviewOutcome,
    ForgeQueryPreviewPromotionDenialEvidence, ForgeQueryPreviewPromotionDenialKind,
    ForgeQueryPreviewResidueClass, ForgeQueryPreviewSession,
};
pub use support::{
    ForgeQueryBranchBasisAdmission, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportDenial, ForgeQueryRuntimeSupportProfile,
};
pub use surface::{
    ForgeQueryArtifactInspector, ForgeQueryInspectedArtifact, ForgeQueryInstalledOperation,
    ForgeQueryInstalledProgram, ForgeQueryLiveView, ForgeQueryPatchBatch, ForgeQueryRunReceipt,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
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

impl ForgeQueryRuntime {
    pub fn builder() -> ForgeQueryRuntimeBuilder {
        ForgeQueryRuntimeBuilder::new()
    }

    pub fn declare_live_view<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Live)?;
        let name = name.into();
        self.backend
            .admit_live_view_declaration(&name, &request, &schema_view)
            .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: name.clone(),
                    stage: "backend-live-admission",
                    message: error.to_string(),
                },
            )?;
        let activation =
            self.install_live_subscription_for_request(&name, &request, schema_view.clone())?;
        let handle = match self
            .backend
            .declare_live_view(name.clone(), request, schema_view)
        {
            Ok(handle) => handle,
            Err(error) => {
                let closeout_result = close_subscription_lifecycle(
                    &mut self.active_subscriptions,
                    &activation.active_lane_handle,
                    SubscriptionLifecycleCloseRequest::DetachConsumer(
                        activation.consumer_attachment.clone(),
                    ),
                );
                let closeout_message = match closeout_result {
                    Ok(closeout) => format!(
                        "active subscription closeout:{}:terminal:{}",
                        closeout.closeout_digest(),
                        closeout.lane_terminal()
                    ),
                    Err(closeout_error) => format!(
                        "active subscription closeout failed:{}:{}",
                        closeout_error.denial_kind().as_str(),
                        closeout_error.message()
                    ),
                };
                return Err(ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: name,
                    stage: "source-declaration",
                    message: format!("{error}; {closeout_message}"),
                });
            }
        };
        register_live_subscription_index(
            &mut self.live_subscription_index,
            &name,
            &activation.request,
        );
        self.live_subscriptions.insert(
            name,
            ForgeQueryRuntimeLiveSubscriptionState {
                installation: activation.installation.clone(),
                active_lane_handle: activation.active_lane_handle,
                consumer_attachment: activation.consumer_attachment,
                request: activation.request,
                delivery_batches: Vec::new(),
            },
        );
        Ok(ForgeQueryLiveView::new(handle, activation.installation))
    }

    pub fn declare_derived_view(
        &mut self,
        view: ForgeQueryDerivedView,
    ) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)?;
        self.admit_derived_view_declaration(&view)?;
        insert_derived_runtime(
            &mut self.derived_views,
            &mut self.derived_dependency_index,
            view.clone(),
            None,
        );
        Ok(view)
    }

    pub fn declare_maintained_derived_view<T>(
        &mut self,
        view: ForgeQueryDerivedView,
        maintainer: impl ForgeQueryDerivedViewMaintainer + 'static,
    ) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)?;
        self.admit_derived_view_declaration(&view)?;
        let name = view.name().to_string();
        insert_derived_runtime(
            &mut self.derived_views,
            &mut self.derived_dependency_index,
            view,
            Some(Box::new(maintainer)),
        );
        Ok(ForgeQueryDerivedViewHandle::new(name))
    }

    fn admit_derived_view_declaration(
        &self,
        view: &ForgeQueryDerivedView,
    ) -> Result<(), ForgeQueryRuntimeError> {
        let live_view_names = self.live_subscriptions.keys().cloned().collect();
        admit_derived_view_declaration(&self.derived_views, &live_view_names, view).map_err(
            |error| ForgeQueryRuntimeError::ComputedDeclaration {
                view_name: view.name().to_string(),
                stage: "dependency-admission",
                message: error.message(),
            },
        )
    }

    pub fn declare_effect<T>(
        &mut self,
        declaration: ForgeQueryEffectDeclaration,
    ) -> Result<ForgeQueryEffectHandle<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        let live_view_names = self.live_subscriptions.keys().cloned().collect();
        let computed_view_names = self.derived_views.keys().cloned().collect();
        admit_effect_declaration(&live_view_names, &computed_view_names, &declaration)?;
        let name = declaration.name().to_string();
        let target_lane = declaration.target_lane();
        insert_effect_runtime(&mut self.effects, &mut self.effect_index, declaration);
        Ok(ForgeQueryEffectHandle::new(name, target_lane))
    }

    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        let receipt = self.backend.write(command)?;
        self.route_authoritative_mutation_receipt(receipt)
    }

    pub fn execute_intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        intent::admit_authoritative_intent_declaration(&declaration).map_err(|denial| {
            let evidence = ForgeQueryIntentDenialEvidence::new(&declaration, &denial, None);
            ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name: declaration.name().to_string(),
                stage: denial.stage(),
                message: denial.message().to_string(),
                evidence,
            }
        })?;
        let execution = self.backend.execute_intent(&declaration)?;
        intent::admit_authoritative_intent_execution(&declaration, &execution).map_err(
            |denial| {
                let evidence =
                    ForgeQueryIntentDenialEvidence::new(&declaration, &denial, Some(&execution));
                ForgeQueryRuntimeError::IntentCommitDenied {
                    intent_name: declaration.name().to_string(),
                    stage: denial.stage(),
                    message: denial.message().to_string(),
                    evidence,
                }
            },
        )?;
        let write_receipt =
            self.route_authoritative_mutation_receipt(execution.mutation_receipt().clone())?;
        Ok(ForgeQueryIntentReceipt::new(
            &declaration,
            execution,
            &write_receipt,
        ))
    }

    pub fn execute_next_effect_write_intent<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        let strategy_version = strategy_version.into();
        let input_contract = input_contract.into();
        let (pending_index, pending_delivery) = {
            let runtime = self
                .effects
                .get(effect.name())
                .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))?;
            runtime
                .deliveries
                .iter()
                .enumerate()
                .find(|(_, delivery)| {
                    delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                })
                .map(|(index, delivery)| (index, delivery.clone()))
                .ok_or_else(|| {
                    ForgeQueryRuntimeError::MissingPendingWriteIntent(effect.name().to_string())
                })?
        };
        let declaration = ForgeQueryIntentDeclaration::strategy_commit(
            format!(
                "effect:{}:{}",
                pending_delivery.effect_name(),
                pending_delivery.commit_identity()
            ),
            pending_delivery.target().to_string(),
            strategy_version,
            input_contract,
            pending_delivery.payload().clone(),
        )
        .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);
        intent::admit_effect_triggered_intent_declaration(&declaration).map_err(|denial| {
            let evidence = ForgeQueryIntentDenialEvidence::new(&declaration, &denial, None);
            ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name: declaration.name().to_string(),
                stage: denial.stage(),
                message: denial.message().to_string(),
                evidence,
            }
        })?;
        let execution = self.backend.execute_intent(&declaration)?;
        intent::admit_authoritative_intent_execution(&declaration, &execution).map_err(
            |denial| {
                let evidence =
                    ForgeQueryIntentDenialEvidence::new(&declaration, &denial, Some(&execution));
                ForgeQueryRuntimeError::IntentCommitDenied {
                    intent_name: declaration.name().to_string(),
                    stage: denial.stage(),
                    message: denial.message().to_string(),
                    evidence,
                }
            },
        )?;
        let write_receipt =
            self.route_authoritative_mutation_receipt(execution.mutation_receipt().clone())?;
        let intent_receipt = ForgeQueryIntentReceipt::new(&declaration, execution, &write_receipt);
        if let Some(runtime) = self.effects.get_mut(effect.name()) {
            if runtime
                .deliveries
                .get(pending_index)
                .is_some_and(|delivery| {
                    delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                        && delivery.effect_name() == pending_delivery.effect_name()
                        && delivery.commit_identity() == pending_delivery.commit_identity()
                })
            {
                runtime.deliveries.remove(pending_index);
            } else if let Some(index) = runtime.deliveries.iter().position(|delivery| {
                delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                    && delivery.effect_name() == pending_delivery.effect_name()
                    && delivery.commit_identity() == pending_delivery.commit_identity()
            }) {
                runtime.deliveries.remove(index);
            }
        }
        Ok(ForgeQueryEffectIntentReceipt::new(
            &pending_delivery,
            intent_receipt,
        ))
    }

    fn route_authoritative_mutation_receipt(
        &mut self,
        receipt: ForgeQueryMutationReceipt,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let affected_live_view_ids = route_live_subscription_delivery(
            &mut self.active_subscriptions,
            &mut self.live_subscriptions,
            &self.live_subscription_index,
            &receipt,
        )?;
        let computed_candidate_live_views = self.computed_candidate_live_views(&receipt);
        let computed_result = route_derived_view_patches(
            &mut self.derived_views,
            &self.derived_dependency_index,
            computed_candidate_live_views,
            &receipt,
        );
        let refresh_fallback = computed_result.refresh_fallback();
        let considered_computed_view_count = computed_result.considered_view_count();
        let affected_derived_view_ids = computed_result.affected_view_ids();
        let live_view_targets = self.live_view_targets();
        let effect_result = route_effect_deliveries(
            &mut self.effects,
            &self.effect_index,
            &self.derived_views,
            &live_view_targets,
            &receipt,
            &affected_live_view_ids,
            &affected_derived_view_ids,
        );
        Ok(ForgeQueryWriteReceipt::from_mutation_receipt(
            receipt,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            effect_result.considered_effect_count(),
            effect_result.delivered_effect_count(),
            effect_result.pending_write_intent_count(),
            effect_result.suppressed_effect_count(),
            effect_result.meaningful_suppression_count(),
            effect_result.expression_failure_count(),
            refresh_fallback,
        ))
    }

    pub fn read_live<T>(&self, view: &ForgeQueryLiveView<T>) -> Vec<ForgeQueryEntity> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)
            .expect("read support was admitted before live view declaration");
        self.backend.live_entities(view.name())
    }

    pub fn drain_patches<T>(&mut self, view: &ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Live)
            .expect("live support was admitted before patch draining");
        let _compatibility_patches = self.backend.drain_live_patches(view.name());
        ForgeQueryPatchBatch {
            view_name: view.name().to_string(),
            live_patches: Vec::new(),
            query_delivery_batches: self
                .live_subscriptions
                .get_mut(view.name())
                .map(|state| std::mem::take(&mut state.delivery_batches))
                .unwrap_or_default(),
            derived_patch_notes: Vec::new(),
            derived_patches: Vec::new(),
        }
    }

    pub fn drain_derived_patches(&mut self, view_name: &str) -> ForgeQueryPatchBatch {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)
            .expect("computed support was admitted before derived patch draining");
        let derived_patches = self
            .derived_views
            .get_mut(view_name)
            .map(|view| std::mem::take(&mut view.patches))
            .unwrap_or_default();
        ForgeQueryPatchBatch {
            view_name: view_name.to_string(),
            live_patches: Vec::new(),
            query_delivery_batches: Vec::new(),
            derived_patch_notes: derived_patches
                .iter()
                .map(ForgeQueryDerivedPatch::note)
                .collect(),
            derived_patches,
        }
    }

    pub fn read_derived<T>(&self, view: &ForgeQueryDerivedViewHandle<T>) -> Vec<Value> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)
            .expect("computed support was admitted before derived view read");
        self.derived_views
            .get(view.name())
            .map(|runtime| runtime.materialization.rows().to_vec())
            .unwrap_or_default()
    }

    pub fn inspect_derived_view<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<ForgeQueryComputedInspectionEvidence, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.derived_views
            .get(view.name())
            .map(ForgeQueryComputedInspectionEvidence::from_runtime)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingDerivedView(view.name().to_string()))
    }

    pub fn drain_effect_deliveries<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<Vec<ForgeQueryEffectDelivery>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        self.effects
            .get_mut(effect.name())
            .map(|runtime| std::mem::take(&mut runtime.deliveries))
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))
    }

    pub fn inspect_effect<T>(
        &self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<ForgeQueryEffectInspectionEvidence, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.inspect_effect_by_name(effect.name())
    }

    pub(super) fn inspect_effect_by_name(
        &self,
        effect_name: &str,
    ) -> Result<ForgeQueryEffectInspectionEvidence, ForgeQueryRuntimeError> {
        self.effects
            .get(effect_name)
            .map(ForgeQueryEffectInspectionEvidence::from_runtime)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect_name.to_string()))
    }

    fn computed_candidate_live_views(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> BTreeSet<String> {
        let mut candidates = BTreeSet::new();
        for delta in &receipt.deltas {
            if let Some(view_names) = self.live_subscription_index.get(&delta.collection) {
                candidates.extend(view_names.iter().cloned());
            }
        }
        candidates
    }

    fn live_view_targets(&self) -> BTreeMap<String, String> {
        self.live_subscriptions
            .iter()
            .map(|(view_name, state)| (view_name.clone(), state.request.target().to_string()))
            .collect()
    }

    pub fn snapshot_token(&self) -> String {
        self.backend.snapshot_token()
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        let program_id = program.id().to_string();
        self.installed_programs.insert(program_id.clone(), program);
        Ok(ForgeQueryInstalledProgram { program_id })
    }

    pub fn run_operation(
        &mut self,
        operation: ForgeQueryInstalledOperation,
        inputs: Vec<ForgeQueryOperationInput>,
    ) -> Result<ForgeQueryRunReceipt, ForgeQueryRuntimeError> {
        let query_operation = self.installed_query_operation(&operation)?;
        admit_authority_requirements(query_operation.authority_requirements())?;
        let bound_inputs = validate_inputs(&query_operation, &inputs)?;
        let mut trace = ForgeQueryProgramTrace::new(
            operation.program_id.clone(),
            operation.operation_id.clone(),
            &bound_inputs,
            query_operation
                .authority_requirements()
                .iter()
                .cloned()
                .collect(),
        );
        let mut outputs = Vec::new();
        let mut write_receipts = Vec::new();
        let mut patch_batches = Vec::new();
        for effect in query_operation.effects() {
            match effect.clone() {
                ForgeQueryProgramEffect::DeclareLiveView {
                    name,
                    request,
                    schema_view,
                } => {
                    let _: ForgeQueryLiveView<Value> =
                        self.declare_live_view(name.clone(), request, schema_view)?;
                    trace.record_declaration(format!("live:{name}"));
                }
                ForgeQueryProgramEffect::DeclareDerivedView(view) => {
                    let name = view.name().to_string();
                    self.declare_derived_view(view)?;
                    trace.record_declaration(format!("derived:{name}"));
                }
                ForgeQueryProgramEffect::Write(command) => {
                    let receipt = self.write(command)?;
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::WriteTemplate(template) => {
                    let command = template.bind(&bound_inputs)?;
                    let receipt = self.write(command)?;
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::ReadLive { view_name } => {
                    let rows = self.backend.live_entities(&view_name);
                    outputs.push(ForgeQueryOperationOutput::new(
                        format!("live:{view_name}"),
                        Value::Array(rows.into_iter().map(|row| row.payload).collect()),
                    ));
                    trace.record_replay_or_parity(format!("read-live:{view_name}"));
                }
                ForgeQueryProgramEffect::DrainPatches { view_name } => {
                    let _compatibility_patches = self.backend.drain_live_patches(&view_name);
                    let query_delivery_batches = self
                        .live_subscriptions
                        .get_mut(&view_name)
                        .map(|state| std::mem::take(&mut state.delivery_batches))
                        .unwrap_or_default();
                    for batch in &query_delivery_batches {
                        trace.record_patch_artifact(format!(
                            "query-delivery:{}:{}",
                            batch.view_name(),
                            batch.delivery_batch_digest()
                        ));
                    }
                    patch_batches.push(ForgeQueryPatchBatch {
                        view_name,
                        live_patches: Vec::new(),
                        query_delivery_batches,
                        derived_patch_notes: Vec::new(),
                        derived_patches: Vec::new(),
                    });
                }
            }
        }
        let run_id = self.next_run_identity(&operation);
        self.run_traces.insert(run_id.clone(), trace);
        Ok(ForgeQueryRunReceipt {
            run_id,
            operation,
            outputs,
            write_receipts,
            patch_batches,
        })
    }

    fn installed_query_operation(
        &self,
        operation: &ForgeQueryInstalledOperation,
    ) -> Result<crate::program::ForgeQueryOperation, ForgeQueryRuntimeError> {
        let program = self
            .installed_programs
            .get(&operation.program_id)
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(operation.program_id.clone()))?;
        program
            .operation(&operation.operation_id)
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownOperation {
                program_id: operation.program_id.clone(),
                operation_id: operation.operation_id.clone(),
            })
            .cloned()
    }

    fn next_run_identity(&mut self, operation: &ForgeQueryInstalledOperation) -> String {
        self.next_run_id += 1;
        format!(
            "query-run:{}:{}:{}",
            operation.program_id, operation.operation_id, self.next_run_id
        )
    }

    pub fn inspect_run(
        &self,
        run: &ForgeQueryRunReceipt,
    ) -> Result<ForgeQueryProgramTrace, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.run_traces
            .get(run.run_id())
            .cloned()
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(run.run_id().to_string()))
    }

    pub fn inspect_live_view<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryRuntimeLiveSubscriptionInstallation, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.live_subscriptions
            .get(view.name())
            .map(|state| &state.installation)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view.name().to_string()))
    }

    pub fn inspect_live_view_explanation<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryLiveViewInspection, ForgeQueryRuntimeError> {
        let installation = self.inspect_live_view(view)?;
        Ok(ForgeQueryLiveViewInspection::from_installation(
            installation,
        ))
    }

    pub fn inspect_receipt<'a>(
        &'a self,
        receipt: &'a ForgeQueryWriteReceipt,
    ) -> ForgeQueryArtifactInspector<'a> {
        self.try_inspect_receipt(receipt)
            .expect("inspect support must be admitted before inspecting receipts")
    }

    pub fn try_inspect_receipt<'a>(
        &'a self,
        receipt: &'a ForgeQueryWriteReceipt,
    ) -> Result<ForgeQueryArtifactInspector<'a>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        let runtime_evidence = self
            .backend
            .inspect_write_receipt(receipt, &self.evidence_authority)?;
        Ok(ForgeQueryArtifactInspector {
            receipt,
            runtime_evidence,
        })
    }

    pub fn inspect_intent_receipt(
        &self,
        receipt: &ForgeQueryIntentReceipt,
    ) -> Result<ForgeQueryIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryIntentReceiptInspection::from_receipt(receipt))
    }

    pub fn inspect_effect_intent_receipt(
        &self,
        receipt: &ForgeQueryEffectIntentReceipt,
    ) -> Result<ForgeQueryEffectIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryEffectIntentReceiptInspection::from_receipt(
            receipt,
        ))
    }

    pub fn inspect_intent_denial(
        &self,
        evidence: &ForgeQueryIntentDenialEvidence,
    ) -> Result<ForgeQueryIntentDenialInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryIntentDenialInspection::from_evidence(evidence))
    }

    pub fn inspect_preview_binding(
        &self,
        binding: &ForgeQueryPreviewHandleBindingEvidence,
    ) -> Result<ForgeQueryPreviewBindingInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryPreviewBindingInspection::from_binding(binding))
    }

    pub fn inspect_preview_outcome(
        &self,
        outcome: &ForgeQueryPreviewOutcome,
    ) -> Result<ForgeQueryPreviewOutcomeInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryPreviewOutcomeInspection::from_outcome(outcome))
    }

    pub fn inspect_preview_intent_receipt(
        &self,
        receipt: &ForgeQueryPreviewIntentReceipt,
    ) -> Result<ForgeQueryPreviewIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryPreviewIntentReceiptInspection::from_receipt(
            receipt,
        ))
    }

    pub fn inspect_branch_intent_receipt(
        &self,
        receipt: &ForgeQueryBranchIntentReceipt,
    ) -> Result<ForgeQueryBranchIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryBranchIntentReceiptInspection::from_receipt(
            receipt,
        ))
    }

    pub fn inspect_feedback_path<T>(
        &self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<ForgeQueryFeedbackPhaseGraphInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        let runtime = self
            .effects
            .get(effect.name())
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))?;
        ForgeQueryFeedbackPhaseGraphInspection::from_effect_runtime(runtime).ok_or_else(|| {
            ForgeQueryRuntimeError::MissingEffect(format!(
                "{} has no retained feedback delivery",
                effect.name()
            ))
        })
    }

    pub fn inspect_effect_feedback_receipt(
        &self,
        receipt: &ForgeQueryEffectIntentReceipt,
    ) -> Result<ForgeQueryFeedbackPhaseGraphInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryFeedbackPhaseGraphInspection::from_effect_intent_receipt(receipt))
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<ForgeQueryInspection, ForgeQueryRuntimeError>
    where
        T: Into<ForgeQueryInspectionTarget<'a>>,
    {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        match target.into() {
            ForgeQueryInspectionTarget::LiveView { name } => {
                let installation = self
                    .live_subscriptions
                    .get(name)
                    .map(|state| &state.installation)
                    .ok_or_else(|| {
                        ForgeQueryRuntimeError::MissingLiveSubscription(name.to_string())
                    })?;
                Ok(ForgeQueryInspection::LiveView(
                    ForgeQueryLiveViewInspection::from_installation(installation),
                ))
            }
            ForgeQueryInspectionTarget::DerivedView { name } => {
                Ok(ForgeQueryInspection::DerivedView(
                    self.derived_views
                        .get(name)
                        .map(ForgeQueryComputedInspectionEvidence::from_runtime)
                        .ok_or_else(|| {
                            ForgeQueryRuntimeError::MissingDerivedView(name.to_string())
                        })?,
                ))
            }
            ForgeQueryInspectionTarget::Effect { name } => Ok(ForgeQueryInspection::Effect(
                self.inspect_effect_by_name(name)?,
            )),
            ForgeQueryInspectionTarget::WriteReceipt(receipt) => {
                let runtime_evidence = self
                    .backend
                    .inspect_write_receipt(receipt, &self.evidence_authority)?;
                Ok(ForgeQueryInspection::WriteReceipt(
                    ForgeQueryWriteReceiptInspection::new(receipt, runtime_evidence),
                ))
            }
            ForgeQueryInspectionTarget::IntentReceipt(receipt) => Ok(
                ForgeQueryInspection::IntentReceipt(self.inspect_intent_receipt(receipt)?),
            ),
            ForgeQueryInspectionTarget::IntentDenial(evidence) => Ok(
                ForgeQueryInspection::IntentDenial(self.inspect_intent_denial(evidence)?),
            ),
            ForgeQueryInspectionTarget::EffectIntentReceipt(receipt) => {
                Ok(ForgeQueryInspection::EffectIntentReceipt(
                    self.inspect_effect_intent_receipt(receipt)?,
                ))
            }
            ForgeQueryInspectionTarget::PreviewBinding(binding) => Ok(
                ForgeQueryInspection::PreviewBinding(self.inspect_preview_binding(binding)?),
            ),
            ForgeQueryInspectionTarget::PreviewOutcome(outcome) => Ok(
                ForgeQueryInspection::PreviewOutcome(self.inspect_preview_outcome(outcome)?),
            ),
            ForgeQueryInspectionTarget::PreviewIntentReceipt(receipt) => {
                Ok(ForgeQueryInspection::PreviewIntentReceipt(
                    self.inspect_preview_intent_receipt(receipt)?,
                ))
            }
            ForgeQueryInspectionTarget::BranchIntentReceipt(receipt) => {
                Ok(ForgeQueryInspection::BranchIntentReceipt(
                    self.inspect_branch_intent_receipt(receipt)?,
                ))
            }
        }
    }

    pub fn preview<'a>(
        &'a mut self,
        label: impl Into<String>,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.preview_with_options(label, ForgeQueryPreviewOptions::default())
    }

    pub fn branch<'a>(
        &'a mut self,
        label: impl Into<String>,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.branch_with_options(label, ForgeQueryBranchOptions::default())
    }

    pub fn branch_with_options<'a>(
        &'a mut self,
        label: impl Into<String>,
        options: ForgeQueryBranchOptions,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.try_branch_with_options(label, options)
    }

    pub fn try_branch<'a>(
        &'a mut self,
        label: impl Into<String>,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.try_branch_with_options(label, ForgeQueryBranchOptions::default())
    }

    pub fn try_branch_with_options<'a>(
        &'a mut self,
        label: impl Into<String>,
        options: ForgeQueryBranchOptions,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        let label = label.into();
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)?;
        let branch_support_evidence = self
            .backend
            .support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::BranchPreview)
            .map(|support| support.evidence().to_vec())
            .unwrap_or_default();
        let mut evidence = vec!["runtime-branch-basis-admission".to_string()];
        evidence.extend(branch_support_evidence);
        let basis_admission = ForgeQueryBranchBasisAdmission::new(
            &self.evidence_authority,
            &label,
            options.effect_policy(),
            evidence,
        );
        Ok(ForgeQueryBranchSession::new(
            label,
            self,
            options,
            basis_admission,
        ))
    }

    pub fn preview_with_options<'a>(
        &'a mut self,
        label: impl Into<String>,
        options: ForgeQueryPreviewOptions,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.try_preview_with_options(label, options)
    }

    pub fn try_preview<'a>(
        &'a mut self,
        label: impl Into<String>,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.try_preview_with_options(label, ForgeQueryPreviewOptions::default())
    }

    pub fn try_preview_with_options<'a>(
        &'a mut self,
        label: impl Into<String>,
        options: ForgeQueryPreviewOptions,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        let label = label.into();
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)?;
        let basis_admission = self.backend.admit_preview_basis(
            &label,
            options.effect_policy(),
            &self.evidence_authority,
        )?;
        Ok(ForgeQueryPreviewSession::new(
            label,
            self,
            options.effect_policy(),
            basis_admission,
        ))
    }

    pub fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.backend.support_profile()
    }

    fn admit_facade_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.backend
            .support_profile()
            .admit(family)
            .map_err(ForgeQueryRuntimeError::UnsupportedFacadeFamily)
    }

    pub(in crate::runtime) fn admit_facade_family_lane(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
        authority_lane: ForgeQueryAuthorityLane,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.admit_facade_family(family)?;
        let support_profile = self.backend.support_profile();
        let Some(row) = support_profile.support_for(family) else {
            return Err(ForgeQueryRuntimeError::UnsupportedFacadeFamily(
                ForgeQueryRuntimeSupportDenial::new(
                    family,
                    "backend support profile does not declare this facade family",
                ),
            ));
        };
        if row.authority_lanes().contains(&authority_lane) {
            Ok(())
        } else {
            Err(ForgeQueryRuntimeError::UnsupportedFacadeFamily(
                ForgeQueryRuntimeSupportDenial::new(
                    family,
                    format!(
                        "backend support profile does not admit `{}` lane for `{}` facade family",
                        authority_lane, family
                    ),
                ),
            ))
        }
    }

    fn install_live_subscription_for_request(
        &mut self,
        view_name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryRuntimeLiveSubscriptionActivation, ForgeQueryRuntimeError> {
        let grouped_baseline_members =
            self.backend
                .grouped_baseline_members(request)
                .map_err(
                    |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                        view_name: view_name.to_string(),
                        stage: "grouped-baseline",
                        message: error.to_string(),
                    },
                )?;
        let session = declare_runtime_live_query_session_with_grouped_baseline(
            request.clone(),
            schema_view,
            self.backend.snapshot_token(),
            grouped_baseline_members,
        )
        .map_err(|error| live_subscription_error(view_name, "live-lowering", error))?;
        let view_family = session.live_view().lowering().family();
        let dimensions = subscription_dimensions_for_request(request, view_family)?;
        let live_admission =
            crate::subscription::LiveQueryAdmissionArtifact::from_live_promotion_with_view(
                session.live_view().core_live_plan().descriptor(),
                crate::subscription::QuerySubscriptionBasisPosture::CurrentHead,
                view_family,
                dimensions,
            );
        let selection = select_query_subscription_family(live_admission, runtime_family_budget())
            .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "family-selection",
                message: format!("{error:?}"),
            },
        )?;
        let subscription_family = selection.family().as_str().to_string();
        let declaration =
            declare_query_subscription(selection, runtime_slice_budget()).map_err(|error| {
                ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.to_string(),
                    stage: "declaration",
                    message: format!("{error:?}"),
                }
            })?;
        let subscription_declaration_digest = declaration.declaration_digest().as_str().to_string();
        let lowering =
            lower_query_subscription_to_bridge(declaration, runtime_bridge_lowering_budget())
                .map_err(
                    |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                        view_name: view_name.to_string(),
                        stage: "bridge-lowering",
                        message: format!("{error:?}"),
                    },
                )?;
        let admission = admit_query_subscription(lowering, runtime_subscription_admission_budget())
            .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.to_string(),
                    stage: "subscription-admission",
                    message: format!("{error:?}"),
                },
            )?;
        let admission_digest = admission.admission_digest().to_string();
        let bridge_declaration_digest = admission.bridge_declaration_digest().to_string();
        let basis_binding_digest = admission.basis_binding_digest().to_string();
        let signal_strategy_digest = admission.signal_strategy_digest().to_string();
        let activation = prepare_subscription_activation(admission);
        let activation_digest = activation.activation_digest().to_string();
        let counters = activation.counters().clone();
        let support_evidence = self
            .backend
            .install_live_subscription(view_name, &activation)
            .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.to_string(),
                    stage: "activation-admission",
                    message: error.to_string(),
                },
            )?;
        let active_lane_admission =
            admit_active_subscription_lane(activation.clone(), runtime_active_lifecycle_budget())
                .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.to_string(),
                    stage: "active-lane-admission",
                    message: format!("{error:?}"),
                },
            )?;
        let active_lane_handle =
            open_active_subscription_lane(&mut self.active_subscriptions, active_lane_admission)
                .map_err(
                    |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                        view_name: view_name.to_string(),
                        stage: "active-lane-open",
                        message: format!("{error:?}"),
                    },
                )?;
        let active_lane_counters = self.active_subscriptions.counters().clone();
        let active_lane_digest = active_lane_handle.lane_digest().as_str().to_string();
        let consumer_attachment = attach_subscription_consumer(
            &mut self.active_subscriptions,
            &active_lane_handle,
            SubscriptionConsumerAttachmentRequest::admitted(
                format!("runtime-live-view:{view_name}"),
                activation_digest.clone(),
            ),
            runtime_consumer_attachment_budget(),
        )
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "consumer-attachment",
                message: format!("{error:?}"),
            },
        )?;
        let consumer_attachment_counters = self.active_subscriptions.counters().clone();

        let installation = ForgeQueryRuntimeLiveSubscriptionInstallation::new(
            view_name,
            session.canonical().query().digest().as_str(),
            session.live_view().lowering().digest(),
            subscription_family,
            subscription_declaration_digest,
            bridge_declaration_digest,
            admission_digest,
            activation_digest,
            basis_binding_digest,
            signal_strategy_digest,
            active_lane_digest,
            &consumer_attachment,
            runtime_subscription_budget_policy(),
            RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY,
            RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY,
            active_lane_counters,
            consumer_attachment_counters,
            support_evidence,
            counters,
        );

        Ok(ForgeQueryRuntimeLiveSubscriptionActivation {
            installation,
            active_lane_handle,
            consumer_attachment,
            request: request.clone(),
        })
    }
}

fn admit_authority_requirements(
    requirements: &std::collections::BTreeSet<ForgeQueryAuthorityRequirement>,
) -> Result<(), ForgeQueryRuntimeError> {
    for requirement in requirements {
        match requirement {
            ForgeQueryAuthorityRequirement::ReadOnly
            | ForgeQueryAuthorityRequirement::Live
            | ForgeQueryAuthorityRequirement::BranchLocal
            | ForgeQueryAuthorityRequirement::Previewable
            | ForgeQueryAuthorityRequirement::Writeback
            | ForgeQueryAuthorityRequirement::ReplayRequired => {}
            ForgeQueryAuthorityRequirement::Merge | ForgeQueryAuthorityRequirement::Destructive => {
                return Err(ForgeQueryRuntimeError::UnsupportedAuthority(
                    requirement.as_str().to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn live_subscription_error(
    view_name: &str,
    stage: &'static str,
    error: DeclarativeLiveQueryError,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: view_name.to_string(),
        stage,
        message: format!("{error:?}"),
    }
}

fn subscription_dimensions_for_request(
    request: &DeclarativeLiveQueryRequest,
    view_family: LiveViewShapeFamily,
) -> Result<QuerySubscriptionAdmissionDimensions, ForgeQueryRuntimeError> {
    let projection_width = NonZeroUsize::new(request.projection().len().max(1))
        .expect("projection width is forced non-zero");
    let ordering_width = NonZeroUsize::new(1).expect("ordering width literal is non-zero");
    let metadata_width = NonZeroUsize::new(1).expect("metadata width literal is non-zero");

    match (request.view_shape(), view_family) {
        (DeclarativeLiveViewShape::ListSplice | DeclarativeLiveViewShape::Table, _) => {
            Ok(QuerySubscriptionAdmissionDimensions::collection_membership(
                projection_width,
                ordering_width,
            ))
        }
        (DeclarativeLiveViewShape::Detail, _) => Ok(
            QuerySubscriptionAdmissionDimensions::detail_exact(projection_width),
        ),
        (
            DeclarativeLiveViewShape::InspectorObserved
            | DeclarativeLiveViewShape::InspectorFocused { .. }
            | DeclarativeLiveViewShape::IdentityAwareInspectorFocused { .. },
            _,
        ) => Ok(
            QuerySubscriptionAdmissionDimensions::inspector_detail_exact(
                projection_width,
                metadata_width,
            ),
        ),
        (DeclarativeLiveViewShape::KanbanGrouped { .. }, _) => Ok(
            QuerySubscriptionAdmissionDimensions::grouped_collection_membership(
                projection_width,
                ordering_width,
                NonZeroUsize::new(1).expect("grouping width literal is non-zero"),
                metadata_width,
            ),
        ),
    }
}

fn runtime_family_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(64, 64, 64, 512, 1)
}

fn runtime_slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(64, 64, 64, 64, 64, 64, 64, 64)
}

fn runtime_bridge_lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 64, 64, 64, 64)
}

fn runtime_subscription_admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(64, 64, 64, 64, 64)
}

fn runtime_active_lifecycle_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::LifecycleArena,
    )
}

fn runtime_consumer_attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(1),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn runtime_subscription_budget_policy() -> String {
    [
        RUNTIME_SUBSCRIPTION_FAMILY_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_SLICE_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_BRIDGE_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_ADMISSION_BUDGET_POLICY,
    ]
    .join("|")
}

#[cfg(test)]
fn runtime_subscription_budget_digest() -> String {
    crate::identity::hash_parts(&[
        "runtime_live_subscription_budget_policy_v1".to_string(),
        runtime_subscription_budget_policy(),
        RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY.to_string(),
        RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY.to_string(),
    ])
}

#[cfg(test)]
mod tests;
