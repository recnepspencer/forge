use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::TopologyRelationKind;
use serde_json::Value;

use super::super::report::{MilestoneThreeHostileScenario, MilestoneThreeHostileSuiteReport};
use super::super::scenario_programs::successor_relocation_batch;
use super::super::shared::first_source_identity_for_relation_kind;
use super::edited_query_traversal_types::{
    MilestoneThreeEditedTopologyQueryTraversalRow, MilestoneThreeEditedTopologyQueryTraversalView,
};
use crate::certification::error::TopologyCertificationError;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::TopologyLoopCycleView;
use crate::projection::{TopologyDomainQuery, TopologyLocalRewireNeighborhoodView};
use crate::topology_operators::TopologyEditApplicationMode;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EditedTraversalProbe {
    view: MilestoneThreeEditedTopologyQueryTraversalView,
    view_digest: String,
    request_count: usize,
    relationship_proof_admission_count: usize,
    traversal_count: usize,
}

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_edited_query_traversal_impl<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<MilestoneThreeEditedTopologyQueryTraversalRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_post_edit_query_traversal_probe(
        runtime_factory,
        &format!("{stem}.edited_query_traversal.left"),
    )?;
    let replay = execute_post_edit_query_traversal_probe(
        runtime_factory,
        &format!("{stem}.edited_query_traversal.replay"),
    )?;

    left.iter()
        .map(|left_probe| {
            let replay_probe = replay
                .iter()
                .find(|probe| probe.view == left_probe.view)
                .ok_or_else(|| edited_query_traversal_error("missing replay probe view"))?;
            Ok(row_from_probes(left_probe, replay_probe))
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_edited_query_traversal_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for view in required_edited_query_traversal_views() {
        let row = report
            .edited_query_traversal_rows
            .iter()
            .find(|row| row.view == *view)
            .ok_or_else(|| {
                edited_query_traversal_error(&format!(
                    "missing edited topology query traversal row for {}",
                    view.as_str()
                ))
            })?;
        if row.scenario != MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
            || !row.parity_verified
            || row.request_count == 0
            || row.relationship_proof_admission_count == 0
            || row.traversal_count == 0
        {
            return Err(edited_query_traversal_error(&format!(
                "edited topology query traversal row is not proof-bearing for {}",
                view.as_str()
            )));
        }
    }
    Ok(())
}

pub(in crate::certification::topology_operator_closeout) fn required_edited_query_traversal_views(
) -> &'static [MilestoneThreeEditedTopologyQueryTraversalView] {
    &[
        MilestoneThreeEditedTopologyQueryTraversalView::PostEditLocalRewireNeighborhood,
        MilestoneThreeEditedTopologyQueryTraversalView::PostEditLoopCycleNeighborhood,
    ]
}

fn execute_post_edit_query_traversal_probe<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<EditedTraversalProbe>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.seed"),
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &workspace.read::<Value>(assembly.relations()),
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let pre_edit_query = TopologyDomainQuery::load();
    let pre_edit_rewire = pre_edit_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let chosen_successor_identity = pre_edit_rewire
        .cycle_identities()
        .get(3)
        .cloned()
        .ok_or_else(|| {
            edited_query_traversal_error(
                "ambiguous local rewire should expose a successor candidate",
            )
        })?;
    let batch = successor_relocation_batch(&pre_edit_rewire, &chosen_successor_identity)?;
    assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;

    let post_edit_query = TopologyDomainQuery::load();
    let local_rewire = post_edit_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let loop_cycle = post_edit_query
        .loop_cycle(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;

    Ok(vec![
        local_rewire_probe(&local_rewire),
        loop_cycle_probe(&loop_cycle),
    ])
}

fn row_from_probes(
    left: &EditedTraversalProbe,
    replay: &EditedTraversalProbe,
) -> MilestoneThreeEditedTopologyQueryTraversalRow {
    let parity_verified = left.view_digest == replay.view_digest
        && left.request_count == replay.request_count
        && left.relationship_proof_admission_count == replay.relationship_proof_admission_count
        && left.traversal_count == replay.traversal_count;
    MilestoneThreeEditedTopologyQueryTraversalRow {
        scenario: MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
        view: left.view,
        left_view_digest: left.view_digest.clone(),
        replay_view_digest: replay.view_digest.clone(),
        parity_verified,
        request_count: left.request_count,
        relationship_proof_admission_count: left.relationship_proof_admission_count,
        traversal_count: left.traversal_count,
        row_digest: format!(
            "scenario={};view={};parity_verified={parity_verified};requests={};relationship_proofs={};traversals={};view_digest={}",
            MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity.as_str(),
            left.view.as_str(),
            left.request_count,
            left.relationship_proof_admission_count,
            left.traversal_count,
            left.view_digest
        ),
    }
}

fn local_rewire_probe(view: &TopologyLocalRewireNeighborhoodView) -> EditedTraversalProbe {
    let request = view.request_report();
    EditedTraversalProbe {
        view: MilestoneThreeEditedTopologyQueryTraversalView::PostEditLocalRewireNeighborhood,
        view_digest: digest_view_parts(&[
            format!("moved_half_edge:{}", view.moved_half_edge_identity()),
            format!("old_successor:{}", view.old_successor_identity()),
            format!("old_predecessor:{}", view.old_predecessor_identity()),
            format!("cycle_identities:{}", view.cycle_identities().join("|")),
        ]),
        request_count: request.query_execution_count(),
        relationship_proof_admission_count: request.relationship_proof_admission_count(),
        traversal_count: request.lowered_traversal_count(),
    }
}

fn loop_cycle_probe(view: &TopologyLoopCycleView) -> EditedTraversalProbe {
    let request = view.request_report();
    EditedTraversalProbe {
        view: MilestoneThreeEditedTopologyQueryTraversalView::PostEditLoopCycleNeighborhood,
        view_digest: digest_view_parts(&[
            format!("start_half_edge:{}", view.start_half_edge_identity()),
            format!("cycle_identities:{}", view.cycle_identities().join("|")),
        ]),
        request_count: request.query_execution_count(),
        relationship_proof_admission_count: request.relationship_proof_admission_count(),
        traversal_count: request.lowered_traversal_count(),
    }
}

fn digest_view_parts(parts: &[String]) -> String {
    let mut state: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x100000001b3);
        }
    }
    format!("{state:016x}")
}

fn edited_query_traversal_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three edited topology query traversal failed: {reason}"
    ))
}
