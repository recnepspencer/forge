use crate::facade::{
    certify_milestone_three_split_collapse_churn, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
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
        report.mutation_families(),
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::RetireTopologyEntity,
        ]
    );
    assert_eq!(
        report.outcome_class,
        MilestoneThreeHostileOutcomeClass::Accepted
    );
    assert!(report.rejection_class.is_none());
    assert!(report.rejected_mutation_scope_report.is_none());
    assert!(report.bowtie_adjacent_witness.is_none());
    assert!(report.ambiguous_local_rewire_witness.is_none());
    assert!(report.broken_radial_witness.is_none());
    assert_eq!(
        report.derived_fallback_policy(),
        Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
    );
    assert!(report
        .fallback_explanation_detail()
        .is_some_and(|detail| detail.contains("fallback")));
    let witness = report
        .split_collapse_churn_witness
        .as_ref()
        .expect("split-collapse churn should expose its owner churn witness");
    assert_eq!(witness.moved_half_edge_identities.len(), 2);
    assert_eq!(witness.retained_half_edge_identities.len(), 2);
    assert_eq!(witness.split_step_wire_count, 2);
    assert_eq!(witness.final_wire_count, 2);
    assert_ne!(witness.original_wire_identity, witness.split_wire_identity);
    assert_ne!(witness.split_wire_identity, witness.collapse_wire_identity);
    assert_eq!(report.topology_mutation_digest().mutation_record_count, 7);
    assert_eq!(report.naming_mutation_continuity_matrix().rows.len(), 7);
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
    assert_eq!(report.mutation_replay_parity_report.step_rows.len(), 2);
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
        2
    );
    assert_eq!(
        report.mutation_replay_parity_report.returned_to_baseline,
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
    assert_eq!(
        left.split_collapse_churn_witness,
        right.split_collapse_churn_witness
    );
}
