use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario, ReplayParityStatus,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily, TopologyMutationNamingOutcome,
    TopologyMutationRejectionClass,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::cached_scenario_report;

#[test]
fn milestone_three_cancellation_chain_parity_replays_and_returns_to_baseline() {
    let report = cached_scenario_report(MilestoneThreeHostileScenario::CancellationChainParity);

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
        report.mutation_families(),
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
    assert_eq!(
        report.derived_fallback_policy(),
        Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
    );
    assert!(report
        .fallback_explanation_detail()
        .is_some_and(|detail| detail.contains("fallback")));
    assert_eq!(report.topology_mutation_digest().mutation_record_count, 4);
    assert_eq!(report.naming_mutation_continuity_matrix().rows.len(), 4);
    assert_eq!(
        report.continuity_outcome_class(),
        TopologyMutationNamingOutcome::Rejected
    );
    assert_eq!(
        report.continuity_rejection_class(),
        Some(TopologyMutationRejectionClass::NamingContinuityRejected)
    );
    assert!(report.mutation_replay_parity_report.replay_checked);
    assert_eq!(
        report.mutation_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.mutation_replay_parity_report.mismatch_count, 0);
    assert_eq!(report.mutation_replay_parity_report.step_rows.len(), 3);
    assert!(report
        .mutation_replay_parity_report
        .step_rows
        .iter()
        .all(|row| {
            row.derived_fallback_policy
                == Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
                && row
                    .fallback_explanation_detail()
                    .is_some_and(|detail| detail.contains("fallback"))
        }));
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
    let left = cached_scenario_report(MilestoneThreeHostileScenario::CancellationChainParity);
    let right = cached_scenario_report(MilestoneThreeHostileScenario::CancellationChainParity);

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(
        left.topology_mutation_digest(),
        right.topology_mutation_digest()
    );
    assert_eq!(
        left.naming_mutation_continuity_matrix(),
        right.naming_mutation_continuity_matrix()
    );
    assert_eq!(
        left.mutation_replay_parity_report,
        right.mutation_replay_parity_report
    );
}
