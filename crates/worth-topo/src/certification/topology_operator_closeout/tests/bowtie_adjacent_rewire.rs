use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario, ReplayParityStatus,
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationFamily,
    TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::cached_scenario_report;

#[test]
fn milestone_three_bowtie_adjacent_rewire_certifies_typed_rejection_with_exact_scope_evidence() {
    let report = cached_scenario_report(MilestoneThreeHostileScenario::BowtieAdjacentRewire);

    assert_eq!(
        report.scenario,
        MilestoneThreeHostileScenario::BowtieAdjacentRewire
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
    let witness = report
        .bowtie_adjacent_witness
        .as_ref()
        .expect("bowtie hostile report should expose explicit witness evidence");
    assert_ne!(
        witness.source_half_edge_identity,
        witness.target_half_edge_identity
    );
    assert_ne!(witness.source_edge_identity, witness.target_edge_identity);
    assert!(!witness.shared_vertex_identity.is_empty());
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Rejected
    );
    assert_eq!(
        report.continuity_outcome_class(),
        TopologyMutationNamingOutcome::Ambiguous
    );
    assert_eq!(
        report.continuity_rejection_class(),
        Some(TopologyMutationRejectionClass::NamingContinuityAmbiguous)
    );
    assert_eq!(
        report.rejection_class,
        Some(TopologyMutationRejectionClass::InvariantBlocked)
    );
    assert_eq!(report.topology_mutation_digest().mutation_record_count, 1);
    assert_eq!(report.naming_mutation_continuity_matrix().rows.len(), 1);
    assert_eq!(
        report.naming_mutation_continuity_matrix().ambiguous_count,
        1
    );
    assert!(report.mutation_replay_parity_report.replay_checked);
    assert_eq!(
        report.mutation_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.mutation_replay_parity_report.step_rows.len(), 1);
    assert_eq!(
        report.mutation_replay_parity_report.replay_step_rows.len(),
        1
    );
    let rejected = report
        .rejected_mutation_scope_report
        .expect("typed rejection should expose rejected mutation scope report");
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
}

#[test]
fn milestone_three_bowtie_adjacent_rewire_report_is_deterministic_for_same_seeded_history() {
    let left = cached_scenario_report(MilestoneThreeHostileScenario::BowtieAdjacentRewire);
    let right = cached_scenario_report(MilestoneThreeHostileScenario::BowtieAdjacentRewire);

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(left.rejection_class, right.rejection_class);
    assert_eq!(
        left.topology_mutation_digest(),
        right.topology_mutation_digest()
    );
    assert_eq!(left.bowtie_adjacent_witness, right.bowtie_adjacent_witness);
    assert_eq!(
        left.naming_mutation_continuity_matrix(),
        right.naming_mutation_continuity_matrix()
    );
    assert_eq!(
        left.rejected_mutation_scope_report,
        right.rejected_mutation_scope_report
    );
    assert_eq!(
        left.mutation_replay_parity_report,
        right.mutation_replay_parity_report
    );
}
