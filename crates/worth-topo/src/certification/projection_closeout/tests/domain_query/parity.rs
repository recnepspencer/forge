use forge_relational::facade::history::BranchId;
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_milestone_one_primitive_on_branch, MilestoneOnePrimitiveCase,
};

use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::read_views::domain::parity::{
    compare_domain_query_view_parity, TopologyDomainQueryParityAggregateReport,
    TopologyDomainQueryParityKind,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

use super::parity_harness::{
    local_rewire_parity_artifact, loop_cycle_parity_artifact, radial_parity_artifact,
};

#[test]
fn domain_query_replay_parity_matches_for_local_rewire_view() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query-parity.replay",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis().replay_of();
    let left_query = TopologyReadProofHarness::new();
    let right_query = TopologyReadProofHarness::new();

    let left = local_rewire_parity_artifact(
        &left_query,
        &runtime,
        "query.domain-query-parity.replay.left",
        &verified.read_basis(),
    );
    let right = local_rewire_parity_artifact(
        &right_query,
        &runtime,
        "query.domain-query-parity.replay.right",
        &replay_basis,
    );
    let report =
        compare_domain_query_view_parity(TopologyDomainQueryParityKind::Replay, &left, &right);

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
fn domain_query_replay_parity_matches_for_radial_view() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query-parity.radial",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis().replay_of();
    let left_query = TopologyReadProofHarness::new();
    let right_query = TopologyReadProofHarness::new();

    let left = radial_parity_artifact(
        &left_query,
        &runtime,
        "query.domain-query-parity.radial.left",
        &verified.read_basis(),
    );
    let right = radial_parity_artifact(
        &right_query,
        &runtime,
        "query.domain-query-parity.radial.right",
        &replay_basis,
    );
    let report =
        compare_domain_query_view_parity(TopologyDomainQueryParityKind::Replay, &left, &right);

    assert_eq!(
        report.request_family,
        crate::projection::read_views::domain::report::TopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
    );
    assert!(report.branch_identity_match);
    assert!(report.snapshot_identity_match);
    assert!(report.execution_engine_match);
    assert!(report.fallback_posture_match);
    assert!(report.canonical_query_digest_match);
    assert!(report.canonical_result_shape_digest_match);
    assert!(report.breadth_counters_match);
    assert!(report.view_digest_match);
    assert!(report.parity_verified);
}

#[test]
fn domain_query_branch_local_parity_matches_for_feature_loop_cycle_view() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
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
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
        BranchId("feature".to_string()),
        MutationOrigin::BranchLocalApplication,
    )
    .expect("seed branch-local primitive");
    let replay_basis = verified.read_basis().replay_of();
    let left_query = TopologyReadProofHarness::new();
    let right_query = TopologyReadProofHarness::new();

    let left = loop_cycle_parity_artifact(
        &left_query,
        &runtime,
        "query.domain-query-parity.branch.left",
        &verified.read_basis(),
        5,
    );
    let right = loop_cycle_parity_artifact(
        &right_query,
        &runtime,
        "query.domain-query-parity.branch.right",
        &replay_basis,
        5,
    );
    let report =
        compare_domain_query_view_parity(TopologyDomainQueryParityKind::BranchLocal, &left, &right);

    assert_eq!(report.left_branch_id, "feature");
    assert_eq!(report.right_branch_id, "feature");
    assert!(report.branch_identity_match);
    assert!(report.snapshot_identity_match);
    assert!(report.parity_verified);
}

#[test]
fn domain_query_parity_aggregate_reports_replay_and_branch_local_coverage() {
    let mut replay_runtime = build_milestone_one_runtime().expect(" runtime");
    let replay_verified = seed_milestone_one_primitive(
        &mut replay_runtime,
        "query.domain-query-parity.aggregate.replay",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed replay primitive");
    let replay_report = compare_domain_query_view_parity(
        TopologyDomainQueryParityKind::Replay,
        &local_rewire_parity_artifact(
            &TopologyReadProofHarness::new(),
            &replay_runtime,
            "query.domain-query-parity.aggregate.replay.left",
            &replay_verified.read_basis(),
        ),
        &local_rewire_parity_artifact(
            &TopologyReadProofHarness::new(),
            &replay_runtime,
            "query.domain-query-parity.aggregate.replay.right",
            &replay_verified.read_basis().replay_of(),
        ),
    );

    let mut branch_runtime = build_milestone_one_runtime().expect(" runtime");
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
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
        BranchId("feature".to_string()),
        MutationOrigin::BranchLocalApplication,
    )
    .expect("seed branch-local primitive");
    let branch_report = compare_domain_query_view_parity(
        TopologyDomainQueryParityKind::BranchLocal,
        &loop_cycle_parity_artifact(
            &TopologyReadProofHarness::new(),
            &branch_runtime,
            "query.domain-query-parity.aggregate.branch.left",
            &branch_verified.read_basis(),
            5,
        ),
        &loop_cycle_parity_artifact(
            &TopologyReadProofHarness::new(),
            &branch_runtime,
            "query.domain-query-parity.aggregate.branch.right",
            &branch_verified.read_basis().replay_of(),
            5,
        ),
    );

    let aggregate = TopologyDomainQueryParityAggregateReport::from_reports(&[
        replay_report.clone(),
        branch_report.clone(),
    ]);

    assert!(replay_report.parity_verified);
    assert!(branch_report.parity_verified);
    assert_eq!(aggregate.domain_query_parity_count, 2);
    assert_eq!(aggregate.view_determinism_checked_count, 2);
    assert_eq!(aggregate.view_determinism_verified_count, 2);
    assert_eq!(aggregate.replay_checked_count, 1);
    assert_eq!(aggregate.replay_verified_count, 1);
    assert_eq!(aggregate.branch_local_checked_count, 1);
    assert_eq!(aggregate.branch_local_verified_count, 1);
    assert_eq!(aggregate.parity_rows.len(), 2);
    assert!(aggregate.parity_rows.iter().any(|row| {
        row.parity_kind == TopologyDomainQueryParityKind::Replay
            && row.checked_count == 1
            && row.verified_count == 1
    }));
    assert!(aggregate.parity_rows.iter().any(|row| {
        row.parity_kind == TopologyDomainQueryParityKind::BranchLocal
            && row.checked_count == 1
            && row.verified_count == 1
    }));
}
