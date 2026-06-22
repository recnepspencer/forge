use crate::memory_workspace::ForgeQueryMutationReceipt;
use crate::runtime::mutation::ForgeQueryMutationMetadata;

use super::{ForgeQueryMutationFamily, ForgeQueryWriteCommand};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::runtime::ForgeQueryJournalPosition;
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryAuthorityLane,
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent,
    ForgeQueryDerivedMaterializationTarget, ForgeQueryExistingTruthAssertionEvidence,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryIntentExecutionProvenance,
    ForgeQueryLiveArtifactTarget, ForgeQueryMutationCausalityEvidence,
    ForgeQueryMutationProvenanceEvidence, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryMutationTargetEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQueryNamingMutationIntent, ForgeQuerySymbolicAspectResolutionEvidence,
    ForgeQuerySymbolicTargetReference, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQueryVerifiedExistingTruthAssertion,
};

mod accessors;
mod helpers;
mod preview;

use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQueryMutationKind,
    ForgeQuerySnapshotIdentity,
};
use helpers::{
    assertion_evidence, continuity_mutation_evidence, naming_mutation_evidence,
    symbolic_target_reference_evidence, target_evidence_from_receipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    pub(super) inner: ForgeQueryMutationReceipt,
    pub(super) commit_evidence_identity: ForgeQueryEvidenceIdentity,
    pub(super) committed_truth_identity: ForgeQueryEvidenceIdentity,
    pub(super) journal_position: ForgeQueryJournalPosition,
    pub(super) snapshot_evidence_identity: ForgeQueryEvidenceIdentity,
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
    pub(super) declared_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
    pub(super) declared_entity_identity: Option<ForgeQueryEntityIdentity>,
    pub(super) target_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
    pub(super) target_entity_identity: Option<ForgeQueryEntityIdentity>,
    pub(super) declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    pub(super) declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
    pub(super) mutation_metadata: ForgeQueryMutationMetadata,
    pub(super) affected_live_view_targets: Vec<ForgeQueryLiveArtifactTarget>,
    pub(super) affected_derived_view_targets: Vec<ForgeQueryDerivedMaterializationTarget>,
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
    pub(super) obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
}

