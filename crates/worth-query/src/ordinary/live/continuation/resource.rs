use std::sync::Arc;

use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryLiveView, WorthQueryManagedLiveWorkspaceCapability, WorthQueryNativeRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveCheckpointReceipt {
    resource_name: String,
    installation_identity: WorthQueryEvidenceIdentity,
    basis_binding_identity: WorthQueryEvidenceIdentity,
    last_delivery_sequence: Option<u64>,
    continuation_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryManagedLiveCheckpointReceipt {
    pub(super) fn new(
        resource_name: &str,
        installation_identity: &WorthQueryEvidenceIdentity,
        basis_binding_identity: &WorthQueryEvidenceIdentity,
        last_delivery_sequence: Option<u64>,
    ) -> Self {
        let continuation_identity = continuation_identity(
            resource_name,
            installation_identity,
            basis_binding_identity,
            last_delivery_sequence,
        );
        Self {
            resource_name: resource_name.to_string(),
            installation_identity: installation_identity.clone(),
            basis_binding_identity: basis_binding_identity.clone(),
            last_delivery_sequence,
            continuation_identity,
        }
    }

    pub fn resource_name(&self) -> &str {
        &self.resource_name
    }

    pub fn installation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.installation_identity
    }

    pub fn basis_binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn last_delivery_sequence(&self) -> Option<u64> {
        self.last_delivery_sequence
    }

    pub fn continuation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.continuation_identity
    }
}

#[derive(Debug)]
#[must_use = "a managed live continuation owns an active Query resource until resumed or dropped"]
pub struct WorthQueryManagedLiveContinuation {
    view: Option<WorthQueryLiveView<WorthQueryNativeRow>>,
    workspace_capability: Arc<WorthQueryManagedLiveWorkspaceCapability>,
    checkpoint: WorthQueryManagedLiveCheckpointReceipt,
}

impl WorthQueryManagedLiveContinuation {
    pub(super) fn new(
        view: WorthQueryLiveView<WorthQueryNativeRow>,
        workspace_capability: Arc<WorthQueryManagedLiveWorkspaceCapability>,
        checkpoint: WorthQueryManagedLiveCheckpointReceipt,
    ) -> Self {
        Self {
            view: Some(view),
            workspace_capability,
            checkpoint,
        }
    }

    pub fn checkpoint(&self) -> &WorthQueryManagedLiveCheckpointReceipt {
        &self.checkpoint
    }

    pub(super) fn view(&self) -> &WorthQueryLiveView<WorthQueryNativeRow> {
        self.view
            .as_ref()
            .expect("active continuation must retain its managed live view")
    }

    pub(super) fn workspace_capability(&self) -> &Arc<WorthQueryManagedLiveWorkspaceCapability> {
        &self.workspace_capability
    }

    pub(super) fn into_resource_parts(
        mut self,
    ) -> (
        WorthQueryLiveView<WorthQueryNativeRow>,
        Arc<WorthQueryManagedLiveWorkspaceCapability>,
    ) {
        let view = self
            .view
            .take()
            .expect("resumed continuation must retain its managed live view");
        (view, Arc::clone(&self.workspace_capability))
    }
}

impl Drop for WorthQueryManagedLiveContinuation {
    fn drop(&mut self) {
        if let Some(view) = self.view.take() {
            self.workspace_capability.abandon(view);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveResumeReceipt {
    continuation_identity: WorthQueryEvidenceIdentity,
    resumed_delivery_sequence: Option<u64>,
    queued_delivery_count: u64,
    resume_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryManagedLiveResumeReceipt {
    pub(super) fn new(
        checkpoint: &WorthQueryManagedLiveCheckpointReceipt,
        resumed_delivery_sequence: Option<u64>,
    ) -> Self {
        let queued_delivery_count = resumed_delivery_sequence
            .unwrap_or(0)
            .saturating_sub(checkpoint.last_delivery_sequence().unwrap_or(0));
        let resume_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_managed_live_resume_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("continuation"),
            checkpoint.continuation_identity(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("resumed_delivery_sequence"),
            resumed_delivery_sequence
                .map(|sequence| sequence.to_string())
                .unwrap_or_else(|| "none".to_string()),
        )
        .seal();
        Self {
            continuation_identity: checkpoint.continuation_identity().clone(),
            resumed_delivery_sequence,
            queued_delivery_count,
            resume_identity,
        }
    }

    pub fn continuation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.continuation_identity
    }

    pub fn resumed_delivery_sequence(&self) -> Option<u64> {
        self.resumed_delivery_sequence
    }

    pub fn queued_delivery_count(&self) -> u64 {
        self.queued_delivery_count
    }

    pub fn resume_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.resume_identity
    }
}

fn continuation_identity(
    resource_name: &str,
    installation_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    last_delivery_sequence: Option<u64>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_managed_live_continuation_v1",
        )
        .field_value(WorthQueryEvidenceTag::new("resource_name"), resource_name)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("installation"),
            installation_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("basis_binding"),
            basis_binding_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("last_delivery_sequence"),
            last_delivery_sequence
                .map(|sequence| sequence.to_string())
                .unwrap_or_else(|| "none".to_string()),
        )
        .seal()
}
