use crate::facade::{
    foundation::{
        DeclarativeLiveQueryRequest, WorthQueryEntity, WorthQueryLivePatch,
        WorthQueryLiveViewHandle, WorthQueryMutationReceipt, WorthQuerySnapshotIdentity,
        WorthQueryWorkspaceError,
    },
    runtime::{
        LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView, SubscriptionActivationInput,
        SubscriptionActivationReceipt, WorthQueryBackendAdmissibleMutation, WorthQueryEffectPolicy,
        WorthQueryIntentDeclaration, WorthQueryIntentExecution, WorthQueryLiveArtifactTarget,
        WorthQueryPreviewBasisAdmission, WorthQueryRuntimeBackend, WorthQueryRuntimeError,
        WorthQueryRuntimeEvidenceAuthority, WorthQueryRuntimeFacadeFamily,
        WorthQueryRuntimeFamilySupport, WorthQueryRuntimeInspectionEvidence,
        WorthQueryRuntimeSupportProfile, WorthQuerySessionLabel, WorthQueryWriteReceipt,
    },
};

use super::WorthQueryRuntimeBuilder;

const DECLARATION_AUTHORITY_DENIAL: &str =
    "declaration-authority runtimes do not own query execution";

impl WorthQueryRuntimeBuilder {
    /// Selects a runtime posture that installs domain packages and mints
    /// declaration contexts without claiming query execution authority.
    pub fn declaration_authority_backend(self) -> Self {
        self.backend(WorthQueryDeclarationAuthorityBackend)
    }
}

struct WorthQueryDeclarationAuthorityBackend;

impl WorthQueryRuntimeBackend for WorthQueryDeclarationAuthorityBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        WorthQueryRuntimeSupportProfile::new(WorthQueryRuntimeFacadeFamily::ALL.into_iter().map(
            |family| {
                WorthQueryRuntimeFamilySupport::unsupported(family, DECLARATION_AUTHORITY_DENIAL)
            },
        ))
    }

    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        WorthQuerySnapshotIdentity::empty_relational_state()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn close_live_view(&mut self, _name: &str) -> Result<(), WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn write(
        &mut self,
        _mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn write_batch(
        &mut self,
        _mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn execute_intent(
        &mut self,
        _declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        Err(WorthQueryRuntimeError::Workspace(
            declaration_authority_denial(),
        ))
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

    fn affected_live_view_targets(
        &self,
        _receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        Vec::new()
    }

    fn install_live_subscription(
        &mut self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn admit_preview_basis(
        &self,
        _label: &WorthQuerySessionLabel,
        _effect_policy: WorthQueryEffectPolicy,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &WorthQueryWriteReceipt,
        _authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        Err(declaration_authority_denial())
    }
}

fn declaration_authority_denial() -> WorthQueryWorkspaceError {
    WorthQueryWorkspaceError::new(DECLARATION_AUTHORITY_DENIAL)
}
