use crate::facade::{
    certify_milestone_three_split_collapse_churn, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyEditFamily,
    TopologyEditNamingOutcome, TopologyEditRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn milestone_three_split_collapse_churn_certifies_topology_operator_owner_churn() {
    let report = certify_milestone_three_split_collapse_churn(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.split_collapse_churn",
    )
    .expect("milestone three split-collapse churn certification should succeed");

    assert_eq!(
        report.scenario,
        MilestoneThreeHostileScenario::SplitCollapseChurn
    );
    assert_eq!(report.primitive_family, "WireOpen(n)");
    assert_eq!(
        report.primitive,
        MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 }
    );
    assert_eq!(
        report.edit_families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::RetireTopologyEntity,
        ]
    );
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Accepted
    );
    assert!(report.rejection_class.is_none());
    assert!(report.rejected_edit_scope_report.is_none());
    assert!(report.bowtie_adjacent_witness.is_none());
    assert!(report.ambiguous_local_rewire_witness.is_none());
    assert!(report.broken_radial_witness.is_none());
    let witness = report
        .split_collapse_churn_witness
        .expect("split-collapse churn should expose its owner churn witness");
    assert_eq!(witness.moved_half_edge_identities.len(), 2);
    assert_eq!(witness.retained_half_edge_identities.len(), 2);
    assert_eq!(witness.split_step_wire_count, 2);
    assert_eq!(witness.final_wire_count, 2);
    assert_ne!(witness.original_wire_identity, witness.split_wire_identity);
    assert_ne!(witness.split_wire_identity, witness.collapse_wire_identity);
    assert_eq!(report.topology_edit_digest.contract_count, 7);
    assert_eq!(report.naming_edit_continuity_matrix.rows.len(), 7);
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
    assert_eq!(report.edit_replay_parity_report.step_rows.len(), 2);
    assert_eq!(report.edit_replay_parity_report.replay_step_rows.len(), 2);
    assert_eq!(
        report.edit_replay_parity_report.returned_to_baseline,
        Some(false)
    );
}

#[test]
fn milestone_three_split_collapse_churn_report_is_deterministic_for_same_seeded_history() {
    let left = certify_milestone_three_split_collapse_churn(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.split_collapse_churn.deterministic",
    )
    .expect("left split-collapse churn certification should succeed");
    let right = certify_milestone_three_split_collapse_churn(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.split_collapse_churn.deterministic",
    )
    .expect("right split-collapse churn certification should succeed");

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
    assert_eq!(
        left.split_collapse_churn_witness,
        right.split_collapse_churn_witness
    );
}




