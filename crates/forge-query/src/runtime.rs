use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
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
#[cfg(test)]
use crate::subscription::QueryPatchGroupKind;
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
mod computed;
mod delivery;
mod effect;
mod intent;
mod live_subscription;
mod preview;
mod support;

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
    ForgeQueryAuthorityLane, ForgeQueryEffectAction, ForgeQueryEffectAdmission,
    ForgeQueryEffectPolicy, ForgeQueryEffectPolicyDenial, ForgeQueryPreviewOptions,
};
pub use backend::{
    ForgeQueryBridgeBackedRuntimeBackend, ForgeQueryRuntimeBackend, ForgeQueryRuntimeBackendParts,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimeIntentAuthorityAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSourceAdapter,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQueryRuntimeWriteAuthorityAdapter,
};
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
pub use intent::{
    ForgeQueryIntentAuthorityAdapter, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryIntentReceipt, ForgeQueryIntentSourceLane,
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
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportDenial, ForgeQueryRuntimeSupportProfile,
};

#[derive(Debug)]
#[non_exhaustive]
pub enum ForgeQueryRuntimeError {
    MissingBackend,
    MissingRuntimeBridge,
    MissingSchemaAdapter,
    MissingSourceAdapter,
    MissingWriteAuthority,
    MissingSignalSink,
    MissingSubscriptionActivation,
    MissingPreviewBasis,
    MissingInspectorEvidence,
    MissingIntentAuthority,
    Workspace(ForgeQueryWorkspaceError),
    Program(ForgeQueryProgramError),
    UnknownProgram(String),
    UnknownOperation {
        program_id: String,
        operation_id: String,
    },
    MissingLiveView(String),
    MissingLiveSubscription(String),
    MissingDerivedView(String),
    MissingEffect(String),
    ComputedDeclaration {
        view_name: String,
        stage: &'static str,
        message: String,
    },
    EffectDeclaration {
        effect_name: String,
        stage: &'static str,
        message: String,
    },
    LiveSubscriptionInstallation {
        view_name: String,
        stage: &'static str,
        message: String,
    },
    UnsupportedAuthority(String),
    IntentCommitDenied {
        intent_name: String,
        stage: &'static str,
        message: String,
    },
    EffectPolicyDenied(ForgeQueryEffectPolicyDenial),
    PreviewPromotionStaleBasis(ForgeQueryPreviewPromotionDenialEvidence),
    PreviewPromotionAtomicBatchUnsupported(ForgeQueryPreviewPromotionDenialEvidence),
    PreviewPromotionWriteFailed {
        evidence: ForgeQueryPreviewPromotionDenialEvidence,
    },
    UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial),
}

