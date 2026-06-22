mod digest;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity,
};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAuthorityLane,
    ForgeQueryContinuityMutationEvidence, ForgeQueryExistingTruthAssertionEvidence,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryInspectedArtifact,
    ForgeQueryMutationCausalityEvidence, ForgeQueryMutationMetadata,
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
    existing_truth_assertion_evidence: Option<ForgeQueryExistingTruthAssertionEvidence>,
    existing_truth_binding_evidence: Option<ForgeQueryExistingTruthBindingEvidence>,
    symbolic_target_reference_evidence: Option<ForgeQuerySymbolicTargetReferenceEvidence>,
    naming_mutation_evidence: Option<ForgeQueryNamingMutationEvidence>,
    continuity_mutation_evidence: Option<ForgeQueryContinuityMutationEvidence>,
    causality_evidence: Option<ForgeQueryMutationCausalityEvidence>,
    provenance_evidence: Option<ForgeQueryMutationProvenanceEvidence>,
    declared_collection: Option<String>,
    declared_entity_identity: Option<ForgeQueryEntityIdentity>,
    target_collection: Option<String>,
    target_entity_identity: Option<ForgeQueryEntityIdentity>,
    commit_identity: ForgeQueryCommitIdentity,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    canonical_artifact: ForgeQueryInspectedArtifact,
    workflow_artifact: ForgeQueryInspectedArtifact,
    bridge_authority_artifact: ForgeQueryInspectedArtifact,
    runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
    live_patch_artifacts: Vec<ForgeQueryEvidenceIdentity>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
    mutation_metadata: ForgeQueryMutationMetadata,
    inspection_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryWriteReceiptInspection {
    pub(in crate::runtime) fn new(
        receipt: &ForgeQueryWriteReceipt,
        runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
    ) -> Self {
        let canonical_artifact = ForgeQueryInspectedArtifact::new(
            "canonical",
            receipt.commit_evidence_identity().clone(),
            receipt.snapshot_evidence_identity().clone(),
        );
        let workflow_artifact = ForgeQueryInspectedArtifact::new(
            "workflow",
            receipt.commit_evidence_identity().clone(),
            receipt.snapshot_evidence_identity().clone(),
        );
        let bridge_authority_artifact = ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            receipt.commit_evidence_identity().clone(),
            receipt.snapshot_evidence_identity().clone(),
        );
        let live_patch_artifacts = receipt
            .deltas()
            .iter()
            .map(|delta| {
                forge_query_evidence_identity(
                    ForgeQueryEvidenceScope::WriteReceiptInspectionArtifact,
                )
                .field_shape(ForgeQueryEvidenceTag::new("role"), "live-patch-artifact")
                .field_value(ForgeQueryEvidenceTag::new("collection"), &delta.collection)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("entity_identity"),
                    &delta.entity_identity.evidence_identity(),
                )
                .seal()
            })
            .collect::<Vec<_>>();
        let declared_aspect_operations = receipt.declared_aspect_operations().to_vec();
        let declared_aspect_value_digest = receipt.declared_aspect_value_identity().cloned();
        let mutation_metadata = receipt.mutation_metadata().clone();
        let target_evidence = receipt.target_evidence().clone();
        let existing_truth_assertion_evidence =
            receipt.existing_truth_assertion_evidence().cloned();
        let existing_truth_binding_evidence = receipt.existing_truth_binding_evidence().cloned();
        let symbolic_target_reference_evidence =
            receipt.symbolic_target_reference_evidence().cloned();
        let naming_mutation_evidence = receipt.naming_mutation_evidence().cloned();
        let continuity_mutation_evidence = receipt.continuity_mutation_evidence().cloned();
        let causality_evidence = receipt.causality_evidence().cloned();
        let provenance_evidence = receipt.provenance_evidence().cloned();
        let inspection_identity = build_write_receipt_inspection_digest(
            receipt,
            &target_evidence,
            existing_truth_assertion_evidence.as_ref(),
            existing_truth_binding_evidence.as_ref(),
            symbolic_target_reference_evidence.as_ref(),
            naming_mutation_evidence.as_ref(),
            continuity_mutation_evidence.as_ref(),
            causality_evidence.as_ref(),
            provenance_evidence.as_ref(),
            &runtime_evidence,
            &declared_aspect_operations,
            declared_aspect_value_digest.as_ref(),
            &mutation_metadata,
            &live_patch_artifacts,
        );
        Self {
            mutation_family: receipt.mutation_family().as_str().to_string(),
            authority_lane: receipt.authority_lane(),
            basis_lane: receipt.basis_lane(),
            target_evidence,
            existing_truth_assertion_evidence,
            existing_truth_binding_evidence,
            symbolic_target_reference_evidence,
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence,
            provenance_evidence,
            declared_collection: receipt
                .terminal_declared_collection_projection()
                .map(str::to_string),
            declared_entity_identity: receipt.declared_entity_identity().cloned(),
            target_collection: receipt
                .terminal_target_collection_projection()
                .map(str::to_string),
            target_entity_identity: receipt.target_entity_identity().cloned(),
            commit_identity: receipt.commit_identity().clone(),
            snapshot_identity: receipt.snapshot_identity().clone(),
            canonical_artifact,
            workflow_artifact,
            bridge_authority_artifact,
            runtime_evidence,
            live_patch_artifacts,
            declared_aspect_operations,
            declared_aspect_value_digest,
            mutation_metadata,
            inspection_identity,
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

    pub fn existing_truth_assertion_evidence(
        &self,
    ) -> Option<&ForgeQueryExistingTruthAssertionEvidence> {
        self.existing_truth_assertion_evidence.as_ref()
    }

    pub fn verified_assumption_set(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryVerifiedAssumptionSet> {
        self.existing_truth_assertion_evidence
            .as_ref()
            .and_then(ForgeQueryExistingTruthAssertionEvidence::verified_assumption_set)
    }

    pub fn verification_read_set_breadth(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryVerificationReadSetBreadth> {
        self.existing_truth_assertion_evidence
            .as_ref()
            .and_then(ForgeQueryExistingTruthAssertionEvidence::verification_read_set_breadth)
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

    pub fn declared_entity_identity(&self) -> Option<&ForgeQueryEntityIdentity> {
        self.declared_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn target_entity_identity(&self) -> Option<&ForgeQueryEntityIdentity> {
        self.target_entity_identity.as_ref()
    }

    pub fn commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.commit_identity
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
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

    pub fn live_patch_artifacts(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.live_patch_artifacts
    }

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }

    pub fn declared_aspect_value_digest(&self) -> Option<&str> {
        self.declared_aspect_value_digest
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn mutation_metadata(&self) -> &ForgeQueryMutationMetadata {
        &self.mutation_metadata
    }

    pub fn inspection_digest(&self) -> &str {
        self.inspection_identity.as_str()
    }

    pub fn inspection_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inspection_identity
    }
}
