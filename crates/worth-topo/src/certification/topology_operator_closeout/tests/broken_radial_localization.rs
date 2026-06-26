use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario, ReplayParityStatus,
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationFamily,
    TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::cached_scenario_report;

#[test]
fn milestone_three_broken_radial_localization_certifies_exact_radial_rejection_and_replay() {
    let report = cached_scenario_report(MilestoneThreeHostileScenario::BrokenRadialLocalization);

    assert_eq!(
        report.scenario,
        MilestoneThreeHostileScenario::BrokenRadialLocalization
    );
    assert_eq!(report.primitive_family, "NmtEdgeFan(k)");
    assert_eq!(
        report.primitive,
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 }
    );
    assert_eq!(
        report.mutation_families(),
        vec![TopologyMutationFamily::SpliceRadialAdjacency]
    );
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Rejected
    );
    assert_eq!(
        report.rejection_class,
        Some(TopologyMutationRejectionClass::InvariantBlocked)
    );
    assert_eq!(
        report.continuity_outcome_class(),
        TopologyMutationNamingOutcome::Ambiguous
    );
    assert_eq!(
        report.continuity_rejection_class(),
        Some(TopologyMutationRejectionClass::NamingContinuityAmbiguous)
    );
    let witness = report
        .broken_radial_witness
        .expect("broken radial report should expose explicit witness evidence");
    assert_ne!(
        witness.current_target_half_edge_identity,
        witness.illegal_target_half_edge_identity
    );
    assert_eq!(
        witness.source_edge_identity,
        witness.current_target_edge_identity
    );
    assert_ne!(
        witness.source_edge_identity,
        witness.illegal_target_edge_identity
    );
    let rejected = report
        .rejected_mutation_scope_report
        .expect("typed radial rejection should expose exact rejected scope report");
    assert_eq!(rejected.rows.len(), 1);
    assert_eq!(
        rejected.rows[0].family,
        TopologyMutationFamily::SpliceRadialAdjacency
    );
    assert!(rejected.rows[0]
        .changed_scopes
        .contains(&TopologyMutationChangedScope::RadialNeighborhood));
    assert!(rejected.rows[0]
        .derived_regions
        .contains(&TopologyDerivedRegion::RadialNeighborhoodRegion));
    assert!(report.mutation_replay_parity_report.replay_checked);
    assert_eq!(
        report.mutation_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.mutation_replay_parity_report.mismatch_count, 0);
    assert_eq!(report.mutation_replay_parity_report.step_rows.len(), 1);
    assert_eq!(
        report.mutation_replay_parity_report.replay_step_rows.len(),
        1
    );
    assert_eq!(
        report.mutation_replay_parity_report.returned_to_baseline,
        Some(true)
    );
}

#[test]
fn milestone_three_broken_radial_localization_report_is_deterministic_for_same_seeded_history() {
    let left = cached_scenario_report(MilestoneThreeHostileScenario::BrokenRadialLocalization);
    let right = cached_scenario_report(MilestoneThreeHostileScenario::BrokenRadialLocalization);

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(left.rejection_class, right.rejection_class);
    assert_eq!(
        left.topology_mutation_digest(),
        right.topology_mutation_digest()
    );
    assert_eq!(
        left.naming_mutation_continuity_matrix(),
        right.naming_mutation_continuity_matrix()
    );
    assert_eq!(left.broken_radial_witness, right.broken_radial_witness);
    assert_eq!(
        left.rejected_mutation_scope_report,
        right.rejected_mutation_scope_report
    );
    assert_eq!(
        left.mutation_replay_parity_report,
        right.mutation_replay_parity_report
    );
}
