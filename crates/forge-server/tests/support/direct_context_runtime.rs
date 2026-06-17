use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEntity, ForgeQueryEvidenceIdentity, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQueryRuntime, ForgeQueryRuntimeBackend, ForgeQueryRuntimeError,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeRemaskProjection, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQueryRuntimeSupportProfile,
    ForgeQuerySessionLabel, ForgeQuerySnapshotIdentity, ForgeQueryWorkspace,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput,
    SubscriptionActivationReceipt,
};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;
use forge_server::{
    ForgeServerDirectDeclarationSourceKind, ForgeServerQueryWorkspaceBindingError,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerQueryWorkspaceBindingTarget,
    ForgeServerQueryWorkspaceProvider,
};
use serde_json::json;

#[derive(Clone, Debug)]
pub(crate) struct RemaskWorkspaceProvider {
    projection: ForgeQueryRuntimeRemaskProjection,
    support_profile: ForgeQueryRuntimeSupportProfile,
}

impl RemaskWorkspaceProvider {
    pub(crate) fn new(projection: ForgeQueryRuntimeRemaskProjection) -> Self {
        Self {
            projection,
            support_profile: ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
        }
    }
}

impl ForgeServerQueryWorkspaceProvider for RemaskWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "remask-direct-context-workspace-provider"
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
            .backend(RemaskRuntimeBackend::new(
                self.support_profile.clone(),
                self.projection.clone(),
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
struct RemaskRuntimeBackend {
    support_profile: ForgeQueryRuntimeSupportProfile,
    projection: ForgeQueryRuntimeRemaskProjection,
    declared_live_views: std::collections::BTreeSet<String>,
}

impl RemaskRuntimeBackend {
    fn new(
        support_profile: ForgeQueryRuntimeSupportProfile,
        projection: ForgeQueryRuntimeRemaskProjection,
    ) -> Self {
        Self {
            support_profile,
            projection,
            declared_live_views: std::collections::BTreeSet::new(),
        }
    }
}

impl ForgeQueryRuntimeBackend for RemaskRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        DirectContextSchemaAdapter.admit_live_view(name, request, schema_view)
    }

    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.declared_live_views.insert(name.clone());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn write(
        &mut self,
        _command: ForgeQueryWriteCommand,
    ) -> Result<forge_query::facade::ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        panic!("unused in direct context remask tests")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<forge_query::facade::ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        panic!("unused in direct context remask tests")
    }

    fn execute_intent(
        &mut self,
        _declaration: &forge_query::facade::ForgeQueryIntentDeclaration,
    ) -> Result<forge_query::facade::ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("unused in direct context remask tests")
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        self.declared_live_views.contains(view_name).then(|| {
            vec![ForgeQueryEntity::from_external_projection(
                forge_query::facade::admit_authored_entity_token(
                    forge_query::facade::QueryExternalIdentityToken::new(
                        std::sync::Arc::from("user-1"),
                    ),
                ),
                json!({ "identity": { "id": "user-1" }, "profile": { "display_name": "Ada Forge" } }),
            )]
        }).unwrap_or_default()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(
        &self,
        _receipt: &forge_query::facade::ForgeQueryMutationReceipt,
    ) -> Vec<String> {
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
        let mut activation_adapter = DirectContextRemaskActivation {
            projection: self.projection.clone(),
        };
        let receipt = activation_adapter.admit_activation(view_name, activation)?;
        Ok(receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        _label: &ForgeQuerySessionLabel,
        _effect_policy: forge_query::facade::ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<forge_query::facade::ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError>
    {
        panic!("unused in direct context remask tests")
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "direct-context-remask-write-receipt",
            receipt.authority_lane(),
            ["direct-context-remask-inspector"],
        ))
    }
}

fn install_requested_named_read(
    workspace: &mut ForgeQueryWorkspace,
    request: &ForgeServerQueryWorkspaceBindingRequest,
) -> Result<(), ForgeServerQueryWorkspaceBindingError> {
    let ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration {
        source_kind: ForgeServerDirectDeclarationSourceKind::NamedRead,
        binding_label,
    } = request.target()
    else {
        return Ok(());
    };

    workspace
        .live_view::<serde_json::Value>(binding_label, |q| {
            q.from("User")
                .select(["identity.id", "profile.display_name"])
                .schema_basis("forge-server-direct-context-remask")
        })
        .map(|_| ())
        .map_err(|error| {
            ForgeServerQueryWorkspaceBindingError::new(
                "workspace_declaration",
                format!("{error:?}"),
            )
        })
}

struct DirectContextSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for DirectContextSchemaAdapter {
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

struct DirectContextRemaskActivation {
    projection: ForgeQueryRuntimeRemaskProjection,
}

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DirectContextRemaskActivation {
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query::facade::runtime_subscription_support_evidence_identity(
            "direct-context-remask-support",
        )
    }

    fn remask_projection(
        &self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Option<ForgeQueryRuntimeRemaskProjection> {
        Some(self.projection.clone())
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
