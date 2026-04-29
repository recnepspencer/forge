use crate::memory_workspace::{ForgeQueryMutationDelta, ForgeQueryMutationReceipt};
use crate::runtime::mutation::ForgeQueryMutationMetadata;

use super::{ForgeQueryMutationFamily, ForgeQueryWriteCommand};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAuthorityLane,
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryMutationCausalityEvidence, ForgeQueryMutationProvenanceEvidence,
    ForgeQueryMutationTargetEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQueryNamingMutationIntent, ForgeQuerySymbolicTargetReference,
    ForgeQuerySymbolicTargetReferenceEvidence,
};

mod helpers;
mod preview;

use helpers::{
    continuity_mutation_evidence, naming_mutation_evidence, symbolic_target_reference_evidence,
    target_evidence_from_receipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    pub(super) inner: ForgeQueryMutationReceipt,
    pub(super) mutation_family: ForgeQueryMutationFamily,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) basis_lane: ForgeQueryAuthorityLane,
    pub(super) target_evidence: ForgeQueryMutationTargetEvidence,
    pub(super) existing_truth_binding_evidence: Option<ForgeQueryExistingTruthBindingEvidence>,
    pub(super) symbolic_target_reference_evidence:
        Option<ForgeQuerySymbolicTargetReferenceEvidence>,
    pub(super) naming_mutation_evidence: Option<ForgeQueryNamingMutationEvidence>,
    pub(super) continuity_mutation_evidence: Option<ForgeQueryContinuityMutationEvidence>,
    pub(super) causality_evidence: Option<ForgeQueryMutationCausalityEvidence>,
    pub(super) provenance_evidence: Option<ForgeQueryMutationProvenanceEvidence>,
    pub(super) declared_collection: Option<String>,
    pub(super) declared_entity_identity: Option<String>,
    pub(super) target_collection: Option<String>,
    pub(super) target_entity_identity: Option<String>,
    pub(super) declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    pub(super) mutation_metadata: ForgeQueryMutationMetadata,
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
    pub(in crate::runtime) fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        target_collection: Option<String>,
        target_entity_identity: Option<String>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        mutation_metadata: ForgeQueryMutationMetadata,
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
        let target_evidence = target_evidence_from_receipt(
            mutation_family,
            declared_collection.clone(),
            declared_entity_identity.clone(),
            target_collection.clone(),
            target_entity_identity.clone(),
        );
        let causality_evidence = inner
            .bridge_authority
            .as_ref()
            .map(ForgeQueryMutationCausalityEvidence::from_bridge);
        let provenance_evidence = inner
            .bridge_authority
            .as_ref()
            .map(ForgeQueryMutationProvenanceEvidence::from_bridge);
        let existing_truth_binding_evidence = inner
            .bridge_authority
            .as_ref()
            .and_then(|bundle| bundle.existing_truth_binding())
            .map(ForgeQueryExistingTruthBindingEvidence::from_bridge)
            .or_else(|| {
                existing_truth_binding
                    .as_ref()
                    .map(ForgeQueryExistingTruthBindingEvidence::from_binding)
            });
        let symbolic_target_reference_evidence = symbolic_target_reference_evidence(
            mutation_family,
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.symbolic_target_reference()),
            symbolic_target_reference.as_ref(),
            target_entity_identity.as_deref(),
        );
        let naming_mutation_evidence = naming_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.naming_mutation()),
            naming_intent.as_ref(),
            target_entity_identity.as_deref(),
            target_collection.as_deref(),
        );
        let continuity_mutation_evidence = continuity_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.continuity_mutation()),
            continuity_intent.as_ref(),
            existing_truth_binding.as_ref(),
            target_entity_identity.as_deref(),
            target_collection.as_deref(),
        );
        Self {
            inner,
            mutation_family,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            basis_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            target_evidence,
            existing_truth_binding_evidence,
            symbolic_target_reference_evidence,
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence,
            provenance_evidence,
            declared_collection,
            declared_entity_identity,
            target_collection,
            target_entity_identity,
            declared_aspect_operations,
            mutation_metadata,
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

    pub(in crate::runtime) fn batch_component(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        basis_lane: ForgeQueryAuthorityLane,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        target_collection: Option<String>,
        target_entity_identity: Option<String>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        mutation_metadata: ForgeQueryMutationMetadata,
        affected_live_view_ids: Vec<String>,
        authority_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        let target_evidence = target_evidence_from_receipt(
            mutation_family,
            declared_collection.clone(),
            declared_entity_identity.clone(),
            target_collection.clone(),
            target_entity_identity.clone(),
        );
        let causality_evidence = inner
            .bridge_authority
            .as_ref()
            .map(ForgeQueryMutationCausalityEvidence::from_bridge);
        let provenance_evidence = inner
            .bridge_authority
            .as_ref()
            .map(ForgeQueryMutationProvenanceEvidence::from_bridge);
        let existing_truth_binding_evidence = inner
            .bridge_authority
            .as_ref()
            .and_then(|bundle| bundle.existing_truth_binding())
            .map(ForgeQueryExistingTruthBindingEvidence::from_bridge)
            .or_else(|| {
                existing_truth_binding
                    .as_ref()
                    .map(ForgeQueryExistingTruthBindingEvidence::from_binding)
            });
        let symbolic_target_reference_evidence = symbolic_target_reference_evidence(
            mutation_family,
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.symbolic_target_reference()),
            symbolic_target_reference.as_ref(),
            target_entity_identity.as_deref(),
        );
        let naming_mutation_evidence = naming_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.naming_mutation()),
            naming_intent.as_ref(),
            target_entity_identity.as_deref(),
            target_collection.as_deref(),
        );
        let continuity_mutation_evidence = continuity_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.continuity_mutation()),
            continuity_intent.as_ref(),
            existing_truth_binding.as_ref(),
            target_entity_identity.as_deref(),
            target_collection.as_deref(),
        );
        Self {
            inner,
            mutation_family,
            authority_lane,
            basis_lane,
            target_evidence,
            existing_truth_binding_evidence,
            symbolic_target_reference_evidence,
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence,
            provenance_evidence,
            declared_collection,
            declared_entity_identity,
            target_collection,
            target_entity_identity,
            declared_aspect_operations,
            mutation_metadata,
            affected_live_view_ids,
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

    pub fn mutation_metadata(&self) -> &ForgeQueryMutationMetadata {
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
