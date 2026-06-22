use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::query_runtime_support::QueryRuntimeSupport;
use crate::projection::read_views::domain::closeout::{
    TopologyReadCloseoutStatus, TopologyReadPhaseThreeBlocker, TopologyReadPhaseThreeBlockerStatus,
};
use crate::projection::read_views::domain::parity::TopologyReadParityKind;
use crate::projection::read_views::domain::report::TopologyReadRequestFamily;
use crate::projection::read_views::domain::{
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractStatus,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::open_topology_read_view;
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn relation_update_query_support_reports_topology_read_proof_report_with_replay_parity() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        ".current-head.topology-read-parity",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis().replay_of();
    let replay_workspace = {
        let read_view = open_topology_read_view(&runtime, &replay_basis)
            .expect("snapshot read view should open");
        let adapters =
            TopologyRuntimeAdapters::snapshot_historical_basis(read_view, replay_basis.clone());
        let mut workspace = topology_runtime(adapters, ".current-head.topology-read-parity.replay")
            .expect("snapshot workspace");
        let surfaces =
            crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
                &mut workspace,
            )
            .expect("declare surfaces");
        (workspace, surfaces)
    };

    let (current_head_support, mut current_head_workspace) = {
        let adapters = TopologyRuntimeAdapters::current_head(runtime);
        let mut workspace =
            topology_runtime(adapters, ".current-head.topology-read-parity.runtime")
                .expect("workspace");
        let surfaces =
            crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
                &mut workspace,
            )
            .expect("declare surfaces");
        (
            QueryRuntimeSupport::load(&mut workspace, &surfaces),
            workspace,
        )
    };
    let (mut snapshot_workspace, snapshot_surfaces) = replay_workspace;
    let replay_support = QueryRuntimeSupport::load(&mut snapshot_workspace, &snapshot_surfaces);
    let moved_identity = current_head_support.first_source_identity_for_relation_kind(
        schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
    );

    let left = current_head_support.local_rewire_parity_artifact(
        &mut current_head_workspace,
        &verified.read_basis(),
        &moved_identity,
        4,
    );
    let right = replay_support.local_rewire_parity_artifact(
        &mut snapshot_workspace,
        &replay_basis,
        &moved_identity,
        4,
    );
    current_head_support.record_view_parity(TopologyReadParityKind::Replay, &left, &right);

    let proof_report = current_head_support.proof_report();
    let closeout_report = current_head_support.closeout_report();
    assert_eq!(proof_report.request_aggregate.request_count, 1);
    assert_eq!(
        proof_report
            .request_aggregate
            .query_runtime_current_execution_count,
        1
    );
    assert_eq!(
        proof_report
            .request_aggregate
            .anchored_expansion_execution_count,
        1
    );
    assert_eq!(proof_report.request_aggregate.lowered_traversal_count, 2);
    assert_eq!(proof_report.parity_aggregate.topology_read_parity_count, 1);
    assert_eq!(
        proof_report.parity_aggregate.view_determinism_checked_count,
        1
    );
    assert_eq!(
        proof_report
            .parity_aggregate
            .view_determinism_verified_count,
        1
    );
    assert_eq!(proof_report.parity_aggregate.replay_checked_count, 1);
    assert_eq!(proof_report.parity_aggregate.replay_verified_count, 1);
    assert_eq!(proof_report.parity_aggregate.branch_local_checked_count, 0);
    assert_eq!(proof_report.parity_aggregate.parity_rows.len(), 1);
    assert_eq!(
        proof_report.parity_aggregate.parity_rows[0].checked_count,
        1
    );
    assert_eq!(
        proof_report.parity_aggregate.parity_rows[0].verified_count,
        1
    );
    assert_eq!(closeout_report.query_executed_family_count, 1);
    assert_eq!(closeout_report.query_executed_debt_free_family_count, 1);
    assert_eq!(closeout_report.query_executed_debt_backed_family_count, 0);
    assert_eq!(closeout_report.debt_family_count, 0);
    assert_eq!(
        closeout_report.status(TopologyReadRequestFamily::LocalRewireNeighborhood),
        TopologyReadCloseoutStatus::QueryExecutedDebtFree
    );
    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::ParityDeterminismGap),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::LoweringBreadth),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::FallbackPosture),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::ViewParity),
        TopologyNoNPlusOneContractStatus::Blocked
    );
    assert_eq!(
        closeout_report
            .no_n_plus_one_contract_status(TopologyNoNPlusOneContract::RelationshipProofPosture),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert!(!closeout_report.phase_three_ready);
}
