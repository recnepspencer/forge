use super::ForgeQueryBatchWriteComponentInspection;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryCommitIdentity;
use crate::runtime::{
    ForgeQueryBatchMutationEvidence, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionEvidence, ForgeQueryGraphCompositionLifecycleOutcomes,
    ForgeQueryGraphCompositionProgram, ForgeQueryGraphCompositionResolutionMap,
    ForgeQueryGraphObligationAttachmentEvidence,
};

#[path = "batch_write_digest_components.rs"]
mod batch_write_digest_components;

pub(super) struct ForgeQueryBatchWriteDigestInputs<'a> {
    pub authority_lane: &'a str,
    pub basis_lane: &'a str,
    pub batch_digest: &'a ForgeQueryEvidenceIdentity,
    pub graph_composition_breadth: &'a ForgeQueryGraphCompositionBreadth,
    pub graph_composition_lifecycle_outcomes:
        Option<&'a ForgeQueryGraphCompositionLifecycleOutcomes>,
    pub graph_composition_program: Option<&'a ForgeQueryGraphCompositionProgram>,
    pub graph_composition_evidence: Option<&'a ForgeQueryGraphCompositionEvidence>,
    pub batch_mutation_evidence: &'a ForgeQueryBatchMutationEvidence,
    pub commit_identities: &'a [ForgeQueryCommitIdentity],
    pub journal_position_identities: &'a [ForgeQueryEvidenceIdentity],
    pub component_operations: &'a [ForgeQueryBatchWriteComponentInspection],
    pub graph_composition_resolution_map: &'a ForgeQueryGraphCompositionResolutionMap,
    pub graph_obligation_evidence: Option<&'a ForgeQueryGraphObligationAttachmentEvidence>,
    pub touched_aspect_paths: &'a [String],
    pub affected_live_view_ids: &'a [String],
    pub affected_derived_view_ids: &'a [String],
}

