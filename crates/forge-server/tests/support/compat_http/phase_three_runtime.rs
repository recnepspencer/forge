use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution, ForgeQueryLivePatch,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime, ForgeQueryRuntimeBackend,
    ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspace,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    SubscriptionActivationInput, SubscriptionActivationReceipt,
};
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityMutationExecutionInput,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityRequestInput,
    ForgeServerConfig, ForgeServerMiddlewareConfig, ForgeServerQueryHandoffConfig,
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig,
};
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub(crate) fn build_phase_three_server() -> ForgeServer {
    build_phase_three_server_with_workspace_provider(
        crate::query_handoff_runtime::TestWorkspaceProvider,
    )
}

pub(crate) fn build_phase_three_server_with_workspace_provider(
    workspace_provider: impl ForgeServerQueryWorkspaceProvider,
) -> ForgeServer {
    ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    ForgeServerRequestContextConfig::builder()
                        .with_preview_targeting_enabled(true)
                        .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                        .build()
                        .expect("request context config should validate"),
                )
                .with_middleware_config(
                    ForgeServerMiddlewareConfig::builder()
                        .with_query_mutation_enabled(true)
                        .build()
                        .expect("middleware config should validate"),
                )
                .with_query_handoff_config(
                    ForgeServerQueryHandoffConfig::builder()
                        .with_workspace_provider(workspace_provider)
                        .build()
                        .expect("query handoff config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_surface(ForgeNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("server should build")
}

pub(crate) fn mutation_input(
    operation_name: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Mutation)
        .with_method("POST")
        .with_path(format!("/compat/mutations/{operation_name}"))
        .with_header("accept", "application/json")
        .with_body_content_type("application/json")
        .with_body_present(true)
}

pub(crate) fn prepared_mutation_request(
    server: &ForgeServer,
    request: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(request) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compatibility mutation request, got {other:?}"),
    }
}

pub(crate) fn compat_mutation_execution_input(
    server: &ForgeServer,
    operation_name: &str,
    body: Value,
) -> ForgeServerCompatibilityMutationExecutionInput {
    ForgeServerCompatibilityMutationExecutionInput::new(
        prepared_mutation_request(
            server,
            mutation_input(operation_name)
                .build()
                .expect("compat mutation input should validate structurally"),
        ),
        operation_name,
        body,
    )
}

pub(crate) fn single_insert_body(identity: &str) -> Value {
    json!({
        "command": {
            "family": "insert",
            "collection": "Task",
            "aspects": {
                "identity.id": identity,
                "title.value": format!("Title for {identity}")
            }
        }
    })
}

pub(crate) fn mutation_request_input_for_workspace(
    operation_name: &str,
    workspace_id: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id(workspace_id)
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Mutation)
        .with_method("POST")
        .with_path(format!("/compat/mutations/{operation_name}"))
        .with_header("accept", "application/json")
        .with_body_content_type("application/json")
        .with_body_present(true)
}

pub(crate) fn insert_task(identity: &str) -> ForgeQueryWriteCommand {
    forge_query::facade::ForgeQueryAspectMutationBuilder::new()
        .aspect("identity.id", identity)
        .aspect("title.value", format!("Title for {identity}"))
        .build_insert("Task")
        .expect("insert command should build")
}

pub(crate) fn compat_mutation_success(
    outcome: forge_server::ForgeServerCompatibilityMutationOutcome<
        forge_server::ForgeServerCompatibilityMutation,
    >,
) -> forge_server::ForgeServerCompatibilityMutation {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility mutation success, got {other:?}"),
    }
}

pub(crate) fn compat_mutation_denied(
    outcome: forge_server::ForgeServerCompatibilityMutationOutcome<
        forge_server::ForgeServerCompatibilityMutation,
    >,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected compatibility mutation denial, got {other:?}"),
    }
}

