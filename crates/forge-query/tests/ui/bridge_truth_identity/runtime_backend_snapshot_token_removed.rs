use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryDerivedView, ForgeQueryEffectPolicy,
    ForgeQueryEntity, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeBackend, ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeError,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSupportProfile, ForgeQuerySessionLabel,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput,
    SubscriptionActivationReceipt,
};

struct StringSnapshotBackend;

impl ForgeQueryRuntimeBackend for StringSnapshotBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        panic!("not executed")
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn write(
        &mut self,
        _command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("not executed")
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

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn admit_preview_basis(
        &self,
        _label: &ForgeQuerySessionLabel,
        _effect_policy: ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &ForgeQueryWriteReceipt,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn declaration_initialization_metadata(
        &self,
        _view: &ForgeQueryDerivedView,
    ) -> Result<forge_query::facade::ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn snapshot_token(&self) -> String {
        "snapshot-1".to_string()
    }
}

fn main() {}
