use forge_relational::facade::history::BranchId;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_milestone_one_primitive_on_branch, MilestoneOnePrimitiveCase,
};
use schema::facade::platform::authority::MutationOrigin;

use crate::projection::read_views::domain::parity::{
    TopologyDomainQueryParityKind, TopologyDomainQueryViewParityArtifact,
};
use crate::projection::read_views::domain::{
    TopologyDomainQuery, TopologyNoNPlusOneContract, TopologyNoNPlusOneContractStatus,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

use super::parity_harness::{local_rewire_parity_artifact, loop_cycle_parity_artifact};

#[test]
fn domain_query_closeout_requires_replay_and_branch_local_view_parity_for_phase_three_ready() {
    let query = TopologyDomainQuery::load();
    let (replay_left, replay_right) = replay_local_rewire_parity_artifacts(&query);
    let replay_parity = query.record_view_parity(
        TopologyDomainQueryParityKind::Replay,
        &replay_left,
        &replay_right,
    );
    let (branch_left, branch_right) = branch_local_loop_cycle_parity_artifacts(&query);
    let branch_parity = query.record_view_parity(
        TopologyDomainQueryParityKind::BranchLocal,
        &branch_left,
        &branch_right,
    );

    let proof_report = query.proof_report();
    let closeout_report = query.closeout_report();

    assert!(replay_parity.parity_verified);
    assert!(branch_parity.parity_verified);
    assert_eq!(proof_report.parity_aggregate.domain_query_parity_count, 2);
    assert_eq!(proof_report.parity_aggregate.replay_checked_count, 1);
    assert_eq!(proof_report.parity_aggregate.replay_verified_count, 1);
    assert_eq!(proof_report.parity_aggregate.branch_local_checked_count, 1);
    assert_eq!(proof_report.parity_aggregate.branch_local_verified_count, 1);
    for contract in TopologyNoNPlusOneContract::ALL {
        assert_eq!(
            closeout_report.no_n_plus_one_contract_status(contract),
            TopologyNoNPlusOneContractStatus::Satisfied,
            "{} should be satisfied after replay and branch-local parity",
            contract.as_str()
        );
    }
    assert_eq!(
        TopologyNoNPlusOneContract::ALL.map(TopologyNoNPlusOneContract::as_str),
        [
            "topology_read_lowering_breadth",
            "topology_read_fallback_posture",
            "topology_read_view_parity",
            "topology_read_relationship_proof_posture",
        ]
    );
    for row in closeout_report.no_n_plus_one_contract_rows() {
        assert!(row.row_digest().starts_with(&format!(
            "contract={};status=Satisfied;reason=",
            row.contract().as_str()
        )));
    }
    assert!(closeout_report.phase_three_ready);
}

#[test]
fn domain_query_closeout_blocks_phase_three_when_branch_local_parity_lacks_replay_parity() {
    let query = TopologyDomainQuery::load();
    let (branch_left, branch_right) = branch_local_loop_cycle_parity_artifacts(&query);
    let branch_parity = query.record_view_parity(
        TopologyDomainQueryParityKind::BranchLocal,
        &branch_left,
        &branch_right,
    );

    let proof_report = query.proof_report();
    let closeout_report = query.closeout_report();

    assert!(branch_parity.parity_verified);
    assert_eq!(proof_report.parity_aggregate.domain_query_parity_count, 1);
    assert_eq!(proof_report.parity_aggregate.replay_checked_count, 0);
    assert_eq!(proof_report.parity_aggregate.replay_verified_count, 0);
    assert_eq!(proof_report.parity_aggregate.branch_local_checked_count, 1);
    assert_eq!(proof_report.parity_aggregate.branch_local_verified_count, 1);
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::ViewParity),
        TopologyNoNPlusOneContractStatus::Blocked
    );
    assert!(!closeout_report.phase_three_ready);
}

fn replay_local_rewire_parity_artifacts(
    query: &TopologyDomainQuery,
) -> (
    TopologyDomainQueryViewParityArtifact,
    TopologyDomainQueryViewParityArtifact,
) {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query-proof.ready.replay",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed replay primitive");
    let replay_basis = verified.read_basis.replay_of();
    let left_artifact = local_rewire_parity_artifact(
        query,
        &runtime,
        "query.domain-query-proof.ready.replay.left",
        &verified.read_basis,
    );
    let right_artifact = local_rewire_parity_artifact(
        query,
        &runtime,
        "query.domain-query-proof.ready.replay.right",
        &replay_basis,
    );
    (left_artifact, right_artifact)
}

fn branch_local_loop_cycle_parity_artifacts(
    query: &TopologyDomainQuery,
) -> (
    TopologyDomainQueryViewParityArtifact,
    TopologyDomainQueryViewParityArtifact,
) {
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
        "query.domain-query-proof.ready.branch",
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
        BranchId("feature".to_string()),
        MutationOrigin::BranchLocalApplication,
    )
    .expect("seed branch-local primitive");
    let replay_basis = verified.read_basis.replay_of();
    let left_artifact = loop_cycle_parity_artifact(
        query,
        &runtime,
        "query.domain-query-proof.ready.branch.left",
        &verified.read_basis,
        5,
    );
    let right_artifact = loop_cycle_parity_artifact(
        query,
        &runtime,
        "query.domain-query-proof.ready.branch.right",
        &replay_basis,
        5,
    );
    (left_artifact, right_artifact)
}