pub(super) fn build_batch_write_receipt_inspection_digest(
    inputs: ForgeQueryBatchWriteDigestInputs<'_>,
) -> ForgeQueryEvidenceIdentity {
    let component_artifact_identities =
        batch_write_digest_components::component_artifact_identities(inputs.component_operations);
    let graph_resolution_identities = batch_write_digest_components::graph_resolution_identities(
        inputs.graph_composition_resolution_map,
    );
    let commit_identities = inputs
        .commit_identities
        .iter()
        .map(|identity| identity.evidence_identity())
        .collect::<Vec<_>>();

    forge_query_evidence_identity(ForgeQueryEvidenceScope::BatchWriteReceiptInspectionArtifact)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            inputs.authority_lane,
        )
        .field_shape(ForgeQueryEvidenceTag::new("basis_lane"), inputs.basis_lane)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_digest"),
            inputs.batch_digest,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("graph_component_count"),
            inputs.graph_composition_breadth.component_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("graph_symbolic_entity_declaration_count"),
            inputs
                .graph_composition_breadth
                .symbolic_entity_declaration_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("graph_symbolic_relation_declaration_count"),
            inputs
                .graph_composition_breadth
                .symbolic_relation_declaration_count(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_breadth_digest"),
            inputs.graph_composition_breadth.breadth_evidence_digest(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_lifecycle_digest"),
            inputs
                .graph_composition_lifecycle_outcomes
                .map(ForgeQueryGraphCompositionLifecycleOutcomes::lifecycle_evidence_digest),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_lifecycle_counter_snapshot"),
            inputs
                .graph_composition_lifecycle_outcomes
                .map(ForgeQueryGraphCompositionLifecycleOutcomes::counter_snapshot),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_program_digest"),
            inputs
                .graph_composition_program
                .map(ForgeQueryGraphCompositionProgram::program_evidence_digest),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_composition_digest"),
            inputs
                .graph_composition_evidence
                .map(ForgeQueryGraphCompositionEvidence::graph_composition_evidence_digest),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_symbolic_resolution_digest"),
            inputs
                .graph_composition_evidence
                .map(ForgeQueryGraphCompositionEvidence::graph_symbolic_resolution_evidence_digest),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_counter_snapshot"),
            inputs
                .graph_composition_evidence
                .map(ForgeQueryGraphCompositionEvidence::counter_snapshot),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_assumption_digest"),
            inputs
                .graph_composition_evidence
                .and_then(ForgeQueryGraphCompositionEvidence::graph_assumption_evidence_digest),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("graph_lineage_digest"),
            inputs
                .graph_composition_evidence
                .and_then(ForgeQueryGraphCompositionEvidence::graph_lineage_evidence_digest),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_assumption_summary_counter"),
            inputs.graph_composition_evidence.and_then(|evidence| {
                evidence
                    .assumption_summary()
                    .map(|summary| summary.counter_snapshot())
            }),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_lineage_summary_counter"),
            inputs.graph_composition_evidence.and_then(|evidence| {
                evidence
                    .lineage_summary()
                    .map(|summary| summary.counter_snapshot())
            }),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_component_count"),
            inputs.batch_mutation_evidence.component_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_target_evidence_count"),
            inputs.batch_mutation_evidence.target_evidence_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_existing_truth_assertion_count"),
            inputs
                .batch_mutation_evidence
                .existing_truth_assertion_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_retained_authoritative_assertion_count"),
            inputs
                .batch_mutation_evidence
                .retained_authoritative_assertion_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_backend_verified_assertion_count"),
            inputs
                .batch_mutation_evidence
                .backend_verified_assertion_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_backend_verified_update_count"),
            inputs
                .batch_mutation_evidence
                .backend_verified_update_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_backend_verified_delete_count"),
            inputs
                .batch_mutation_evidence
                .backend_verified_delete_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_existing_truth_binding_count"),
            inputs
                .batch_mutation_evidence
                .existing_truth_binding_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_symbolic_target_reference_count"),
            inputs
                .batch_mutation_evidence
                .symbolic_target_reference_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_symbolic_resolution_count"),
            inputs.batch_mutation_evidence.symbolic_resolution_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_naming_mutation_count"),
            inputs.batch_mutation_evidence.naming_mutation_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_continuity_mutation_count"),
            inputs.batch_mutation_evidence.continuity_mutation_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_resolved_target_count"),
            inputs.batch_mutation_evidence.resolved_target_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_target_collection_count"),
            inputs.batch_mutation_evidence.target_collection_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_target_entity_count"),
            inputs.batch_mutation_evidence.target_entity_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_causality_bundle_count"),
            inputs.batch_mutation_evidence.causality_bundle_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_provenance_bundle_count"),
            inputs.batch_mutation_evidence.provenance_bundle_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_outcome_class_count"),
            inputs.batch_mutation_evidence.outcome_class_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_authority_request_count"),
            inputs.batch_mutation_evidence.authority_request_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("batch_authority_receipt_count"),
            inputs.batch_mutation_evidence.authority_receipt_count(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_target_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_target_digest()
                .evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_existing_truth_assertion_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_existing_truth_assertion_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_existing_truth_mode_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_existing_truth_mode_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_continuity_mutation_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_continuity_mutation_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_existing_truth_binding_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_existing_truth_binding_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_symbolic_target_reference_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_symbolic_target_reference_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_symbolic_resolution_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_symbolic_resolution_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_naming_mutation_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_naming_mutation_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_causality_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_causality_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("batch_aggregate_provenance_digest"),
            inputs
                .batch_mutation_evidence
                .aggregate_provenance_digest()
                .map(|digest| digest.evidence_identity()),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("commit_identity"),
            commit_identities.iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("journal_position_identity"),
            inputs.journal_position_identities.iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("component_artifact_identity"),
            component_artifact_identities.iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("graph_resolution_identity"),
            graph_resolution_identities.iter(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_obligation_evidence"),
            inputs
                .graph_obligation_evidence
                .map(ForgeQueryGraphObligationAttachmentEvidence::evidence_digest),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("touched_aspect_path"),
            evidence_value_identities(
                "batch-inspection-touched-aspect-path",
                inputs.touched_aspect_paths,
            )
            .iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("affected_live_view_id"),
            evidence_value_identities(
                "batch-inspection-affected-live-view",
                inputs.affected_live_view_ids,
            )
            .iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("affected_derived_view_id"),
            evidence_value_identities(
                "batch-inspection-affected-derived-view",
                inputs.affected_derived_view_ids,
            )
            .iter(),
        )
        .seal()
}

fn evidence_value_identities(
    role: &'static str,
    values: &[String],
) -> Vec<ForgeQueryEvidenceIdentity> {
    values
        .iter()
        .map(|value| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::BatchWriteReceiptInspectionArtifact,
            )
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_value(ForgeQueryEvidenceTag::new("value"), value)
            .seal()
        })
        .collect()
}
