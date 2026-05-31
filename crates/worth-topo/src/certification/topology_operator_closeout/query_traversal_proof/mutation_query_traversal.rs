use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::report::{MilestoneThreeHostileScenario, MilestoneThreeHostileSuiteReport};
use super::super::scenario_programs::successor_relocation_declaration;
use super::super::shared::first_source_identity_for_relation_kind;
use super::mutation_query_traversal_types::{
    MilestoneThreeMutationTopologyQueryTraversalRow,
    MilestoneThreeMutationTopologyQueryTraversalView,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::TopologyLocalRewireNeighborhoodView;
use crate::projection::TopologyLoopCycleView;
use crate::topology_operators::TopologyRewireLoopSuccessorProgramDeclaration;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MutationTraversalProbe {
    view: MilestoneThreeMutationTopologyQueryTraversalView,
    view_digest: String,
    request_count: usize,
    relationship_proof_admission_count: usize,
    traversal_count: usize,
}

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_mutation_query_traversal_impl<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<MilestoneThreeMutationTopologyQueryTraversalRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_post_mutation_query_traversal_probe(
        runtime_factory,
        &format!("{stem}.mutation_query_traversal.left"),
    )?;
    let replay = execute_post_mutation_query_traversal_probe(
        runtime_factory,
        &format!("{stem}.mutation_query_traversal.replay"),
    )?;

    left.iter()
        .map(|left_probe| {
            let replay_probe = replay
                .iter()
                .find(|probe| probe.view == left_probe.view)
                .ok_or_else(|| mutation_query_traversal_error("missing replay probe view"))?;
            Ok(row_from_probes(left_probe, replay_probe))
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_mutation_query_traversal_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for view in required_mutation_query_traversal_views() {
        let row = report
            .mutation_query_traversal_rows
            .iter()
            .find(|row| row.view == *view)
            .ok_or_else(|| {
                mutation_query_traversal_error(&format!(
                    "missing mutation topology query traversal row for {}",
                    view.as_str()
                ))
            })?;
        if row.scenario != MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
            || !row.parity_verified
            || row.request_count == 0
            || row.relationship_proof_admission_count == 0
            || row.traversal_count == 0
        {
            return Err(mutation_query_traversal_error(&format!(
                "mutation topology query traversal row is not proof-bearing for {}",
                view.as_str()
            )));
        }
    }
    Ok(())
}

pub(in crate::certification::topology_operator_closeout) fn required_mutation_query_traversal_views(
) -> &'static [MilestoneThreeMutationTopologyQueryTraversalView] {
    &[
        MilestoneThreeMutationTopologyQueryTraversalView::PostMutationLocalRewireNeighborhood,
        MilestoneThreeMutationTopologyQueryTraversalView::PostMutationLoopCycleNeighborhood,
    ]
}

fn execute_post_mutation_query_traversal_probe<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<MutationTraversalProbe>, TopologyCertificationError>
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
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &workspace.read::<Value>(surfaces.relations()),
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let pre_mutation_query = TopologyReadProofHarness::new();
    let pre_mutation_rewire = pre_mutation_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let chosen_successor_identity = pre_mutation_rewire
        .cycle_identities()
        .get(3)
        .cloned()
        .ok_or_else(|| {
            mutation_query_traversal_error(
                "ambiguous local rewire should expose a successor candidate",
            )
        })?;
    let declaration =
        successor_relocation_declaration(&pre_mutation_rewire, &chosen_successor_identity)?;
    execute_current_head_topology_declaration::<TopologyRewireLoopSuccessorProgramDeclaration>(
        &mut workspace,
        &surfaces,
        declaration,
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;

    let post_mutation_query = TopologyReadProofHarness::new();
    let local_rewire = post_mutation_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let loop_cycle = post_mutation_query
        .loop_cycle(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;

    Ok(vec![
        local_rewire_probe(&local_rewire),
        loop_cycle_probe(&loop_cycle),
    ])
}

fn row_from_probes(
    left: &MutationTraversalProbe,
    replay: &MutationTraversalProbe,
) -> MilestoneThreeMutationTopologyQueryTraversalRow {
    let parity_verified = left.view_digest == replay.view_digest
        && left.request_count == replay.request_count
        && left.relationship_proof_admission_count == replay.relationship_proof_admission_count
        && left.traversal_count == replay.traversal_count;
    MilestoneThreeMutationTopologyQueryTraversalRow {
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

fn local_rewire_probe(view: &TopologyLocalRewireNeighborhoodView) -> MutationTraversalProbe {
    let request = view.request_report();
    MutationTraversalProbe {
        view: MilestoneThreeMutationTopologyQueryTraversalView::PostMutationLocalRewireNeighborhood,
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

fn loop_cycle_probe(view: &TopologyLoopCycleView) -> MutationTraversalProbe {
    let request = view.request_report();
    MutationTraversalProbe {
        view: MilestoneThreeMutationTopologyQueryTraversalView::PostMutationLoopCycleNeighborhood,
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

fn mutation_query_traversal_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three mutation topology query traversal failed: {reason}"
    ))
}