impl ForgeQueryWriteReceipt {
    pub(in crate::runtime) fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
        declared_entity_identity: Option<ForgeQueryEntityIdentity>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        target_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
        target_entity_identity: Option<ForgeQueryEntityIdentity>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
        mutation_metadata: ForgeQueryMutationMetadata,
        affected_live_view_targets: Vec<ForgeQueryLiveArtifactTarget>,
        affected_derived_view_targets: Vec<ForgeQueryDerivedMaterializationTarget>,
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
        obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    ) -> Self {
        let commit_evidence_identity =
            write_receipt_commit_evidence_identity(&inner.commit_identity);
        let committed_truth_identity = write_receipt_committed_truth_identity(&inner);
        let journal_position =
            ForgeQueryJournalPosition::from_commit_identity(&inner.commit_identity);
        let snapshot_evidence_identity =
            write_receipt_snapshot_evidence_identity(&inner.snapshot_identity);
        let target_evidence = target_evidence_from_receipt(
            mutation_family,
            declared_collection_identity.clone(),
            declared_entity_identity.clone(),
            target_collection_identity.clone(),
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
        let existing_truth_binding_evidence = existing_truth_binding
            .as_ref()
            .map(ForgeQueryExistingTruthBindingEvidence::from_binding)
            .or_else(|| {
                inner
                    .bridge_authority
                    .as_ref()
                    .and_then(|bundle| bundle.existing_truth_binding())
                    .map(ForgeQueryExistingTruthBindingEvidence::from_bridge)
            });
        let existing_truth_assertion_evidence = assertion_evidence(
            mutation_family,
            existing_truth_binding.as_ref(),
            existing_truth_assertion.as_ref(),
            &declared_aspect_operations,
            declared_aspect_value_digest.as_ref(),
            &inner.snapshot_identity,
        );
        let symbolic_target_reference_evidence = symbolic_target_reference_evidence(
            mutation_family,
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.symbolic_target_reference()),
            symbolic_target_reference.as_ref(),
            target_entity_identity.as_ref(),
        );
        let naming_mutation_evidence = naming_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.naming_mutation()),
            naming_intent.as_ref(),
            target_entity_identity.as_ref(),
            target_collection_identity
                .as_ref()
                .map(ForgeQueryMutationTargetCollectionIdentity::as_str),
        );
        let continuity_mutation_evidence = continuity_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.continuity_mutation()),
            continuity_intent.as_ref(),
            existing_truth_binding.as_ref(),
            target_entity_identity.as_ref(),
            target_collection_identity
                .as_ref()
                .map(ForgeQueryMutationTargetCollectionIdentity::as_str),
        );
        Self {
            inner,
            commit_evidence_identity,
            committed_truth_identity,
            journal_position,
            snapshot_evidence_identity,
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
            declared_collection_identity,
            declared_entity_identity,
            target_collection_identity,
            target_entity_identity,
            declared_aspect_operations,
            declared_aspect_value_digest,
            mutation_metadata,
            affected_live_view_targets,
            affected_derived_view_targets,
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
            obligation_dispatch,
        }
    }

    pub(in crate::runtime) fn batch_component(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        basis_lane: ForgeQueryAuthorityLane,
        declared_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
        declared_entity_identity: Option<ForgeQueryEntityIdentity>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        target_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
        target_entity_identity: Option<ForgeQueryEntityIdentity>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
        mutation_metadata: ForgeQueryMutationMetadata,
        affected_live_view_targets: Vec<ForgeQueryLiveArtifactTarget>,
        authority_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        let commit_evidence_identity =
            write_receipt_commit_evidence_identity(&inner.commit_identity);
        let committed_truth_identity = write_receipt_committed_truth_identity(&inner);
        let journal_position =
            ForgeQueryJournalPosition::from_commit_identity(&inner.commit_identity);
        let snapshot_evidence_identity =
            write_receipt_snapshot_evidence_identity(&inner.snapshot_identity);
        let target_evidence = target_evidence_from_receipt(
            mutation_family,
            declared_collection_identity.clone(),
            declared_entity_identity.clone(),
            target_collection_identity.clone(),
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
        let existing_truth_binding_evidence = existing_truth_binding
            .as_ref()
            .map(ForgeQueryExistingTruthBindingEvidence::from_binding)
            .or_else(|| {
                inner
                    .bridge_authority
                    .as_ref()
                    .and_then(|bundle| bundle.existing_truth_binding())
                    .map(ForgeQueryExistingTruthBindingEvidence::from_bridge)
            });
        let existing_truth_assertion_evidence = assertion_evidence(
            mutation_family,
            existing_truth_binding.as_ref(),
            existing_truth_assertion.as_ref(),
            &declared_aspect_operations,
            declared_aspect_value_digest.as_ref(),
            &inner.snapshot_identity,
        );
        let symbolic_target_reference_evidence = symbolic_target_reference_evidence(
            mutation_family,
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.symbolic_target_reference()),
            symbolic_target_reference.as_ref(),
            target_entity_identity.as_ref(),
        );
        let naming_mutation_evidence = naming_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.naming_mutation()),
            naming_intent.as_ref(),
            target_entity_identity.as_ref(),
            target_collection_identity
                .as_ref()
                .map(ForgeQueryMutationTargetCollectionIdentity::as_str),
        );
        let continuity_mutation_evidence = continuity_mutation_evidence(
            inner
                .bridge_authority
                .as_ref()
                .and_then(|bundle| bundle.continuity_mutation()),
            continuity_intent.as_ref(),
            existing_truth_binding.as_ref(),
            target_entity_identity.as_ref(),
            target_collection_identity
                .as_ref()
                .map(ForgeQueryMutationTargetCollectionIdentity::as_str),
        );
        Self {
            inner,
            commit_evidence_identity,
            committed_truth_identity,
            journal_position,
            snapshot_evidence_identity,
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
            declared_collection_identity,
            declared_entity_identity,
            target_collection_identity,
            target_entity_identity,
            declared_aspect_operations,
            declared_aspect_value_digest,
            mutation_metadata,
            affected_live_view_targets,
            affected_derived_view_targets: Vec::new(),
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
            obligation_dispatch: None,
        }
    }
}

fn write_receipt_commit_evidence_identity(
    commit_identity: &ForgeQueryCommitIdentity,
) -> ForgeQueryEvidenceIdentity {
    commit_identity.evidence_identity()
}

fn write_receipt_snapshot_evidence_identity(
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> ForgeQueryEvidenceIdentity {
    snapshot_identity.evidence_identity()
}

fn write_receipt_committed_truth_identity(
    receipt: &ForgeQueryMutationReceipt,
) -> ForgeQueryEvidenceIdentity {
    let delta_descriptors = receipt
        .deltas
        .iter()
        .map(|delta| {
            format!(
                "{}:{}:{}",
                delta.collection(),
                mutation_kind_as_str(delta.kind()),
                touched_aspect_digest_parts(delta.admitted_touched_aspects()).join("|")
            )
        })
        .collect::<Vec<_>>();
    crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::JournalReplayOutcome,
    )
    .field_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("committed_write_identity"),
        &receipt.commit_identity.evidence_identity(),
    )
    .field_value_sequence(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("committed_delta_descriptor"),
        delta_descriptors.iter().map(String::as_str),
    )
    .seal()
}

fn touched_aspect_digest_parts(touches: &[ForgeQueryAspectTouch]) -> Vec<String> {
    touches
        .iter()
        .map(ForgeQueryAspectTouch::admitted_touch_digest_part)
        .collect()
}

fn mutation_kind_as_str(kind: &ForgeQueryMutationKind) -> &'static str {
    match kind {
        ForgeQueryMutationKind::Created => "created",
        ForgeQueryMutationKind::Updated => "updated",
        ForgeQueryMutationKind::Deleted => "deleted",
    }
}
