use crate::facade::{
    certify_milestone_three_cancellation_chain_parity, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyEditFamily,
    TopologyEditNamingOutcome, TopologyEditRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn milestone_three_cancellation_chain_parity_replays_and_returns_to_baseline() {
    let report = certify_milestone_three_cancellation_chain_parity(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.cancellation",
    )
    .expect("milestone three cancellation-chain certification should succeed");

    assert_eq!(
        report.scenario,
        MilestoneThreeHostileScenario::CancellationChainParity
    );
    assert_eq!(report.primitive_family, "SheetDisk(n)");
    assert_eq!(
        report.primitive,
        MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 }
    );
    assert_eq!(
        report.edit_families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachBoundaryMembership,
            TopologyEditFamily::DetachBoundaryMembership,
            TopologyEditFamily::RetireTopologyEntity,
        ]
    );
    assert!(report.bowtie_adjacent_witness.is_none());
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Accepted
    );
    assert!(report.rejection_class.is_none());
    assert!(report.rejected_edit_scope_report.is_none());
    assert_eq!(report.topology_edit_digest.contract_count, 4);
    assert_eq!(report.naming_edit_continuity_matrix.rows.len(), 4);
    assert_eq!(
        report.continuity_outcome_class,
        TopologyEditNamingOutcome::Rejected
    );
    assert_eq!(
        report.continuity_rejection_class,
        Some(TopologyEditRejectionClass::NamingContinuityRejected)
    );
    assert!(report.edit_replay_parity_report.replay_checked);
    assert_eq!(
        report.edit_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.edit_replay_parity_report.mismatch_count, 0);
    assert_eq!(report.edit_replay_parity_report.step_rows.len(), 3);
    assert_eq!(report.edit_replay_parity_report.replay_step_rows.len(), 3);
    assert_eq!(
        report.edit_replay_parity_report.returned_to_baseline,
        Some(true)
    );
    assert_eq!(
        report
            .edit_replay_parity_report
            .baseline_materialized_topology_digest,
        report
            .edit_replay_parity_report
            .final_materialized_topology_digest
    );
    assert_eq!(
        report
            .edit_replay_parity_report
            .final_materialized_topology_digest,
        report
            .edit_replay_parity_report
            .replay_final_materialized_topology_digest
    );
}

#[test]
fn milestone_three_cancellation_chain_report_is_deterministic_for_same_seeded_history() {
    let left = certify_milestone_three_cancellation_chain_parity(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.cancellation.deterministic",
    )
    .expect("left cancellation-chain certification should succeed");
    let right = certify_milestone_three_cancellation_chain_parity(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.cancellation.deterministic",
    )
    .expect("right cancellation-chain certification should succeed");

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(left.topology_edit_digest, right.topology_edit_digest);
    assert_eq!(
        left.naming_edit_continuity_matrix,
        right.naming_edit_continuity_matrix
    );
    assert_eq!(
        left.edit_replay_parity_report,
        right.edit_replay_parity_report
    );
}




