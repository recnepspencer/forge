use crate::facade::{
    certify_milestone_three_cancellation_chain_parity, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyMutationFamily,
    TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
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
        report.mutation_families,
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachBoundaryMembership,
            TopologyMutationFamily::DetachBoundaryMembership,
            TopologyMutationFamily::RetireTopologyEntity,
        ]
    );
    assert!(report.bowtie_adjacent_witness.is_none());
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Accepted
    );
    assert!(report.rejection_class.is_none());
    assert!(report.rejected_mutation_scope_report.is_none());
    assert_eq!(report.topology_mutation_digest.mutation_record_count, 4);
    assert_eq!(report.naming_mutation_continuity_matrix.rows.len(), 4);
    assert_eq!(
        report.continuity_outcome_class,
        TopologyMutationNamingOutcome::Rejected
    );
    assert_eq!(
        report.continuity_rejection_class,
        Some(TopologyMutationRejectionClass::NamingContinuityRejected)
    );
    assert!(report.mutation_replay_parity_report.replay_checked);
    assert_eq!(
        report.mutation_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.mutation_replay_parity_report.mismatch_count, 0);
    assert_eq!(report.mutation_replay_parity_report.step_rows.len(), 3);
    assert_eq!(
        report.mutation_replay_parity_report.replay_step_rows.len(),
        3
    );
    assert_eq!(
        report.mutation_replay_parity_report.returned_to_baseline,
        Some(true)
    );
    assert_eq!(
        report
            .mutation_replay_parity_report
            .baseline_materialized_topology_digest,
        report
            .mutation_replay_parity_report
            .final_materialized_topology_digest
    );
    assert_eq!(
        report
            .mutation_replay_parity_report
            .final_materialized_topology_digest,
        report
            .mutation_replay_parity_report
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
    assert_eq!(
        left.topology_mutation_digest,
        right.topology_mutation_digest
    );
    assert_eq!(
        left.naming_mutation_continuity_matrix,
        right.naming_mutation_continuity_matrix
    );
    assert_eq!(
        left.mutation_replay_parity_report,
        right.mutation_replay_parity_report
    );
}
