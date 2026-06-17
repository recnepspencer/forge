use crate::evidence_identity::{ForgeQueryEvidenceIdentityEncoder, ForgeQueryEvidenceTag};
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityOutcomeClass,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryMutationCausalityEvidence,
    ForgeQueryMutationProvenanceEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQuerySymbolicTargetReferenceEvidence,
};

pub(super) fn append_mutation_evidence_fields(
    encoder: ForgeQueryEvidenceIdentityEncoder,
    existing_truth_binding_evidence: Option<&ForgeQueryExistingTruthBindingEvidence>,
    symbolic_target_reference_evidence: Option<&ForgeQuerySymbolicTargetReferenceEvidence>,
    naming_mutation_evidence: Option<&ForgeQueryNamingMutationEvidence>,
    continuity_mutation_evidence: Option<&ForgeQueryContinuityMutationEvidence>,
    causality_evidence: Option<&ForgeQueryMutationCausalityEvidence>,
    provenance_evidence: Option<&ForgeQueryMutationProvenanceEvidence>,
) -> ForgeQueryEvidenceIdentityEncoder {
    let symbolic_target_resolved_entity_identity = symbolic_target_reference_evidence
        .map(ForgeQuerySymbolicTargetReferenceEvidence::resolved_entity_identity)
        .map(|identity| identity.evidence_identity());
    let naming_resolved_target_entity_identity = naming_mutation_evidence
        .and_then(ForgeQueryNamingMutationEvidence::resolved_target_entity_identity)
        .map(|identity| identity.evidence_identity());
    let continuity_resolved_target_entity_identity = continuity_mutation_evidence
        .and_then(ForgeQueryContinuityMutationEvidence::resolved_target_entity_identity)
        .map(|identity| identity.evidence_identity());
    let existing_truth_resolved_entity_identity = existing_truth_binding_evidence
        .map(ForgeQueryExistingTruthBindingEvidence::resolved_entity_identity)
        .map(|identity| identity.evidence_identity());
    let successor_authoritative_identities = continuity_mutation_evidence
        .map(|evidence| evidence.successor_authoritative_identities().to_vec())
        .unwrap_or_default();
    let provenance_outcome_class = provenance_evidence
        .and_then(|evidence| evidence.outcome_class().map(|value| format!("{value:?}")));
    let provenance_failure_class = provenance_evidence
        .and_then(|evidence| evidence.failure_class().map(|value| format!("{value:?}")));

    encoder
        .optional_shape(
            ForgeQueryEvidenceTag::new("existing_truth_family"),
            existing_truth_binding_evidence.map(|evidence| evidence.family().as_str()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_authoritative_identity"),
            existing_truth_binding_evidence
                .map(ForgeQueryExistingTruthBindingEvidence::authoritative_identity)
                .map(|identity| identity.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_resolved_entity_identity"),
            existing_truth_resolved_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_target_collection"),
            existing_truth_binding_evidence
                .and_then(ForgeQueryExistingTruthBindingEvidence::target_collection)
                .map(|collection| collection.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_binding_digest"),
            existing_truth_binding_evidence
                .map(ForgeQueryExistingTruthBindingEvidence::binding_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("symbolic_target_symbol"),
            symbolic_target_reference_evidence
                .map(ForgeQuerySymbolicTargetReferenceEvidence::symbol)
                .map(|symbol| symbol.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("symbolic_target_resolved_entity_identity"),
            symbolic_target_resolved_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("symbolic_target_collection"),
            symbolic_target_reference_evidence
                .and_then(ForgeQuerySymbolicTargetReferenceEvidence::target_collection)
                .map(|collection| collection.evidence_identity()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("naming_family"),
            naming_mutation_evidence.map(|evidence| evidence.family().as_str()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("naming_attachment_identity"),
            naming_mutation_evidence
                .map(ForgeQueryNamingMutationEvidence::attachment_identity)
                .map(|identity| identity.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("naming_prior_authoritative_identity"),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::prior_authoritative_identity)
                .map(|identity| identity.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("naming_target_authoritative_identity"),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::target_authoritative_identity)
                .map(|identity| identity.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("naming_resolved_target_entity_identity"),
            naming_resolved_target_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("naming_target_collection"),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::target_collection)
                .map(|collection| collection.evidence_identity()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("naming_outcome"),
            naming_mutation_evidence.map(|evidence| evidence.outcome().as_str()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("continuity_family"),
            continuity_mutation_evidence.map(|evidence| evidence.family().as_str()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_prior_authoritative_identity"),
            continuity_mutation_evidence
                .map(ForgeQueryContinuityMutationEvidence::prior_authoritative_identity)
                .map(|identity| identity.evidence_identity()),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("continuity_successor_authoritative_identity"),
            successor_authoritative_identities
                .iter()
                .map(|identity| identity.evidence_identity()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("continuity_outcome_class"),
            continuity_mutation_evidence.map(continuity_outcome_label),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_basis_binding_digest"),
            continuity_mutation_evidence
                .and_then(ForgeQueryContinuityMutationEvidence::basis_binding_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_resolved_target_entity_identity"),
            continuity_resolved_target_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_target_collection"),
            continuity_mutation_evidence
                .and_then(ForgeQueryContinuityMutationEvidence::target_collection)
                .map(|collection| collection.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_lineage_digest"),
            continuity_mutation_evidence
                .map(ForgeQueryContinuityMutationEvidence::lineage_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_resolution_digest"),
            continuity_mutation_evidence
                .map(ForgeQueryContinuityMutationEvidence::continuity_resolution_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("causality_digest"),
            causality_evidence
                .map(ForgeQueryMutationCausalityEvidence::causality_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("causality_truth_trigger_digest"),
            causality_evidence
                .map(ForgeQueryMutationCausalityEvidence::truth_trigger_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("causality_route_digest"),
            causality_evidence
                .map(ForgeQueryMutationCausalityEvidence::route_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("causality_evaluation_surface_digest"),
            causality_evidence
                .map(ForgeQueryMutationCausalityEvidence::evaluation_surface_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("causality_truth_view_digest"),
            causality_evidence
                .map(ForgeQueryMutationCausalityEvidence::truth_view_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_contract_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::contract_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_writeback_effect_artifact_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::writeback_effect_artifact_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_effect_intent_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::effect_intent_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_effect_intent_patch_canonical_basis"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::effect_intent_patch_canonical_basis)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_feedback_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::feedback_provenance_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_causality_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::causality_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_strategy_descriptor_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::strategy_descriptor_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_execution_record_digest"),
            provenance_evidence
                .map(ForgeQueryMutationProvenanceEvidence::execution_record_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_authoritative_artifact_digest"),
            provenance_evidence
                .and_then(ForgeQueryMutationProvenanceEvidence::authoritative_artifact_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_request_digest"),
            provenance_evidence
                .and_then(ForgeQueryMutationProvenanceEvidence::request_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_receipt_digest"),
            provenance_evidence
                .and_then(ForgeQueryMutationProvenanceEvidence::receipt_digest)
                .map(|digest| digest.evidence_identity()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("provenance_outcome_class"),
            provenance_outcome_class.as_deref(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("provenance_failure_class"),
            provenance_failure_class.as_deref(),
        )
}

fn continuity_outcome_label(evidence: &ForgeQueryContinuityMutationEvidence) -> &'static str {
    match evidence.outcome_class() {
        ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
            "continues_as_single_successor"
        }
        ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
            "continues_as_split_successors"
        }
        ForgeQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            "continues_via_truth_lowered_canonical_merge_successor"
        }
        ForgeQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor => {
            "rejected_no_authoritative_successor"
        }
        ForgeQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
            "rejected_ambiguous_successor"
        }
        ForgeQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass => {
            "rejected_unsupported_continuity_class"
        }
        ForgeQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
            "rejected_historical_resolution_failure"
        }
    }
}
