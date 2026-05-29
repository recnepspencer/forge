use crate::facade::{
    certify_milestone_three_bowtie_adjacent_rewire, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyDerivedRegion,
    TopologyEditChangedScope, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn milestone_three_bowtie_adjacent_rewire_certifies_typed_rejection_with_exact_scope_evidence() {
    let report = certify_milestone_three_bowtie_adjacent_rewire(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.bowtie",
    )
    .expect("milestone three hostile certification should succeed");

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
        report.edit_families,
        vec![TopologyEditFamily::SpliceRadialAdjacency]
    );
    let witness = report
        .bowtie_adjacent_witness
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
        report.continuity_outcome_class,
        TopologyEditNamingOutcome::Ambiguous
    );
    assert_eq!(
        report.continuity_rejection_class,
        Some(TopologyEditRejectionClass::NamingContinuityAmbiguous)
    );
    assert_eq!(
        report.rejection_class,
        Some(TopologyEditRejectionClass::InvariantBlocked)
    );
    assert_eq!(report.topology_edit_digest.contract_count, 1);
    assert_eq!(report.naming_edit_continuity_matrix.rows.len(), 1);
    assert_eq!(report.naming_edit_continuity_matrix.ambiguous_count, 1);
    assert!(report.edit_replay_parity_report.replay_checked);
    assert_eq!(
        report.edit_replay_parity_report.parity_status,
        ReplayParityStatus::Match
    );
    assert_eq!(report.edit_replay_parity_report.step_rows.len(), 1);
    assert_eq!(report.edit_replay_parity_report.replay_step_rows.len(), 1);
    let rejected = report
        .rejected_edit_scope_report
        .expect("typed rejection should expose rejected edit scope report");
    assert_eq!(rejected.rows.len(), 1);
    assert_eq!(
        rejected.rows[0].family,
        TopologyEditFamily::SpliceRadialAdjacency
    );
    assert!(rejected.rows[0]
        .changed_scopes
        .contains(&TopologyEditChangedScope::RadialNeighborhood));
    assert!(rejected.rows[0]
        .derived_regions
        .contains(&TopologyDerivedRegion::RadialNeighborhoodRegion));
}

#[test]
fn milestone_three_bowtie_adjacent_rewire_report_is_deterministic_for_same_seeded_history() {
    let left = certify_milestone_three_bowtie_adjacent_rewire(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.bowtie.deterministic",
    )
    .expect("left hostile certification should succeed");
    let right = certify_milestone_three_bowtie_adjacent_rewire(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.bowtie.deterministic",
    )
    .expect("right hostile certification should succeed");

    assert_eq!(left.outcome_class, right.outcome_class);
    assert_eq!(left.rejection_class, right.rejection_class);
    assert_eq!(left.topology_edit_digest, right.topology_edit_digest);
    assert_eq!(left.bowtie_adjacent_witness, right.bowtie_adjacent_witness);
    assert_eq!(
        left.naming_edit_continuity_matrix,
        right.naming_edit_continuity_matrix
    );
    assert_eq!(
        left.rejected_edit_scope_report,
        right.rejected_edit_scope_report
    );
    assert_eq!(
        left.edit_replay_parity_report,
        right.edit_replay_parity_report
    );
}
