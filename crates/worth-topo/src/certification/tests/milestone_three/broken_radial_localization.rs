use crate::facade::{
    certify_milestone_three_broken_radial_localization, WorthMilestoneThreeHostileOutcomeClass,
    WorthMilestoneThreeHostileScenario, WorthReplayParityStatus, WorthTopologyDerivedRegion,
    WorthTopologyEditChangedScope, WorthTopologyEditFamily, WorthTopologyEditNamingOutcome,
    WorthTopologyEditRejectionClass,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use worth_schema::facade::topology_authoring::WorthMilestoneOnePrimitiveCase;

#[test]
fn milestone_three_broken_radial_localization_certifies_exact_radial_rejection_and_replay() {
    let report = certify_milestone_three_broken_radial_localization(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.broken_radial",
    )
    .expect("milestone three broken radial certification should succeed");

    assert_eq!(
        report.scenario,
        WorthMilestoneThreeHostileScenario::BrokenRadialLocalization
    );
    assert_eq!(report.primitive_family, "NmtEdgeFan(k)");
    assert_eq!(
        report.primitive,
        WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 }
    );
    assert_eq!(
        report.edit_families,
        vec![WorthTopologyEditFamily::SpliceRadialAdjacency]
    );
    assert_eq!(
        report.outcome_class,
        WorthMilestoneThreeHostileOutcomeClass::Rejected
    );
    assert_eq!(
        report.rejection_class,
        Some(WorthTopologyEditRejectionClass::InvariantBlocked)
    );
    assert_eq!(
        report.continuity_outcome_class,
        WorthTopologyEditNamingOutcome::Ambiguous
    );
    assert_eq!(
        report.continuity_rejection_class,
        Some(WorthTopologyEditRejectionClass::NamingContinuityAmbiguous)
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
        .rejected_edit_scope_report
        .expect("typed radial rejection should expose exact rejected scope report");
    assert_eq!(rejected.rows.len(), 1);
    assert_eq!(
        rejected.rows[0].family,
        WorthTopologyEditFamily::SpliceRadialAdjacency
    );
    assert!(rejected.rows[0]
        .changed_scopes
        .contains(&WorthTopologyEditChangedScope::RadialNeighborhood));
    assert!(rejected.rows[0]
        .derived_regions
        .contains(&WorthTopologyDerivedRegion::RadialNeighborhoodRegion));
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
        Some(true)
    );
}

#[test]
fn milestone_three_broken_radial_localization_report_is_deterministic_for_same_seeded_history() {
    let left = certify_milestone_three_broken_radial_localization(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.broken_radial.deterministic",
    )
    .expect("left broken radial certification should succeed");
    let right = certify_milestone_three_broken_radial_localization(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.broken_radial.deterministic",
    )
    .expect("right broken radial certification should succeed");

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(left.rejection_class, right.rejection_class);
    assert_eq!(left.topology_edit_digest, right.topology_edit_digest);
    assert_eq!(
        left.naming_edit_continuity_matrix,
        right.naming_edit_continuity_matrix
    );
    assert_eq!(left.broken_radial_witness, right.broken_radial_witness);
    assert_eq!(
        left.rejected_edit_scope_report,
        right.rejected_edit_scope_report
    );
    assert_eq!(
        left.edit_replay_parity_report,
        right.edit_replay_parity_report
    );
}
