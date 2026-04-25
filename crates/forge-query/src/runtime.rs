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
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeWriteAuthorityAdapter,
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
    ForgeQueryEffectExpressionFailurePosture, ForgeQueryEffectHandle,
    ForgeQueryEffectInspectionEvidence, ForgeQueryEffectSuppressionPolicy, ForgeQueryEffectTrigger,
    ForgeQueryEffectTriggerSourceKind,
};
pub use live_subscription::ForgeQueryRuntimeLiveSubscriptionInstallation;
pub use preview::{ForgeQueryPreviewDiff, ForgeQueryPreviewOutcome, ForgeQueryPreviewSession};
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
    EffectPolicyDenied(ForgeQueryEffectPolicyDenial),
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
            Self::EffectPolicyDenied(denial) => write!(f, "{denial}"),
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
        insert_effect_runtime(&mut self.effects, &mut self.effect_index, declaration);
        Ok(ForgeQueryEffectHandle::new(name))
    }

    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        let receipt = self.backend.write(command)?;
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
        self.effects
            .get(effect.name())
            .map(ForgeQueryEffectInspectionEvidence::from_runtime)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))
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
mod tests {
    use serde_json::json;

    use super::*;
    use crate::declarative_live::{DeclarativeLiveViewShape, DeclarativeProjectionField};
    use crate::program::{
        ForgeQueryOperation, ForgeQueryPortType, ForgeQueryProgramSource, ForgeQuerySchemaAdapter,
        ForgeQueryTypedPort, ForgeQueryValueExpr, ForgeQueryWriteCommandTemplate,
    };
    use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
    use forge_runtime_bridge::facade::{
        BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId,
        BridgeMappingRegistration, CoarseRoutingMode, InvalidationSink, MappingSelector,
        RawCommittedPatchEnvelope, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
        RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
        SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
        TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
        TruthSnapshotReader,
    };

    #[test]
    fn runtime_builder_rejects_missing_backend_inputs() {
        let error = match ForgeQueryRuntime::builder().build() {
            Ok(_) => panic!("builder should reject missing v1 backend"),
            Err(error) => error,
        };

        assert!(matches!(error, ForgeQueryRuntimeError::MissingBackend));
    }

