use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

use super::side_quest_types::{
    MilestoneThreeSideQuestBlockerRow, MilestoneThreeSideQuestCloseoutReport,
    MilestoneThreeSideQuestContractRow,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::{
    build_topology_read_view_parity_artifact, query_entity_identity,
    TopologyReadViewParityArtifact, TopologyReadViewRef,
};
use crate::query_domain::TopologyReadParityKind;
use crate::test_support::schema_topology_authoring_boundary::{
    seed_milestone_one_primitive_in_new_branch_through_schema_execution,
    seed_milestone_one_primitive_through_schema_execution,
};

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_side_quest_closeout_impl<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeSideQuestCloseoutReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let query = TopologyReadProofHarness::historical_from_workspace_token();
    let (replay_left, replay_right) =
        replay_local_rewire_parity_artifacts(runtime_factory, stem, &query)?;
    let replay_parity =
        query.record_view_parity(TopologyReadParityKind::Replay, &replay_left, &replay_right);
    let (branch_left, branch_right) =
        branch_local_loop_cycle_parity_artifacts(runtime_factory, stem, &query)?;
    let branch_parity = query.record_view_parity(
        TopologyReadParityKind::BranchLocal,
        &branch_left,
        &branch_right,
    );
    if !replay_parity.parity_verified || !branch_parity.parity_verified {
        return Err(TopologyCertificationError::Query(
            "milestone three side quest closeout failed replay or branch-local parity".to_string(),
        ));
    }

    let closeout = query.closeout_report();
    let proof_report = closeout.proof_report();
    Ok(MilestoneThreeSideQuestCloseoutReport {
        domain_read_request_count: proof_report.request_aggregate().request_count(),
        domain_read_parity_count: proof_report.parity_aggregate().topology_read_parity_count(),
        replay_checked_count: proof_report.parity_aggregate().replay_checked_count(),
        replay_verified_count: proof_report.parity_aggregate().replay_verified_count(),
        branch_local_checked_count: proof_report.parity_aggregate().branch_local_checked_count(),
        branch_local_verified_count: proof_report
            .parity_aggregate()
            .branch_local_verified_count(),
        contract_rows: closeout
            .no_n_plus_one_contract_rows()
            .iter()
            .map(|row| MilestoneThreeSideQuestContractRow {
                contract_name: row.contract().as_str().to_string(),
                status: row.status().as_str().to_string(),
                reason: row.reason().to_string(),
                row_digest: row.row_digest().to_string(),
            })
            .collect(),
        blocker_rows: closeout
            .phase_three_blocker_rows()
            .iter()
            .map(|row| MilestoneThreeSideQuestBlockerRow {
                blocker_name: row.blocker().as_str().to_string(),
                status: row.status().as_str().to_string(),
                reason: row.reason().to_string(),
                row_digest: row.row_digest().to_string(),
            })
            .collect(),
        phase_three_ready: closeout.phase_three_ready(),
    })
}

fn replay_local_rewire_parity_artifacts<F>(
    runtime_factory: &mut F,
    stem: &str,
    query: &TopologyReadProofHarness,
) -> Result<
    (
        TopologyReadViewParityArtifact,
        TopologyReadViewParityArtifact,
    ),
    TopologyCertificationError,
>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.side_quest.replay"),
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )?;
    let replay_basis = verified.read_basis().replay_of();
    Ok((
        local_rewire_parity_artifact(
            &runtime,
            &format!("{stem}.side_quest.replay.left"),
            &verified.read_basis(),
            query,
        )?,
        local_rewire_parity_artifact(
            &runtime,
            &format!("{stem}.side_quest.replay.right"),
            &replay_basis,
            query,
        )?,
    ))
}

fn branch_local_loop_cycle_parity_artifacts<F>(
    runtime_factory: &mut F,
    stem: &str,
    query: &TopologyReadProofHarness,
) -> Result<
    (
        TopologyReadViewParityArtifact,
        TopologyReadViewParityArtifact,
    ),
    TopologyCertificationError,
>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive_in_new_branch_through_schema_execution(
        &mut runtime,
        &format!("{stem}.side_quest.branch"),
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
        "feature",
        MutationOrigin::BranchLocalApplication,
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let replay_basis = verified.read_basis().replay_of();
    Ok((
        loop_cycle_parity_artifact(
            &runtime,
            &format!("{stem}.side_quest.branch.left"),
            &verified.read_basis(),
            query,
            5,
        )?,
        loop_cycle_parity_artifact(
            &runtime,
            &format!("{stem}.side_quest.branch.right"),
            &replay_basis,
            query,
            5,
        )?,
    ))
}

fn local_rewire_parity_artifact(
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
    query: &TopologyReadProofHarness,
) -> Result<TopologyReadViewParityArtifact, TopologyCertificationError> {
    let moved_identity = first_source_identity_for_snapshot_relation(
        runtime,
        read_basis,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let (mut workspace, _assembly) = snapshot_basis_workspace(runtime, stem, read_basis)?;
    let local_rewire = query
        .local_rewire_neighborhood(&mut workspace, &moved_identity, 4)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    Ok(build_topology_read_view_parity_artifact(
        read_basis,
        TopologyReadViewRef::LocalRewire(&local_rewire),
    ))
}

fn loop_cycle_parity_artifact(
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
    query: &TopologyReadProofHarness,
    depth: usize,
) -> Result<TopologyReadViewParityArtifact, TopologyCertificationError> {
    let start_identity = first_source_identity_for_snapshot_relation(
        runtime,
        read_basis,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let (mut workspace, _assembly) = snapshot_basis_workspace(runtime, stem, read_basis)?;
    let loop_cycle = query
        .loop_cycle(&mut workspace, &start_identity, depth)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    Ok(build_topology_read_view_parity_artifact(
        read_basis,
        TopologyReadViewRef::LoopCycle(&loop_cycle),
    ))
}

fn snapshot_basis_workspace(
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<(ForgeQueryWorkspace, TopologyDeclaredQuerySurfaces), TopologyCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "milestone three side quest could not open snapshot {:?}",
                read_basis.snapshot()
            ))
        })?;
    let adapters =
        TopologyRuntimeAdapters::snapshot_historical_basis(read_view, read_basis.clone());
    let mut workspace = topology_runtime(adapters, stem)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    Ok((workspace, surfaces))
}

fn first_source_identity_for_snapshot_relation(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "milestone three side quest could not open snapshot {:?}",
                read_basis.snapshot()
            ))
        })?;
    let expected_kind = RelationKind::Topology(relation_kind).kind_id();
    read_view
        .relations()
        .iter()
        .find(|record| record.kind.kind_id == expected_kind)
        .map(|record| query_entity_identity(record.source))
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "milestone three side quest snapshot did not expose `{}` source identities",
                relation_kind.kind_name()
            ))
        })
}
