#![allow(dead_code)]

use serde_json::Value;
use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use worth_proof::TransitionOutcome;
use worth_query::facade::foundation::{
    DeclarativeLiveQueryRequest, WorthQueryEntity, WorthQueryLivePatch, WorthQueryLiveViewHandle,
    WorthQueryMutationReceipt, WorthQuerySnapshotIdentity, WorthQueryWorkspaceError,
};
use worth_query::facade::runtime::{
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput,
    SubscriptionActivationReceipt, WorthQueryBackendAdmissibleMutation, WorthQueryEvidenceIdentity,
    WorthQueryIntentDeclaration, WorthQueryIntentExecution, WorthQueryLiveArtifactTarget,
    WorthQueryPreviewBasisAdmission, WorthQueryRuntime, WorthQueryRuntimeBackend,
    WorthQueryRuntimeError, WorthQueryRuntimeEvidenceAuthority,
    WorthQueryRuntimeInspectionEvidence, WorthQueryRuntimeSchemaAdapter,
    WorthQueryRuntimeSubscriptionActivationAdapter, WorthQueryRuntimeSupportProfile,
    WorthQuerySessionLabel, WorthQueryWorkspace, WorthQueryWriteReceipt,
};
use worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;
use worth_server::{
    request_context::DiagnosticRichnessProfile,
    surfaces::{CompatHttpSurface, WorthNativeSurface},
    WorthServer, WorthServerCompatHttpRouteFamily, WorthServerCompatibilityExecutionInput,
    WorthServerCompatibilityPreparedRequest, WorthServerCompatibilityRequestInput,
    WorthServerConfig, WorthServerQueryHandoffConfig, WorthServerQueryWorkspaceBindingError,
    WorthServerQueryWorkspaceBindingRequest, WorthServerQueryWorkspaceBindingTarget,
    WorthServerQueryWorkspaceProvider, WorthServerRequestContextConfig, WorthServerStreamSelection,
    WorthServerStreamingResponse,
};

use crate::query_handoff_runtime::TestWorkspaceProvider;

pub(crate) fn build_phase_four_server() -> WorthServer {
    build_phase_four_server_with_workspace_provider(TestWorkspaceProvider)
}

