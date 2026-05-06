use forge_relational::facade::history::BranchId;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_milestone_one_primitive_on_branch,
    WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::{
    DerivedTopologyReadBasis, WorthMutationOrigin, WorthTopologyRelationKind,
};

use crate::query::domain::parity::{
    build_domain_query_view_parity_artifact, compare_domain_query_view_parity,
    WorthTopologyDomainQueryParityAggregateReport, WorthTopologyDomainQueryParityKind,
    WorthTopologyDomainQueryViewRef,
};
use crate::query::domain::WorthTopologyDomainQuery;
use crate::runtime_invariants::build_worth_milestone_one_runtime;

use super::support::snapshot_basis_workspace;

#[test]
fn domain_query_replay_parity_matches_for_local_rewire_view() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query-parity.replay",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis.replay_of();

    let left = local_rewire_parity_artifact(
        &runtime,
        "query.domain-query-parity.replay.left",
        &verified.read_basis,
    );
    let right = local_rewire_parity_artifact(
        &runtime,
        "query.domain-query-parity.replay.right",
        &replay_basis,
    );
    let report =
        compare_domain_query_view_parity(WorthTopologyDomainQueryParityKind::Replay, &left, &right);

    assert_eq!(report.request_family, left.request_family());
    assert!(report.branch_identity_match);
    assert!(report.snapshot_identity_match);
    assert!(report.execution_engine_match);
    assert!(report.fallback_posture_match);
    assert!(report.canonical_query_digest_match);
    assert!(report.canonical_result_shape_digest_match);
    assert!(report.breadth_counters_match);
    assert!(report.view_digest_match);
    assert!(report.parity_verified);
    assert_eq!(left.authority_branch_id(), "main");
    assert_eq!(right.authority_branch_id(), "main");
}

#[test]
fn domain_query_branch_local_parity_matches_for_feature_loop_cycle_view() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("feature branch");
    let verified = seed_milestone_one_primitive_on_branch(
        &mut runtime,
        "query.domain-query-parity.branch",
        &WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
        BranchId("feature".to_string()),
        WorthMutationOrigin::BranchLocalApplication,
    )
    .expect("seed branch-local primitive");
    let replay_basis = verified.read_basis.replay_of();

    let left = loop_cycle_parity_artifact(
        &runtime,
        "query.domain-query-parity.branch.left",
        &verified.read_basis,
        5,
    );
    let right = loop_cycle_parity_artifact(
        &runtime,
        "query.domain-query-parity.branch.right",
        &replay_basis,
        5,
    );
    let report = compare_domain_query_view_parity(
        WorthTopologyDomainQueryParityKind::BranchLocal,
        &left,
        &right,
    );

    assert_eq!(report.left_branch_id, "feature");
    assert_eq!(report.right_branch_id, "feature");
    assert!(report.branch_identity_match);
    assert!(report.snapshot_identity_match);
    assert!(report.parity_verified);
}

