use std::collections::BTreeSet;

use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::report::MilestoneThreeHostileSuiteReport;
use super::super::scenario_programs::successor_relocation_batch;
use super::super::shared::{
    aggregate_topology_edit_digest, derived_validation_report_from_materialized,
    first_source_identity_for_relation_kind,
};
use super::scale_pressure_branch::certify_large_branch_history_row;
use super::scale_pressure_detach::{certify_detach_pressure_row, DetachPressureKind};
use super::scale_pressure_loop::high_cardinality_loop_batches;
use super::scale_pressure_shell::high_face_count_shell_rehome_batch;
use super::scale_pressure_types::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use super::sweeps::certify_radial_splice_pressure_row;
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::TopologyDomainQuery;
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyEditApplicationMode, TopologyEditBatch, TopologyEditDigest,
    TopologyEditFamily,
};

#[derive(Debug, Clone)]
struct ScaleRewireCase {
    sweep: MilestoneThreeScalePressureSweep,
    primitive: MilestoneOnePrimitiveCase,
    cycle_depth: usize,
    candidate_offset: usize,
    workload_size: usize,
}

struct ScaleRewireExecution {
    primitive_family: String,
    topology_edit_digest: TopologyEditDigest,
    edit_families: Vec<TopologyEditFamily>,
    final_state_digest: String,
    derived_validation_row_count: usize,
}

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_scale_pressure_impl<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<MilestoneThreeScalePressureRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut rows = scale_rewire_cases()
        .iter()
        .map(|case| certify_scale_rewire_row(runtime_factory, stem, case.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    rows.push(certify_large_branch_history_row(runtime_factory, stem)?);
    rows.push(certify_detach_pressure_row(
        runtime_factory,
        stem,
        MilestoneThreeScalePressureSweep::WireMembershipDetach,
        MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
        TopologyRelationKind::WireOwnsHalfEdge,
        DetachPressureKind::ShellOrWire(ShellOrWireMembershipKind::WireOwnsHalfEdge),
    )?);
    rows.push(certify_radial_splice_pressure_row(runtime_factory, stem)?);
    rows.push(certify_detach_pressure_row(
        runtime_factory,
        stem,
        MilestoneThreeScalePressureSweep::RadialAdjacencyDetach,
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
        TopologyRelationKind::HalfEdgeRadialNext,
        DetachPressureKind::Radial,
    )?);
    Ok(rows)
}

pub(in crate::certification::topology_operator_closeout) fn ensure_scale_pressure_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let covered = covered_scale_pressure_sweeps(report);
    for sweep in required_scale_pressure_sweeps() {
        if !covered.contains(sweep) {
            return Err(scale_pressure_error(&format!(
                "missing scale pressure sweep {}",
                sweep.as_str()
            )));
        }
    }
    for row in &report.scale_pressure_rows {
        if !row.replay_verified
            || row.topology_edit_digest.contract_count == 0
            || row.workload_size == 0
            || row.edit_step_count == 0
            || row.final_state_digest != row.replay_final_state_digest
        {
            return Err(scale_pressure_error(&format!(
                "scale pressure row is not proof-bearing for {}",
                row.sweep.as_str()
            )));
        }
    }
    Ok(())
}

pub(in crate::certification::topology_operator_closeout) fn scale_pressure_labels(
    report: &MilestoneThreeHostileSuiteReport,
) -> (usize, Vec<String>, Vec<String>) {
    let covered = covered_scale_pressure_sweeps(report);
    let evidence_labels = report
        .scale_pressure_rows
        .iter()
        .filter(|row| row.replay_verified)
        .map(|row| format!("scale_pressure={}", row.sweep.as_str()))
        .collect::<Vec<_>>();
    let gap_labels = required_scale_pressure_sweeps()
        .iter()
        .filter(|sweep| !covered.contains(sweep))
        .map(|sweep| format!("missing_scale_sweep={}", sweep.as_str()))
        .collect::<Vec<_>>();
    (covered.len(), evidence_labels, gap_labels)
}

fn certify_scale_rewire_row<F>(
    runtime_factory: &mut F,
    stem: &str,
    case: ScaleRewireCase,
) -> Result<MilestoneThreeScalePressureRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_scale_rewire(runtime_factory, stem, &case)?;
    let replay = execute_scale_rewire(runtime_factory, stem, &case)?;
    let replay_verified = left.topology_edit_digest == replay.topology_edit_digest
        && left.final_state_digest == replay.final_state_digest
        && left.derived_validation_row_count == replay.derived_validation_row_count;
    Ok(MilestoneThreeScalePressureRow {
        sweep: case.sweep,
        primitive_family: left.primitive_family,
        primitive: case.primitive,
        workload_size: case.workload_size,
        edit_step_count: left.topology_edit_digest.contract_count,
        edit_families: left.edit_families,
        branch_local: false,
        topology_edit_digest: left.topology_edit_digest,
        replay_verified,
        final_state_digest: left.final_state_digest.clone(),
        replay_final_state_digest: replay.final_state_digest,
        derived_validation_row_count: left.derived_validation_row_count,
        row_digest: scale_pressure_row_digest(case.sweep, replay_verified, case.workload_size),
    })
}

