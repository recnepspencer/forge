use super::*;
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    Insert {
        collection: String,
        payload: Value,
    },
    UpdateAspect {
        entity_identity: String,
        aspect_path: String,
        value: Value,
    },
    Delete {
        entity_identity: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    pub(super) inner: ForgeQueryMutationReceipt,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) affected_live_view_ids: Vec<String>,
    pub(super) affected_derived_view_ids: Vec<String>,
    pub(super) considered_computed_view_count: usize,
    pub(super) considered_effect_count: usize,
    pub(super) delivered_effect_count: usize,
    pub(super) pending_write_intent_count: usize,
    pub(super) suppressed_effect_count: usize,
    pub(super) meaningful_effect_suppression_count: usize,
    pub(super) effect_expression_failure_count: usize,
    pub(super) refresh_fallback: bool,
}

impl ForgeQueryWriteReceipt {
    pub(super) fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
        affected_live_view_ids: Vec<String>,
        affected_derived_view_ids: Vec<String>,
        considered_computed_view_count: usize,
        considered_effect_count: usize,
        delivered_effect_count: usize,
        pending_write_intent_count: usize,
        suppressed_effect_count: usize,
        meaningful_effect_suppression_count: usize,
        effect_expression_failure_count: usize,
        refresh_fallback: bool,
    ) -> Self {
        Self {
            inner,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        }
    }

    pub(super) fn preview(
        label: &str,
        sequence: usize,
        command: &ForgeQueryWriteCommand,
        snapshot_token: String,
    ) -> Self {
        let delta = match command {
            ForgeQueryWriteCommand::Insert {
                collection,
                payload: _,
            } => crate::memory_workspace::ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: format!("preview:{label}:{sequence}"),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: Vec::new(),
            },
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value: _,
            } => crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec![aspect_path.clone()],
            },
            ForgeQueryWriteCommand::Delete { entity_identity } => {
                crate::memory_workspace::ForgeQueryMutationDelta {
                    collection: "preview".to_string(),
                    entity_identity: entity_identity.clone(),
                    kind: ForgeQueryMutationKind::Deleted,
                    aspect_paths: Vec::new(),
                }
            }
        };
        Self {
            inner: ForgeQueryMutationReceipt {
                commit_identity: format!("preview:{label}:{sequence}"),
                snapshot_token,
                deltas: vec![delta],
            },
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            affected_live_view_ids: Vec::new(),
            affected_derived_view_ids: Vec::new(),
            considered_computed_view_count: 0,
            considered_effect_count: 0,
            delivered_effect_count: 0,
            pending_write_intent_count: 0,
            suppressed_effect_count: 0,
            meaningful_effect_suppression_count: 0,
            effect_expression_failure_count: 0,
            refresh_fallback: false,
        }
    }

    pub fn commit_identity(&self) -> &str {
        &self.inner.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.inner.snapshot_token
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn deltas(&self) -> &[crate::memory_workspace::ForgeQueryMutationDelta] {
        &self.inner.deltas
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub fn into_inner(self) -> ForgeQueryMutationReceipt {
        self.inner
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryPatchBatch {
    pub view_name: String,
    pub live_patches: Vec<ForgeQueryLivePatch>,
    pub query_delivery_batches: Vec<ForgeQueryRuntimeDeliveryBatch>,
    pub derived_patch_notes: Vec<String>,
    pub derived_patches: Vec<ForgeQueryDerivedPatch>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveView<T = Value> {
    pub(super) handle: ForgeQueryLiveViewHandle,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) subscription_installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    pub(super) marker: PhantomData<T>,
}

impl<T> ForgeQueryLiveView<T> {
    pub(super) fn new(
        handle: ForgeQueryLiveViewHandle,
        subscription_installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Self {
        Self {
            handle,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            subscription_installation,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.handle.name()
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn subscription_installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        &self.subscription_installation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledProgram {
    pub(super) program_id: String,
}

impl ForgeQueryInstalledProgram {
    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn operation(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<ForgeQueryInstalledOperation, ForgeQueryRuntimeError> {
        Ok(ForgeQueryInstalledOperation {
            program_id: self.program_id.clone(),
            operation_id: operation_id.into(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledOperation {
    pub(super) program_id: String,
    pub(super) operation_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRunReceipt {
    pub(super) run_id: String,
    pub(super) operation: ForgeQueryInstalledOperation,
    pub(super) outputs: Vec<ForgeQueryOperationOutput>,
    pub(super) write_receipts: Vec<ForgeQueryWriteReceipt>,
    pub(super) patch_batches: Vec<ForgeQueryPatchBatch>,
}

impl ForgeQueryRunReceipt {
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn outputs(&self) -> &[ForgeQueryOperationOutput] {
        &self.outputs
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn patch_batches(&self) -> &[ForgeQueryPatchBatch] {
        &self.patch_batches
    }
}

pub struct ForgeQueryArtifactInspector<'a> {
    pub(super) receipt: &'a ForgeQueryWriteReceipt,
    pub(super) runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
}

impl<'a> ForgeQueryArtifactInspector<'a> {
    pub fn canonical(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "canonical",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn workflow(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "workflow",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn bridge_authority(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.receipt.authority_lane()
    }

    pub fn runtime_evidence(&self) -> &ForgeQueryRuntimeInspectionEvidence {
        &self.runtime_evidence
    }

    pub fn live_patch_artifacts(&self) -> Vec<String> {
        self.receipt
            .deltas()
            .iter()
            .map(|delta| format!("{}:{}", delta.collection, delta.entity_identity))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInspectedArtifact {
    pub(super) family: String,
    pub(super) identity: String,
    pub(super) basis: String,
}

impl ForgeQueryInspectedArtifact {
    pub(super) fn new(
        family: impl Into<String>,
        identity: impl Into<String>,
        basis: impl Into<String>,
    ) -> Self {
        Self {
            family: family.into(),
            identity: identity.into(),
            basis: basis.into(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn basis(&self) -> &str {
        &self.basis
    }
}