impl std::fmt::Display for ForgeQueryRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingBackend => {
                write!(
                    f,
                    "forge query runtime builder requires a backend, for example in_memory_collections(...)"
                )
            }
            Self::MissingRuntimeBridge => write!(
                f,
                "forge query runtime backend parts require a RuntimeBridge"
            ),
            Self::MissingSchemaAdapter => write!(
                f,
                "forge query runtime backend parts require a schema adapter"
            ),
            Self::MissingSourceAdapter => write!(
                f,
                "forge query runtime backend parts require a source adapter"
            ),
            Self::MissingWriteAuthority => write!(
                f,
                "forge query runtime backend parts require a write authority adapter"
            ),
            Self::MissingSignalSink => write!(
                f,
                "forge query runtime backend parts require a signal sink adapter"
            ),
            Self::MissingSubscriptionActivation => write!(
                f,
                "forge query runtime backend parts require a subscription activation adapter"
            ),
            Self::MissingPreviewBasis => write!(
                f,
                "forge query runtime backend parts require a preview basis adapter"
            ),
            Self::MissingInspectorEvidence => write!(
                f,
                "forge query runtime backend parts require an inspector evidence adapter"
            ),
            Self::MissingIntentAuthority => write!(
                f,
                "forge query runtime backend parts that claim intent support require an intent authority adapter"
            ),
            Self::Workspace(error) => write!(f, "{error}"),
            Self::Program(error) => write!(f, "{error}"),
            Self::UnknownProgram(program) => write!(f, "unknown query program `{program}`"),
            Self::UnknownOperation {
                program_id,
                operation_id,
            } => write!(
                f,
                "unknown query operation `{operation_id}` in program `{program_id}`"
            ),
            Self::MissingLiveView(view) => write!(f, "unknown live view `{view}`"),
            Self::MissingLiveSubscription(view) => {
                write!(
                    f,
                    "live view `{view}` has no retained subscription installation"
                )
            }
            Self::MissingDerivedView(view) => write!(f, "unknown computed view `{view}`"),
            Self::MissingEffect(effect) => write!(f, "unknown effect `{effect}`"),
            Self::ComputedDeclaration {
                view_name,
                stage,
                message,
            } => write!(
                f,
                "computed declaration `{view_name}` failed during {stage}: {message}"
            ),
            Self::EffectDeclaration {
                effect_name,
                stage,
                message,
            } => write!(
                f,
                "effect declaration `{effect_name}` failed during {stage}: {message}"
            ),
            Self::LiveSubscriptionInstallation {
                view_name,
                stage,
                message,
            } => write!(
                f,
                "live view `{view_name}` subscription installation failed during {stage}: {message}"
            ),
            Self::UnsupportedAuthority(authority) => {
                write!(
                    f,
                    "authority requirement `{authority}` is not admitted by this runtime"
                )
            }
            Self::IntentCommitDenied {
                intent_name,
                stage,
                message,
            } => write!(
                f,
                "intent `{intent_name}` commit failed during {stage}: {message}"
            ),
            Self::EffectPolicyDenied(denial) => write!(f, "{denial}"),
            Self::PreviewPromotionStaleBasis(evidence) => write!(
                f,
                "preview promotion `{}` failed during {}: {}",
                evidence.label(),
                evidence.kind().as_str(),
                evidence.reason()
            ),
            Self::PreviewPromotionAtomicBatchUnsupported(evidence) => write!(
                f,
                "preview promotion `{}` failed during {}: {}",
                evidence.label(),
                evidence.kind().as_str(),
                evidence.reason()
            ),
            Self::PreviewPromotionWriteFailed { evidence } => write!(
                f,
                "preview promotion `{}` failed during {}: {}",
                evidence.label(),
                evidence.kind().as_str(),
                evidence.reason()
            ),
            Self::UnsupportedFacadeFamily(denial) => write!(f, "{denial}"),
        }
    }
}

impl std::error::Error for ForgeQueryRuntimeError {}

impl From<ForgeQueryWorkspaceError> for ForgeQueryRuntimeError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Workspace(value)
    }
}

impl From<ForgeQueryProgramError> for ForgeQueryRuntimeError {
    fn from(value: ForgeQueryProgramError) -> Self {
        Self::Program(value)
    }
}

#[derive(Default)]
pub struct ForgeQueryRuntimeBuilder {
    backend: Option<Result<Box<dyn ForgeQueryRuntimeBackend>, ForgeQueryRuntimeError>>,
    backend_parts: ForgeQueryRuntimeBackendParts,
}

