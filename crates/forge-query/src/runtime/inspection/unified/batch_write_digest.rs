use crate::identity::hash_parts;

use super::ForgeQueryBatchWriteComponentInspection;
use crate::runtime::{
    ForgeQueryBatchMutationEvidence, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionEvidence, ForgeQueryGraphCompositionLifecycleOutcomes,
    ForgeQueryGraphCompositionProgram, ForgeQueryGraphCompositionResolutionMap,
    ForgeQueryMutationCausalityEvidence, ForgeQueryMutationProvenanceEvidence,
};

pub(super) struct ForgeQueryBatchWriteDigestInputs<'a> {
    pub authority_lane: &'a str,
    pub basis_lane: &'a str,
    pub batch_digest: &'a str,
    pub graph_composition_breadth: &'a ForgeQueryGraphCompositionBreadth,
    pub graph_composition_lifecycle_outcomes:
        Option<&'a ForgeQueryGraphCompositionLifecycleOutcomes>,
    pub graph_composition_program: Option<&'a ForgeQueryGraphCompositionProgram>,
    pub graph_composition_evidence: Option<&'a ForgeQueryGraphCompositionEvidence>,
    pub batch_mutation_evidence: &'a ForgeQueryBatchMutationEvidence,
    pub commit_identities: &'a [String],
    pub component_operations: &'a [ForgeQueryBatchWriteComponentInspection],
    pub graph_composition_resolution_map: &'a ForgeQueryGraphCompositionResolutionMap,
    pub touched_aspect_paths: &'a [String],
    pub affected_live_view_ids: &'a [String],
    pub affected_derived_view_ids: &'a [String],
}