fn execute_scale_rewire<F>(
    runtime_factory: &mut F,
    stem: &str,
    case: &ScaleRewireCase,
) -> Result<ScaleRewireExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family = primitive_family_name(&case.primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.scale_pressure.{}", case.sweep.as_str()),
        &case.primitive,
    )?;
    let high_face_shell_batch =
        if case.sweep == MilestoneThreeScalePressureSweep::HighFaceCountShells {
            Some(high_face_count_shell_rehome_batch(
                &runtime,
                &verified.read_basis(),
                stem,
                case.workload_size,
            )?)
        } else {
            None
        };
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, format!("{stem}.scale_pressure.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let batches =
        scale_rewire_batches(&mut workspace, &relation_rows, high_face_shell_batch, case)?;
    scale_rewire_execution(primitive_family, &mut workspace, &assembly, batches)
}

fn scale_rewire_batches(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    high_face_shell_batch: Option<TopologyEditBatch>,
    case: &ScaleRewireCase,
) -> Result<Vec<TopologyEditBatch>, TopologyCertificationError> {
    let batches = if let Some(batch) = high_face_shell_batch {
        vec![batch]
    } else if case.sweep == MilestoneThreeScalePressureSweep::HighCardinalityLoops {
        let moved_half_edge_identity = first_source_identity_for_relation_kind(
            &relation_rows,
            TopologyRelationKind::HalfEdgeNext,
        )?;
        high_cardinality_loop_batches(workspace, relation_rows, &moved_half_edge_identity)?
    } else {
        vec![local_successor_batch(
            workspace,
            relation_rows,
            case.cycle_depth,
            case.candidate_offset,
        )?]
    };
    Ok(batches)
}

fn local_successor_batch(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    cycle_depth: usize,
    candidate_offset: usize,
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let moved_half_edge_identity =
        first_source_identity_for_relation_kind(relation_rows, TopologyRelationKind::HalfEdgeNext)?;
    let neighborhood = TopologyDomainQuery::load()
        .local_rewire_neighborhood(workspace, &moved_half_edge_identity, cycle_depth)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let chosen_successor_identity = neighborhood
        .cycle_identities()
        .get(candidate_offset)
        .cloned()
        .ok_or_else(|| scale_pressure_error("scale rewire candidate should exist"))?;
    successor_relocation_batch(&neighborhood, &chosen_successor_identity)
}
fn scale_rewire_execution(
    primitive_family: String,
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    assembly: &TopologyQueryAssembly,
    batches: Vec<TopologyEditBatch>,
) -> Result<ScaleRewireExecution, TopologyCertificationError> {
    let topology_edit_digest = aggregate_topology_edit_digest(&batches);
    let edit_families = batches.iter().flat_map(|batch| batch.families()).collect();
    let mut final_state_digest = String::new();
    let mut final_materialized = None;
    for batch in batches {
        let execution = assembly
            .apply_edit(workspace, batch, TopologyEditApplicationMode::Mainline)
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
        final_state_digest = digest_materialized_topology_view(&execution.materialized).digest_hex;
        final_materialized = Some(execution.materialized);
    }
    let final_materialized = final_materialized
        .ok_or_else(|| scale_pressure_error("scale pressure executed no batches"))?;
    let validation = derived_validation_report_from_materialized(&final_materialized)?;
    Ok(ScaleRewireExecution {
        primitive_family,
        topology_edit_digest,
        edit_families,
        final_state_digest,
        derived_validation_row_count: validation.rows.len(),
    })
}

fn covered_scale_pressure_sweeps(
    report: &MilestoneThreeHostileSuiteReport,
) -> BTreeSet<MilestoneThreeScalePressureSweep> {
    report
        .scale_pressure_rows
        .iter()
        .filter(|row| row.replay_verified)
        .map(|row| row.sweep)
        .collect()
}

fn required_scale_pressure_sweeps() -> &'static [MilestoneThreeScalePressureSweep] {
    &[
        MilestoneThreeScalePressureSweep::HighCardinalityLoops,
        MilestoneThreeScalePressureSweep::HighFaceCountShells,
        MilestoneThreeScalePressureSweep::LargeBranchLocalHistories,
        MilestoneThreeScalePressureSweep::RadialAdjacencySplice,
    ]
}

fn scale_rewire_cases() -> &'static [ScaleRewireCase] {
    &[
        ScaleRewireCase {
            sweep: MilestoneThreeScalePressureSweep::HighCardinalityLoops,
            primitive: MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 8 },
            cycle_depth: 5,
            candidate_offset: 3,
            workload_size: 8,
        },
        ScaleRewireCase {
            sweep: MilestoneThreeScalePressureSweep::HighFaceCountShells,
            primitive: MilestoneOnePrimitiveCase::SolidShell { face_count: 16 },
            cycle_depth: 3,
            candidate_offset: 2,
            workload_size: 16,
        },
    ]
}

fn scale_pressure_row_digest(
    sweep: MilestoneThreeScalePressureSweep,
    replay_verified: bool,
    workload_size: usize,
) -> String {
    format!(
        "scale_pressure={};replay_verified={replay_verified};workload_size={workload_size}",
        sweep.as_str()
    )
}

fn scale_pressure_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale pressure failed: {reason}"))
}
