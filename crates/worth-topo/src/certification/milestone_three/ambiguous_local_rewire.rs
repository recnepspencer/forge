use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::TopologyRelationKind;
use serde_json::Value;

use super::report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeEditReplayStepRow,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    entity_id_from_query_identity, first_source_identity_for_relation_kind,
    relation_id_from_query_identity, replay_checked,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::edit::{
    TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract, TopologyEditDigest,
    TopologyEditFamily,
};
use crate::parity::digest_materialized_topology_view;
use crate::query::{
    topology_runtime, TopologyDomainQuery, TopologyLocalRewireNeighborhoodView,
    TopologyLoopNeighborEvidence, TopologyQueryAssembly, TopologyRuntimeAdapters,
};

struct MilestoneThreeAmbiguousLocalRewireRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    edit_families: Vec<TopologyEditFamily>,
    topology_edit_digest: TopologyEditDigest,
    naming_edit_continuity_matrix: crate::edit::NamingEditContinuityMatrix,
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: crate::certification::DeterministicDigest,
    moved_half_edge_identity: String,
    old_successor_identity: String,
    chosen_successor_identity: String,
}

pub(crate) fn certify_milestone_three_ambiguous_local_rewire_continuity_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let chosen = execute_ambiguous_local_rewire(&mut runtime_factory, stem, 3)?;
    let alternate = execute_ambiguous_local_rewire(&mut runtime_factory, stem, 4)?;
    let replay = execute_ambiguous_local_rewire(&mut runtime_factory, stem, 3)?;
    let replay_report = replay_checked(
        chosen.step_rows.clone(),
        replay.step_rows.clone(),
        chosen.baseline_materialized_topology_digest.clone(),
        chosen.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    );
    let continuity_outcome_class = chosen.naming_edit_continuity_matrix.outcome_class();
    let continuity_rejection_class = chosen.naming_edit_continuity_matrix.rejection_class();

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
        primitive_family: chosen.primitive_family,
        primitive: chosen.primitive,
        edit_families: chosen.edit_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: Some(MilestoneThreeAmbiguousLocalRewireWitness {
            moved_half_edge_identity: chosen.moved_half_edge_identity.clone(),
            alternate_moved_half_edge_identity: alternate.moved_half_edge_identity.clone(),
            old_successor_identity: chosen.old_successor_identity.clone(),
            alternate_old_successor_identity: alternate.old_successor_identity.clone(),
            chosen_successor_identity: chosen.chosen_successor_identity.clone(),
            alternate_successor_identity: alternate.chosen_successor_identity.clone(),
            chosen_materialized_topology_digest: chosen.final_materialized_topology_digest.clone(),
            alternate_materialized_topology_digest: alternate
                .final_materialized_topology_digest
                .clone(),
        }),
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        topology_edit_digest: chosen.topology_edit_digest,
        naming_edit_continuity_matrix: chosen.naming_edit_continuity_matrix,
        continuity_outcome_class,
        continuity_rejection_class,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_edit_scope_report: None,
        edit_replay_parity_report: replay_report,
        detail: format!(
            "local successor rewire remained admitted while continuity stayed `{continuity_outcome_class:?}` because more than one accepted successor placement exists from the same basis"
        ),
    })
}

fn execute_ambiguous_local_rewire<F>(
    runtime_factory: &mut F,
    stem: &str,
    candidate_offset: usize,
) -> Result<MilestoneThreeAmbiguousLocalRewireRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.ambiguous_local_rewire.{candidate_offset}"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        &format!("{stem}.ambiguous_local_rewire.{candidate_offset}.runtime"),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let domain_query = TopologyDomainQuery::load();
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let neighborhood = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let old_successor_identity = neighborhood.old_successor_identity.clone();
    let chosen_successor_identity = neighborhood
        .cycle_identities
        .get(candidate_offset)
        .cloned()
        .ok_or_else(|| cycle_query_error("requested successor candidate should exist in cycle"))?;
    let batch = successor_relocation_batch(&neighborhood, &chosen_successor_identity)?;
    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_rows = vec![accepted_step_row(0, &batch, &execution)];
    let batches = vec![batch];

    Ok(MilestoneThreeAmbiguousLocalRewireRun {
        primitive_family,
        primitive,
        edit_families: batches.iter().flat_map(|batch| batch.families()).collect(),
        topology_edit_digest: aggregate_topology_edit_digest(&batches),
        naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
        step_rows,
        baseline_materialized_topology_digest,
        final_materialized_topology_digest: digest_materialized_topology_view(
            &execution.materialized,
        ),
        moved_half_edge_identity,
        old_successor_identity,
        chosen_successor_identity,
    })
}

fn successor_relocation_batch(
    neighborhood: &TopologyLocalRewireNeighborhoodView,
    new_successor_identity: &str,
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let moved = loop_neighbor_evidence(neighborhood, neighborhood.moved_half_edge_identity())?;
    let old_successor =
        loop_neighbor_evidence(neighborhood, neighborhood.old_successor_identity())?;
    let old_predecessor =
        loop_neighbor_evidence(neighborhood, neighborhood.old_predecessor_identity())?;
    let new_successor = loop_neighbor_evidence(neighborhood, new_successor_identity)?;
    let new_predecessor =
        loop_neighbor_evidence(neighborhood, new_successor.previous_half_edge_identity())?;

    let moved_half_edge_id = entity_id_from_query_identity(moved.half_edge_identity())?;
    let old_successor_id = entity_id_from_query_identity(old_successor.half_edge_identity())?;
    let old_predecessor_id = entity_id_from_query_identity(old_predecessor.half_edge_identity())?;
    let new_successor_id = entity_id_from_query_identity(new_successor.half_edge_identity())?;
    let new_predecessor_id = entity_id_from_query_identity(new_predecessor.half_edge_identity())?;

    TopologyEditBatch::new(vec![
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(moved.next_relation_identity())?,
            crate::edit::LoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(moved.previous_relation_identity())?,
            crate::edit::LoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(old_predecessor.next_relation_identity())?,
            crate::edit::LoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(old_successor.previous_relation_identity())?,
            crate::edit::LoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(new_predecessor.next_relation_identity())?,
            crate::edit::LoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(new_successor.previous_relation_identity())?,
            crate::edit::LoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ])
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn loop_neighbor_evidence<'a>(
    neighborhood: &'a TopologyLocalRewireNeighborhoodView,
    half_edge_identity: &str,
) -> Result<&'a TopologyLoopNeighborEvidence, TopologyCertificationError> {
    neighborhood
        .cycle_half_edges()
        .iter()
        .find(|evidence| evidence.half_edge_identity() == half_edge_identity)
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "local rewire neighborhood should expose cycle evidence for `{half_edge_identity}`"
            ))
        })
}

fn cycle_query_error(detail: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(detail.to_string())
}