pub(crate) fn build_phase_four_server_with_workspace_provider(
    workspace_provider: impl WorthServerQueryWorkspaceProvider,
) -> WorthServer {
    WorthServer::builder()
        .with_config(
            WorthServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    WorthServerRequestContextConfig::builder()
                        .with_preview_targeting_enabled(true)
                        .with_default_diagnostics_profile(DiagnosticRichnessProfile::Standard)
                        .build()
                        .expect("request context config should validate"),
                )
                .with_query_handoff_config(
                    WorthServerQueryHandoffConfig::builder()
                        .with_workspace_provider(workspace_provider)
                        .build()
                        .expect("query handoff config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_operations(worth_server::WorthServerOperationRegistration::phase_two_defaults())
        .register_surface(WorthNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("server should build")
}

pub(crate) fn prepared_stream_request(
    server: &WorthServer,
    request: WorthServerCompatibilityRequestInput,
) -> WorthServerCompatibilityPreparedRequest {
    match server.compat_http().prepare_request(request) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected prepared compatibility streaming request, got {other:?}"),
    }
}

pub(crate) fn compat_stream_input(
    server: &WorthServer,
    operation_name: &str,
) -> WorthServerCompatibilityExecutionInput {
    WorthServerCompatibilityExecutionInput::new(
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
    server: &WorthServer,
    operation_name: &str,
) -> WorthServerCompatibilityExecutionInput {
    WorthServerCompatibilityExecutionInput::new(
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
    server: &WorthServer,
    operation_name: &str,
) -> WorthServerCompatibilityExecutionInput {
    WorthServerCompatibilityExecutionInput::new(
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
) -> worth_server::WorthServerCompatibilityRequestInputBuilder {
    base_input(operation_name, WorthServerCompatHttpRouteFamily::Read)
        .with_path(format!("/compat/reads/{operation_name}"))
}

pub(crate) fn stream_input(
    operation_name: &str,
) -> worth_server::WorthServerCompatibilityRequestInputBuilder {
    base_input(operation_name, WorthServerCompatHttpRouteFamily::Streaming)
        .with_path(format!("/compat/streams/{operation_name}"))
}

fn base_input(
    _operation_name: &str,
    route_family: WorthServerCompatHttpRouteFamily,
) -> worth_server::WorthServerCompatibilityRequestInputBuilder {
    WorthServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(route_family)
        .with_method("GET")
        .with_header("accept", "application/json")
}

pub(crate) fn streaming_response_success(
    response: worth_server::WorthServerCompatibilityExecutionOutcome<WorthServerStreamingResponse>,
) -> WorthServerStreamingResponse {
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

impl WorthServerQueryWorkspaceProvider for StreamingDatasetWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "streaming-dataset-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &WorthServerQueryWorkspaceBindingRequest,
    ) -> Result<WorthQueryWorkspace, WorthServerQueryWorkspaceBindingError> {
        let workspace_id = request
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .workspace_id();
        let mut workspace = WorthQueryRuntime::builder()
            .backend(StreamingDatasetRuntimeBackend::new(
                self.row_count,
                self.payload_width,
            ))
            .build()
            .map_err(|error| {
                WorthServerQueryWorkspaceBindingError::new("runtime_build", format!("{error:?}"))
            })?
            .workspace(workspace_id)
            .map_err(|error| {
                WorthServerQueryWorkspaceBindingError::new("workspace_bind", format!("{error:?}"))
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

impl WorthQueryRuntimeBackend for StreamingDatasetRuntimeBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        WorthQueryRuntimeSupportProfile::scaffold_backend_profile()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        TestSchemaAdapter.admit_live_view(name, request, schema_view)
    }

    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        Ok(WorthQueryLiveViewHandle::new(name))
    }

    fn write(
        &mut self,
        _command: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        panic!("phase four streaming runtime does not write")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        panic!("phase four streaming runtime does not write batches")
    }

    fn execute_intent(
        &mut self,
        _declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        panic!("phase four streaming runtime does not execute generic intents")
    }

    fn live_entities_for_target(
        &self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        (0..self.row_count)
            .map(|index| {
                let payload = "x".repeat(self.payload_width);
                WorthQueryEntity::from_native_field_values(
                    worth_query::facade::foundation::admit_authored_entity_token(
                        worth_query::facade::foundation::QueryExternalIdentityToken::new(
                            std::sync::Arc::from(format!("stream-row-{index}")),
                        ),
                    ),
                    std::collections::BTreeMap::from([
                        (
                            field_path("identity.id"),
                            AspectValue::String(format!("stream-row-{index}").into()),
                        ),
                        (
                            field_path("payload.value"),
                            AspectValue::String(payload.into()),
                        ),
                    ]),
                )
            })
            .collect()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        _receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        Vec::new()
    }

    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        WorthQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        )
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        let mut activation_adapter = TestSubscriptionActivation;
        let receipt = activation_adapter.admit_activation(view_name, activation)?;
        Ok(receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        _label: &WorthQuerySessionLabel,
        _effect_policy: worth_query::facade::runtime::WorthQueryEffectPolicy,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        panic!("phase four streaming runtime does not admit preview basis")
    }

    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        Ok(WorthQueryRuntimeInspectionEvidence::new(
            authority,
            "phase-four-streaming-inspection",
            receipt.authority_lane(),
            ["phase-four-streaming-runtime"],
        ))
    }
}

fn field_path(path: &str) -> CanonicalFieldPath {
    let fields = path
        .split('.')
        .map(|field| {
            FieldKey::new(field).expect("streaming runtime field segments should be foundational")
        })
        .collect::<Vec<_>>();
    CanonicalFieldPath::new(fields).expect("streaming runtime field path should be non-empty")
}

fn install_requested_named_read(
    workspace: &mut WorthQueryWorkspace,
    request: &WorthServerQueryWorkspaceBindingRequest,
) -> Result<(), WorthServerQueryWorkspaceBindingError> {
    let WorthServerQueryWorkspaceBindingTarget::DirectDeclaration { binding_label, .. } =
        request.target()
    else {
        return Ok(());
    };
    workspace
        .live_view::<Value>(binding_label, |q| {
            q.from("User")
                .select([
                    aspect_field_key("identity", "id"),
                    aspect_field_key("profile", "display_name"),
                ])
                .schema_basis("worth-server-phase-four-streaming")
        })
        .map(|_| ())
        .map_err(|error| {
            WorthServerQueryWorkspaceBindingError::new(
                "workspace_declaration",
                format!("{error:?}"),
            )
        })
}

fn aspect_field_key(aspect: &str, field: &str) -> worth_query::facade::foundation::AspectFieldKey {
    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(aspect, field)
        .expect("streaming runtime field keys should be foundational")
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

impl WorthQueryRuntimeSchemaAdapter for TestSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        let admission = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, admission))
    }
}

struct TestSubscriptionActivation;

impl WorthQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        worth_query::facade::runtime::runtime_subscription_support_evidence_identity(
            "phase-four-streaming-support",
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<
        worth_query::facade::runtime::SubscriptionActivationBoundaryReceipt,
        WorthQueryWorkspaceError,
    > {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

pub(crate) fn default_stream_selection() -> WorthServerStreamSelection {
    WorthServerStreamSelection::incremental().with_chunk_bytes(32)
}
