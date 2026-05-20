use crate::memory_workspace::ForgeQueryMutationReceipt;
use crate::runtime::mutation::ForgeQueryMutationMetadata;

use super::{ForgeQueryMutationFamily, ForgeQueryWriteCommand};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAuthorityLane,
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthAssertionEvidence, ForgeQueryExistingTruthBindingEvidence,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentExecutionProvenance, ForgeQueryMutationCausalityEvidence,
    ForgeQueryMutationProvenanceEvidence, ForgeQueryMutationTargetEvidence,
    ForgeQueryNamingMutationEvidence, ForgeQueryNamingMutationIntent,
    ForgeQuerySymbolicAspectResolutionEvidence, ForgeQuerySymbolicTargetReference,
    ForgeQuerySymbolicTargetReferenceEvidence, ForgeQueryVerifiedExistingTruthAssertion,
};

mod accessors;
mod helpers;
mod preview;

use helpers::{
    assertion_evidence, continuity_mutation_evidence, naming_mutation_evidence,
    symbolic_target_reference_evidence, target_evidence_from_receipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    pub(super) inner: ForgeQueryMutationReceipt,
    pub(super) mutation_family: ForgeQueryMutationFamily,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) basis_lane: ForgeQueryAuthorityLane,
    pub(super) target_evidence: ForgeQueryMutationTargetEvidence,
    pub(super) existing_truth_assertion_evidence: Option<ForgeQueryExistingTruthAssertionEvidence>,
    pub(super) existing_truth_binding_evidence: Option<ForgeQueryExistingTruthBindingEvidence>,
    pub(super) symbolic_target_reference_evidence:
        Option<ForgeQuerySymbolicTargetReferenceEvidence>,
    pub(super) symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
    pub(super) naming_mutation_evidence: Option<ForgeQueryNamingMutationEvidence>,
    pub(super) continuity_mutation_evidence: Option<ForgeQueryContinuityMutationEvidence>,
    pub(super) causality_evidence: Option<ForgeQueryMutationCausalityEvidence>,
    pub(super) provenance_evidence: Option<ForgeQueryMutationProvenanceEvidence>,
    pub(super) declared_collection: Option<String>,
    pub(super) declared_entity_identity: Option<String>,
    pub(super) target_collection: Option<String>,
    pub(super) target_entity_identity: Option<String>,
    pub(super) declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    pub(super) declared_aspect_value_digest: Option<String>,
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
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}

impl ForgeQueryWriteReceipt {
    pub(in crate::runtime) fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        target_collection: Option<String>,
        target_entity_identity: Option<String>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<String>,
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
        decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
        execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
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
        let existing_truth_assertion_evidence = assertion_evidence(
            mutation_family,
            existing_truth_binding.as_ref(),
            existing_truth_assertion.as_ref(),
            &declared_aspect_operations,
            declared_aspect_value_digest.as_deref(),
            &inner.snapshot_token,
        );
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
            existing_truth_assertion_evidence,
            existing_truth_binding_evidence,
            symbolic_target_reference_evidence,
            symbolic_aspect_resolution_evidence,
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence,
            provenance_evidence,
            declared_collection,
            declared_entity_identity,
            target_collection,
            target_entity_identity,
            declared_aspect_operations,
            declared_aspect_value_digest,
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
            decision_trace_envelope,
            execution_provenance,
        }
    }

    pub(in crate::runtime) fn batch_component(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        basis_lane: ForgeQueryAuthorityLane,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        target_collection: Option<String>,
        target_entity_identity: Option<String>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<String>,
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
        let existing_truth_assertion_evidence = assertion_evidence(
            mutation_family,
            existing_truth_binding.as_ref(),
            existing_truth_assertion.as_ref(),
            &declared_aspect_operations,
            declared_aspect_value_digest.as_deref(),
            &inner.snapshot_token,
        );
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
            existing_truth_assertion_evidence,
            existing_truth_binding_evidence,
            symbolic_target_reference_evidence,
            symbolic_aspect_resolution_evidence,
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence,
            provenance_evidence,
            declared_collection,
            declared_entity_identity,
            target_collection,
            target_entity_identity,
            declared_aspect_operations,
            declared_aspect_value_digest,
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
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }
}