#[test]
fn domain_query_parity_aggregate_reports_replay_and_branch_local_coverage() {
    let mut replay_runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let replay_verified = seed_milestone_one_primitive(
        &mut replay_runtime,
        "query.domain-query-parity.aggregate.replay",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed replay primitive");
    let replay_report = compare_domain_query_view_parity(
        WorthTopologyDomainQueryParityKind::Replay,
        &local_rewire_parity_artifact(
            &replay_runtime,
            "query.domain-query-parity.aggregate.replay.left",
            &replay_verified.read_basis,
        ),
        &local_rewire_parity_artifact(
            &replay_runtime,
            "query.domain-query-parity.aggregate.replay.right",
            &replay_verified.read_basis.replay_of(),
        ),
    );

    let mut branch_runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    branch_runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("feature branch");
    let branch_verified = seed_milestone_one_primitive_on_branch(
        &mut branch_runtime,
        "query.domain-query-parity.aggregate.branch",
        &WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
        BranchId("feature".to_string()),
        WorthMutationOrigin::BranchLocalApplication,
    )
    .expect("seed branch-local primitive");
    let branch_report = compare_domain_query_view_parity(
        WorthTopologyDomainQueryParityKind::BranchLocal,
        &loop_cycle_parity_artifact(
            &branch_runtime,
            "query.domain-query-parity.aggregate.branch.left",
            &branch_verified.read_basis,
            5,
        ),
        &loop_cycle_parity_artifact(
            &branch_runtime,
            "query.domain-query-parity.aggregate.branch.right",
            &branch_verified.read_basis.replay_of(),
            5,
        ),
    );

    let aggregate = WorthTopologyDomainQueryParityAggregateReport::from_reports(&[
        replay_report.clone(),
        branch_report.clone(),
    ]);

    assert!(replay_report.parity_verified);
    assert!(branch_report.parity_verified);
    assert_eq!(aggregate.domain_query_parity_count, 2);
    assert_eq!(aggregate.replay_checked_count, 1);
    assert_eq!(aggregate.replay_verified_count, 1);
    assert_eq!(aggregate.branch_local_checked_count, 1);
    assert_eq!(aggregate.branch_local_verified_count, 1);
    assert_eq!(aggregate.parity_rows.len(), 2);
    assert!(aggregate.parity_rows.iter().any(|row| {
        row.parity_kind == WorthTopologyDomainQueryParityKind::Replay
            && row.checked_count == 1
            && row.verified_count == 1
    }));
    assert!(aggregate.parity_rows.iter().any(|row| {
        row.parity_kind == WorthTopologyDomainQueryParityKind::BranchLocal
            && row.checked_count == 1
            && row.verified_count == 1
    }));
}

#[test]
fn domain_query_proof_report_aggregates_request_and_parity_evidence_on_the_boundary() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query-proof.replay",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis.replay_of();
    let (left_workspace, left_assembly) = snapshot_basis_workspace(
        &runtime,
        "query.domain-query-proof.replay.left",
        &verified.read_basis,
    );
    let (right_workspace, right_assembly) = snapshot_basis_workspace(
        &runtime,
        "query.domain-query-proof.replay.right",
        &replay_basis,
    );
    let left_query =
        WorthTopologyDomainQuery::load(&left_workspace, &left_assembly).expect("domain query");
    let right_query =
        WorthTopologyDomainQuery::load(&right_workspace, &right_assembly).expect("domain query");
    let moved_identity = left_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let left_view = left_query
        .local_rewire_neighborhood(&moved_identity, 6)
        .expect("left local rewire neighborhood should load");
    let right_view = right_query
        .local_rewire_neighborhood(&moved_identity, 6)
        .expect("right local rewire neighborhood should load");
    let left_artifact = build_domain_query_view_parity_artifact(
        &verified.read_basis,
        WorthTopologyDomainQueryViewRef::LocalRewire(&left_view),
    );
    let right_artifact = build_domain_query_view_parity_artifact(
        &replay_basis,
        WorthTopologyDomainQueryViewRef::LocalRewire(&right_view),
    );
    let parity = left_query.record_view_parity(
        WorthTopologyDomainQueryParityKind::Replay,
        &left_artifact,
        &right_artifact,
    );
    let proof_report = left_query.proof_report();

    assert!(parity.parity_verified);
    assert_eq!(proof_report.request_aggregate.request_count, 1);
    assert_eq!(proof_report.request_aggregate.lowered_traversal_count, 2);
    assert_eq!(proof_report.parity_aggregate.domain_query_parity_count, 1);
    assert_eq!(proof_report.parity_aggregate.replay_checked_count, 1);
    assert_eq!(proof_report.parity_aggregate.replay_verified_count, 1);
    assert_eq!(proof_report.parity_aggregate.parity_rows.len(), 1);
    assert_eq!(
        proof_report.parity_aggregate.parity_rows[0].request_family,
        proof_report.request_aggregate.family_rows[0].request_family
    );
}

fn local_rewire_parity_artifact(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> crate::query::domain::parity::WorthTopologyDomainQueryViewParityArtifact {
    let (workspace, assembly) = snapshot_basis_workspace(runtime, stem, read_basis);
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");
    let moved_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let local_rewire = domain_query
        .local_rewire_neighborhood(&moved_identity, 6)
        .expect("local rewire neighborhood should load");
    build_domain_query_view_parity_artifact(
        read_basis,
        WorthTopologyDomainQueryViewRef::LocalRewire(&local_rewire),
    )
}

fn loop_cycle_parity_artifact(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
    depth: usize,
) -> crate::query::domain::parity::WorthTopologyDomainQueryViewParityArtifact {
    let (mut workspace, assembly) = snapshot_basis_workspace(runtime, stem, read_basis);
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");
    let start_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");
    let loop_cycle = domain_query
        .loop_cycle(&mut workspace, &start_identity, depth)
        .expect("loop cycle should load");
    build_domain_query_view_parity_artifact(
        read_basis,
        WorthTopologyDomainQueryViewRef::LoopCycle(&loop_cycle),
    )
}
