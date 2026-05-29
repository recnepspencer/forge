use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::shared::{aggregate_topology_edit_digest, relation_id_from_query_identity};
use super::scale_pressure_types::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyEditApplicationMode, TopologyEditBatch,
    TopologyEditContract, TopologyEditDigest, TopologyEditFamily,
};

#[derive(Debug, Clone, Copy)]
pub(super) enum DetachPressureKind {
    ShellOrWire(ShellOrWireMembershipKind),
    Radial,
}

struct DetachPressureExecution {
    primitive_family: String,
    topology_edit_digest: TopologyEditDigest,
    edit_families: Vec<TopologyEditFamily>,
    final_state_digest: String,
}

pub(super) fn certify_detach_pressure_row<F>(
    runtime_factory: &mut F,
    stem: &str,
    sweep: MilestoneThreeScalePressureSweep,
    primitive: MilestoneOnePrimitiveCase,
    relation_kind: TopologyRelationKind,
    detach_kind: DetachPressureKind,
) -> Result<MilestoneThreeScalePressureRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_detach_pressure(
        runtime_factory,
        stem,
        sweep,
        primitive.clone(),
        relation_kind,
        detach_kind,
    )?;
    let replay = execute_detach_pressure(
        runtime_factory,
        stem,
        sweep,
        primitive.clone(),
        relation_kind,
        detach_kind,
    )?;
    let replay_verified = left.topology_edit_digest == replay.topology_edit_digest
        && left.final_state_digest == replay.final_state_digest
        && left.edit_families == replay.edit_families;
    Ok(MilestoneThreeScalePressureRow {
        sweep,
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
        derived_validation_row_count: 0,
        row_digest: scale_pressure_row_digest(sweep, replay_verified),
    })
}

fn execute_detach_pressure<F>(
    runtime_factory: &mut F,
    stem: &str,
    sweep: MilestoneThreeScalePressureSweep,
    primitive: MilestoneOnePrimitiveCase,
    relation_kind: TopologyRelationKind,
    detach_kind: DetachPressureKind,
) -> Result<DetachPressureExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.scale_pressure.{}", sweep.as_str()),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, format!("{stem}.scale_pressure.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let relation_id = first_relation_id_for_kind(&relation_rows, relation_kind)?;
    let detach_contract = match detach_kind {
        DetachPressureKind::ShellOrWire(kind) => {
            TopologyEditContract::detach_shell_or_wire_membership(relation_id, kind)
        }
        DetachPressureKind::Radial => TopologyEditContract::detach_radial_adjacency(relation_id),
    };
    let batch = TopologyEditBatch::new(vec![detach_contract])
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let batches = vec![batch];
    let topology_edit_digest = aggregate_topology_edit_digest(&batches);
    let edit_families = batches.iter().flat_map(|batch| batch.families()).collect();
    let mut final_state_digest = String::new();
    for batch in batches {
        let execution = assembly
            .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
        final_state_digest = digest_materialized_topology_view(&execution.materialized).digest_hex;
    }
    Ok(DetachPressureExecution {
        primitive_family,
        topology_edit_digest,
        edit_families,
        final_state_digest,
    })
}

fn first_relation_id_for_kind(
    relation_rows: &[ForgeQueryEntity],
    relation_kind: TopologyRelationKind,
) -> Result<forge_relational::facade::identity::RelationId, TopologyCertificationError> {
    let relation_identity = relation_rows
        .iter()
        .find(|row| relation_kind_label(row) == Some(relation_kind.kind_name()))
        .map(|row| row.identity.as_str())
        .ok_or_else(|| scale_pressure_detach_error("detach pressure relation should resolve"))?;
    relation_id_from_query_identity(relation_identity)
}

fn relation_kind_label(row: &ForgeQueryEntity) -> Option<&str> {
    row.payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
}

fn scale_pressure_row_digest(
    sweep: MilestoneThreeScalePressureSweep,
    replay_verified: bool,
) -> String {
    format!(
        "scale_pressure={};replay_verified={replay_verified};workload_size=1",
        sweep.as_str()
    )
}

fn scale_pressure_detach_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale detach failed: {reason}"))
}
