use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQueryEntityIdentity};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryContinuityMutationEvidence,
    ForgeQueryExistingTruthAssertionEvidence, ForgeQueryExistingTruthBindingEvidence,
    ForgeQueryMutationCausalityEvidence, ForgeQueryMutationProvenanceEvidence,
    ForgeQueryMutationTargetEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQuerySymbolicAspectResolutionEvidence, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteComponentInspection {
    family: String,
    commit_identity: ForgeQueryCommitIdentity,
    target_evidence: ForgeQueryMutationTargetEvidence,
    existing_truth_assertion_evidence: Option<ForgeQueryExistingTruthAssertionEvidence>,
    existing_truth_binding_evidence: Option<ForgeQueryExistingTruthBindingEvidence>,
    symbolic_target_reference_evidence: Option<ForgeQuerySymbolicTargetReferenceEvidence>,
    symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
    naming_mutation_evidence: Option<ForgeQueryNamingMutationEvidence>,
    continuity_mutation_evidence: Option<ForgeQueryContinuityMutationEvidence>,
    causality_evidence: Option<ForgeQueryMutationCausalityEvidence>,
    provenance_evidence: Option<ForgeQueryMutationProvenanceEvidence>,
    declared_collection: Option<String>,
    declared_entity_identity: Option<ForgeQueryEntityIdentity>,
    target_collection: Option<String>,
    target_entity_identity: Option<ForgeQueryEntityIdentity>,
    collections: Vec<String>,
    entity_identities: Vec<ForgeQueryEntityIdentity>,
    touched_aspects: Vec<ForgeQueryAspectTouch>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
}

impl ForgeQueryBatchWriteComponentInspection {
    pub(super) fn from_write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        let collections = receipt
            .terminal_declared_collection_projection()
            .map(|collection| vec![collection.to_string()])
            .unwrap_or_else(|| {
                let mut collections = receipt
                    .deltas()
                    .iter()
                    .map(|delta| delta.collection().to_string())
                    .collect::<Vec<_>>();
                collections.sort();
                collections.dedup();
                collections
            });

        let entity_identities = receipt
            .declared_entity_identity()
            .map(|entity| vec![entity.clone()])
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

        let mut touched_aspects = std::collections::BTreeSet::new();
        for touch in receipt
            .deltas()
            .iter()
            .flat_map(|delta| delta.admitted_touched_aspects())
        {
            touched_aspects.insert(touch.clone());
        }

        Self {
            family: receipt.mutation_family().as_str().to_string(),
            commit_identity: receipt.commit_identity().clone(),
            target_evidence: receipt.target_evidence().clone(),
            existing_truth_assertion_evidence: receipt.existing_truth_assertion_evidence().cloned(),
            existing_truth_binding_evidence: receipt.existing_truth_binding_evidence().cloned(),
            symbolic_target_reference_evidence: receipt
                .symbolic_target_reference_evidence()
                .cloned(),
            symbolic_aspect_resolution_evidence: receipt
                .symbolic_aspect_resolution_evidence()
                .to_vec(),
            naming_mutation_evidence: receipt.naming_mutation_evidence().cloned(),
            continuity_mutation_evidence: receipt.continuity_mutation_evidence().cloned(),
            causality_evidence: receipt.causality_evidence().cloned(),
            provenance_evidence: receipt.provenance_evidence().cloned(),
            declared_collection: receipt
                .terminal_declared_collection_projection()
                .map(str::to_string),
            declared_entity_identity: receipt.declared_entity_identity().cloned(),
            target_collection: receipt
                .terminal_target_collection_projection()
                .map(str::to_string),
            target_entity_identity: receipt.target_entity_identity().cloned(),
            collections,
            entity_identities,
            touched_aspects: touched_aspects.into_iter().collect(),
            declared_aspect_operations: receipt.declared_aspect_operations().to_vec(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.commit_identity
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

    pub fn symbolic_aspect_resolution_evidence(
        &self,
    ) -> &[ForgeQuerySymbolicAspectResolutionEvidence] {
        &self.symbolic_aspect_resolution_evidence
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

    pub fn collections(&self) -> &[String] {
        &self.collections
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

    pub fn entity_identities(&self) -> &[ForgeQueryEntityIdentity] {
        &self.entity_identities
    }

    pub fn admitted_touched_aspects(&self) -> &[ForgeQueryAspectTouch] {
        &self.touched_aspects
    }

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }
}
