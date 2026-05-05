use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};

use super::relation_update_support::RelationUpdateQuerySupport;
use crate::query::domain::parity::WorthTopologyDomainQueryParityKind;
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::read_stage::open_topology_read_view;
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn relation_update_query_support_reports_domain_query_proof_report_with_replay_parity() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.domain-query-parity",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis.replay_of();
    let replay_workspace = {
        let read_view = open_topology_read_view(&runtime, &replay_basis)
            .expect("snapshot read view should open");
        let adapters = WorthTopologyRuntimeAdapters::snapshot_read_only(
            read_view,
            replay_basis.snapshot().clone(),
        );
        let mut workspace =
            worth_topology_runtime(adapters, "worth.current-head.domain-query-parity.replay")
                .expect("snapshot workspace");
        let assembly =
            WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
        (workspace, assembly)
    };

    let current_head_support = {
        let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
        let mut workspace =
            worth_topology_runtime(adapters, "worth.current-head.domain-query-parity.runtime")
                .expect("workspace");
        let assembly =
            WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
        RelationUpdateQuerySupport::load(&workspace, &assembly)
    };
    let (snapshot_workspace, snapshot_assembly) = replay_workspace;
    let replay_support = RelationUpdateQuerySupport::load(&snapshot_workspace, &snapshot_assembly);
    let moved_identity = current_head_support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );

    let left =
        current_head_support.local_rewire_parity_artifact(&verified.read_basis, &moved_identity, 6);
    let right = replay_support.local_rewire_parity_artifact(&replay_basis, &moved_identity, 6);
    current_head_support.record_view_parity(
        WorthTopologyDomainQueryParityKind::Replay,
        &left,
        &right,
    );

    let proof_report = current_head_support.proof_report();
    assert_eq!(proof_report.request_aggregate.request_count, 1);
    assert_eq!(proof_report.request_aggregate.lowered_traversal_count, 2);
    assert_eq!(proof_report.parity_aggregate.domain_query_parity_count, 1);
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
}
