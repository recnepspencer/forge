use crate::facade::{
    certify_milestone_three_ambiguous_local_rewire_continuity,
    WorthMilestoneThreeHostileOutcomeClass, WorthMilestoneThreeHostileScenario,
    WorthReplayParityStatus, WorthTopologyEditFamily, WorthTopologyEditNamingOutcome,
    WorthTopologyEditRejectionClass,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use worth_schema::facade::topology_authoring::WorthMilestoneOnePrimitiveCase;

#[test]
fn milestone_three_ambiguous_local_rewire_continuity_certifies_accepted_ambiguity_with_witness() {
    let report = certify_milestone_three_ambiguous_local_rewire_continuity(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.ambiguous_local_rewire",
    )
    .expect("milestone three ambiguous local rewire certification should succeed");

    assert_eq!(
        report.scenario,
        WorthMilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
    );
    assert_eq!(report.primitive_family, "SheetDisk(n)");
    assert_eq!(
        report.primitive,
        WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 }
    );
    assert_eq!(
        report.outcome_class,
        WorthMilestoneThreeHostileOutcomeClass::Accepted
    );
    assert_eq!(
        report.continuity_outcome_class,
        WorthTopologyEditNamingOutcome::Ambiguous
    );
    assert_eq!(
        report.continuity_rejection_class,
        Some(WorthTopologyEditRejectionClass::NamingContinuityAmbiguous)
    );
    assert!(report.rejection_class.is_none());
    assert!(report.rejected_edit_scope_report.is_none());
    assert!(report.bowtie_adjacent_witness.is_none());
    let witness = report
        .ambiguous_local_rewire_witness
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
    assert_eq!(report.edit_families.len(), 6);
    assert!(report
        .edit_families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    assert_eq!(report.topology_edit_digest.contract_count, 6);
    assert_eq!(report.naming_edit_continuity_matrix.rows.len(), 6);
    assert_eq!(report.naming_edit_continuity_matrix.ambiguous_count, 6);
    assert_eq!(report.naming_edit_continuity_matrix.preserved_count, 0);
    assert_eq!(report.naming_edit_continuity_matrix.rejected_count, 0);
    assert!(report.edit_replay_parity_report.replay_checked);
    assert_eq!(
        report.edit_replay_parity_report.parity_status,
        WorthReplayParityStatus::Match
    );
    assert_eq!(report.edit_replay_parity_report.mismatch_count, 0);
    assert_eq!(report.edit_replay_parity_report.step_rows.len(), 1);
    assert_eq!(report.edit_replay_parity_report.replay_step_rows.len(), 1);
    assert_eq!(
        report.edit_replay_parity_report.returned_to_baseline,
        Some(false)
    );
}

#[test]
fn milestone_three_ambiguous_local_rewire_report_is_deterministic_for_same_seeded_history() {
    let left = certify_milestone_three_ambiguous_local_rewire_continuity(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.ambiguous_local_rewire.deterministic",
    )
    .expect("left ambiguous local rewire certification should succeed");
    let right = certify_milestone_three_ambiguous_local_rewire_continuity(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.ambiguous_local_rewire.deterministic",
    )
    .expect("right ambiguous local rewire certification should succeed");

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(
        left.continuity_outcome_class,
        right.continuity_outcome_class
    );
    assert_eq!(left.topology_edit_digest, right.topology_edit_digest);
    assert_eq!(
        left.naming_edit_continuity_matrix,
        right.naming_edit_continuity_matrix
    );
    assert_eq!(
        left.ambiguous_local_rewire_witness,
        right.ambiguous_local_rewire_witness
    );
    assert_eq!(
        left.edit_replay_parity_report,
        right.edit_replay_parity_report
    );
}
