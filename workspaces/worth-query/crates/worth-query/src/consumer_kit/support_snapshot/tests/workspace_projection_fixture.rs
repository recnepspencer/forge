use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    WorthQueryEntity, WorthQueryLivePatch, WorthQueryLiveViewHandle, WorthQueryMutationReceipt,
    WorthQueryWorkspaceError,
};
use crate::runtime::{
    LiveViewDeclarationAdmissionBoundaryReceipt, SubscriptionActivationReceipt,
    WorthQueryBackendAdmissibleMutation, WorthQueryEffectPolicy, WorthQueryIntentDeclaration,
    WorthQueryIntentExecution, WorthQueryLiveArtifactTarget, WorthQueryPreviewBasisAdmission,
    WorthQueryRuntime, WorthQueryRuntimeBackend, WorthQueryRuntimeBackendPosture,
    WorthQueryRuntimeError, WorthQueryRuntimeEvidenceAuthority,
    WorthQueryRuntimeInspectionEvidence, WorthQueryRuntimePublicApiContract,
    WorthQueryRuntimePublicSupportMatrix, WorthQueryRuntimeSupportProfile, WorthQueryWorkspace,
    WorthQueryWriteReceipt,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::WorthQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

pub(in super::super::tests) fn live_support_matrix() -> WorthQueryRuntimePublicSupportMatrix {
    support_snapshot_workspace().public_support_matrix()
}

pub(in super::super::tests) fn support_snapshot_workspace() -> WorthQueryWorkspace {
    WorthQueryRuntime::builder()
        .backend(SupportSnapshotRuntimeBackend::new(primary_support_profile()))
        .build()
        .expect("support snapshot runtime should build")
        .workspace("support-snapshot-real-workspace")
        .expect("support snapshot workspace should build")
}

pub(in super::super::tests) fn scaffold_support_matrix() -> WorthQueryRuntimePublicSupportMatrix {
    WorthQueryRuntimePublicSupportMatrix::from_public_api_contract(
        &WorthQueryRuntimePublicApiContract::from_support_profile(
            &WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
        ),
    )
}

fn primary_support_profile() -> WorthQueryRuntimeSupportProfile {
    WorthQueryRuntimeSupportProfile::scaffold_backend_profile()
        .with_posture(WorthQueryRuntimeBackendPosture::Primary)
}

struct SupportSnapshotRuntimeBackend {
    support_profile: WorthQueryRuntimeSupportProfile,
}

impl SupportSnapshotRuntimeBackend {
    fn new(support_profile: WorthQueryRuntimeSupportProfile) -> Self {
        Self { support_profile }
    }
}

impl crate::runtime::WorthQueryMergeSnapshotOwner for SupportSnapshotRuntimeBackend {}

impl crate::runtime::WorthQuerySettlementRecoveryBackend for SupportSnapshotRuntimeBackend {}

impl WorthQueryRuntimeBackend for SupportSnapshotRuntimeBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn close_live_view(&mut self, _name: &str) -> Result<(), WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn write(
        &mut self,
        _mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn write_batch(
        &mut self,
        _mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn execute_intent(
        &mut self,
        _declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn live_entities_for_target(
        &self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn affected_live_view_targets(
        &self,
        _receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn admit_preview_basis(
        &self,
        _label: &WorthQuerySessionLabel,
        _effect_policy: WorthQueryEffectPolicy,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &WorthQueryWriteReceipt,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        panic!("support snapshot tests only exercise workspace support matrix projection")
    }
}
