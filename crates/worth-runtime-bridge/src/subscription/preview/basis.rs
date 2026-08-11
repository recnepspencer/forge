use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::input::envelope::TruthBranchIdentity;
use crate::snapshot::TruthSnapshotIdentity;
use crate::speculation::{
    BridgePreviewExecutionRecord, BridgePreviewLifecycleStateKind, BridgePreviewSession,
    BridgePreviewSessionIdentity, PreviewActive, PreviewExecutionRecordIdentity,
};

use super::super::{
    BridgeSubscriptionCounters, BridgeSubscriptionPreviewBasisIdentity,
    BridgeSubscriptionPreviewLifecycleIdentity, BridgeSubscriptionPreviewParentBasisIdentity,
    BridgeSubscriptionPreviewResidueScopeIdentity, BridgeSubscriptionPreviewScopeIdentity,
};
use super::rejection::{
    BridgeSubscriptionPreviewBasisRejection, BridgeSubscriptionPreviewBasisRejectionContext,
    BridgeSubscriptionPreviewBasisRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewBasisBinding {
    pub(super) preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity,
    pub(super) preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity,
    pub(super) preview_parent_basis_identity: BridgeSubscriptionPreviewParentBasisIdentity,
    pub(super) preview_lifecycle_identity: BridgeSubscriptionPreviewLifecycleIdentity,
    pub(super) preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    pub(super) preview_session_identity: BridgePreviewSessionIdentity,
    pub(super) preview_execution_record_identity: PreviewExecutionRecordIdentity,
    truth_branch_identity: TruthBranchIdentity,
    truth_snapshot_identity: TruthSnapshotIdentity,
    preview_declaration_digest: Arc<str>,
    pub(super) branch_binding_digest: Arc<str>,
    pub(super) parent_truth_view_basis_digest: Arc<str>,
    pub(super) preview_lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    pub(super) active_preview_session_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    pub(super) digest: Arc<str>,
}

impl BridgeSubscriptionPreviewBasisBinding {
    pub(crate) fn admit(
        active_preview_session: &BridgePreviewSession<PreviewActive>,
        preview_execution_record: &BridgePreviewExecutionRecord,
    ) -> Result<Self, BridgeSubscriptionPreviewBasisRejection> {
        let session_execution_identity = active_preview_session
            .execution_record_identity()
            .expect("active preview sessions must carry execution record identity");
        if preview_execution_record.record_identity() != session_execution_identity {
            return Err(BridgeSubscriptionPreviewBasisRejection::new(
                BridgeSubscriptionPreviewBasisRejectionKind::PreviewExecutionRecordMismatch,
                BridgeSubscriptionPreviewBasisRejectionContext::session_execution_record_mismatch(
                    active_preview_session.session_identity().clone(),
                    session_execution_identity.clone(),
                    preview_execution_record.record_identity().clone(),
                ),
            ));
        }
        if preview_execution_record.preview_session_identity()
            != active_preview_session.session_identity().as_str()
        {
            return Err(BridgeSubscriptionPreviewBasisRejection::new(
                BridgeSubscriptionPreviewBasisRejectionKind::PreviewExecutionRecordMismatch,
                BridgeSubscriptionPreviewBasisRejectionContext::execution_record_session_mismatch(
                    active_preview_session.session_identity().clone(),
                    preview_execution_record.preview_session_identity(),
                    preview_execution_record.record_identity().clone(),
                ),
            ));
        }
        if preview_execution_record.preview_declaration_digest()
            != active_preview_session.declaration().digest()
        {
            return Err(BridgeSubscriptionPreviewBasisRejection::new(
                BridgeSubscriptionPreviewBasisRejectionKind::PreviewDeclarationDigestMismatch,
                BridgeSubscriptionPreviewBasisRejectionContext::declaration_digest_mismatch(
                    active_preview_session.session_identity().clone(),
                    active_preview_session.declaration().digest(),
                    preview_execution_record.preview_declaration_digest(),
                ),
            ));
        }

        let declaration = active_preview_session.declaration().declaration();
        let truth_view_selector = declaration.session_basis().truth_view_selector();
        let truth_snapshot_identity = truth_view_selector
            .snapshot_identity()
            .expect("preview session basis must carry explicit snapshot identity")
            .clone();
        let preview_scope_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-scope|session={}|signal-branch={}",
            active_preview_session.session_identity().as_str(),
            declaration
                .branch_binding()
                .signal_branch_identity()
                .as_str(),
        ));
        let preview_scope_digest = Sha256::digest(preview_scope_basis.as_bytes());
        let preview_parent_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-parent-basis|truth-branch={}|branch-binding={}|truth-view={}|structural-basis={}",
            declaration.branch_binding().truth_branch_identity().as_str(),
            declaration.branch_binding().digest(),
            declaration.truth_view_basis_digest(),
            declaration.structural_basis_digest().unwrap_or("none"),
        ));
        let preview_parent_digest = Sha256::digest(preview_parent_basis.as_bytes());
        let preview_lifecycle_state_kind = active_preview_session.lifecycle_state_kind();
        let preview_lifecycle_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-lifecycle|session={}|state={preview_lifecycle_state_kind:?}|execution-record={}|active-preview={}",
            active_preview_session.session_identity().as_str(),
            preview_execution_record.record_identity().as_str(),
            active_preview_session.digest(),
        ));
        let preview_lifecycle_digest = Sha256::digest(preview_lifecycle_basis.as_bytes());
        let preview_residue_scope_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-residue-scope|session={}|branch-binding={}|artifact-schema={}",
            active_preview_session.session_identity().as_str(),
            declaration.branch_binding().digest(),
            declaration.retained_artifact_schema_digest(),
        ));
        let preview_residue_scope_digest = Sha256::digest(preview_residue_scope_basis.as_bytes());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-basis|scope=bridge-subscription-preview-scope-id:sha256:{preview_scope_digest:x}|parent-basis=bridge-subscription-preview-parent-basis-id:sha256:{preview_parent_digest:x}|preview-lifecycle=bridge-subscription-preview-lifecycle-id:sha256:{preview_lifecycle_digest:x}|residue-scope=bridge-subscription-preview-residue-scope-id:sha256:{preview_residue_scope_digest:x}|session={}|execution-record={}|declaration={}|branch-binding={}|truth-view={}|active-preview={}",
            active_preview_session.session_identity().as_str(),
            preview_execution_record.record_identity().as_str(),
            active_preview_session.declaration().digest(),
            declaration.branch_binding().digest(),
            declaration.truth_view_basis_digest(),
            active_preview_session.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity::admit_bridge_owned(
                format!("bridge-subscription-preview-basis-id:sha256:{digest:x}"),
            ),
            preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity::admit_bridge_owned(
                format!("bridge-subscription-preview-scope-id:sha256:{preview_scope_digest:x}"),
            ),
            preview_parent_basis_identity:
                BridgeSubscriptionPreviewParentBasisIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-preview-parent-basis-id:sha256:{preview_parent_digest:x}"
                )),
            preview_lifecycle_identity:
                BridgeSubscriptionPreviewLifecycleIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-preview-lifecycle-id:sha256:{preview_lifecycle_digest:x}"
                )),
            preview_residue_scope_identity:
                BridgeSubscriptionPreviewResidueScopeIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-preview-residue-scope-id:sha256:{preview_residue_scope_digest:x}"
                )),
            preview_session_identity: active_preview_session.session_identity().clone(),
            preview_execution_record_identity: preview_execution_record.record_identity().clone(),
            truth_branch_identity: truth_view_selector.branch_identity().clone(),
            truth_snapshot_identity,
            preview_declaration_digest: Arc::from(active_preview_session.declaration().digest()),
            branch_binding_digest: Arc::from(declaration.branch_binding().digest()),
            parent_truth_view_basis_digest: Arc::from(declaration.truth_view_basis_digest()),
            preview_lifecycle_state_kind,
            active_preview_session_digest: Arc::from(active_preview_session.digest()),
            counters: BridgeSubscriptionCounters::from_subscription_preview_basis_admission(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-preview-basis:sha256:{digest:x}")),
        })
    }

    pub fn preview_basis_identity(&self) -> &BridgeSubscriptionPreviewBasisIdentity {
        &self.preview_basis_identity
    }
    pub fn preview_scope_identity(&self) -> &BridgeSubscriptionPreviewScopeIdentity {
        &self.preview_scope_identity
    }
    pub fn preview_parent_basis_identity(&self) -> &BridgeSubscriptionPreviewParentBasisIdentity {
        &self.preview_parent_basis_identity
    }
    pub fn preview_lifecycle_identity(&self) -> &BridgeSubscriptionPreviewLifecycleIdentity {
        &self.preview_lifecycle_identity
    }
    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }
    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.preview_session_identity
    }
    pub fn preview_execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.preview_execution_record_identity
    }
    pub fn truth_branch_identity(&self) -> &TruthBranchIdentity {
        &self.truth_branch_identity
    }
    pub fn truth_snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.truth_snapshot_identity
    }
    pub fn preview_declaration_digest(&self) -> &str {
        self.preview_declaration_digest.as_ref()
    }
    pub fn branch_binding_digest(&self) -> &str {
        self.branch_binding_digest.as_ref()
    }
    pub fn parent_truth_view_basis_digest(&self) -> &str {
        self.parent_truth_view_basis_digest.as_ref()
    }
    pub fn preview_lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.preview_lifecycle_state_kind
    }
    pub fn active_preview_session_digest(&self) -> &str {
        self.active_preview_session_digest.as_ref()
    }
    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