pub(crate) fn direct_mutation_success(
    outcome: forge_server::ForgeServerDirectMutationOutcome,
) -> forge_server::ForgeServerDirectMutation {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected direct mutation success, got {other:?}"),
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StatefulCountingMutationWorkspaceProvider {
    support_profile: ForgeQueryRuntimeSupportProfile,
    attempted_writes: Arc<AtomicUsize>,
    snapshot_version: Arc<AtomicUsize>,
}

impl StatefulCountingMutationWorkspaceProvider {
    pub(crate) fn new(
        support_profile: ForgeQueryRuntimeSupportProfile,
        attempted_writes: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            support_profile,
            attempted_writes,
            snapshot_version: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl ForgeServerQueryWorkspaceProvider for StatefulCountingMutationWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "stateful-counting-mutation-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError> {
        let workspace_id = request
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .workspace_id();
        ForgeQueryRuntime::builder()
            .backend(StatefulCountingMutationRuntimeBackend::new(
                self.support_profile.clone(),
                self.attempted_writes.clone(),
                self.snapshot_version.clone(),
            ))
            .build()
            .map_err(|error| {
                ForgeServerQueryWorkspaceBindingError::new("runtime_build", format!("{error:?}"))
            })?
            .workspace(workspace_id)
            .map_err(|error| {
                ForgeServerQueryWorkspaceBindingError::new("workspace_bind", format!("{error:?}"))
            })
    }
}

#[derive(Clone, Debug)]
struct StatefulCountingMutationRuntimeBackend {
    support_profile: ForgeQueryRuntimeSupportProfile,
    attempted_writes: Arc<AtomicUsize>,
    snapshot_version: Arc<AtomicUsize>,
}

impl StatefulCountingMutationRuntimeBackend {
    fn new(
        support_profile: ForgeQueryRuntimeSupportProfile,
        attempted_writes: Arc<AtomicUsize>,
        snapshot_version: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            support_profile,
            attempted_writes,
            snapshot_version,
        }
    }

    fn next_snapshot_ordinal(&self) -> usize {
        self.snapshot_version.fetch_add(1, Ordering::Relaxed) + 1
    }

    fn snapshot_token_value(&self) -> String {
        format!(
            "phase-three-stateful-mutation-snapshot-{}",
            self.snapshot_version.load(Ordering::Relaxed)
        )
    }
}

impl ForgeQueryRuntimeBackend for StatefulCountingMutationRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &forge_query::facade::DeclarativeLiveQueryRequest,
        _schema_view: &forge_query::facade::QuerySchemaView,
    ) -> Result<
        forge_query::facade::LiveViewDeclarationAdmissionBoundaryReceipt,
        ForgeQueryWorkspaceError,
    > {
        panic!("phase three mutation runtime does not declare live views")
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: forge_query::facade::DeclarativeLiveQueryRequest,
        _schema_view: forge_query::facade::QuerySchemaView,
    ) -> Result<forge_query::facade::ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        panic!("phase three mutation runtime does not declare live views")
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.attempted_writes.fetch_add(1, Ordering::Relaxed);
        Ok(test_mutation_receipt(
            &command,
            self.next_snapshot_ordinal(),
            ForgeQueryMutationKind::Created,
        ))
    }

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        self.attempted_writes
            .fetch_add(commands.len(), Ordering::Relaxed);
        Ok(commands
            .iter()
            .enumerate()
            .map(|(index, command)| {
                test_mutation_receipt(
                    command,
                    self.next_snapshot_ordinal() + index,
                    mutation_kind(command),
                )
            })
            .collect())
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("phase three mutation runtime does not execute generic intents")
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        self.snapshot_token_value()
    }

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        panic!("phase three mutation runtime does not install subscriptions")
    }

    fn admit_preview_basis(
        &self,
        _label: &str,
        _effect_policy: forge_query::facade::ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        panic!("phase three mutation runtime does not admit preview basis")
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "phase-three-mutation-inspection",
            receipt.authority_lane(),
            ["phase-three-mutation-runtime"],
        ))
    }
}

fn test_mutation_receipt(
    command: &ForgeQueryWriteCommand,
    ordinal: usize,
    kind: ForgeQueryMutationKind,
) -> ForgeQueryMutationReceipt {
    ForgeQueryMutationReceipt {
        commit_identity: format!("phase-three-mutation-commit-{ordinal}"),
        snapshot_token: format!("phase-three-stateful-mutation-snapshot-{ordinal}"),
        deltas: vec![ForgeQueryMutationDelta {
            collection: command
                .declared_collection()
                .unwrap_or_else(|| "Task".to_string()),
            entity_identity: command
                .declared_entity_identity()
                .unwrap_or_else(|| format!("phase-three-mutation-entity-{ordinal}")),
            kind,
            aspect_paths: command.declared_aspect_paths(),
        }],
        bridge_authority: None,
    }
}

fn mutation_kind(command: &ForgeQueryWriteCommand) -> ForgeQueryMutationKind {
    match command.mutation_family().as_str() {
        "insert" => ForgeQueryMutationKind::Created,
        "delete" => ForgeQueryMutationKind::Deleted,
        _ => ForgeQueryMutationKind::Updated,
    }
}
