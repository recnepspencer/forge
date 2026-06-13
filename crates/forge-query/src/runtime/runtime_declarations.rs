use super::async_result_state::ForgeQueryRuntimeAsyncResultProjection;
use super::*;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError,
};

impl ForgeQueryRuntime {
    pub fn declare_live_view<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Live)?;
        let name = name.into();
        admit_live_view_declaration_receipt(&*self.backend, &name, &request, &schema_view)?;
        let activation =
            self.install_live_subscription_for_request(&name, &request, schema_view.clone())?;
        let handle = declare_live_view_source_handle(
            &mut *self.backend,
            &mut self.active_subscriptions,
            &name,
            request,
            schema_view,
            &activation,
        )?;
        register_live_subscription_index(
            &mut self.live_subscription_index,
            &name,
            &activation.request,
        );
        let pending_async_result_state = activation
            .active_lane_handle
            .future_selection()
            .requests_completion_lifecycle()
            .then(|| {
                ForgeQueryRuntimeAsyncResultProjection::pending(&format!(
                    "async-pending:{}",
                    activation
                        .active_lane_handle
                        .future_selection()
                        .projection_digest()
                ))
            });
        let checkpoint_identity_digest = activation
            .active_lane_handle
            .checkpoint_identity_digest()
            .to_string();
        self.live_subscriptions.insert(
            name.clone(),
            ForgeQueryRuntimeLiveSubscriptionState {
                installation: activation.installation.clone(),
                active_lane_handle: activation.active_lane_handle,
                consumer_attachment: activation.consumer_attachment,
                request: activation.request,
                delivery_batches: Vec::new(),
                last_delivery: None,
                async_result_state: None,
                remask_posture: activation.remask_posture,
            },
        );
        if let Some(projection) = pending_async_result_state.as_ref() {
            self.project_async_result_state(
                &name,
                projection,
                activation.installation.basis_binding_for_reporting(),
                &checkpoint_identity_digest,
            )?;
        }
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
        self.initialize_maintained_derived_view_materialization(&name)?;
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
}

impl ForgeQueryRuntime {
    fn initialize_maintained_derived_view_materialization(
        &mut self,
        view_name: &str,
    ) -> Result<(), ForgeQueryRuntimeError> {
        let declaration = match self.derived_views.get(view_name) {
            Some(runtime) if !runtime.declaration.incremental() && runtime.maintainer.is_some() => {
                runtime.declaration.clone()
            }
            _ => return Ok(()),
        };
        let upstreams = retained_upstream_inputs_for_declaration(self, &declaration)?;
        let snapshot_identity = self.current_snapshot_identity();
        let refresh_identity = ForgeQueryCommitIdentity::preview(
            ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("refresh_origin"),
                "derived-declaration",
            )
            .field_value(ForgeQueryEvidenceTag::new("view_name"), view_name)
            .seal(),
        );
        let refresh_metadata = self
            .backend
            .declaration_initialization_metadata(&declaration)
            .map_err(ForgeQueryRuntimeError::Workspace)?;
        let refresh = ForgeQueryRetainedRefreshContext::from_declaration_initialization(
            refresh_identity,
            snapshot_identity,
            refresh_metadata,
        );
        let Some(runtime) = self.derived_views.get_mut(view_name) else {
            return Ok(());
        };
        let Some(maintainer) = runtime.maintainer.as_mut() else {
            return Ok(());
        };
        let _ = maintainer.refresh_from_upstreams(
            &declaration,
            &refresh,
            &upstreams,
            &mut runtime.materialization,
        );
        Ok(())
    }
}

fn admit_live_view_declaration_receipt(
    backend: &dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
    schema_view: &QuerySchemaView,
) -> Result<(), ForgeQueryRuntimeError> {
    let admission_receipt = backend
        .admit_live_view_declaration(view_name, request, schema_view)
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "backend-live-admission",
                message: error.to_string(),
            },
        )?;
    if let Some(message) = admission_receipt.drift_from_request(view_name, request) {
        return Err(ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "backend-live-admission-receipt",
            message,
        });
    }
    Ok(())
}

fn declare_live_view_source_handle(
    backend: &mut dyn ForgeQueryRuntimeBackend,
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    view_name: &str,
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    activation: &ForgeQueryRuntimeLiveSubscriptionActivation,
) -> Result<ForgeQueryLiveViewHandle, ForgeQueryRuntimeError> {
    backend
        .declare_live_view(view_name.to_string(), request, schema_view)
        .map_err(|error| {
            live_source_declaration_error(active_subscriptions, view_name, error, activation)
        })
}

fn live_source_declaration_error(
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    view_name: &str,
    error: ForgeQueryWorkspaceError,
    activation: &ForgeQueryRuntimeLiveSubscriptionActivation,
) -> ForgeQueryRuntimeError {
    let closeout_result = close_subscription_lifecycle(
        active_subscriptions,
        &activation.active_lane_handle,
        SubscriptionLifecycleCloseRequest::DetachConsumer(activation.consumer_attachment.clone()),
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
    ForgeQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: view_name.to_string(),
        stage: "source-declaration",
        message: format!("{error}; {closeout_message}"),
    }
}

fn retained_upstream_inputs_for_declaration(
    runtime: &mut ForgeQueryRuntime,
    declaration: &ForgeQueryDerivedView,
) -> Result<ForgeQueryRetainedUpstreamInputs, ForgeQueryRuntimeError> {
    let live_rows = declaration
        .upstream_live_views()
        .iter()
        .map(|view_name| {
            runtime
                .execute_live_read_by_name(view_name)
                .map(|read| (view_name.clone(), read.rows().to_vec()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let computed_rows = declaration
        .upstream_derived_views()
        .iter()
        .filter_map(|view_name| {
            runtime
                .derived_views
                .get(view_name)
                .map(|runtime| (view_name.clone(), runtime.materialization.rows().to_vec()))
        })
        .collect::<Vec<_>>();
    Ok(ForgeQueryRetainedUpstreamInputs::new(
        live_rows,
        computed_rows,
    ))
}
