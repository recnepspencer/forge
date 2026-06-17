#![allow(dead_code)]

use forge_proof::TransitionOutcome;
use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEntity, ForgeQueryEvidenceIdentity,
    ForgeQueryIntentDeclaration, ForgeQueryIntentExecution, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntime, ForgeQueryRuntimeBackend, ForgeQueryRuntimeError,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeSupportProfile, ForgeQuerySessionLabel, ForgeQuerySnapshotIdentity,
    ForgeQueryWorkspace, ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput,
    SubscriptionActivationReceipt,
};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;
use forge_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, ForgeNativeSurface},
    ForgeServer, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityExecutionInput,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityRequestInput,
    ForgeServerConfig, ForgeServerQueryHandoffConfig, ForgeServerQueryWorkspaceBindingError,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerQueryWorkspaceBindingTarget,
    ForgeServerQueryWorkspaceProvider, ForgeServerRequestContextConfig, ForgeServerStreamSelection,
    ForgeServerStreamingResponse,
};
use serde_json::Value;

use crate::query_handoff_runtime::TestWorkspaceProvider;

pub(crate) fn build_phase_four_server() -> ForgeServer {
    build_phase_four_server_with_workspace_provider(TestWorkspaceProvider)
}

pub(crate) fn build_phase_four_server_with_workspace_provider(
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

pub(crate) fn prepared_stream_request(
    server: &ForgeServer,
    request: ForgeServerCompatibilityRequestInput,
) -> ForgeServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(request) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compatibility streaming request, got {other:?}"),
    }
}

pub(crate) fn compat_stream_input(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCompatibilityExecutionInput {
    ForgeServerCompatibilityExecutionInput::new(
        prepared_stream_request(
            server,
            stream_input(operation_name)
                .build()
                .expect("compat stream input should validate structurally"),
        ),
        operation_name,
    )
}

pub(crate) fn compat_stream_head_input(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCompatibilityExecutionInput {
    ForgeServerCompatibilityExecutionInput::new(
        prepared_stream_request(
            server,
            stream_input(operation_name)
                .with_method("HEAD")
                .build()
                .expect("compat stream head input should validate structurally"),
        ),
        operation_name,
    )
}

pub(crate) fn compat_read_input(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCompatibilityExecutionInput {
    ForgeServerCompatibilityExecutionInput::new(
        prepared_stream_request(
            server,
            read_input(operation_name)
                .build()
                .expect("compat read input should validate structurally"),
        ),
        operation_name,
    )
}

pub(crate) fn read_input(
    operation_name: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    base_input(operation_name, ForgeServerCompatHttpRouteFamily::Read)
        .with_path(format!("/compat/reads/{operation_name}"))
}

pub(crate) fn stream_input(
    operation_name: &str,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    base_input(operation_name, ForgeServerCompatHttpRouteFamily::Streaming)
        .with_path(format!("/compat/streams/{operation_name}"))
}

fn base_input(
    _operation_name: &str,
    route_family: ForgeServerCompatHttpRouteFamily,
) -> forge_server::ForgeServerCompatibilityRequestInputBuilder {
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(route_family)
        .with_method("GET")
        .with_header("accept", "application/json")
}

pub(crate) fn streaming_response_success(
    response: forge_server::ForgeServerCompatibilityExecutionOutcome<ForgeServerStreamingResponse>,
) -> ForgeServerStreamingResponse {
    match response {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility streaming success, got {other:?}"),
    }
}

pub(crate) fn oversized_streaming_provider(
    row_count: usize,
    payload_width: usize,
) -> StreamingDatasetWorkspaceProvider {
    StreamingDatasetWorkspaceProvider::new(row_count, payload_width)
}

#[derive(Clone, Debug)]
pub(crate) struct StreamingDatasetWorkspaceProvider {
    row_count: usize,
    payload_width: usize,
}

impl StreamingDatasetWorkspaceProvider {
    pub(crate) fn new(row_count: usize, payload_width: usize) -> Self {
        Self {
            row_count,
            payload_width,
        }
    }
}

impl ForgeServerQueryWorkspaceProvider for StreamingDatasetWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "streaming-dataset-workspace-provider"
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
        let mut workspace = ForgeQueryRuntime::builder()
            .backend(StreamingDatasetRuntimeBackend::new(
                self.row_count,
                self.payload_width,
            ))
            .build()
            .map_err(|error| {
                ForgeServerQueryWorkspaceBindingError::new("runtime_build", format!("{error:?}"))
            })?
            .workspace(workspace_id)
            .map_err(|error| {
                ForgeServerQueryWorkspaceBindingError::new("workspace_bind", format!("{error:?}"))
            })?;
        install_requested_named_read(&mut workspace, request)?;
        Ok(workspace)
    }
}

#[derive(Clone, Debug)]
struct StreamingDatasetRuntimeBackend {
    row_count: usize,
    payload_width: usize,
}

impl StreamingDatasetRuntimeBackend {
    fn new(row_count: usize, payload_width: usize) -> Self {
        Self {
            row_count,
            payload_width,
        }
    }
}

impl ForgeQueryRuntimeBackend for StreamingDatasetRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        TestSchemaAdapter.admit_live_view(name, request, schema_view)
    }

    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn write(
        &mut self,
        _command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        panic!("phase four streaming runtime does not write")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        panic!("phase four streaming runtime does not write batches")
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("phase four streaming runtime does not execute generic intents")
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        (0..self.row_count)
            .map(|index| {
                let payload = "x".repeat(self.payload_width);
                ForgeQueryEntity::from_external_projection(
                    forge_query::facade::admit_authored_entity_token(
                        forge_query::facade::QueryExternalIdentityToken::new(std::sync::Arc::from(
                            format!("stream-row-{index}"),
                        )),
                    ),
                    streaming_row(index, &payload),
                )
            })
            .collect()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        )
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        let mut activation_adapter = TestSubscriptionActivation;
        let receipt = activation_adapter.admit_activation(view_name, activation)?;
        Ok(receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        _label: &ForgeQuerySessionLabel,
        _effect_policy: forge_query::facade::ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        panic!("phase four streaming runtime does not admit preview basis")
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "phase-four-streaming-inspection",
            receipt.authority_lane(),
            ["phase-four-streaming-runtime"],
        ))
    }
}

fn install_requested_named_read(
    workspace: &mut ForgeQueryWorkspace,
    request: &ForgeServerQueryWorkspaceBindingRequest,
) -> Result<(), ForgeServerQueryWorkspaceBindingError> {
    let ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration { binding_label, .. } =
        request.target()
    else {
        return Ok(());
    };
    workspace
        .live_view::<Value>(binding_label, |q| {
            q.from("User")
                .select(["identity.id", "profile.display_name"])
                .schema_basis("forge-server-phase-four-streaming")
        })
        .map(|_| ())
        .map_err(|error| {
            ForgeServerQueryWorkspaceBindingError::new(
                "workspace_declaration",
                format!("{error:?}"),
            )
        })
}

fn streaming_row(index: usize, payload: &str) -> Value {
    serde_json::json!({
        "identity": { "id": format!("user-{index}") },
        "profile": {
            "display_name": format!("Stream User {index}"),
            "payload": payload,
        }
    })
}

struct TestSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let admission = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, admission))
    }
}

struct TestSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query::facade::runtime_subscription_support_evidence_identity(
            "phase-four-streaming-support",
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<forge_query::facade::SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError>
    {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

pub(crate) fn default_stream_selection() -> ForgeServerStreamSelection {
    ForgeServerStreamSelection::incremental().with_chunk_bytes(32)
}
