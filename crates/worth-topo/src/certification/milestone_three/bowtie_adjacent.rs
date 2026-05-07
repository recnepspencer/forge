use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

use super::report::{
    WorthMilestoneThreeBowtieAdjacentWitness, WorthMilestoneThreeHostileOutcomeClass,
    WorthMilestoneThreeHostileScenario, WorthMilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    rejected_step_row, replay_not_checked,
};
use crate::certification::error::WorthTopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditBatch, WorthTopologyEditContract,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyDomainQuery, WorthTopologyQueryAssembly,
    WorthTopologyRuntimeAdapters,
};

pub(crate) fn certify_milestone_three_bowtie_adjacent_rewire_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneThreeHostileScenarioReport, WorthTopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(&mut runtime, &format!("{stem}.bowtie_adjacent"), &primitive)?;
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, &format!("{stem}.bowtie_adjacent.runtime"))
            .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let domain_query = WorthTopologyDomainQuery::load(&workspace, &assembly)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let source_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeRadialNext)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let source_half_edge_id = domain_query
        .find_entity_id_by_identity(&source_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let bowtie_adjacent_witness =
        build_bowtie_adjacent_witness(&domain_query, &mut workspace, &source_identity)?;
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            domain_query
                .relation_id_for_source_kind(
                    &source_identity,
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                )
                .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?,
            source_half_edge_id,
            domain_query
                .find_entity_id_by_identity(
                    bowtie_adjacent_witness.target_half_edge_identity.as_str(),
                )
                .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?,
        )])
        .expect("milestone three hostile batches are non-empty");
    let batches = vec![batch.clone()];

    match assembly.apply_edit(
        &mut workspace,
        batch.clone(),
        WorthTopologyEditApplicationMode::Mainline,
    ) {
        Ok(execution) => {
            let step_rows = vec![accepted_step_row(0, &batch, &execution)];
            Ok(WorthMilestoneThreeHostileScenarioReport {
                scenario: WorthMilestoneThreeHostileScenario::BowtieAdjacentRewire,
                primitive_family,
                primitive,
                edit_families: batch.families(),
                bowtie_adjacent_witness: Some(bowtie_adjacent_witness),
                ambiguous_local_rewire_witness: None,
                broken_radial_witness: None,
                topology_edit_digest: aggregate_topology_edit_digest(&batches),
                naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
                continuity_outcome_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .outcome_class(),
                continuity_rejection_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .rejection_class(),
                outcome_class: WorthMilestoneThreeHostileOutcomeClass::Accepted,
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
            Ok(WorthMilestoneThreeHostileScenarioReport {
                scenario: WorthMilestoneThreeHostileScenario::BowtieAdjacentRewire,
                primitive_family,
                primitive,
                edit_families: batch.families(),
                bowtie_adjacent_witness: Some(bowtie_adjacent_witness),
                ambiguous_local_rewire_witness: None,
                broken_radial_witness: None,
                topology_edit_digest: aggregate_topology_edit_digest(&batches),
                naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
                continuity_outcome_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .outcome_class(),
                continuity_rejection_class: aggregate_naming_edit_continuity_matrix(&batches)
                    .rejection_class(),
                outcome_class: WorthMilestoneThreeHostileOutcomeClass::Rejected,
                rejection_class: error.rejection_class(),
                rejected_edit_scope_report: error.rejected_edit_scope_report(&batch),
                edit_replay_parity_report: replay_not_checked(step_rows),
                detail: error.to_string(),
            })
        }
    }
}

fn build_bowtie_adjacent_witness(
    domain_query: &WorthTopologyDomainQuery,
    workspace: &mut ForgeQueryWorkspace,
    source_identity: &str,
) -> Result<WorthMilestoneThreeBowtieAdjacentWitness, WorthTopologyCertificationError> {
    let neighborhood = domain_query
        .shared_vertex_half_edge_neighborhood(workspace, source_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let target_half_edge_identity = neighborhood
        .vertex_adjacent_different_edge_half_edge_identities
        .iter()
        .find(|identity| {
            domain_query
                .half_edge_vertex_identities(identity)
                .is_ok_and(|candidate_vertices| {
                    candidate_vertices
                        .iter()
                        .any(|candidate| neighborhood.source_vertex_identities.contains(candidate))
                })
        })
        .cloned()
        .ok_or_else(|| {
            WorthTopologyCertificationError::Query(
                "seeded edge fan should provide a vertex-adjacent halfedge on a different edge"
                    .to_string(),
            )
        })?;
    let target_edge_identity = domain_query
        .edge_identity_of_half_edge(target_half_edge_identity.as_str())
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let target_vertex_identities = domain_query
        .half_edge_vertex_identities(target_half_edge_identity.as_str())
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let shared_vertex_identity = neighborhood
        .source_vertex_identities
        .iter()
        .find(|candidate| target_vertex_identities.contains(candidate))
        .cloned()
        .ok_or_else(|| {
            WorthTopologyCertificationError::Query(
                "seeded edge fan should expose a shared vertex for bowtie-adjacent witness"
                    .to_string(),
            )
        })?;
    Ok(WorthMilestoneThreeBowtieAdjacentWitness {
        source_half_edge_identity: source_identity.to_string(),
        target_half_edge_identity,
        source_edge_identity: neighborhood.source_edge_identity,
        target_edge_identity,
        shared_vertex_identity,
    })
}