pub(super) fn build_batch_write_receipt_inspection_digest(
    inputs: ForgeQueryBatchWriteDigestInputs<'_>,
) -> String {
    hash_parts(
        &std::iter::once("forge_query_batch_write_receipt_inspection_v1".to_string())
            .chain(std::iter::once(format!(
                "authority:{}",
                inputs.authority_lane
            )))
            .chain(std::iter::once(format!("basis:{}", inputs.basis_lane)))
            .chain(std::iter::once(format!("batch:{}", inputs.batch_digest)))
            .chain(std::iter::once(format!(
                "graph-breadth:{}:{}:{}:{}",
                inputs.graph_composition_breadth.component_count(),
                inputs
                    .graph_composition_breadth
                    .symbolic_entity_declaration_count(),
                inputs
                    .graph_composition_breadth
                    .symbolic_relation_declaration_count(),
                inputs.graph_composition_breadth.breadth_digest()
            )))
            .chain(std::iter::once(format!(
                "graph-lifecycle:{}:{}",
                inputs
                    .graph_composition_lifecycle_outcomes
                    .map_or("none", |outcomes| outcomes.lifecycle_digest()),
                inputs
                    .graph_composition_lifecycle_outcomes
                    .map_or("none", |outcomes| outcomes.counter_snapshot())
            )))
            .chain(std::iter::once(format!(
                "graph-program:{}",
                inputs
                    .graph_composition_program
                    .map_or("none", |program| program.program_digest())
            )))
            .chain(std::iter::once(format!(
                "graph-composition-evidence:{}:{}:{}:{}:{}:{}:{}",
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| evidence.graph_composition_digest()),
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| {
                        evidence.graph_symbolic_resolution_digest()
                    }),
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| evidence.counter_snapshot()),
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| {
                        evidence.graph_assumption_digest().unwrap_or("none")
                    }),
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| {
                        evidence.graph_lineage_digest().unwrap_or("none")
                    }),
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| {
                        evidence
                            .assumption_summary()
                            .map_or("none", |summary| summary.counter_snapshot())
                    }),
                inputs
                    .graph_composition_evidence
                    .map_or("none", |evidence| {
                        evidence
                            .lineage_summary()
                            .map_or("none", |summary| summary.counter_snapshot())
                    })
            )))
            .chain(std::iter::once(format!(
                "batch-evidence:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                inputs.batch_mutation_evidence.component_count(),
                inputs.batch_mutation_evidence.target_evidence_count(),
                inputs
                    .batch_mutation_evidence
                    .existing_truth_assertion_count(),
                inputs
                    .batch_mutation_evidence
                    .retained_authoritative_assertion_count(),
                inputs
                    .batch_mutation_evidence
                    .backend_verified_assertion_count(),
                inputs
                    .batch_mutation_evidence
                    .backend_verified_update_count(),
                inputs
                    .batch_mutation_evidence
                    .backend_verified_delete_count(),
                inputs
                    .batch_mutation_evidence
                    .existing_truth_binding_count(),
                inputs
                    .batch_mutation_evidence
                    .symbolic_target_reference_count(),
                inputs.batch_mutation_evidence.symbolic_resolution_count(),
                inputs.batch_mutation_evidence.naming_mutation_count(),
                inputs.batch_mutation_evidence.continuity_mutation_count(),
                inputs.batch_mutation_evidence.resolved_target_count(),
                inputs.batch_mutation_evidence.target_collection_count(),
                inputs.batch_mutation_evidence.target_entity_count(),
                inputs.batch_mutation_evidence.causality_bundle_count(),
                inputs.batch_mutation_evidence.provenance_bundle_count(),
                inputs.batch_mutation_evidence.outcome_class_count(),
                inputs.batch_mutation_evidence.authority_request_count(),
                inputs.batch_mutation_evidence.authority_receipt_count(),
                inputs.batch_mutation_evidence.aggregate_target_digest()
            )))
            .chain(std::iter::once(format!(
                "batch-assertion:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_existing_truth_assertion_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-existing-truth-mode:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_existing_truth_mode_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-continuity:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_continuity_mutation_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-existing:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_existing_truth_binding_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-symbolic:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_symbolic_target_reference_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-symbolic-resolution:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_symbolic_resolution_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-naming:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_naming_mutation_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-causality:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_causality_digest()
                    .unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "batch-provenance:{}",
                inputs
                    .batch_mutation_evidence
                    .aggregate_provenance_digest()
                    .unwrap_or("none")
            )))
            .chain(
                inputs
                    .commit_identities
                    .iter()
                    .map(|commit| format!("commit:{commit}")),
            )
            .chain(inputs.component_operations.iter().flat_map(|component| {
                std::iter::once(format!("family:{}", component.family()))
                    .chain(std::iter::once(format!(
                        "component-target:{}:{}:{}:{}:{}:{}",
                        component.target_evidence().declared().target_class(),
                        component
                            .target_evidence()
                            .declared()
                            .collection()
                            .unwrap_or(""),
                        component
                            .target_evidence()
                            .declared()
                            .entity_identity()
                            .unwrap_or(""),
                        component.target_evidence().resolved().target_class(),
                        component
                            .target_evidence()
                            .resolved()
                            .collection()
                            .unwrap_or(""),
                        component
                            .target_evidence()
                            .resolved()
                            .entity_identity()
                            .unwrap_or("")
                    )))
                    .chain(std::iter::once(format!(
                        "component-assertion:{}",
                        component
                            .existing_truth_assertion_evidence()
                            .map_or("none", |evidence| evidence.verification_digest())
                    )))
                    .chain(std::iter::once(format!(
                        "component-existing-truth:{}:{}:{}:{}:{}",
                        component
                            .existing_truth_binding_evidence()
                            .map_or("none", |evidence| evidence.family().as_str()),
                        component
                            .existing_truth_binding_evidence()
                            .map_or("none", |evidence| evidence.authoritative_identity()),
                        component
                            .existing_truth_binding_evidence()
                            .map_or("none", |evidence| evidence.resolved_target_identity()),
                        component
                            .existing_truth_binding_evidence()
                            .and_then(|evidence| evidence.target_collection())
                            .unwrap_or("none"),
                        component
                            .existing_truth_binding_evidence()
                            .map_or("none", |evidence| evidence.binding_digest())
                    )))
                    .chain(std::iter::once(format!(
                        "component-symbolic:{}",
                        component
                            .symbolic_target_reference_evidence()
                            .map_or("none", |evidence| evidence.symbol())
                    )))
                    .chain(
                        component
                            .symbolic_aspect_resolution_evidence()
                            .iter()
                            .map(|evidence| {
                                format!(
                                    "component-symbolic-aspect:{}:{}:{}:{}",
                                    evidence.aspect_path(),
                                    evidence.symbol(),
                                    evidence.resolved_entity_identity(),
                                    evidence.target_collection().unwrap_or("none")
                                )
                            }),
                    )
                    .chain(std::iter::once(format!(
                        "component-causality:{}",
                        component.causality_evidence().map_or(
                            "none",
                            ForgeQueryMutationCausalityEvidence::causality_digest
                        )
                    )))
                    .chain(std::iter::once(format!(
                        "component-continuity:{}:{}",
                        component
                            .continuity_mutation_evidence()
                            .map_or("none", |evidence| evidence.lineage_digest()),
                        component
                            .continuity_mutation_evidence()
                            .map_or("none", |evidence| {
                                evidence.continuity_resolution_digest()
                            })
                    )))
                    .chain(std::iter::once(format!(
                        "component-provenance:{}",
                        component.provenance_evidence().map_or(
                            "none",
                            ForgeQueryMutationProvenanceEvidence::execution_record_digest
                        )
                    )))
                    .chain(
                        component
                            .collections()
                            .iter()
                            .map(|collection| format!("collection:{collection}")),
                    )
                    .chain(
                        component
                            .entity_identities()
                            .iter()
                            .map(|entity| format!("entity:{entity}")),
                    )
                    .chain(
                        component
                            .declared_aspect_operations()
                            .iter()
                            .map(|operation| {
                                format!(
                                    "component-operation:{}:{}",
                                    operation.kind(),
                                    operation.aspect_path()
                                )
                            }),
                    )
                    .chain(
                        component
                            .touched_aspect_paths()
                            .iter()
                            .map(|path| format!("component-aspect:{path}")),
                    )
            }))
            .chain(
                inputs
                    .graph_composition_resolution_map
                    .entries()
                    .iter()
                    .map(|entry| {
                        format!(
                            "graph-resolution:{}:{}:{}:{}:{}",
                            entry.component_index(),
                            entry.aspect_path().unwrap_or("target"),
                            entry.symbol(),
                            entry.resolved_entity_identity(),
                            entry.target_collection().unwrap_or("none")
                        )
                    }),
            )
            .chain(
                inputs
                    .touched_aspect_paths
                    .iter()
                    .map(|path| format!("aspect:{path}")),
            )
            .chain(
                inputs
                    .affected_live_view_ids
                    .iter()
                    .map(|view| format!("live:{view}")),
            )
            .chain(
                inputs
                    .affected_derived_view_ids
                    .iter()
                    .map(|view| format!("derived:{view}")),
            )
            .collect::<Vec<_>>(),
    )
}
