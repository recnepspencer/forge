use crate::memory_workspace::{WorthQueryCommitIdentity, WorthQueryEntityIdentity};
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch, WorthQueryContinuityMutationEvidence,
    WorthQueryExistingTruthAssertionEvidence, WorthQueryExistingTruthBindingEvidence,
    WorthQueryMutationCausalityEvidence, WorthQueryMutationProvenanceEvidence,
    WorthQueryMutationTargetEvidence, WorthQueryNamingMutationEvidence,
    WorthQuerySymbolicAspectResolutionEvidence, WorthQuerySymbolicTargetReferenceEvidence,
    WorthQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBatchWriteComponentInspection {
    family: String,
    commit_identity: WorthQueryCommitIdentity,
    target_evidence: WorthQueryMutationTargetEvidence,
    existing_truth_assertion_evidence: Option<WorthQueryExistingTruthAssertionEvidence>,
    existing_truth_binding_evidence: Option<WorthQueryExistingTruthBindingEvidence>,
    symbolic_target_reference_evidence: Option<WorthQuerySymbolicTargetReferenceEvidence>,
    symbolic_aspect_resolution_evidence: Vec<WorthQuerySymbolicAspectResolutionEvidence>,
    naming_mutation_evidence: Option<WorthQueryNamingMutationEvidence>,
    continuity_mutation_evidence: Option<WorthQueryContinuityMutationEvidence>,
    causality_evidence: Option<WorthQueryMutationCausalityEvidence>,
    provenance_evidence: Option<WorthQueryMutationProvenanceEvidence>,
    declared_collection: Option<String>,
    declared_entity_identity: Option<WorthQueryEntityIdentity>,
    target_collection: Option<String>,
    target_entity_identity: Option<WorthQueryEntityIdentity>,
    collections: Vec<String>,
    entity_identities: Vec<WorthQueryEntityIdentity>,
    touched_aspects: Vec<WorthQueryAspectTouch>,
    declared_aspect_operations: Vec<WorthQueryAspectMutationOperation>,
}

impl WorthQueryBatchWriteComponentInspection {
    pub(super) fn from_write_receipt(receipt: &WorthQueryWriteReceipt) -> Self {
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

    pub fn commit_identity(&self) -> &WorthQueryCommitIdentity {
        &self.commit_identity
    }

    pub fn target_evidence(&self) -> &WorthQueryMutationTargetEvidence {
        &self.target_evidence
    }

    pub fn causality_evidence(&self) -> Option<&WorthQueryMutationCausalityEvidence> {
        self.causality_evidence.as_ref()
    }

    pub fn existing_truth_assertion_evidence(
        &self,
    ) -> Option<&WorthQueryExistingTruthAssertionEvidence> {
        self.existing_truth_assertion_evidence.as_ref()
    }

    pub fn existing_truth_binding_evidence(
        &self,
    ) -> Option<&WorthQueryExistingTruthBindingEvidence> {
        self.existing_truth_binding_evidence.as_ref()
    }

    pub fn symbolic_target_reference_evidence(
        &self,
    ) -> Option<&WorthQuerySymbolicTargetReferenceEvidence> {
        self.symbolic_target_reference_evidence.as_ref()
    }

    pub fn symbolic_aspect_resolution_evidence(
        &self,
    ) -> &[WorthQuerySymbolicAspectResolutionEvidence] {
        &self.symbolic_aspect_resolution_evidence
    }

    pub fn naming_mutation_evidence(&self) -> Option<&WorthQueryNamingMutationEvidence> {
        self.naming_mutation_evidence.as_ref()
    }

    pub fn continuity_mutation_evidence(&self) -> Option<&WorthQueryContinuityMutationEvidence> {
        self.continuity_mutation_evidence.as_ref()
    }

    pub fn provenance_evidence(&self) -> Option<&WorthQueryMutationProvenanceEvidence> {
        self.provenance_evidence.as_ref()
    }

    pub fn collections(&self) -> &[String] {
        &self.collections
    }

    pub fn declared_collection(&self) -> Option<&str> {
        self.declared_collection.as_deref()
    }

    pub fn declared_entity_identity(&self) -> Option<&WorthQueryEntityIdentity> {
        self.declared_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn target_entity_identity(&self) -> Option<&WorthQueryEntityIdentity> {
        self.target_entity_identity.as_ref()
    }

    pub fn entity_identities(&self) -> &[WorthQueryEntityIdentity] {
        &self.entity_identities
    }

    pub fn admitted_touched_aspects(&self) -> &[WorthQueryAspectTouch] {
        &self.touched_aspects
    }

    pub fn declared_aspect_operations(&self) -> &[WorthQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }
}
