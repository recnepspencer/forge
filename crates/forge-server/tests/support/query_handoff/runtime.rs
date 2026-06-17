#![allow(dead_code)]

use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEntity, ForgeQueryIntentDeclaration,
    ForgeQueryIntentExecution, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime,
    ForgeQueryRuntimeBackend, ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQueryRuntimeSupportProfile,
    ForgeQuerySessionLabel, ForgeQuerySnapshotIdentity, ForgeQueryWorkspace,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput,
    SubscriptionActivationReceipt,
};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

#[path = "runtime_mutation_support.rs"]
mod runtime_mutation_support;

use forge_server::{
    ForgeServerDirectDeclarationSourceKind, ForgeServerQueryWorkspaceBindingError,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerQueryWorkspaceBindingTarget,
    ForgeServerQueryWorkspaceProvider,
};
use runtime_mutation_support::{test_mutation_receipt, TestSubscriptionActivation};
use serde_json::json;
use std::collections::BTreeSet;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct TestWorkspaceProvider;

impl ForgeServerQueryWorkspaceProvider for TestWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "test-workspace-provider"
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
            .backend(TestQueryRuntimeBackend::default())
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

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct ProfiledTestWorkspaceProvider {
    support_profile: ForgeQueryRuntimeSupportProfile,
}

#[allow(dead_code)]
impl ProfiledTestWorkspaceProvider {
    pub(crate) fn new(support_profile: ForgeQueryRuntimeSupportProfile) -> Self {
        Self { support_profile }
    }
}

impl ForgeServerQueryWorkspaceProvider for ProfiledTestWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "profiled-test-workspace-provider"
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
            .backend(TestQueryRuntimeBackend::new(self.support_profile.clone()))
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

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct ProfiledCountingTestWorkspaceProvider {
    support_profile: ForgeQueryRuntimeSupportProfile,
    attempted_writes: Arc<AtomicUsize>,
}

#[allow(dead_code)]
impl ProfiledCountingTestWorkspaceProvider {
    pub(crate) fn new(
        support_profile: ForgeQueryRuntimeSupportProfile,
        attempted_writes: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            support_profile,
            attempted_writes,
        }
    }
}

impl ForgeServerQueryWorkspaceProvider for ProfiledCountingTestWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "profiled-counting-test-workspace-provider"
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
            .backend(TestQueryRuntimeBackend::new_with_attempted_writes(
                self.support_profile.clone(),
                self.attempted_writes.clone(),
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
pub(crate) struct PanicOnReadTestWorkspaceProvider;

impl ForgeServerQueryWorkspaceProvider for PanicOnReadTestWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "panic-on-read-test-workspace-provider"
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
            .backend(TestQueryRuntimeBackend::new_panicking_on_live_reads())
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
struct TestQueryRuntimeBackend {
    support_profile: ForgeQueryRuntimeSupportProfile,
    declared_live_views: BTreeSet<String>,
    attempted_writes: Option<Arc<AtomicUsize>>,
    panic_on_live_reads: bool,
}

impl Default for TestQueryRuntimeBackend {
    fn default() -> Self {
        Self::new(ForgeQueryRuntimeSupportProfile::scaffold_backend_profile())
    }
}

impl TestQueryRuntimeBackend {
    fn new(support_profile: ForgeQueryRuntimeSupportProfile) -> Self {
        Self {
            support_profile,
            declared_live_views: BTreeSet::new(),
            attempted_writes: None,
            panic_on_live_reads: false,
        }
    }

    #[allow(dead_code)]
    fn new_with_attempted_writes(
        support_profile: ForgeQueryRuntimeSupportProfile,
        attempted_writes: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            support_profile,
            declared_live_views: BTreeSet::new(),
            attempted_writes: Some(attempted_writes),
            panic_on_live_reads: false,
        }
    }

    fn new_panicking_on_live_reads() -> Self {
        Self {
            support_profile: ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
            declared_live_views: BTreeSet::new(),
            attempted_writes: None,
            panic_on_live_reads: true,
        }
    }

    fn record_attempted_write(&self, count: usize) {
        if let Some(attempted_writes) = &self.attempted_writes {
            attempted_writes.fetch_add(count, Ordering::Relaxed);
        }
    }
}
impl ForgeQueryRuntimeBackend for TestQueryRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
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
        self.declared_live_views.insert(name.clone());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.record_attempted_write(1);
        Ok(test_mutation_receipt(&command, 1))
    }

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        self.record_attempted_write(commands.len());
        Ok(commands
            .iter()
            .enumerate()
            .map(|(index, command)| test_mutation_receipt(command, index + 1))
            .collect())
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("unused in query handoff phase tests")
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        assert!(
            !self.panic_on_live_reads,
            "live entity reads must not execute for this hostile denial seam"
        );
        if !self.declared_live_views.contains(_view_name) {
            return Vec::new();
        }

        vec![ForgeQueryEntity::from_external_projection(
            forge_query::facade::admit_authored_entity_token(
                forge_query::facade::QueryExternalIdentityToken::new(std::sync::Arc::from(
                    "user-1",
                )),
            ),
            json!({
                "identity": { "id": "user-1" },
                "profile": { "display_name": "Ada Forge" }
            }),
        )]
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
        panic!("unused in query handoff phase tests")
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "query-handoff-phase-test-write-receipt",
            receipt.authority_lane(),
            ["query-handoff-phase-test-inspector"],
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

    if binding_label.ends_with(".missing") {
        return Ok(());
    }

    workspace
        .live_view::<serde_json::Value>(binding_label, |q| {
            q.from("User")
                .select(["identity.id", "profile.display_name"])
                .schema_basis("forge-server-test-named-read")
        })
        .map(|_| ())
        .map_err(|error| {
            ForgeServerQueryWorkspaceBindingError::new(
                "workspace_declaration",
                format!("{error:?}"),
            )
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