    #[test]
    fn runtime_builder_rejects_incomplete_backend_parts() {
        let error = ForgeQueryRuntime::builder()
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing bridge should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingRuntimeBridge
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing schema adapter should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingSchemaAdapter
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing source adapter should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingSourceAdapter
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing write authority should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingWriteAuthority
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing signal sink should reject"),
            Err(error) => error,
        };
        assert!(matches!(error, ForgeQueryRuntimeError::MissingSignalSink));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing subscription activation should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingSubscriptionActivation
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing preview basis should reject"),
            Err(error) => error,
        };
        assert!(matches!(error, ForgeQueryRuntimeError::MissingPreviewBasis));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing inspector evidence should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingInspectorEvidence
        ));
    }

    #[test]
    fn runtime_builder_accepts_bridge_backed_backend_parts() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("complete backend parts should build");
        let view: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("external.tasks", task_live_request(), task_schema())
            .expect("external backend should declare live view");
        let receipt = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "external-1" },
                    "title": { "value": "External task" },
                }),
            })
            .expect("external write authority should execute");

        assert_eq!(view.name(), "external.tasks");
        assert_eq!(
            view.subscription_installation().subscription_family(),
            "collection_membership"
        );
        assert_eq!(
            view.subscription_installation().authority_lane(),
            ForgeQueryAuthorityLane::AuthoritativeTruth
        );
        assert_eq!(
            view.subscription_installation()
                .counters()
                .activation_input_count(),
            1
        );
        assert!(view
            .subscription_installation()
            .support_evidence()
            .starts_with("test-subscription-activation:external.tasks:"));
        assert!(!view
            .subscription_installation()
            .active_lane_digest()
            .is_empty());
        assert!(!view
            .subscription_installation()
            .consumer_attachment_digest()
            .is_empty());
        assert!(!view
            .subscription_installation()
            .consumer_digest()
            .is_empty());
        assert!(!view
            .subscription_installation()
            .delivery_cursor_digest()
            .is_empty());
        assert_eq!(
            view.subscription_installation()
                .active_lane_counters()
                .active_lane_creation_count(),
            1
        );
        assert_eq!(
            view.subscription_installation()
                .consumer_attachment_counters()
                .consumer_attachment_count(),
            1
        );
        assert_eq!(
            view.subscription_installation()
                .subscription_budget_policy(),
            runtime_subscription_budget_policy()
        );
        assert_eq!(
            view.subscription_installation()
                .active_lifecycle_budget_policy(),
            RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY
        );
        assert_eq!(
            view.subscription_installation()
                .consumer_attachment_budget_policy(),
            RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY
        );
        assert_eq!(
            view.subscription_installation().runtime_budget_digest(),
            runtime_subscription_budget_digest()
        );
        let live_inspection = runtime
            .inspect_live_view(&view)
            .expect("inspector should retain live subscription installation");
        assert_eq!(
            live_inspection.installation_digest(),
            view.subscription_installation().installation_digest()
        );
        assert_eq!(receipt.commit_identity(), "external-commit-1");
        assert_eq!(
            receipt.affected_live_view_ids(),
            &["external.tasks".to_string()]
        );
        {
            let inspector = runtime
                .try_inspect_receipt(&receipt)
                .expect("inspector evidence adapter should inspect receipt");
            assert_eq!(
                inspector.runtime_evidence().artifact_family(),
                "test-write-receipt"
            );
            assert_eq!(
                inspector.runtime_evidence().evidence(),
                &["test-inspector-evidence".to_string()]
            );
        }
        {
            let preview = runtime
                .try_preview("external preview")
                .expect("preview basis adapter should admit preview basis");
            assert_eq!(preview.basis_admission().label(), "external preview");
            assert_eq!(
                preview.basis_admission().evidence(),
                &["test-preview-basis".to_string()]
            );
        }
    }

    #[test]
    fn runtime_live_declaration_denies_backend_admission_before_subscription_install() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(DenyingSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("backend with denying schema admission should still build");

        let error = runtime
            .declare_live_view::<Value>(
                "external.schema-denied",
                task_live_request(),
                task_schema(),
            )
            .expect_err("backend admission denial must block subscription installation");

        match error {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name,
                stage,
                message,
            } => {
                assert_eq!(view_name, "external.schema-denied");
                assert_eq!(stage, "backend-live-admission");
                assert!(message.contains("schema admission denied by test adapter"));
            }
            other => panic!("expected backend admission denial, got {other:?}"),
        }
    }

    #[test]
    fn runtime_live_declaration_closes_active_subscription_when_source_declaration_fails() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::fail_declare())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("backend with failing source declaration should still build");

        let error = runtime
            .declare_live_view::<Value>(
                "external.source-denied",
                task_live_request(),
                task_schema(),
            )
            .expect_err("source declaration denial must close active subscription");

        match error {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name,
                stage,
                message,
            } => {
                assert_eq!(view_name, "external.source-denied");
                assert_eq!(stage, "source-declaration");
                assert!(message.contains("source declaration denied by test adapter"));
                assert!(message.contains("active subscription closeout:"));
                assert!(message.contains("terminal:true"));
            }
            other => panic!("expected source declaration denial, got {other:?}"),
        }
        assert_eq!(runtime.active_subscriptions.lane_count(), 0);
        assert!(runtime.live_subscriptions.is_empty());
    }

    #[test]
    fn runtime_equivalent_live_declarations_share_active_lane_with_distinct_consumers() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("complete backend parts should build");

        let first: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("external.tasks.first", task_live_request(), task_schema())
            .expect("first live view should install active lane");
        let second: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("external.tasks.second", task_live_request(), task_schema())
            .expect("equivalent live view should join active lane");

        assert_eq!(
            first.subscription_installation().active_lane_digest(),
            second.subscription_installation().active_lane_digest()
        );
        assert_ne!(
            first
                .subscription_installation()
                .consumer_attachment_digest(),
            second
                .subscription_installation()
                .consumer_attachment_digest()
        );
        assert_eq!(
            second
                .subscription_installation()
                .active_lane_counters()
                .active_lane_join_count(),
            1
        );
        assert_eq!(
            second
                .subscription_installation()
                .active_lane_counters()
                .shared_lane_count(),
            1
        );
        assert_eq!(
            second
                .subscription_installation()
                .consumer_attachment_counters()
                .consumer_attachment_count(),
            1
        );
        assert_eq!(
            second
                .subscription_installation()
                .consumer_attachment_counters()
                .affected_consumer_attachment_width(),
            2
        );
    }

    #[test]
    fn runtime_live_declaration_denies_before_source_when_subscription_activation_rejects() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(DenyingSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("backend with denying activation should still build");

        let error = runtime
            .declare_live_view::<Value>("external.denied", task_live_request(), task_schema())
            .expect_err("activation denial must block source declaration");

        match error {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name,
                stage,
                message,
            } => {
                assert_eq!(view_name, "external.denied");
                assert_eq!(stage, "activation-admission");
                assert!(message.contains("activation denied by test adapter"));
            }
            other => panic!("expected live subscription installation denial, got {other:?}"),
        }
    }

    #[test]
    fn runtime_support_profiles_expose_facade_family_posture() {
        let memory_runtime = task_runtime();
        let bridge_runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("complete backend parts should build");

        for family in [
            ForgeQueryRuntimeFacadeFamily::Read,
            ForgeQueryRuntimeFacadeFamily::Live,
            ForgeQueryRuntimeFacadeFamily::Computed,
            ForgeQueryRuntimeFacadeFamily::Effect,
            ForgeQueryRuntimeFacadeFamily::BranchPreview,
            ForgeQueryRuntimeFacadeFamily::Write,
            ForgeQueryRuntimeFacadeFamily::Inspect,
        ] {
            assert_eq!(
                memory_runtime
                    .support_profile()
                    .support_for(family)
                    .expect("memory support row should exist")
                    .status(),
                ForgeQueryRuntimeFamilySupportStatus::Supported
            );
            assert_eq!(
                bridge_runtime
                    .support_profile()
                    .support_for(family)
                    .expect("bridge-backed support row should exist")
                    .status(),
                ForgeQueryRuntimeFamilySupportStatus::Supported
            );
        }

        assert_eq!(
            bridge_runtime
                .support_profile()
                .support_for(ForgeQueryRuntimeFacadeFamily::Intent)
                .expect("intent support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Unsupported
        );
        assert!(bridge_runtime
            .support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::Live)
            .expect("live support row should exist")
            .evidence()
            .iter()
            .any(|evidence| evidence == "test-subscription-activation"));
    }

    #[test]
    fn runtime_support_denies_unsupported_write_family_before_execution() {
        let mut runtime = bridge_runtime_with_support(
            ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
                ForgeQueryRuntimeFamilySupport::unsupported(
                    ForgeQueryRuntimeFacadeFamily::Write,
                    "test backend disabled write authority",
                ),
            ),
        );

        let error = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "external-1" },
                    "title": { "value": "Should not write" },
                }),
            })
            .expect_err("unsupported write family should deny before write authority");

        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Write);
                assert_eq!(denial.reason(), "test backend disabled write authority");
            }
            other => panic!("expected unsupported facade family denial, got {other:?}"),
        }
    }

    #[test]
    fn runtime_builder_rejects_support_profiles_that_overclaim_unimplemented_families() {
        let profile = ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Intent,
                [ForgeQueryAuthorityLane::PendingWriteIntent],
                [ForgeQueryEffectPolicy::AuthoritativeAllowed],
                ["fake-intent-adapter"],
            ),
        );

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .support_profile(profile)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("support profile must not claim unimplemented facade support"),
            Err(error) => error,
        };

        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Intent);
                assert!(denial.reason().contains("no executable facade path"));
            }
            other => panic!("expected unsupported facade family denial, got {other:?}"),
        }
    }

    #[test]
    fn runtime_support_denies_unsupported_computed_family_before_registration() {
        let mut runtime = bridge_runtime_with_support(
            ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
                ForgeQueryRuntimeFamilySupport::unsupported(
                    ForgeQueryRuntimeFacadeFamily::Computed,
                    "test backend disabled computed resources",
                ),
            ),
        );

        let error = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("task_titles.unsupported", ["title".to_string()]),
                TitleListMaintainer,
            )
            .expect_err("unsupported computed family should deny before registration");

        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Computed);
                assert_eq!(denial.reason(), "test backend disabled computed resources");
            }
            other => panic!("expected unsupported facade family denial, got {other:?}"),
        }
    }

    #[test]
    fn runtime_declares_live_view_and_routes_minimal_write_patches() {
        let mut runtime = task_runtime();
        let view: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");

        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Buy milk" },
                }),
            })
            .expect("insert should execute through runtime facade");
        let task_id = insert.deltas()[0].entity_identity.clone();
        let insert_patches = runtime.drain_patches(&view);

        assert_eq!(insert.deltas().len(), 1);
        assert!(insert.deltas()[0].aspect_paths.is_empty());
        assert_eq!(
            insert.affected_live_view_ids(),
            &["tasks.table".to_string()]
        );
        assert!(insert_patches.live_patches.is_empty());
        assert_eq!(insert_patches.query_delivery_batches.len(), 1);
        assert_eq!(
            insert_patches.query_delivery_batches[0].patch_group_kind(),
            QueryPatchGroupKind::CollectionMembershipPatchGroup
        );
        assert_eq!(insert_patches.query_delivery_batches[0].sequence(), 1);

        let update = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: task_id,
                aspect_path: "title.value".to_string(),
                value: Value::String("Buy oat milk".to_string()),
            })
            .expect("update should execute through runtime facade");
        let update_patches = runtime.drain_patches(&view);

        assert_eq!(update.deltas()[0].aspect_paths, vec!["title.value"]);
        assert!(update_patches.live_patches.is_empty());
        assert_eq!(update_patches.query_delivery_batches.len(), 1);
        assert_eq!(
            update_patches.query_delivery_batches[0].patch_group_kind(),
            QueryPatchGroupKind::DetailFieldPatchGroup
        );
        assert_eq!(update_patches.query_delivery_batches[0].sequence(), 2);

        let irrelevant = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: update.deltas()[0].entity_identity.clone(),
                aspect_path: "description.value".to_string(),
                value: Value::String("ignored by task table".to_string()),
            })
            .expect("irrelevant update should execute");
        let irrelevant_patches = runtime.drain_patches(&view);
        assert!(irrelevant.affected_live_view_ids().is_empty());
        assert!(irrelevant_patches.query_delivery_batches.is_empty());
    }

    #[test]
    fn runtime_grouped_live_view_uses_backend_baseline_and_delivers_grouped_membership_patch() {
        let mut runtime = grouped_task_runtime();
        let table: ForgeQueryLiveView<Value> = runtime
            .declare_live_view(
                "tasks.seed-table",
                grouped_task_table_live_request(),
                grouped_task_schema(),
            )
            .expect("table live view should declare before seed write");
        let seed = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Seed task" },
                    "status": { "value": "todo" },
                }),
            })
            .expect("seed insert should write through table declaration");
        let task_id = seed.deltas()[0].entity_identity.clone();
        let _ = runtime.drain_patches(&table);
        let grouped: ForgeQueryLiveView<Value> = runtime
            .declare_live_view(
                "tasks.grouped",
                grouped_task_live_request(),
                grouped_task_schema(),
            )
            .expect("grouped live view should declare with backend-owned baseline");

        let receipt = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: task_id,
                aspect_path: "status.value".to_string(),
                value: Value::String("done".to_string()),
            })
            .expect("grouping aspect update should write");
        let patches = runtime.drain_patches(&grouped);

        assert!(receipt
            .affected_live_view_ids()
            .contains(&"tasks.grouped".to_string()));
        assert_eq!(patches.query_delivery_batches.len(), 1);
        assert_eq!(
            patches.query_delivery_batches[0].patch_group_kind(),
            QueryPatchGroupKind::GroupedMembershipPatchGroup
        );
        assert_eq!(
            grouped.subscription_installation().subscription_family(),
            "grouped_collection_membership"
        );
    }

    #[test]
    fn redeclared_live_view_replaces_runtime_delivery_index_membership() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .build_backend_from_parts()
            .build()
            .expect("bridge-backed runtime should build");
        let task_view: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("shared.surface", task_live_request(), task_schema())
            .expect("task live view should declare");
        let task_seed = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Task seed" },
                }),
            })
            .expect("task seed should write");
        let _ = runtime.drain_patches(&task_view);

        let issue_view: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("shared.surface", issue_live_request(), issue_schema())
            .expect("same live view name should redeclare against issue collection");
        let stale_task_update = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: task_seed.deltas()[0].entity_identity.clone(),
                aspect_path: "title.value".to_string(),
                value: Value::String("Task update after redeclare".to_string()),
            })
            .expect("task update should still write");
        let stale_task_patches = runtime.drain_patches(&issue_view);

        assert!(stale_task_update.affected_live_view_ids().is_empty());
        assert!(stale_task_patches.query_delivery_batches.is_empty());

        let issue_write = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Issue".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "summary": { "value": "Issue seed" },
                }),
            })
            .expect("issue insert should write");
        let issue_patches = runtime.drain_patches(&issue_view);

        assert_eq!(
            issue_write.affected_live_view_ids(),
            &["shared.surface".to_string()]
        );
        assert_eq!(issue_patches.query_delivery_batches.len(), 1);
    }

    #[test]
    fn compiled_typed_program_installs_runs_and_emits_trace() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let run = runtime
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Typed task".to_string()),
                )],
            )
            .expect("program should run");
        let trace = runtime.inspect_run(&run).expect("trace should be retained");

        assert_eq!(trace.operation_id(), "create_task");
        assert_eq!(run.outputs()[0].name(), "live:tasks.table");
        assert_eq!(run.outputs()[0].value()[0]["title"]["value"], "Typed task");
        assert!(trace
            .generated_declarations()
            .iter()
            .any(|declaration| declaration == "live:tasks.table"));
        assert_eq!(trace.write_receipts().len(), 1);
        assert_eq!(trace.patch_artifacts().len(), 1);
        assert!(trace
            .patch_artifacts()
            .iter()
            .any(|artifact| artifact.starts_with("query-delivery:tasks.table:")));
    }

    #[test]
    fn compiled_typed_program_rejects_type_mismatch_before_execution() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let error = runtime
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new("title", Value::Bool(true))],
            )
            .expect_err("type mismatch should reject before effects execute");

        assert!(matches!(error, ForgeQueryRuntimeError::Program(_)));
    }

    #[test]
    fn runtime_surfaces_authority_lanes_on_public_handles_and_receipts() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.authority", task_live_request(), task_schema())
            .expect("live view should declare");
        let derived = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("task_titles.authority", ["title".to_string()]),
                TitleListMaintainer,
            )
            .expect("derived view should declare");

        let receipt = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Authority lane task" },
                }),
            })
            .expect("insert should write");
        let patches = runtime.drain_derived_patches(derived.name());
        let inspector = runtime.inspect_receipt(&receipt);

        assert_eq!(
            live.authority_lane(),
            ForgeQueryAuthorityLane::AuthoritativeTruth
        );
        assert_eq!(
            derived.authority_lane(),
            ForgeQueryAuthorityLane::DerivedRuntimeState
        );
        assert_eq!(
            receipt.authority_lane(),
            ForgeQueryAuthorityLane::AuthoritativeTruth
        );
        assert_eq!(
            inspector.authority_lane(),
            ForgeQueryAuthorityLane::AuthoritativeTruth
        );
        assert_eq!(
            patches.derived_patches[0].authority_lane(),
            ForgeQueryAuthorityLane::DerivedRuntimeState
        );
    }

    #[test]
    fn preview_defaults_to_derive_only_effect_policy_but_keeps_explicit_writes_preview_local() {
        let mut runtime = task_runtime();
        let mut preview = runtime.preview("default policy");

        assert_eq!(preview.effect_policy(), ForgeQueryEffectPolicy::DeriveOnly);
        assert!(preview
            .admit_effect_action(
                ForgeQueryEffectAction::Derive,
                ForgeQueryAuthorityLane::DerivedRuntimeState
            )
            .is_ok());

        let delivery_denial = preview
            .admit_effect_action(
                ForgeQueryEffectAction::Deliver,
                ForgeQueryAuthorityLane::EffectDeliveryState,
            )
            .expect_err("derive-only preview should deny effect delivery");
        assert!(matches!(
            delivery_denial,
            ForgeQueryRuntimeError::EffectPolicyDenied(_)
        ));

        let write_denial = preview
            .admit_effect_action(
                ForgeQueryEffectAction::WriteIntent,
                ForgeQueryAuthorityLane::AuthoritativeTruth,
            )
            .expect_err("derive-only preview should deny authoritative write intent");
        assert!(matches!(
            write_denial,
            ForgeQueryRuntimeError::EffectPolicyDenied(_)
        ));

        let preview_receipt = preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Preview-local task" },
                }),
            })
            .expect("explicit preview write should stage");
        assert_eq!(
            preview_receipt.authority_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );

        let outcome = preview.discard();
        assert_eq!(outcome.effect_policy(), ForgeQueryEffectPolicy::DeriveOnly);
        assert_eq!(outcome.source_lane(), ForgeQueryAuthorityLane::PreviewTruth);
        assert_eq!(outcome.target_lane(), ForgeQueryAuthorityLane::PreviewTruth);
    }

    #[test]
    fn sandboxed_preview_policy_admits_only_sandboxed_write_intents() {
        let mut runtime = task_runtime();
        let preview = runtime.preview_with_options(
            "sandboxed writes",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );

        let admission = preview
            .admit_effect_action(
                ForgeQueryEffectAction::WriteIntent,
                ForgeQueryAuthorityLane::PreviewTruth,
            )
            .expect("sandboxed write intent should be admitted to preview truth");
        assert_eq!(
            admission.policy(),
            ForgeQueryEffectPolicy::SandboxedWriteIntent
        );
        assert_eq!(admission.action(), ForgeQueryEffectAction::WriteIntent);
        assert_eq!(
            admission.target_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );

        let denial = preview
            .admit_effect_action(
                ForgeQueryEffectAction::WriteIntent,
                ForgeQueryAuthorityLane::AuthoritativeTruth,
            )
            .expect_err("sandboxed write intent must not target authoritative truth");
        assert!(matches!(
            denial,
            ForgeQueryRuntimeError::EffectPolicyDenied(_)
        ));
    }

    #[test]
    fn derive_only_preview_denies_operation_write_effects() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let mut preview = runtime.preview("derive-only operation");
        let error = preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Should not stage".to_string()),
                )],
            )
            .expect_err("derive-only preview should deny write-effect operations");

        assert!(matches!(
            error,
            ForgeQueryRuntimeError::EffectPolicyDenied(_)
        ));
        assert_eq!(preview.compare_to_authoritative().write_count(), 0);
    }

    #[test]
    fn sandboxed_preview_run_operation_stages_compiled_writes_until_promote() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let preview_run = {
            let mut preview = runtime.preview_with_options(
                "draft create",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
            );
            let run = preview
                .run_operation(
                    operation.clone(),
                    vec![ForgeQueryOperationInput::new(
                        "title",
                        Value::String("Preview-only task".to_string()),
                    )],
                )
                .expect("preview operation should run");

            assert_eq!(run.write_receipts().len(), 1);
            assert!(run.write_receipts()[0]
                .commit_identity()
                .starts_with("preview:draft create"));
            assert_eq!(
                run.write_receipts()[0].authority_lane(),
                ForgeQueryAuthorityLane::PreviewTruth
            );
            run
        };

        assert_eq!(
            preview_run.outputs()[0].value().as_array().unwrap().len(),
            0
        );

        {
            let mut preview = runtime.preview_with_options(
                "promote create",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
            );
            preview
                .run_operation(
                    operation,
                    vec![ForgeQueryOperationInput::new(
                        "title",
                        Value::String("Promoted preview task".to_string()),
                    )],
                )
                .expect("preview operation should stage");
            let outcome = preview.promote();
            assert!(outcome.promoted());
            assert_eq!(outcome.write_count(), 1);
            assert_eq!(
                outcome.effect_policy(),
                ForgeQueryEffectPolicy::SandboxedWriteIntent
            );
            assert_eq!(outcome.source_lane(), ForgeQueryAuthorityLane::PreviewTruth);
            assert_eq!(
                outcome.target_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
        }

        let view = runtime
            .declare_live_view::<Value>("tasks.after-preview", task_live_request(), task_schema())
            .expect("live view should declare");
        let rows = runtime.read_live(&view);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].payload["title"]["value"], "Promoted preview task");
    }

    #[test]
    fn preview_run_operation_discard_keeps_authoritative_state_unchanged() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        {
            let mut preview = runtime.preview_with_options(
                "discard create",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
            );
            preview
                .run_operation(
                    operation,
                    vec![ForgeQueryOperationInput::new(
                        "title",
                        Value::String("Discarded preview task".to_string()),
                    )],
                )
                .expect("preview operation should stage");
            let outcome = preview.discard();
            assert!(outcome.discarded());
        }

        let view = runtime
            .declare_live_view::<Value>("tasks.after-discard", task_live_request(), task_schema())
            .expect("live view should declare");
        assert!(runtime.read_live(&view).is_empty());
    }

    #[test]
    fn derived_view_receives_narrow_or_fallback_patch_notes() {
        let mut runtime = task_runtime();
        let _: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");
        runtime
            .declare_derived_view(
                ForgeQueryDerivedView::new("task_titles", ["title".to_string()])
                    .whole_refresh_fallback(),
            )
            .expect("derived view should declare");
        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Derived task" },
                }),
            })
            .expect("insert should route to derived view");
        let update = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: insert.deltas()[0].entity_identity.clone(),
                aspect_path: "title.value".to_string(),
                value: Value::String("Derived task renamed".to_string()),
            })
            .expect("title update should route to derived view");

        let patches = runtime.drain_derived_patches("task_titles");

        assert_eq!(
            update.affected_derived_view_ids(),
            &["task_titles".to_string()]
        );
        assert!(update.refresh_fallback());
        assert!(patches
            .derived_patch_notes
            .iter()
            .any(|note| note.starts_with("whole-refresh-fallback")));
    }

    #[test]
    fn maintained_derived_view_materializes_incremental_patches() {
        let mut runtime = task_runtime();
        let _: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");
        let titles = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("task_titles", ["title".to_string()]),
                TitleListMaintainer,
            )
            .expect("maintained derived view should declare");

        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "First title" },
                }),
            })
            .expect("insert should route derived patch");
        let patches = runtime.drain_derived_patches(titles.name());

        assert_eq!(
            insert.affected_derived_view_ids(),
            &["task_titles".to_string()]
        );
        let expected_row = Value::String(insert.deltas()[0].entity_identity.clone());
        assert_eq!(runtime.read_derived(&titles), vec![expected_row.clone()]);
        assert_eq!(patches.derived_patches.len(), 1);
        assert_eq!(patches.derived_patches[0].payload(), &expected_row);

        runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: insert.deltas()[0].entity_identity.clone(),
                aspect_path: "identity.id".to_string(),
                value: Value::String("ignored".to_string()),
            })
            .expect("irrelevant update should not route derived patch");
        let irrelevant = runtime.drain_derived_patches(titles.name());

        assert!(irrelevant.derived_patches.is_empty());
    }

    #[test]
    fn nested_computed_views_route_in_deterministic_dependency_order() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");
        let titles = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.titles", ["title".to_string()])
                    .depends_on_live(&live)
                    .produces(["title.summary".to_string()]),
                TitleListMaintainer,
            )
            .expect("source computed view should declare");
        let summary = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.summary", ["title.summary".to_string()])
                    .depends_on_derived(&titles)
                    .produces(["validation.state".to_string()]),
                SummaryMaintainer,
            )
            .expect("nested computed view should declare");

        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Nested title" },
                }),
            })
            .expect("insert should update nested computeds");
        let title_patches = runtime.drain_derived_patches(titles.name());
        let summary_patches = runtime.drain_derived_patches(summary.name());

        assert_eq!(
            insert.affected_derived_view_ids(),
            &[
                "computed.summary".to_string(),
                "computed.titles".to_string()
            ]
        );
        assert_eq!(insert.considered_computed_view_count(), 2);
        assert_eq!(title_patches.derived_patches.len(), 1);
        assert_eq!(
            title_patches.derived_patches[0].aspect_paths(),
            &["title.summary".to_string()]
        );
        assert_eq!(summary_patches.derived_patches.len(), 1);
        assert_eq!(
            summary_patches.derived_patches[0].aspect_paths(),
            &["validation.state".to_string()]
        );
        assert_eq!(
            runtime.read_derived(&summary),
            vec![Value::String(format!(
                "summary:{}",
                insert.deltas()[0].entity_identity
            ))]
        );

        runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: insert.deltas()[0].entity_identity.clone(),
                aspect_path: "identity.id".to_string(),
                value: Value::String("ignored".to_string()),
            })
            .expect("irrelevant update should still write");
        assert!(runtime
            .drain_derived_patches(titles.name())
            .derived_patches
            .is_empty());
        assert!(runtime
            .drain_derived_patches(summary.name())
            .derived_patches
            .is_empty());
    }

    #[test]
    fn computed_dependency_index_replaces_redeclared_view_membership() {
        let mut runtime = task_issue_memory_runtime();
        let task_live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("task live should declare");
        let issue_live = runtime
            .declare_live_view::<Value>("issues.table", issue_live_request(), issue_schema())
            .expect("issue live should declare");
        let computed = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.shared", ["title".to_string()])
                    .depends_on_live(&task_live)
                    .produces(["title.summary".to_string()]),
                TitleListMaintainer,
            )
            .expect("task-backed computed should declare");

        runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.shared", ["summary".to_string()])
                    .depends_on_live(&issue_live)
                    .produces(["issue.summary".to_string()]),
                SummaryMaintainer,
            )
            .expect("redeclared computed should replace old dependency index membership");

        let task_write = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Task should not wake redeclared computed" },
                }),
            })
            .expect("task write should execute");
        assert!(task_write.affected_derived_view_ids().is_empty());
        assert_eq!(task_write.considered_computed_view_count(), 0);
        assert!(runtime
            .drain_derived_patches(computed.name())
            .derived_patches
            .is_empty());

        let issue_write = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Issue".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "summary": { "value": "Issue wakes computed" },
                }),
            })
            .expect("issue write should execute");
        let issue_patches = runtime.drain_derived_patches(computed.name());

        assert_eq!(
            issue_write.affected_derived_view_ids(),
            &["computed.shared".to_string()]
        );
        assert_eq!(issue_write.considered_computed_view_count(), 1);
        assert_eq!(issue_patches.derived_patches.len(), 1);
        assert_eq!(
            issue_patches.derived_patches[0].aspect_paths(),
            &["issue.summary".to_string()]
        );
    }

    #[test]
    fn computed_handle_inspection_reports_dependencies_aspects_and_materialization() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let computed = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.inspectable", ["title".to_string()])
                    .depends_on_live(&live)
                    .produces(["title.summary".to_string()]),
                TitleListMaintainer,
            )
            .expect("computed should declare");
        runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Inspectable task" },
                }),
            })
            .expect("write should materialize computed output");

        let evidence = runtime
            .inspect_derived_view(&computed)
            .expect("computed handle should inspect");

        assert_eq!(evidence.name(), "computed.inspectable");
        assert_eq!(
            evidence.authority_lane(),
            ForgeQueryAuthorityLane::DerivedRuntimeState
        );
        assert_eq!(evidence.upstream_live_views(), &["tasks.table".to_string()]);
        assert!(evidence.upstream_derived_views().is_empty());
        assert_eq!(evidence.dependency_aspects(), &["title".to_string()]);
        assert_eq!(evidence.produced_aspects(), &["title.summary".to_string()]);
        assert_eq!(evidence.materialized_row_count(), 1);
        assert_eq!(evidence.pending_patch_count(), 1);

        let foreign_runtime = task_runtime();
        let error = foreign_runtime
            .inspect_derived_view(&computed)
            .expect_err("foreign computed handle should not inspect in another runtime");
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingDerivedView(_)
        ));
    }

    #[test]
    fn effect_delivery_routes_from_live_trigger_with_expression_metadata() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let effect = runtime
            .declare_effect::<Value>(
                ForgeQueryEffectDeclaration::deliver(
                    "ui.title-badges",
                    ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                    "ui.badges",
                )
                .with_condition(ForgeQueryEffectCondition::expression(
                    "expr.title.badge",
                    ["title"],
                    ["ui.badge"],
                )),
            )
            .expect("effect should declare");

        let write = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Effect task" },
                }),
            })
            .expect("write should route effect");
        let evidence = runtime
            .inspect_effect(&effect)
            .expect("effect should inspect before drain");
        let deliveries = runtime
            .drain_effect_deliveries(&effect)
            .expect("effect deliveries should drain");

        assert_eq!(write.considered_effect_count(), 1);
        assert_eq!(write.delivered_effect_count(), 1);
        assert_eq!(write.suppressed_effect_count(), 0);
        assert_eq!(write.effect_expression_failure_count(), 0);
        assert_eq!(evidence.name(), "ui.title-badges");
        assert_eq!(evidence.trigger_source(), "tasks.table");
        assert_eq!(
            evidence.trigger_source_kind(),
            ForgeQueryEffectTriggerSourceKind::LiveView
        );
        assert_eq!(evidence.condition_descriptor(), "expr.title.badge");
        assert_eq!(evidence.condition_inputs(), &["title".to_string()]);
        assert_eq!(evidence.condition_outputs(), &["ui.badge".to_string()]);
        assert_eq!(evidence.pending_delivery_count(), 1);
        assert_eq!(deliveries.len(), 1);
        assert_eq!(
            deliveries[0].family(),
            &ForgeQueryEffectDeliveryFamily::Delivered
        );
        assert_eq!(deliveries[0].target(), "ui.badges");
        assert_eq!(
            deliveries[0].authority_lane(),
            ForgeQueryAuthorityLane::EffectDeliveryState
        );
        assert_eq!(deliveries[0].aspect_paths(), &["title".to_string()]);
        assert_eq!(deliveries[0].payload()["condition"], "expr.title.badge");
    }

    #[test]
    fn effect_delivery_routes_from_computed_trigger_after_computed_patch() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let titles = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.titles.effect", ["title".to_string()])
                    .depends_on_live(&live)
                    .produces(["title.summary".to_string()]),
                TitleListMaintainer,
            )
            .expect("computed should declare");
        let effect = runtime
            .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
                "ui.summary-badges",
                ForgeQueryEffectTrigger::computed_view(&titles, ["title.summary"]),
                "ui.summary",
            ))
            .expect("computed-triggered effect should declare");

        let write = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Computed effect task" },
                }),
            })
            .expect("write should route computed effect");
        let deliveries = runtime
            .drain_effect_deliveries(&effect)
            .expect("effect deliveries should drain");

        assert_eq!(write.considered_computed_view_count(), 1);
        assert_eq!(write.considered_effect_count(), 1);
        assert_eq!(write.delivered_effect_count(), 1);
        assert_eq!(deliveries.len(), 1);
        assert_eq!(
            deliveries[0].trigger_source_kind(),
            ForgeQueryEffectTriggerSourceKind::ComputedView
        );
        assert_eq!(deliveries[0].trigger_source(), "computed.titles.effect");
        assert_eq!(deliveries[0].aspect_paths(), &["title.summary".to_string()]);
        assert_eq!(
            runtime.read_derived(&titles),
            vec![Value::String(write.deltas()[0].entity_identity.clone())]
        );
    }

    #[test]
    fn computed_effect_does_not_replay_stale_undrained_computed_patch() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let titles = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.titles.stale-effect", ["title".to_string()])
                    .depends_on_live(&live)
                    .produces(["title.summary".to_string()]),
                TitleListMaintainer,
            )
            .expect("computed should declare");
        let effect = runtime
            .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
                "ui.stale-summary-badges",
                ForgeQueryEffectTrigger::computed_view(&titles, ["title.summary"]),
                "ui.summary",
            ))
            .expect("computed-triggered effect should declare");

        runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "First effect task" },
                }),
            })
            .expect("first write should route computed effect");
        let first_deliveries = runtime
            .drain_effect_deliveries(&effect)
            .expect("first effect deliveries should drain");
        assert_eq!(first_deliveries.len(), 1);

        let unrelated = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: runtime.read_derived(&titles)[0]
                    .as_str()
                    .expect("computed row should be an entity id")
                    .to_string(),
                aspect_path: "identity.id".to_string(),
                value: Value::String("irrelevant".to_string()),
            })
            .expect("irrelevant write should not replay stale computed patch");
        let stale_deliveries = runtime
            .drain_effect_deliveries(&effect)
            .expect("stale effect deliveries should drain");

        assert_eq!(unrelated.considered_computed_view_count(), 1);
        assert!(unrelated.affected_derived_view_ids().is_empty());
        assert_eq!(unrelated.considered_effect_count(), 0);
        assert!(stale_deliveries.is_empty());
    }

    #[test]
    fn effect_expression_suppression_and_failure_are_typed_and_counted() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let suppressed_effect = runtime
            .declare_effect::<Value>(
                ForgeQueryEffectDeclaration::deliver(
                    "ui.suppressed",
                    ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                    "ui.suppressed",
                )
                .with_condition(ForgeQueryEffectCondition::expression(
                    "expr.needs-validation",
                    ["validation.state"],
                    ["ui.badge"],
                )),
            )
            .expect("suppressed effect should declare");
        let failing_effect = runtime
            .declare_effect::<Value>(
                ForgeQueryEffectDeclaration::deliver(
                    "ui.failing",
                    ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                    "ui.failing",
                )
                .with_condition(ForgeQueryEffectCondition::failing_expression(
                    "expr.fail.validation",
                    ["title"],
                    ["ui.badge"],
                )),
            )
            .expect("failing effect should declare");

        let write = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Conditional task" },
                }),
            })
            .expect("write should route effects");
        let suppressed = runtime
            .drain_effect_deliveries(&suppressed_effect)
            .expect("suppressed effect should drain");
        let failed = runtime
            .drain_effect_deliveries(&failing_effect)
            .expect("failing effect should drain");

        assert_eq!(write.considered_effect_count(), 2);
        assert_eq!(write.delivered_effect_count(), 0);
        assert_eq!(write.suppressed_effect_count(), 1);
        assert_eq!(write.effect_expression_failure_count(), 1);
        assert_eq!(
            suppressed[0].family(),
            &ForgeQueryEffectDeliveryFamily::Suppressed
        );
        assert!(suppressed[0]
            .reason()
            .expect("suppression reason should exist")
            .contains("inputs were not changed"));
        assert_eq!(
            failed[0].family(),
            &ForgeQueryEffectDeliveryFamily::ExpressionFailed
        );
        assert!(failed[0]
            .reason()
            .expect("failure reason should exist")
            .contains("deterministic failure"));
    }

    #[test]
    fn meaningful_change_suppression_counts_semantic_delta_suppression() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let effect = runtime
            .declare_effect::<Value>(
                ForgeQueryEffectDeclaration::deliver(
                    "ui.meaningful-title",
                    ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                    "ui.badges",
                )
                .with_meaningful_change_suppression(),
            )
            .expect("meaningful effect should declare");

        let inserted = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Meaningful task" },
                }),
            })
            .expect("insert should deliver because whole-row delta is meaningful");
        assert_eq!(inserted.delivered_effect_count(), 1);
        assert_eq!(inserted.meaningful_effect_suppression_count(), 0);
        assert_eq!(
            runtime
                .drain_effect_deliveries(&effect)
                .expect("insert delivery should drain")
                .len(),
            1
        );

        let churn = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: inserted.deltas()[0].entity_identity.clone(),
                aspect_path: "identity.id".to_string(),
                value: Value::String("semantic-churn".to_string()),
            })
            .expect("irrelevant aspect update should be suppressed as churn");
        let evidence = runtime
            .inspect_effect(&effect)
            .expect("meaningful effect should inspect");
        let suppressed = runtime
            .drain_effect_deliveries(&effect)
            .expect("suppressed effect should drain");

        assert_eq!(churn.considered_effect_count(), 1);
        assert_eq!(churn.delivered_effect_count(), 0);
        assert_eq!(churn.suppressed_effect_count(), 1);
        assert_eq!(churn.meaningful_effect_suppression_count(), 1);
        assert_eq!(
            evidence.suppression_policy(),
            ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
        );
        assert_eq!(evidence.counters().meaningful_suppressions(), 1);
        assert_eq!(suppressed.len(), 1);
        assert_eq!(
            suppressed[0].family(),
            &ForgeQueryEffectDeliveryFamily::Suppressed
        );
        assert_eq!(
            suppressed[0].suppression_policy(),
            ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
        );
        assert!(suppressed[0]
            .reason()
            .expect("meaningful suppression should explain itself")
            .contains("meaningful semantic delta suppression"));
    }

    #[test]
    fn effect_declaration_rejects_missing_triggers_before_registration() {
        let mut runtime = task_runtime();
        let missing = ForgeQueryEffectDeclaration::deliver(
            "ui.missing",
            ForgeQueryEffectTrigger::live_view_name("tasks.missing", ["title"]),
            "ui.badges",
        );
        let error = runtime
            .declare_effect::<Value>(missing)
            .expect_err("missing live trigger should reject");

        match error {
            ForgeQueryRuntimeError::EffectDeclaration {
                effect_name,
                stage,
                message,
            } => {
                assert_eq!(effect_name, "ui.missing");
                assert_eq!(stage, "trigger-admission");
                assert!(message.contains("tasks.missing"));
            }
            other => panic!("expected effect declaration denial, got {other:?}"),
        }
    }

    #[test]
    fn effect_declaration_rejects_truth_delivery_without_intent_boundary() {
        let mut runtime = task_runtime();
        let live = runtime
            .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
            .expect("live should declare");
        let declaration = ForgeQueryEffectDeclaration::deliver(
            "ui.truth-smuggle",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "Task",
        )
        .with_target_lane(ForgeQueryAuthorityLane::AuthoritativeTruth);

        let error = runtime
            .declare_effect::<Value>(declaration)
            .expect_err("effect delivery must not target truth");

        match error {
            ForgeQueryRuntimeError::EffectDeclaration { stage, message, .. } => {
                assert_eq!(stage, "authority-admission");
                assert!(message.contains("intent authority"));
            }
            other => panic!("expected authority admission denial, got {other:?}"),
        }
    }

    #[test]
    fn computed_dependency_admission_rejects_missing_or_cyclic_upstream_views() {
        let mut runtime = task_runtime();
        let missing_live = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.missing-live", ["title".to_string()])
                    .depends_on_live_name("tasks.not-declared"),
                TitleListMaintainer,
            )
            .expect_err("missing live dependency should reject before registration");
        match missing_live {
            ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
                assert!(message.contains("tasks.not-declared"));
            }
            other => panic!("expected computed declaration error, got {other:?}"),
        }

        let missing = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.missing", ["title.summary".to_string()])
                    .depends_on_derived_name("computed.unknown"),
                SummaryMaintainer,
            )
            .expect_err("missing computed dependency should reject before registration");
        match missing {
            ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
                assert!(message.contains("computed.unknown"));
            }
            other => panic!("expected computed declaration error, got {other:?}"),
        }

        let first = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.first", ["title".to_string()])
                    .produces(["title.summary".to_string()]),
                TitleListMaintainer,
            )
            .expect("first computed should declare");
        let second = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.second", ["title.summary".to_string()])
                    .depends_on_derived(&first)
                    .produces(["validation.state".to_string()]),
                SummaryMaintainer,
            )
            .expect("second computed should declare");

        let cycle = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("computed.first", ["validation.state".to_string()])
                    .depends_on_derived(&second),
                SummaryMaintainer,
            )
            .expect_err("redeclared computed dependency should not create a cycle");
        match cycle {
            ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
                assert!(message.contains("cycle"));
            }
            other => panic!("expected computed cycle declaration error, got {other:?}"),
        }
    }

    struct FakeDsl;

    struct FakeSchemaAdapter;

    struct TitleListMaintainer;
    struct SummaryMaintainer;

    impl ForgeQueryDerivedViewMaintainer for TitleListMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = Value::String(delta.entity_identity.clone());
            materialization.push_row(row.clone());
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                "derived-test-commit",
                delta.entity_identity.clone(),
                if view.produced_aspects().is_empty() {
                    delta.aspect_paths.clone()
                } else {
                    view.produced_aspects().to_vec()
                },
                row,
            )
        }
    }

    impl ForgeQueryDerivedViewMaintainer for SummaryMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = Value::String(format!("summary:{}", delta.entity_identity));
            materialization.replace_rows([row.clone()]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                "derived-summary-commit",
                delta.entity_identity.clone(),
                if view.produced_aspects().is_empty() {
                    delta.aspect_paths.clone()
                } else {
                    view.produced_aspects().to_vec()
                },
                row,
            )
        }
    }

    impl ForgeQuerySchemaAdapter for FakeSchemaAdapter {
        fn schema_view(&self, operation_id: &str) -> Option<QuerySchemaView> {
            (operation_id == "create_task").then(task_schema)
        }
    }

    impl ForgeQueryProgramSource for FakeDsl {
        fn compile_program<A>(
            self,
            schema_adapter: &A,
        ) -> Result<ForgeQueryProgram, ForgeQueryProgramError>
        where
            A: ForgeQuerySchemaAdapter + ?Sized,
        {
            let schema_view = schema_adapter
                .schema_view("create_task")
                .ok_or_else(|| ForgeQueryProgramError::new("missing schema for create_task"))?;
            ForgeQueryProgram::new(
                "fake.strict.dsl",
                [ForgeQueryOperation::new("create_task")
                    .with_input(ForgeQueryTypedPort::new(
                        "title",
                        ForgeQueryPortType::String,
                    ))
                    .requires(ForgeQueryAuthorityRequirement::Live)
                    .requires(ForgeQueryAuthorityRequirement::Writeback)
                    .with_effect(ForgeQueryProgramEffect::DeclareLiveView {
                        name: "tasks.table".to_string(),
                        request: task_live_request(),
                        schema_view,
                    })
                    .with_effect(ForgeQueryProgramEffect::WriteTemplate(
                        ForgeQueryWriteCommandTemplate::Insert {
                            collection: "Task".to_string(),
                            payload: ForgeQueryValueExpr::object([
                                (
                                    "identity".to_string(),
                                    ForgeQueryValueExpr::object([(
                                        "id".to_string(),
                                        ForgeQueryValueExpr::literal(Value::String(String::new())),
                                    )]),
                                ),
                                (
                                    "title".to_string(),
                                    ForgeQueryValueExpr::object([(
                                        "value".to_string(),
                                        ForgeQueryValueExpr::input("title"),
                                    )]),
                                ),
                            ]),
                        },
                    ))
                    .with_effect(ForgeQueryProgramEffect::ReadLive {
                        view_name: "tasks.table".to_string(),
                    })
                    .with_effect(ForgeQueryProgramEffect::DrainPatches {
                        view_name: "tasks.table".to_string(),
                    })],
            )
        }
    }

    impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
        fn admit_live_view(
            &self,
            _name: &str,
            _request: &DeclarativeLiveQueryRequest,
            _schema_view: &QuerySchemaView,
        ) -> Result<(), ForgeQueryWorkspaceError> {
            Ok(())
        }
    }

    struct TestSchemaAdapter;

    struct DenyingSchemaAdapter;

    impl ForgeQueryRuntimeSchemaAdapter for DenyingSchemaAdapter {
        fn admit_live_view(
            &self,
            _name: &str,
            _request: &DeclarativeLiveQueryRequest,
            _schema_view: &QuerySchemaView,
        ) -> Result<(), ForgeQueryWorkspaceError> {
            Err(ForgeQueryWorkspaceError::new(
                "schema admission denied by test adapter",
            ))
        }
    }

    #[derive(Default)]
    struct TestSourceAdapter {
        live_views: BTreeMap<String, String>,
        fail_declare: bool,
    }

    impl TestSourceAdapter {
        fn fail_declare() -> Self {
            Self {
                live_views: BTreeMap::new(),
                fail_declare: true,
            }
        }
    }

    impl ForgeQueryRuntimeSourceAdapter for TestSourceAdapter {
        fn declare_live_view(
            &mut self,
            name: String,
            request: DeclarativeLiveQueryRequest,
            _schema_view: QuerySchemaView,
        ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
            if self.fail_declare {
                return Err(ForgeQueryWorkspaceError::new(
                    "source declaration denied by test adapter",
                ));
            }
            self.live_views
                .insert(name.clone(), request.target().to_string());
            Ok(ForgeQueryLiveViewHandle::new(name))
        }

        fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
            Vec::new()
        }

        fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
            Vec::new()
        }

        fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
            let mut affected = receipt
                .deltas
                .iter()
                .flat_map(|delta| {
                    self.live_views
                        .iter()
                        .filter(move |(_, collection)| *collection == &delta.collection)
                        .map(|(name, _)| name.clone())
                })
                .collect::<Vec<_>>();
            affected.sort();
            affected.dedup();
            affected
        }

        fn snapshot_token(&self) -> String {
            "external-snapshot".to_string()
        }
    }

    struct TestWriteAuthority;

    impl ForgeQueryRuntimeWriteAuthorityAdapter for TestWriteAuthority {
        fn write(
            &mut self,
            _bridge: &RuntimeBridge,
            _relational_runtime: Option<&mut RelationalRuntime>,
            command: ForgeQueryWriteCommand,
        ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
            let collection = match command {
                ForgeQueryWriteCommand::Insert { collection, .. } => collection,
                ForgeQueryWriteCommand::UpdateAspect { .. } => "Task".to_string(),
                ForgeQueryWriteCommand::Delete { .. } => "Task".to_string(),
            };
            Ok(ForgeQueryMutationReceipt {
                commit_identity: "external-commit-1".to_string(),
                snapshot_token: "external-snapshot-1".to_string(),
                deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                    collection,
                    entity_identity: "external-entity-1".to_string(),
                    kind: ForgeQueryMutationKind::Created,
                    aspect_paths: Vec::new(),
                }],
            })
        }
    }

    struct TestSignalSink;

    impl ForgeQueryRuntimeSignalSinkAdapter for TestSignalSink {
        fn route_write_receipt(
            &mut self,
            _receipt: &ForgeQueryMutationReceipt,
        ) -> Result<(), ForgeQueryWorkspaceError> {
            Ok(())
        }
    }

    struct TestSubscriptionActivation;

    impl ForgeQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
        fn support_evidence(&self) -> String {
            "test-subscription-activation".to_string()
        }

        fn admit_activation(
            &mut self,
            view_name: &str,
            activation: &crate::subscription::SubscriptionActivationInput,
        ) -> Result<String, ForgeQueryWorkspaceError> {
            Ok(format!(
                "test-subscription-activation:{view_name}:{}",
                activation.activation_digest()
            ))
        }
    }

    struct DenyingSubscriptionActivation;

    impl ForgeQueryRuntimeSubscriptionActivationAdapter for DenyingSubscriptionActivation {
        fn support_evidence(&self) -> String {
            "denying-subscription-activation".to_string()
        }

        fn admit_activation(
            &mut self,
            _view_name: &str,
            _activation: &crate::subscription::SubscriptionActivationInput,
        ) -> Result<String, ForgeQueryWorkspaceError> {
            Err(ForgeQueryWorkspaceError::new(
                "activation denied by test adapter",
            ))
        }
    }

    struct TestPreviewBasis;

    impl ForgeQueryRuntimePreviewBasisAdapter for TestPreviewBasis {
        fn admit_preview_basis(
            &self,
            label: &str,
            effect_policy: ForgeQueryEffectPolicy,
            authority: &ForgeQueryRuntimeEvidenceAuthority,
        ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
            Ok(ForgeQueryPreviewBasisAdmission::new(
                authority,
                label,
                effect_policy,
                ["test-preview-basis"],
            ))
        }
    }

    struct TestInspectorEvidence;

    impl ForgeQueryRuntimeInspectorEvidenceAdapter for TestInspectorEvidence {
        fn inspect_write_receipt(
            &self,
            receipt: &ForgeQueryWriteReceipt,
            authority: &ForgeQueryRuntimeEvidenceAuthority,
        ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
            Ok(ForgeQueryRuntimeInspectionEvidence::new(
                authority,
                "test-write-receipt",
                receipt.authority_lane(),
                ["test-inspector-evidence"],
            ))
        }
    }

    #[derive(Clone, Debug)]
    struct TestBridgeSource;

    impl forge_runtime_bridge::facade::CommittedPatchSource for TestBridgeSource {
        fn load_committed_patch(
            &self,
            request: RelationalCommittedPatchRequest,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            Ok(RawCommittedPatchEnvelope::new(
                TruthCommitIdentity::new(request.commit_identity()),
                TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
                TruthSnapshotIdentity::new("external-snapshot"),
                TruthBranchIdentity::new("main"),
                vec![BridgeCommittedPatchItem::new("entity", "aspect", "field")],
            ))
        }
    }

    impl SnapshotReadSource for TestBridgeSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
            Ok(Box::new(TestSnapshotReader {
                identity: identity.clone(),
            }))
        }
    }

    struct TestSnapshotReader {
        identity: TruthSnapshotIdentity,
    }

    impl TruthSnapshotReader for TestSnapshotReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            self.identity.clone()
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
        {
            Ok(SnapshotReadPacketResult::new(
                self.identity.clone(),
                request
                    .reads()
                    .iter()
                    .map(|read| SnapshotReadRecord::new(read.request_key(), Vec::new()))
                    .collect(),
            ))
        }
    }

    struct TestBridgeSink;

    impl InvalidationSink for TestBridgeSink {
        fn deliver_invalidation(
            &self,
            delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
        ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
            Ok(BridgeDeliveryReceipt::new(
                delivery.invalidation_targets().len(),
                delivery.source_snapshot().clone(),
            ))
        }
    }

    fn test_bridge() -> RuntimeBridge {
        RuntimeBridgeBuilder::new()
            .with_relational_source(TestBridgeSource)
            .with_signal_sink(TestBridgeSink)
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("external-test"),
                TruthPatchScope::new(
                    MappingSelector::any(),
                    MappingSelector::any(),
                    MappingSelector::any(),
                ),
                SignalInvalidationScope::new("external-test"),
                CoarseRoutingMode::Direct,
            ))
            .build()
            .expect("test bridge should build")
    }

    fn bridge_runtime_with_support(profile: ForgeQueryRuntimeSupportProfile) -> ForgeQueryRuntime {
        ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .support_profile(profile)
            .build_backend_from_parts()
            .build()
            .expect("complete backend parts should build")
    }

    fn task_runtime() -> ForgeQueryRuntime {
        ForgeQueryRuntime::builder()
            .in_memory_collections([ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                ],
            )])
            .build()
            .expect("runtime should build")
    }

    fn task_issue_memory_runtime() -> ForgeQueryRuntime {
        ForgeQueryRuntime::builder()
            .in_memory_collections([
                ForgeQueryCollection::new(
                    "Task",
                    [
                        crate::memory_workspace::ForgeQueryAspect::new(
                            "identity.id",
                            "identity.id",
                        ),
                        crate::memory_workspace::ForgeQueryAspect::new(
                            "title.value",
                            "title.value",
                        ),
                    ],
                ),
                ForgeQueryCollection::new(
                    "Issue",
                    [
                        crate::memory_workspace::ForgeQueryAspect::new(
                            "identity.id",
                            "identity.id",
                        ),
                        crate::memory_workspace::ForgeQueryAspect::new(
                            "summary.value",
                            "summary.value",
                        ),
                    ],
                ),
            ])
            .build()
            .expect("runtime should build")
    }

    fn grouped_task_runtime() -> ForgeQueryRuntime {
        ForgeQueryRuntime::builder()
            .in_memory_collections([ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                    crate::memory_workspace::ForgeQueryAspect::new("status.value", "status.value"),
                ],
            )])
            .build()
            .expect("runtime should build")
    }

    fn task_live_request() -> DeclarativeLiveQueryRequest {
        DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
            .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
            .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
            .order_by(DeclarativeProjectionField::new("title", "value"))
    }

    fn task_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "runtime-task",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            ],
            [],
        )
    }

    fn issue_live_request() -> DeclarativeLiveQueryRequest {
        DeclarativeLiveQueryRequest::new("Issue", DeclarativeLiveViewShape::table())
            .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
            .project(DeclarativeProjectionField::new("summary", "value").delivered_as("summary"))
            .order_by(DeclarativeProjectionField::new("summary", "value"))
    }

    fn issue_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "runtime-issue",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("summary", "value", SchemaFieldKind::String),
            ],
            [],
        )
    }

    fn grouped_task_live_request() -> DeclarativeLiveQueryRequest {
        DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::kanban_grouped("status"))
            .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
            .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
            .project(DeclarativeProjectionField::new("status", "value").delivered_as("status"))
            .order_by(DeclarativeProjectionField::new("title", "value"))
    }

    fn grouped_task_table_live_request() -> DeclarativeLiveQueryRequest {
        DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
            .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
            .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
            .project(DeclarativeProjectionField::new("status", "value").delivered_as("status"))
            .order_by(DeclarativeProjectionField::new("title", "value"))
    }

    fn grouped_task_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "runtime-grouped-task",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("title", "value", SchemaFieldKind::String),
                SchemaFieldView::new("status", "value", SchemaFieldKind::String),
            ],
            [],
        )
    }
}
