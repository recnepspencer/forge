use crate::facade::{
    certify_milestone_three_ambiguous_local_rewire_continuity, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn milestone_three_ambiguous_local_rewire_continuity_certifies_accepted_ambiguity_with_witness() {
    let report = certify_milestone_three_ambiguous_local_rewire_continuity(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.ambiguous_local_rewire",
    )
    .expect("milestone three ambiguous local rewire certification should succeed");

    assert_eq!(
        report.scenario,
        MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
    );
    assert_eq!(report.primitive_family, "SheetDisk(n)");
    assert_eq!(
        report.primitive,
        MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 }
    );
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Accepted
    );
    assert_eq!(
        report.continuity_outcome_class(),
        TopologyMutationNamingOutcome::Ambiguous
    );
    assert_eq!(
        report.continuity_rejection_class(),
        Some(TopologyMutationRejectionClass::NamingContinuityAmbiguous)
    );
    assert!(report.rejection_class.is_none());
    assert!(report.rejected_mutation_scope_report.is_none());
    assert!(report.bowtie_adjacent_witness.is_none());
    assert_eq!(
        report.derived_fallback_policy(),
        Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
    );
    assert!(report
        .fallback_explanation_detail()
        .is_some_and(|detail| detail.contains("fallback")));
    let witness = report
        .ambiguous_local_rewire_witness
        .as_ref()
        .expect("ambiguous local rewire report should expose accepted alternate witness");
    assert_eq!(
        witness.moved_half_edge_identity,
        witness.alternate_moved_half_edge_identity
    );
    assert_eq!(
        witness.old_successor_identity,
        witness.alternate_old_successor_identity
    );
    assert_ne!(
        witness.chosen_successor_identity,
        witness.alternate_successor_identity
    );
    assert_ne!(
        witness.chosen_materialized_topology_digest,
        witness.alternate_materialized_topology_digest
    );
    assert_eq!(report.mutation_families().len(), 6);
    assert!(report
        .mutation_families()
        .iter()
        .all(|family| *family == TopologyMutationFamily::RewireLoopSuccessor));
    assert_eq!(report.topology_mutation_digest().mutation_record_count, 6);
    assert_eq!(report.naming_mutation_continuity_matrix().rows.len(), 6);
    assert_eq!(
        report.naming_mutation_continuity_matrix().ambiguous_count,
        6
    );
    assert_eq!(
        report.naming_mutation_continuity_matrix().preserved_count,
        0
    );
    assert_eq!(report.naming_mutation_continuity_matrix().rejected_count, 0);
    assert!(report.mutation_replay_parity_report.replay_checked);
    assert_eq!(
        report.mutation_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.mutation_replay_parity_report.mismatch_count, 0);
    assert_eq!(report.mutation_replay_parity_report.step_rows.len(), 1);
    assert_eq!(
        report.mutation_replay_parity_report.step_rows[0].derived_fallback_policy,
        Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
    );
    assert_eq!(
        report.mutation_replay_parity_report.replay_step_rows.len(),
        1
    );
    assert_eq!(
        report.mutation_replay_parity_report.returned_to_baseline,
        Some(false)
    );
}

#[test]
fn milestone_three_ambiguous_local_rewire_report_is_deterministic_for_same_seeded_history() {
    let left = certify_milestone_three_ambiguous_local_rewire_continuity(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.ambiguous_local_rewire.deterministic",
    )
    .expect("left ambiguous local rewire certification should succeed");
    let right = certify_milestone_three_ambiguous_local_rewire_continuity(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.ambiguous_local_rewire.deterministic",
    )
    .expect("right ambiguous local rewire certification should succeed");

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(
        left.continuity_outcome_class(),
        right.continuity_outcome_class()
    );
    assert_eq!(
        left.topology_mutation_digest(),
        right.topology_mutation_digest()
    );
    assert_eq!(
        left.naming_mutation_continuity_matrix(),
        right.naming_mutation_continuity_matrix()
    );
    assert_eq!(
        left.ambiguous_local_rewire_witness,
        right.ambiguous_local_rewire_witness
    );
    assert_eq!(
        left.mutation_replay_parity_report,
        right.mutation_replay_parity_report
    );
}
