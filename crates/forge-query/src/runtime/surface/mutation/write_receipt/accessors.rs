use super::ForgeQueryWriteReceipt;
use crate::memory_workspace::{ForgeQueryMutationDelta, ForgeQueryMutationReceipt};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAuthorityLane,
    ForgeQueryContinuityMutationEvidence, ForgeQueryMutationCausalityEvidence,
    ForgeQueryMutationFamily, ForgeQueryMutationProvenanceEvidence,
    ForgeQueryMutationTargetEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQuerySymbolicTargetReference, ForgeQuerySymbolicTargetReferenceEvidence,
};

impl ForgeQueryWriteReceipt {
    pub fn commit_identity(&self) -> &str {
        &self.inner.commit_identity
    }

    pub fn mutation_family(&self) -> ForgeQueryMutationFamily {
        self.mutation_family
    }

    pub fn snapshot_token(&self) -> &str {
        &self.inner.snapshot_token
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
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
    ) -> Option<&crate::runtime::ForgeQueryExistingTruthAssertionEvidence> {
        self.existing_truth_assertion_evidence.as_ref()
    }

    pub fn existing_truth_binding_evidence(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryExistingTruthBindingEvidence> {
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
        self.target_collection
            .as_deref()
            .or(self.declared_collection.as_deref())
    }

    pub fn target_entity_identity(&self) -> Option<&str> {
        self.target_entity_identity
            .as_deref()
            .or(self.declared_entity_identity.as_deref())
    }

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }

    pub fn declared_aspect_value_digest(&self) -> Option<&str> {
        self.declared_aspect_value_digest.as_deref()
    }

    pub fn mutation_metadata(&self) -> &crate::runtime::ForgeQueryMutationMetadata {
        &self.mutation_metadata
    }

    pub fn deltas(&self) -> &[ForgeQueryMutationDelta] {
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

    pub(in crate::runtime) fn with_symbolic_target_reference(
        mut self,
        reference: &ForgeQuerySymbolicTargetReference,
        resolved_entity_identity: impl Into<String>,
        resolved_collection: Option<String>,
    ) -> Self {
        let resolved_entity_identity = resolved_entity_identity.into();
        self.symbolic_target_reference_evidence =
            Some(ForgeQuerySymbolicTargetReferenceEvidence::from_reference(
                reference,
                &resolved_entity_identity,
            ));
        self.target_entity_identity = Some(resolved_entity_identity);
        self.target_collection = resolved_collection;
        self
    }
}
