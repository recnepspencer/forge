use crate::identity::hash_parts;

use super::super::{
    ForgeQueryAspectMutationOperation, ForgeQueryAuthorityLane, ForgeQueryBatchWriteReceipt,
    ForgeQueryComputedInspectionEvidence, ForgeQueryDerivedViewHandle, ForgeQueryEffectHandle,
    ForgeQueryEffectInspectionEvidence, ForgeQueryInspectedArtifact,
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
pub struct ForgeQueryBatchWriteComponentInspection {
    family: String,
    commit_identity: String,
    collections: Vec<String>,
    entity_identities: Vec<String>,
    touched_aspect_paths: Vec<String>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
}

impl ForgeQueryBatchWriteComponentInspection {
    fn from_write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        let collections = receipt
            .declared_collection()
            .map(|collection| vec![collection.to_string()])
            .unwrap_or_else(|| {
                let mut collections = receipt
                    .deltas()
                    .iter()
                    .map(|delta| delta.collection.clone())
                    .collect::<Vec<_>>();
                collections.sort();
                collections.dedup();
                collections
            });

        let entity_identities = receipt
            .declared_entity_identity()
            .map(|entity| vec![entity.to_string()])
            .unwrap_or_else(|| {
                let mut entity_identities = receipt
                    .deltas()
                    .iter()
                    .map(|delta| delta.entity_identity.clone())
                    .collect::<Vec<_>>();
                entity_identities.sort();
                entity_identities.dedup();
                entity_identities
            });

        let mut touched_aspect_paths = receipt
            .deltas()
            .iter()
            .flat_map(|delta| delta.aspect_paths.iter().cloned())
            .collect::<Vec<_>>();
        touched_aspect_paths.sort();
        touched_aspect_paths.dedup();

        Self {
            family: receipt.mutation_family().as_str().to_string(),
            commit_identity: receipt.commit_identity().to_string(),
            collections,
            entity_identities,
            touched_aspect_paths,
            declared_aspect_operations: receipt.declared_aspect_operations().to_vec(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn collections(&self) -> &[String] {
        &self.collections
    }

    pub fn entity_identities(&self) -> &[String] {
        &self.entity_identities
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceiptInspection {
    mutation_family: String,
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    declared_collection: Option<String>,
    declared_entity_identity: Option<String>,
    commit_identity: String,
    snapshot_token: String,
    canonical_artifact: ForgeQueryInspectedArtifact,
    workflow_artifact: ForgeQueryInspectedArtifact,
    bridge_authority_artifact: ForgeQueryInspectedArtifact,
    runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
    live_patch_artifacts: Vec<String>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
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
        let declared_aspect_operations = receipt.declared_aspect_operations().to_vec();
        let inspection_digest = hash_parts(&[
            "forge_query_write_receipt_inspection_v1".to_string(),
            format!("family:{}", receipt.mutation_family()),
            format!("authority:{}", receipt.authority_lane()),
            format!("basis:{}", receipt.basis_lane()),
            format!("commit:{}", receipt.commit_identity()),
            format!("snapshot:{}", receipt.snapshot_token()),
            format!(
                "declared-collection:{}",
                receipt.declared_collection().unwrap_or("")
            ),
            format!(
                "declared-entity:{}",
                receipt.declared_entity_identity().unwrap_or("")
            ),
            format!(
                "runtime:{}:{}:{}",
                runtime_evidence.artifact_family(),
                runtime_evidence.authority_lane(),
                runtime_evidence.evidence().join("|")
            ),
            format!(
                "declared-aspect-operations:{}",
                declared_aspect_operations
                    .iter()
                    .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            format!("patches:{}", live_patch_artifacts.join("|")),
        ]);
        Self {
            mutation_family: receipt.mutation_family().as_str().to_string(),
            authority_lane: receipt.authority_lane(),
            basis_lane: receipt.basis_lane(),
            declared_collection: receipt.declared_collection().map(str::to_string),
            declared_entity_identity: receipt.declared_entity_identity().map(str::to_string),
            commit_identity: receipt.commit_identity().to_string(),
            snapshot_token: receipt.snapshot_token().to_string(),
            canonical_artifact,
            workflow_artifact,
            bridge_authority_artifact,
            runtime_evidence,
            live_patch_artifacts,
            declared_aspect_operations,
            inspection_digest,
        }
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn mutation_family(&self) -> &str {
        &self.mutation_family
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn declared_collection(&self) -> Option<&str> {
        self.declared_collection.as_deref()
    }

    pub fn declared_entity_identity(&self) -> Option<&str> {
        self.declared_entity_identity.as_deref()
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

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceiptInspection {
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_digest: String,
    write_receipt_count: usize,
    commit_identities: Vec<String>,
    component_operations: Vec<ForgeQueryBatchWriteComponentInspection>,
    touched_aspect_paths: Vec<String>,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    inspection_digest: String,
}

impl ForgeQueryBatchWriteReceiptInspection {
    pub(in crate::runtime) fn new(receipt: &ForgeQueryBatchWriteReceipt) -> Self {
        let commit_identities = receipt
            .write_receipts()
            .iter()
            .map(|entry| entry.commit_identity().to_string())
            .collect::<Vec<_>>();
        let component_operations = receipt
            .write_receipts()
            .iter()
            .map(ForgeQueryBatchWriteComponentInspection::from_write_receipt)
            .collect::<Vec<_>>();
        let touched_aspect_paths = receipt.touched_aspect_paths().to_vec();
        let affected_live_view_ids = receipt.affected_live_view_ids().to_vec();
        let affected_derived_view_ids = receipt.affected_derived_view_ids().to_vec();
        let inspection_digest = hash_parts(
            &std::iter::once("forge_query_batch_write_receipt_inspection_v1".to_string())
                .chain(std::iter::once(format!(
                    "authority:{}",
                    receipt.authority_lane()
                )))
                .chain(std::iter::once(format!("basis:{}", receipt.basis_lane())))
                .chain(std::iter::once(format!("batch:{}", receipt.batch_digest())))
                .chain(
                    commit_identities
                        .iter()
                        .map(|commit| format!("commit:{commit}")),
                )
                .chain(component_operations.iter().flat_map(|component| {
                    std::iter::once(format!("family:{}", component.family()))
                        .chain(
                            component
                                .collections()
                                .iter()
                                .map(|collection| format!("collection:{collection}")),
                        )
                        .chain(
                            component
                                .entity_identities()
                                .iter()
                                .map(|entity| format!("entity:{entity}")),
                        )
                        .chain(
                            component
                                .declared_aspect_operations()
                                .iter()
                                .map(|operation| {
                                    format!(
                                        "component-operation:{}:{}",
                                        operation.kind(),
                                        operation.aspect_path()
                                    )
                                }),
                        )
                        .chain(
                            component
                                .touched_aspect_paths()
                                .iter()
                                .map(|path| format!("component-aspect:{path}")),
                        )
                }))
                .chain(
                    touched_aspect_paths
                        .iter()
                        .map(|path| format!("aspect:{path}")),
                )
                .chain(
                    affected_live_view_ids
                        .iter()
                        .map(|view| format!("live:{view}")),
                )
                .chain(
                    affected_derived_view_ids
                        .iter()
                        .map(|view| format!("derived:{view}")),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            authority_lane: receipt.authority_lane(),
            basis_lane: receipt.basis_lane(),
            batch_digest: receipt.batch_digest().to_string(),
            write_receipt_count: receipt.write_count(),
            commit_identities,
            component_operations,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            inspection_digest,
        }
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn batch_digest(&self) -> &str {
        &self.batch_digest
    }

    pub fn write_receipt_count(&self) -> usize {
        self.write_receipt_count
    }

    pub fn commit_identities(&self) -> &[String] {
        &self.commit_identities
    }

    pub fn component_operations(&self) -> &[ForgeQueryBatchWriteComponentInspection] {
        &self.component_operations
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
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
    BatchWriteReceipt(&'a ForgeQueryBatchWriteReceipt),
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

impl<'a> From<&'a ForgeQueryBatchWriteReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryBatchWriteReceipt) -> Self {
        Self::BatchWriteReceipt(value)
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
    BatchWriteReceipt(ForgeQueryBatchWriteReceiptInspection),
    IntentReceipt(ForgeQueryIntentReceiptInspection),
    IntentDenial(ForgeQueryIntentDenialInspection),
    EffectIntentReceipt(ForgeQueryEffectIntentReceiptInspection),
    PreviewBinding(ForgeQueryPreviewBindingInspection),
    PreviewOutcome(ForgeQueryPreviewOutcomeInspection),
    PreviewIntentReceipt(ForgeQueryPreviewIntentReceiptInspection),
    BranchIntentReceipt(ForgeQueryBranchIntentReceiptInspection),
}
