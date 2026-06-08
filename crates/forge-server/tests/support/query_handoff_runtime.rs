use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntime, ForgeQueryRuntimeBackend, ForgeQueryRuntimeError,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspace, ForgeQueryWorkspaceError,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt,
    QuerySchemaView, SubscriptionActivationInput, SubscriptionActivationReceipt,
};
use forge_server::{
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceProvider,
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
        ForgeQueryRuntime::builder()
            .backend(TestQueryRuntimeBackend::default())
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
        ForgeQueryRuntime::builder()
            .backend(TestQueryRuntimeBackend::new(self.support_profile.clone()))
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
struct TestQueryRuntimeBackend {
    support_profile: ForgeQueryRuntimeSupportProfile,
}

impl Default for TestQueryRuntimeBackend {
    fn default() -> Self {
        Self::new(ForgeQueryRuntimeSupportProfile::scaffold_backend_profile())
    }
}

impl TestQueryRuntimeBackend {
    fn new(support_profile: ForgeQueryRuntimeSupportProfile) -> Self {
        Self { support_profile }
    }
}

impl ForgeQueryRuntimeBackend for TestQueryRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &forge_query::facade::DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: forge_query::facade::DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }

    fn write(
        &mut self,
        _command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("unused in query handoff phase tests")
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
        "query-handoff-phase-test-snapshot".to_string()
    }

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }

    fn admit_preview_basis(
        &self,
        _label: &str,
        _effect_policy: forge_query::facade::ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &ForgeQueryWriteReceipt,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        panic!("unused in query handoff phase tests")
    }
}
