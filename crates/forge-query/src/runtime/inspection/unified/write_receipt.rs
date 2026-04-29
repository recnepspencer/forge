mod digest;

use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAuthorityLane,
    ForgeQueryContinuityMutationEvidence, ForgeQueryExistingTruthBindingEvidence,
    ForgeQueryInspectedArtifact, ForgeQueryMutationCausalityEvidence, ForgeQueryMutationMetadata,
    ForgeQueryMutationProvenanceEvidence, ForgeQueryMutationTargetEvidence,
    ForgeQueryNamingMutationEvidence, ForgeQueryRuntimeInspectionEvidence,
    ForgeQuerySymbolicTargetReferenceEvidence, ForgeQueryWriteReceipt,
};
use digest::build_write_receipt_inspection_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceiptInspection {
    mutation_family: String,
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    target_evidence: ForgeQueryMutationTargetEvidence,
    existing_truth_binding_evidence: Option<ForgeQueryExistingTruthBindingEvidence>,
    symbolic_target_reference_evidence: Option<ForgeQuerySymbolicTargetReferenceEvidence>,
    naming_mutation_evidence: Option<ForgeQueryNamingMutationEvidence>,
    continuity_mutation_evidence: Option<ForgeQueryContinuityMutationEvidence>,
    causality_evidence: Option<ForgeQueryMutationCausalityEvidence>,
    provenance_evidence: Option<ForgeQueryMutationProvenanceEvidence>,
    declared_collection: Option<String>,
    declared_entity_identity: Option<String>,
    target_collection: Option<String>,
    target_entity_identity: Option<String>,
    commit_identity: String,
    snapshot_token: String,
    canonical_artifact: ForgeQueryInspectedArtifact,
    workflow_artifact: ForgeQueryInspectedArtifact,
    bridge_authority_artifact: ForgeQueryInspectedArtifact,
    runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
    live_patch_artifacts: Vec<String>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    mutation_metadata: ForgeQueryMutationMetadata,
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
        let mutation_metadata = receipt.mutation_metadata().clone();
        let target_evidence = receipt.target_evidence().clone();
        let existing_truth_binding_evidence = receipt.existing_truth_binding_evidence().cloned();
        let symbolic_target_reference_evidence =
            receipt.symbolic_target_reference_evidence().cloned();
        let naming_mutation_evidence = receipt.naming_mutation_evidence().cloned();
        let continuity_mutation_evidence = receipt.continuity_mutation_evidence().cloned();
        let causality_evidence = receipt.causality_evidence().cloned();
        let provenance_evidence = receipt.provenance_evidence().cloned();
        let inspection_digest = build_write_receipt_inspection_digest(
            receipt,
            &target_evidence,
            existing_truth_binding_evidence.as_ref(),
            symbolic_target_reference_evidence.as_ref(),
            naming_mutation_evidence.as_ref(),
            continuity_mutation_evidence.as_ref(),
            causality_evidence.as_ref(),
            provenance_evidence.as_ref(),
            &runtime_evidence,
            &declared_aspect_operations,
            &mutation_metadata,
            &live_patch_artifacts,
        );
        Self {
            mutation_family: receipt.mutation_family().as_str().to_string(),
            authority_lane: receipt.authority_lane(),
            basis_lane: receipt.basis_lane(),
            target_evidence,
            existing_truth_binding_evidence,
            symbolic_target_reference_evidence,
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence,
            provenance_evidence,
            declared_collection: receipt.declared_collection().map(str::to_string),
            declared_entity_identity: receipt.declared_entity_identity().map(str::to_string),
            target_collection: receipt.target_collection().map(str::to_string),
            target_entity_identity: receipt.target_entity_identity().map(str::to_string),
            commit_identity: receipt.commit_identity().to_string(),
            snapshot_token: receipt.snapshot_token().to_string(),
            canonical_artifact,
            workflow_artifact,
            bridge_authority_artifact,
            runtime_evidence,
            live_patch_artifacts,
            declared_aspect_operations,
            mutation_metadata,
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

    pub fn target_evidence(&self) -> &ForgeQueryMutationTargetEvidence {
        &self.target_evidence
    }

    pub fn causality_evidence(&self) -> Option<&ForgeQueryMutationCausalityEvidence> {
        self.causality_evidence.as_ref()
    }

    pub fn existing_truth_binding_evidence(
        &self,
    ) -> Option<&ForgeQueryExistingTruthBindingEvidence> {
        self.existing_truth_binding_evidence.as_ref()
    }

    pub fn symbolic_target_reference_evidence(
        &self,
    ) -> Option<&ForgeQuerySymbolicTargetReferenceEvidence> {
        self.symbolic_target_reference_evidence.as_ref()
    }

    pub fn naming_mutation_evidence(&self) -> Option<&ForgeQueryNamingMutationEvidence> {
        self.naming_mutation_evidence.as_ref()
    }

    pub fn continuity_mutation_evidence(&self) -> Option<&ForgeQueryContinuityMutationEvidence> {
        self.continuity_mutation_evidence.as_ref()
    }

    pub fn provenance_evidence(&self) -> Option<&ForgeQueryMutationProvenanceEvidence> {
        self.provenance_evidence.as_ref()
    }

    pub fn declared_collection(&self) -> Option<&str> {
        self.declared_collection.as_deref()
    }

    pub fn declared_entity_identity(&self) -> Option<&str> {
        self.declared_entity_identity.as_deref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn target_entity_identity(&self) -> Option<&str> {
        self.target_entity_identity.as_deref()
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

    pub fn mutation_metadata(&self) -> &ForgeQueryMutationMetadata {
        &self.mutation_metadata
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
