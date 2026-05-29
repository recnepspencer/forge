use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::platform::relations::TopologyRelationKind;
use serde_json::Value;

use super::super::super::shared::{
    aggregate_topology_edit_digest, derived_validation_report_from_materialized,
    entity_id_from_query_identity, first_source_identity_for_relation_kind,
    relation_id_from_query_identity,
};
use super::super::scale_pressure_types::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::TopologyDomainQuery;
use crate::topology_operators::{
    TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract, TopologyEditDigest,
    TopologyEditFamily,
};

struct RadialSpliceExecution {
    primitive_family: String,
    topology_edit_digest: TopologyEditDigest,
    edit_families: Vec<TopologyEditFamily>,
    final_state_digest: String,
    derived_validation_row_count: usize,
}

pub(in crate::certification::topology_operator_closeout::scale_pressure_proof) fn certify_radial_splice_pressure_row<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeScalePressureRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let left = execute_radial_splice_pressure(runtime_factory, stem, primitive.clone())?;
    let replay = execute_radial_splice_pressure(runtime_factory, stem, primitive.clone())?;
    let replay_verified = left.topology_edit_digest == replay.topology_edit_digest
        && left.final_state_digest == replay.final_state_digest
        && left.edit_families == replay.edit_families
        && left.derived_validation_row_count == replay.derived_validation_row_count;

    Ok(MilestoneThreeScalePressureRow {
        sweep: MilestoneThreeScalePressureSweep::RadialAdjacencySplice,
        primitive_family: left.primitive_family,
        primitive,
        workload_size: 1,
        edit_step_count: left.topology_edit_digest.contract_count,
        edit_families: left.edit_families,
        branch_local: false,
        topology_edit_digest: left.topology_edit_digest,
        replay_verified,
        final_state_digest: left.final_state_digest.clone(),
        replay_final_state_digest: replay.final_state_digest,
        derived_validation_row_count: left.derived_validation_row_count,
        row_digest: format!(
            "scale_pressure={};replay_verified={replay_verified};workload_size=1",
            MilestoneThreeScalePressureSweep::RadialAdjacencySplice.as_str()
        ),
    })
}

fn execute_radial_splice_pressure<F>(
    runtime_factory: &mut F,
    stem: &str,
    primitive: MilestoneOnePrimitiveCase,
) -> Result<RadialSpliceExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.scale_pressure.radial_adjacency_splice"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, format!("{stem}.scale_pressure.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let source_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeRadialNext,
    )?;
    let _radial = TopologyDomainQuery::load()
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let cycle = radial_cycle_identities(&relation_rows, &source_identity)?;
    let reordered_cycle = reorder_radial_cycle(&cycle)?;
    let batch = radial_cycle_reorder_batch(&relation_rows, &cycle, &reordered_cycle)?;
    let batches = vec![batch];
    let topology_edit_digest = aggregate_topology_edit_digest(&batches);
    let edit_families = batches.iter().flat_map(|batch| batch.families()).collect();
    let mut final_materialized = None;
    let mut final_state_digest = String::new();
    for batch in batches {
        let execution = assembly
            .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
        final_state_digest = digest_materialized_topology_view(&execution.materialized).digest_hex;
        final_materialized = Some(execution.materialized);
    }
    let final_materialized = final_materialized.ok_or_else(|| {
        radial_splice_pressure_error("radial splice pressure executed no batches")
    })?;
    let validation = derived_validation_report_from_materialized(&final_materialized)?;
    Ok(RadialSpliceExecution {
        primitive_family,
        topology_edit_digest,
        edit_families,
        final_state_digest,
        derived_validation_row_count: validation.rows.len(),
    })
}

fn radial_cycle_reorder_batch(
    relation_rows: &[ForgeQueryEntity],
    current_cycle: &[String],
    reordered_cycle: &[String],
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let contracts = reordered_cycle
        .iter()
        .enumerate()
        .filter_map(|(index, source_identity)| {
            let current_target = next_identity(current_cycle, source_identity)?;
            let reordered_target = reordered_cycle[(index + 1) % reordered_cycle.len()].as_str();
            (current_target != reordered_target).then_some((source_identity, reordered_target))
        })
        .map(|(source_identity, reordered_target)| {
            radial_splice_contract(relation_rows, source_identity, reordered_target)
        })
        .collect::<Result<Vec<_>, _>>()?;
    TopologyEditBatch::new(contracts)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn radial_splice_contract(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    target_identity: &str,
) -> Result<TopologyEditContract, TopologyCertificationError> {
    Ok(TopologyEditContract::splice_radial_adjacency(
        relation_id_for_source_kind(
            relation_rows,
            source_identity,
            TopologyRelationKind::HalfEdgeRadialNext,
        )?,
        entity_id_from_query_identity(source_identity)?,
        entity_id_from_query_identity(target_identity)?,
    ))
}

fn radial_cycle_identities(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
) -> Result<Vec<String>, TopologyCertificationError> {
    let mut cycle = vec![source_identity.to_string()];
    loop {
        let next = relation_target_identity_for_source_kind(
            relation_rows,
            cycle
                .last()
                .ok_or_else(|| radial_splice_pressure_error("radial cycle unexpectedly empty"))?,
            TopologyRelationKind::HalfEdgeRadialNext,
        )?;
        if next == source_identity {
            return Ok(cycle);
        }
        if cycle.iter().any(|identity| identity == &next) {
            return Err(radial_splice_pressure_error(
                "radial cycle revisited a half-edge before closing",
            ));
        }
        cycle.push(next);
    }
}

fn reorder_radial_cycle(cycle: &[String]) -> Result<Vec<String>, TopologyCertificationError> {
    if cycle.len() < 3 {
        return Err(radial_splice_pressure_error(
            "radial splice pressure requires at least three same-edge half-edges",
        ));
    }
    let mut reordered_cycle = cycle.to_vec();
    reordered_cycle.swap(1, 2);
    Ok(reordered_cycle)
}

fn next_identity<'a>(cycle: &'a [String], source_identity: &str) -> Option<&'a str> {
    cycle
        .iter()
        .position(|identity| identity == source_identity)
        .map(|index| cycle[(index + 1) % cycle.len()].as_str())
}

fn relation_id_for_source_kind(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> Result<forge_relational::facade::identity::RelationId, TopologyCertificationError> {
    let relation_identity = relation_rows
        .iter()
        .find(|row| row_matches_source_kind(row, source_identity, relation_kind))
        .map(|row| row.identity.as_str())
        .ok_or_else(|| radial_splice_pressure_error("radial splice relation id should resolve"))?;
    relation_id_from_query_identity(relation_identity)
}

fn relation_target_identity_for_source_kind(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyCertificationError> {
    relation_rows
        .iter()
        .find(|row| row_matches_source_kind(row, source_identity, relation_kind))
        .and_then(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("target_identity"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .ok_or_else(|| radial_splice_pressure_error("radial splice target identity should resolve"))
}

fn row_matches_source_kind(
    row: &ForgeQueryEntity,
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> bool {
    row.payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
        == Some(relation_kind.kind_name())
        && row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(|value| value.as_str())
            == Some(source_identity)
}

fn radial_splice_pressure_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three radial splice failed: {reason}"))
}