impl ForgeQueryRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_memory_collections(
        mut self,
        collections: impl IntoIterator<Item = ForgeQueryCollection>,
    ) -> Self {
        self.backend = Some(
            ForgeQueryMemoryApp::new(collections)
                .map(|backend| Box::new(backend) as Box<dyn ForgeQueryRuntimeBackend>)
                .map_err(ForgeQueryRuntimeError::Workspace),
        );
        self
    }

    pub fn backend(mut self, backend: impl ForgeQueryRuntimeBackend + 'static) -> Self {
        self.backend = Some(Ok(Box::new(backend)));
        self
    }

    pub fn relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.backend_parts = self.backend_parts.relational_runtime(runtime);
        self
    }

    pub fn runtime_bridge(mut self, bridge: RuntimeBridge) -> Self {
        self.backend_parts = self.backend_parts.runtime_bridge(bridge);
        self
    }

    pub fn schema_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSchemaAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.schema_adapter(adapter);
        self
    }

    pub fn source_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSourceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.source_adapter(adapter);
        self
    }

    pub fn write_authority(
        mut self,
        authority: impl ForgeQueryRuntimeWriteAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.write_authority(authority);
        self
    }

    pub fn signal_sink(mut self, sink: impl ForgeQueryRuntimeSignalSinkAdapter + 'static) -> Self {
        self.backend_parts = self.backend_parts.signal_sink(sink);
        self
    }

    pub fn subscription_activation(
        mut self,
        adapter: impl ForgeQueryRuntimeSubscriptionActivationAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.subscription_activation(adapter);
        self
    }

    pub fn preview_basis(
        mut self,
        adapter: impl ForgeQueryRuntimePreviewBasisAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.preview_basis(adapter);
        self
    }

    pub fn inspector_evidence(
        mut self,
        adapter: impl ForgeQueryRuntimeInspectorEvidenceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.inspector_evidence(adapter);
        self
    }

    pub fn intent_authority(
        mut self,
        adapter: impl ForgeQueryIntentAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.intent_authority(adapter);
        self
    }

    pub fn support_profile(mut self, profile: ForgeQueryRuntimeSupportProfile) -> Self {
        self.backend_parts = self.backend_parts.support_profile(profile);
        self
    }

    pub fn build_backend_from_parts(mut self) -> Self {
        self.backend = Some(
            ForgeQueryBridgeBackedRuntimeBackend::from_parts(self.backend_parts)
                .map(|backend| Box::new(backend) as Box<dyn ForgeQueryRuntimeBackend>),
        );
        self.backend_parts = ForgeQueryRuntimeBackendParts::new();
        self
    }

    pub fn build(self) -> Result<ForgeQueryRuntime, ForgeQueryRuntimeError> {
        let backend = self
            .backend
            .ok_or(ForgeQueryRuntimeError::MissingBackend)??;
        Ok(ForgeQueryRuntime {
            backend,
            evidence_authority: ForgeQueryRuntimeEvidenceAuthority::new(),
            active_subscriptions: ActiveSubscriptionRuntime::new(),
            live_subscriptions: BTreeMap::new(),
            live_subscription_index: BTreeMap::new(),
            installed_programs: BTreeMap::new(),
            run_traces: BTreeMap::new(),
            derived_views: BTreeMap::new(),
            derived_dependency_index: ForgeQueryComputedDependencyIndex::default(),
            effects: BTreeMap::new(),
            effect_index: ForgeQueryEffectIndex::default(),
            next_run_id: 0,
        })
    }
}

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
        if declaration.target_lane() != ForgeQueryAuthorityLane::AuthoritativeTruth {
            return Err(ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name: declaration.name().to_string(),
                stage: "authority-admission",
                message: format!(
                    "Batch 7A admits only authoritative truth targets, got `{}`",
                    declaration.target_lane()
                ),
            });
        }
        let execution = self
            .backend
            .execute_intent(&declaration)
            .map_err(ForgeQueryRuntimeError::Workspace)?;
        if execution.canonical_input_digest() != declaration.input_digest() {
            return Err(ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name: declaration.name().to_string(),
                stage: "input-digest-admission",
                message: "intent authority returned a canonical input digest that does not match the declaration".to_string(),
            });
        }
        let write_receipt =
            self.route_authoritative_mutation_receipt(execution.mutation_receipt().clone())?;
        Ok(ForgeQueryIntentReceipt::new(
            &declaration,
            execution,
            &write_receipt,
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

    pub fn preview<'a>(&'a mut self, label: impl Into<String>) -> ForgeQueryPreviewSession<'a> {
        self.preview_with_options(label, ForgeQueryPreviewOptions::default())
    }

    pub fn preview_with_options<'a>(
        &'a mut self,
        label: impl Into<String>,
        options: ForgeQueryPreviewOptions,
    ) -> ForgeQueryPreviewSession<'a> {
        self.try_preview_with_options(label, options)
            .expect("branch/preview support must be admitted before creating preview sessions")
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

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    Insert {
        collection: String,
        payload: Value,
    },
    UpdateAspect {
        entity_identity: String,
        aspect_path: String,
        value: Value,
    },
    Delete {
        entity_identity: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    inner: ForgeQueryMutationReceipt,
    authority_lane: ForgeQueryAuthorityLane,
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

impl ForgeQueryWriteReceipt {
    fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
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
    ) -> Self {
        Self {
            inner,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        }
    }

    fn preview(
        label: &str,
        sequence: usize,
        command: &ForgeQueryWriteCommand,
        snapshot_token: String,
    ) -> Self {
        let delta = match command {
            ForgeQueryWriteCommand::Insert {
                collection,
                payload: _,
            } => crate::memory_workspace::ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: format!("preview:{label}:{sequence}"),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: Vec::new(),
            },
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value: _,
            } => crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec![aspect_path.clone()],
            },
            ForgeQueryWriteCommand::Delete { entity_identity } => {
                crate::memory_workspace::ForgeQueryMutationDelta {
                    collection: "preview".to_string(),
                    entity_identity: entity_identity.clone(),
                    kind: ForgeQueryMutationKind::Deleted,
                    aspect_paths: Vec::new(),
                }
            }
        };
        Self {
            inner: ForgeQueryMutationReceipt {
                commit_identity: format!("preview:{label}:{sequence}"),
                snapshot_token,
                deltas: vec![delta],
            },
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            affected_live_view_ids: Vec::new(),
            affected_derived_view_ids: Vec::new(),
            considered_computed_view_count: 0,
            considered_effect_count: 0,
            delivered_effect_count: 0,
            pending_write_intent_count: 0,
            suppressed_effect_count: 0,
            meaningful_effect_suppression_count: 0,
            effect_expression_failure_count: 0,
            refresh_fallback: false,
        }
    }

    pub fn commit_identity(&self) -> &str {
        &self.inner.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.inner.snapshot_token
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn deltas(&self) -> &[crate::memory_workspace::ForgeQueryMutationDelta] {
        &self.inner.deltas
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub fn into_inner(self) -> ForgeQueryMutationReceipt {
        self.inner
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryPatchBatch {
    pub view_name: String,
    pub live_patches: Vec<ForgeQueryLivePatch>,
    pub query_delivery_batches: Vec<ForgeQueryRuntimeDeliveryBatch>,
    pub derived_patch_notes: Vec<String>,
    pub derived_patches: Vec<ForgeQueryDerivedPatch>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveView<T = Value> {
    handle: ForgeQueryLiveViewHandle,
    authority_lane: ForgeQueryAuthorityLane,
    subscription_installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    marker: PhantomData<T>,
}

impl<T> ForgeQueryLiveView<T> {
    fn new(
        handle: ForgeQueryLiveViewHandle,
        subscription_installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Self {
        Self {
            handle,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            subscription_installation,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.handle.name()
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn subscription_installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        &self.subscription_installation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledProgram {
    program_id: String,
}

impl ForgeQueryInstalledProgram {
    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn operation(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<ForgeQueryInstalledOperation, ForgeQueryRuntimeError> {
        Ok(ForgeQueryInstalledOperation {
            program_id: self.program_id.clone(),
            operation_id: operation_id.into(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledOperation {
    program_id: String,
    operation_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRunReceipt {
    run_id: String,
    operation: ForgeQueryInstalledOperation,
    outputs: Vec<ForgeQueryOperationOutput>,
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    patch_batches: Vec<ForgeQueryPatchBatch>,
}

impl ForgeQueryRunReceipt {
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn outputs(&self) -> &[ForgeQueryOperationOutput] {
        &self.outputs
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn patch_batches(&self) -> &[ForgeQueryPatchBatch] {
        &self.patch_batches
    }
}

pub struct ForgeQueryArtifactInspector<'a> {
    receipt: &'a ForgeQueryWriteReceipt,
    runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
}

impl<'a> ForgeQueryArtifactInspector<'a> {
    pub fn canonical(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "canonical",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn workflow(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "workflow",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn bridge_authority(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.receipt.authority_lane()
    }

    pub fn runtime_evidence(&self) -> &ForgeQueryRuntimeInspectionEvidence {
        &self.runtime_evidence
    }

    pub fn live_patch_artifacts(&self) -> Vec<String> {
        self.receipt
            .deltas()
            .iter()
            .map(|delta| format!("{}:{}", delta.collection, delta.entity_identity))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInspectedArtifact {
    family: String,
    identity: String,
    basis: String,
}

impl ForgeQueryInspectedArtifact {
    fn new(
        family: impl Into<String>,
        identity: impl Into<String>,
        basis: impl Into<String>,
    ) -> Self {
        Self {
            family: family.into(),
            identity: identity.into(),
            basis: basis.into(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn basis(&self) -> &str {
        &self.basis
    }
}

#[cfg(test)]
mod tests;
