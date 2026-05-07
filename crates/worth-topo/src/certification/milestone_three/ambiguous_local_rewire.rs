use forge_relational::facade::identity::RelationId;
use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

use super::report::{
    WorthMilestoneThreeAmbiguousLocalRewireWitness, WorthMilestoneThreeEditReplayStepRow,
    WorthMilestoneThreeHostileOutcomeClass, WorthMilestoneThreeHostileScenario,
    WorthMilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    replay_checked,
};
use crate::certification::error::WorthTopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditBatch, WorthTopologyEditContract,
    WorthTopologyEditDigest, WorthTopologyEditFamily,
};
use crate::parity::digest_materialized_topology_view;
use crate::query::{
    worth_topology_runtime, WorthTopologyDomainQuery, WorthTopologyQueryAssembly,
    WorthTopologyRuntimeAdapters,
};

struct WorthMilestoneThreeAmbiguousLocalRewireRun {
    primitive_family: String,
    primitive: WorthMilestoneOnePrimitiveCase,
    edit_families: Vec<WorthTopologyEditFamily>,
    topology_edit_digest: WorthTopologyEditDigest,
    naming_edit_continuity_matrix: crate::edit::WorthNamingEditContinuityMatrix,
    step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::WorthDeterministicDigest,
    final_materialized_topology_digest: crate::certification::WorthDeterministicDigest,
    moved_half_edge_identity: String,
    old_successor_identity: String,
    chosen_successor_identity: String,
}

pub(crate) fn certify_milestone_three_ambiguous_local_rewire_continuity_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneThreeHostileScenarioReport, WorthTopologyCertificationError>
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

    Ok(WorthMilestoneThreeHostileScenarioReport {
        scenario: WorthMilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
        primitive_family: chosen.primitive_family,
        primitive: chosen.primitive,
        edit_families: chosen.edit_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: Some(WorthMilestoneThreeAmbiguousLocalRewireWitness {
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
        broken_radial_witness: None,
        topology_edit_digest: chosen.topology_edit_digest,
        naming_edit_continuity_matrix: chosen.naming_edit_continuity_matrix,
        continuity_outcome_class,
        continuity_rejection_class,
        outcome_class: WorthMilestoneThreeHostileOutcomeClass::Accepted,
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
) -> Result<WorthMilestoneThreeAmbiguousLocalRewireRun, WorthTopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.ambiguous_local_rewire.{candidate_offset}"),
        &primitive,
    )?;
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        &format!("{stem}.ambiguous_local_rewire.{candidate_offset}.runtime"),
    )
    .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let domain_query = WorthTopologyDomainQuery::load(&workspace, &assembly)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let moved_half_edge_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let neighborhood = domain_query
        .local_rewire_neighborhood(&moved_half_edge_identity, 6)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let old_successor_identity = neighborhood.old_successor_identity.clone();
    let chosen_successor_identity = neighborhood
        .cycle_identities
        .get(candidate_offset)
        .cloned()
        .ok_or_else(|| cycle_query_error("requested successor candidate should exist in cycle"))?;
    let batch = successor_relocation_batch(
        &domain_query,
        &moved_half_edge_identity,
        &chosen_successor_identity,
    )?;
    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch.clone(),
            WorthTopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let step_rows = vec![accepted_step_row(0, &batch, &execution)];
    let batches = vec![batch];

    Ok(WorthMilestoneThreeAmbiguousLocalRewireRun {
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
    domain_query: &WorthTopologyDomainQuery,
    moved_identity: &str,
    new_successor_identity: &str,
) -> Result<WorthTopologyEditBatch, WorthTopologyCertificationError> {
    let moved_half_edge_id = domain_query
        .find_entity_id_by_identity(moved_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let old_successor_identity = domain_query
        .outgoing_target_identity(moved_identity, WorthTopologyRelationKind::HalfEdgeNext)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let old_successor_id = domain_query
        .find_entity_id_by_identity(&old_successor_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let old_predecessor_identity = domain_query
        .outgoing_target_identity(moved_identity, WorthTopologyRelationKind::HalfEdgePrev)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let old_predecessor_id = domain_query
        .find_entity_id_by_identity(&old_predecessor_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let new_successor_id = domain_query
        .find_entity_id_by_identity(new_successor_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let new_predecessor_identity = domain_query
        .outgoing_target_identity(
            new_successor_identity,
            WorthTopologyRelationKind::HalfEdgePrev,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let new_predecessor_id = domain_query
        .find_entity_id_by_identity(&new_predecessor_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;

    WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                domain_query,
                moved_identity,
                WorthTopologyRelationKind::HalfEdgeNext,
            )?,
            crate::edit::WorthLoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                domain_query,
                moved_identity,
                WorthTopologyRelationKind::HalfEdgePrev,
            )?,
            crate::edit::WorthLoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                domain_query,
                &old_predecessor_identity,
                WorthTopologyRelationKind::HalfEdgeNext,
            )?,
            crate::edit::WorthLoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                domain_query,
                &old_successor_identity,
                WorthTopologyRelationKind::HalfEdgePrev,
            )?,
            crate::edit::WorthLoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                domain_query,
                &new_predecessor_identity,
                WorthTopologyRelationKind::HalfEdgeNext,
            )?,
            crate::edit::WorthLoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                domain_query,
                new_successor_identity,
                WorthTopologyRelationKind::HalfEdgePrev,
            )?,
            crate::edit::WorthLoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ])
    .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))
}

fn relation_id_for_source_kind(
    domain_query: &WorthTopologyDomainQuery,
    source_identity: &str,
    relation_kind: WorthTopologyRelationKind,
) -> Result<RelationId, WorthTopologyCertificationError> {
    domain_query
        .relation_id_for_source_kind(source_identity, relation_kind)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))
}

fn cycle_query_error(detail: &str) -> WorthTopologyCertificationError {
    WorthTopologyCertificationError::Query(detail.to_string())
}
