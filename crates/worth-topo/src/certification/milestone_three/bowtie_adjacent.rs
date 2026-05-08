use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::TopologyRelationKind;
use serde_json::Value;

use super::report::{
    MilestoneThreeBowtieAdjacentWitness, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    entity_id_from_query_identity, first_source_identity_for_relation_kind, rejected_step_row,
    relation_id_from_query_identity, replay_not_checked,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::edit::{TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract};
use crate::query::{
    topology_runtime, TopologyDomainQuery, TopologyQueryAssembly, TopologyRuntimeAdapters,
};

pub(crate) fn certify_milestone_three_bowtie_adjacent_rewire_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(&mut runtime, &format!("{stem}.bowtie_adjacent"), &primitive)?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.bowtie_adjacent.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let domain_query = TopologyDomainQuery::load();
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let source_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeRadialNext,
    )?;
    let source_half_edge_id = entity_id_from_query_identity(&source_identity)?;
    let bowtie_adjacent_witness =
        build_bowtie_adjacent_witness(&domain_query, &mut workspace, &source_identity)?;
    let radial = domain_query
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let batch = TopologyEditBatch::new(vec![TopologyEditContract::splice_radial_adjacency(
        relation_id_from_query_identity(radial.source_radial_next_relation_identity())?,
        source_half_edge_id,
        entity_id_from_query_identity(bowtie_adjacent_witness.target_half_edge_identity.as_str())?,
    )])
    .expect("milestone three hostile batches are non-empty");
    let batches = vec![batch.clone()];

    match assembly.apply_edit(
        &mut workspace,
        batch.clone(),
        TopologyEditApplicationMode::Mainline,
    ) {
        Ok(execution) => {
            let step_rows = vec![accepted_step_row(0, &batch, &execution)];
            Ok(MilestoneThreeHostileScenarioReport {
                scenario: MilestoneThreeHostileScenario::BowtieAdjacentRewire,
                primitive_family,
                primitive,
                edit_families: batch.families(),
                bowtie_adjacent_witness: Some(bowtie_adjacent_witness),
                ambiguous_local_rewire_witness: None,
                split_collapse_churn_witness: None,
                broken_radial_witness: None,
                topology_edit_digest: aggregate_topology_edit_digest(&batches),
                naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
                continuity_outcome_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .outcome_class(),
                continuity_rejection_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .rejection_class(),
                outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
                rejection_class: None,
                rejected_edit_scope_report: None,
                edit_replay_parity_report: replay_not_checked(step_rows),
                detail:
                    "bowtie-adjacent rewire committed successfully on the admitted runtime lane"
                        .to_string(),
            })
        }
        Err(error) => {
            let step_rows = vec![rejected_step_row(0, &batch, &error)];
            Ok(MilestoneThreeHostileScenarioReport {
                scenario: MilestoneThreeHostileScenario::BowtieAdjacentRewire,
                primitive_family,
                primitive,
                edit_families: batch.families(),
                bowtie_adjacent_witness: Some(bowtie_adjacent_witness),
                ambiguous_local_rewire_witness: None,
                split_collapse_churn_witness: None,
                broken_radial_witness: None,
                topology_edit_digest: aggregate_topology_edit_digest(&batches),
                naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
                continuity_outcome_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .outcome_class(),
                continuity_rejection_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .rejection_class(),
                outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
                rejection_class: error.rejection_class(),
                rejected_edit_scope_report: error.rejected_edit_scope_report(&batch),
                edit_replay_parity_report: replay_not_checked(step_rows),
                detail: error.to_string(),
            })
        }
    }
}

fn build_bowtie_adjacent_witness(
    domain_query: &TopologyDomainQuery,
    workspace: &mut ForgeQueryWorkspace,
    source_identity: &str,
) -> Result<MilestoneThreeBowtieAdjacentWitness, TopologyCertificationError> {
    let neighborhood = domain_query
        .shared_vertex_half_edge_neighborhood(workspace, source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let target = neighborhood
        .vertex_adjacent_different_edge_half_edges()
        .iter()
        .find(|candidate| !candidate.shared_vertex_identities().is_empty())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should provide a vertex-adjacent halfedge on a different edge"
                    .to_string(),
            )
        })?;
    let shared_vertex_identity = target
        .shared_vertex_identities()
        .first()
        .cloned()
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should expose a shared vertex for bowtie-adjacent witness"
                    .to_string(),
            )
        })?;
    let target_half_edge_identity = target.half_edge_identity().to_string();
    let target_edge_identity = target.edge_identity().to_string();
    Ok(MilestoneThreeBowtieAdjacentWitness {
        source_half_edge_identity: source_identity.to_string(),
        target_half_edge_identity,
        source_edge_identity: neighborhood.source_edge_identity,
        target_edge_identity,
        shared_vertex_identity,
    })
}
