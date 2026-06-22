use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError,
};
use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime, ForgeQueryRuntimeBackend,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspace,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt,
    SubscriptionActivationReceipt,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::ForgeQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

pub(in super::super::tests) fn live_support_matrix() -> ForgeQueryRuntimePublicSupportMatrix {
    support_snapshot_workspace().public_support_matrix()
}

pub(in super::super::tests) fn support_snapshot_workspace() -> ForgeQueryWorkspace {
    ForgeQueryRuntime::builder()
        .backend(SupportSnapshotRuntimeBackend::new(primary_support_profile()))
        .build()
        .expect("support snapshot runtime should build")
        .workspace("support-snapshot-real-workspace")
        .expect("support snapshot workspace should build")
}

pub(in super::super::tests) fn scaffold_support_matrix() -> ForgeQueryRuntimePublicSupportMatrix {
    ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(
        &ForgeQueryRuntimePublicApiContract::from_support_profile(
            &ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
        ),
    )
}

fn primary_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::scaffold_backend_profile()
        .with_posture(ForgeQueryRuntimeBackendPosture::Primary)
}

struct SupportSnapshotRuntimeBackend {
    support_profile: ForgeQueryRuntimeSupportProfile,
}

impl SupportSnapshotRuntimeBackend {
    fn new(support_profile: ForgeQueryRuntimeSupportProfile) -> Self {
        Self { support_profile }
    }
}

impl ForgeQueryRuntimeBackend for SupportSnapshotRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn write(
        &mut self,
        _command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn write_batch(
        &mut self,
        _commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn admit_preview_basis(
        &self,
        _label: &ForgeQuerySessionLabel,
        _effect_policy: ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &ForgeQueryWriteReceipt,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }
}
