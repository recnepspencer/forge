use worth_query::facade::foundation::{DeclarativeLiveQueryRequest, WorthQueryEntity, WorthQueryLivePatch, WorthQueryLiveViewHandle, WorthQueryMutationReceipt, WorthQueryWorkspaceError};
use worth_query::facade::policy::WorthQueryDerivedView;
use worth_query::facade::runtime::{WorthQueryEffectPolicy, WorthQueryIntentDeclaration, WorthQueryIntentExecution, WorthQueryLiveArtifactTarget, WorthQueryPreviewBasisAdmission, WorthQueryBackendAdmissibleMutation, WorthQueryRuntimeBackend, WorthQueryRuntimeEvidenceAuthority, WorthQueryRuntimeError, WorthQueryRuntimeInspectionEvidence, WorthQueryRuntimeSupportProfile, WorthQuerySessionLabel, WorthQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput, SubscriptionActivationReceipt};

struct StringSnapshotBackend;

impl WorthQueryRuntimeBackend for StringSnapshotBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        panic!("not executed")
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn close_live_view(&mut self, _name: &str) -> Result<(), WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn write(
        &mut self,
        _command: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn execute_intent(
        &mut self,
        _declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        panic!("not executed")
    }

    fn live_entities_for_target(
        &self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &WorthQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn admit_preview_basis(
        &self,
        _label: &WorthQuerySessionLabel,
        _effect_policy: WorthQueryEffectPolicy,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &WorthQueryWriteReceipt,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn declaration_initialization_metadata(
        &self,
        _view: &WorthQueryDerivedView,
    ) -> Result<worth_query::facade::runtime::WorthQueryMutationMetadata, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn snapshot_token(&self) -> String {
        "snapshot-1".to_string()
    }
}

fn main() {}
