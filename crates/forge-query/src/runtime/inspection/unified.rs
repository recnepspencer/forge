use crate::identity::hash_parts;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryComputedInspectionEvidence, ForgeQueryDerivedViewHandle,
    ForgeQueryEffectHandle, ForgeQueryEffectInspectionEvidence, ForgeQueryInspectedArtifact,
    ForgeQueryIntentDenialEvidence, ForgeQueryIntentReceipt, ForgeQueryLiveView,
    ForgeQueryPreviewHandleBindingEvidence, ForgeQueryPreviewIntentReceipt,
    ForgeQueryPreviewOutcome, ForgeQueryRuntimeInspectionEvidence, ForgeQueryWriteReceipt,
};
use super::super::{ForgeQueryBranchIntentReceipt, ForgeQueryEffectIntentReceipt};
use super::{
    ForgeQueryBranchIntentReceiptInspection, ForgeQueryEffectIntentReceiptInspection,
    ForgeQueryIntentDenialInspection, ForgeQueryIntentReceiptInspection,
    ForgeQueryLiveViewInspection, ForgeQueryPreviewBindingInspection,
    ForgeQueryPreviewIntentReceiptInspection, ForgeQueryPreviewOutcomeInspection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceiptInspection {
    authority_lane: ForgeQueryAuthorityLane,
    commit_identity: String,
    snapshot_token: String,
    canonical_artifact: ForgeQueryInspectedArtifact,
    workflow_artifact: ForgeQueryInspectedArtifact,
    bridge_authority_artifact: ForgeQueryInspectedArtifact,
    runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
    live_patch_artifacts: Vec<String>,
    inspection_digest: String,
}

impl ForgeQueryWriteReceiptInspection {
    pub(in crate::runtime) fn new(
        receipt: &ForgeQueryWriteReceipt,
        runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
    ) -> Self {
        let canonical_artifact = ForgeQueryInspectedArtifact::new(
            "canonical",
            receipt.commit_identity(),
            receipt.snapshot_token(),
        );
        let workflow_artifact = ForgeQueryInspectedArtifact::new(
            "workflow",
            receipt.commit_identity(),
            receipt.snapshot_token(),
        );
        let bridge_authority_artifact = ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            receipt.commit_identity(),
            receipt.snapshot_token(),
        );
        let live_patch_artifacts = receipt
            .deltas()
            .iter()
            .map(|delta| format!("{}:{}", delta.collection, delta.entity_identity))
            .collect::<Vec<_>>();
        let inspection_digest = hash_parts(&[
            "forge_query_write_receipt_inspection_v1".to_string(),
            format!("authority:{}", receipt.authority_lane()),
            format!("commit:{}", receipt.commit_identity()),
            format!("snapshot:{}", receipt.snapshot_token()),
            format!(
                "runtime:{}:{}:{}",
                runtime_evidence.artifact_family(),
                runtime_evidence.authority_lane(),
                runtime_evidence.evidence().join("|")
            ),
            format!("patches:{}", live_patch_artifacts.join("|")),
        ]);
        Self {
            authority_lane: receipt.authority_lane(),
            commit_identity: receipt.commit_identity().to_string(),
            snapshot_token: receipt.snapshot_token().to_string(),
            canonical_artifact,
            workflow_artifact,
            bridge_authority_artifact,
            runtime_evidence,
            live_patch_artifacts,
            inspection_digest,
        }
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn canonical_artifact(&self) -> &ForgeQueryInspectedArtifact {
        &self.canonical_artifact
    }

    pub fn workflow_artifact(&self) -> &ForgeQueryInspectedArtifact {
        &self.workflow_artifact
    }

    pub fn bridge_authority_artifact(&self) -> &ForgeQueryInspectedArtifact {
        &self.bridge_authority_artifact
    }

    pub fn runtime_evidence(&self) -> &ForgeQueryRuntimeInspectionEvidence {
        &self.runtime_evidence
    }

    pub fn live_patch_artifacts(&self) -> &[String] {
        &self.live_patch_artifacts
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

pub enum ForgeQueryInspectionTarget<'a> {
    LiveView { name: &'a str },
    DerivedView { name: &'a str },
    Effect { name: &'a str },
    WriteReceipt(&'a ForgeQueryWriteReceipt),
    IntentReceipt(&'a ForgeQueryIntentReceipt),
    IntentDenial(&'a ForgeQueryIntentDenialEvidence),
    EffectIntentReceipt(&'a ForgeQueryEffectIntentReceipt),
    PreviewBinding(&'a ForgeQueryPreviewHandleBindingEvidence),
    PreviewOutcome(&'a ForgeQueryPreviewOutcome),
    PreviewIntentReceipt(&'a ForgeQueryPreviewIntentReceipt),
    BranchIntentReceipt(&'a ForgeQueryBranchIntentReceipt),
}

impl<'a, T> From<&'a ForgeQueryLiveView<T>> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryLiveView<T>) -> Self {
        Self::LiveView { name: value.name() }
    }
}

impl<'a, T> From<&'a ForgeQueryDerivedViewHandle<T>> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryDerivedViewHandle<T>) -> Self {
        Self::DerivedView { name: value.name() }
    }
}

impl<'a, T> From<&'a ForgeQueryEffectHandle<T>> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryEffectHandle<T>) -> Self {
        Self::Effect { name: value.name() }
    }
}

impl<'a> From<&'a ForgeQueryWriteReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryWriteReceipt) -> Self {
        Self::WriteReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryIntentReceipt) -> Self {
        Self::IntentReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryIntentDenialEvidence> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryIntentDenialEvidence) -> Self {
        Self::IntentDenial(value)
    }
}

impl<'a> From<&'a ForgeQueryEffectIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryEffectIntentReceipt) -> Self {
        Self::EffectIntentReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryPreviewHandleBindingEvidence> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryPreviewHandleBindingEvidence) -> Self {
        Self::PreviewBinding(value)
    }
}

impl<'a> From<&'a ForgeQueryPreviewOutcome> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryPreviewOutcome) -> Self {
        Self::PreviewOutcome(value)
    }
}

impl<'a> From<&'a ForgeQueryPreviewIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryPreviewIntentReceipt) -> Self {
        Self::PreviewIntentReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryBranchIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryBranchIntentReceipt) -> Self {
        Self::BranchIntentReceipt(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryInspection {
    LiveView(ForgeQueryLiveViewInspection),
    DerivedView(ForgeQueryComputedInspectionEvidence),
    Effect(ForgeQueryEffectInspectionEvidence),
    WriteReceipt(ForgeQueryWriteReceiptInspection),
    IntentReceipt(ForgeQueryIntentReceiptInspection),
    IntentDenial(ForgeQueryIntentDenialInspection),
    EffectIntentReceipt(ForgeQueryEffectIntentReceiptInspection),
    PreviewBinding(ForgeQueryPreviewBindingInspection),
    PreviewOutcome(ForgeQueryPreviewOutcomeInspection),
    PreviewIntentReceipt(ForgeQueryPreviewIntentReceiptInspection),
    BranchIntentReceipt(ForgeQueryBranchIntentReceiptInspection),
}
