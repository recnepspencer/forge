use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    TopologyEditDerivedFallbackPolicy, TopologyEditRejectionClass,
};

use super::certify_hostile_suite_report;

#[test]
fn hostile_suite_direct_acceptance_rows_are_proof_shaped() {
    let report = certify_hostile_suite_report("m3.hostile_suite.direct_acceptance");

    assert!(report.topology_edit_digest_rows.iter().all(|row| {
        row.topology_edit_digest.contract_count > 0
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.naming_edit_continuity_matrix_rows.iter().all(|row| {
        !row.naming_edit_continuity_matrix.rows.is_empty()
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.naming_continuity_breadth_rows.iter().all(|row| {
        row.continuity_row_count() > 0
            && row.naming_scope_count() > 0
            && row.replay_checked()
            && row.continuity_row_count()
                == row.preserved_count() + row.ambiguous_count() + row.rejected_count()
            && row
                .row_digest()
                .starts_with(&format!("scenario={};", row.scenario().as_str()))
    }));
    assert!(report.edit_replay_parity_rows.iter().all(|row| row
        .row_digest
        .starts_with(&format!("scenario={};", row.scenario.as_str()))));
    assert!(report.edit_branch_local_parity_rows.iter().all(|row| {
        row.mutation_origin == "branch_local_application"
            && row.topology_edit_digest.contract_count > 0
            && row
                .row_digest
                .starts_with(&format!("branch={};", row.branch_label))
    }));
    assert_eq!(
        report
            .edit_branch_local_parity_rows
            .iter()
            .filter(
                |row| row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                    && row.branch_head_unchanged_after_rejection
                    && row.branch_truth_digest.is_none()
                    && row.rejection_class == Some(TopologyEditRejectionClass::InvariantBlocked)
            )
            .count(),
        2
    );
    assert!(report.edit_branch_local_parity_rows.iter().any(|row| {
        row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
            && row.branch_head_diverged_from_main
            && row.branch_truth_digest.is_some()
    }));
}

#[test]
fn hostile_suite_derived_and_scale_rows_are_breadth_honest() {
    let report = certify_hostile_suite_report("m3.hostile_suite.breadth");

    assert!(report.edit_breadth_counter_rows.iter().all(|row| {
        row.contract_count > 0
            && row.replay_checked
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.edit_fallout_breadth_rows.iter().all(|row| {
        row.declared_derived_region_count > 0
            && row.fallback_policy == TopologyEditDerivedFallbackPolicy::AllowExplicitFallback
            && !row.fallback_policy_exceeded
            && row.fallback_rejection_class.is_none()
            && !row.locality_claim_mismatch
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.edit_fallout_breadth_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.fallout_class == crate::facade::MilestoneThreeEditFalloutClass::WholeViewFallback
            && row.derived_validation_row_count > 0
            && row.fallback_count == 1
            && row
                .row_digest
                .contains("fallback_policy=allow_explicit_fallback")
    }));
    assert!(report
        .derived_fallback_policy_denial_rows
        .iter()
        .any(|row| {
            row.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn
                && row.strict_fallback_policy
                    == TopologyEditDerivedFallbackPolicy::RejectAnyFallback
                && row.policy_exceeded
                && row.denied_rejection_class == TopologyEditRejectionClass::DerivedFallbackExceeded
        }));
    assert!(report.edit_fallout_breadth_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.fallout_class
                == crate::facade::MilestoneThreeEditFalloutClass::RejectedBeforeDerivedWork
            && row.derived_validation_row_count == 0
            && row.fallback_count == 0
    }));
    assert!(report.scale_pressure_rows.iter().all(|row| {
        row.replay_verified
            && row.topology_edit_digest.contract_count > 0
            && row.workload_size > 0
            && row.edit_step_count > 0
            && row.final_state_digest == row.replay_final_state_digest
            && row
                .row_digest
                .starts_with(&format!("scale_pressure={};", row.sweep_label()))
    }));
    assert!(report
        .scale_pressure_rows
        .iter()
        .any(|row| row.sweep_label() == "large_branch_local_histories" && row.branch_local));
    assert!(report.scale_pressure_rows.iter().any(|row| {
        row.sweep_label() == "radial_adjacency_splice"
            && row.edit_step_count > 1
            && row.derived_validation_row_count > 0
            && row
                .edit_families()
                .contains(&crate::topology_operators::TopologyEditFamily::SpliceRadialAdjacency)
    }));
    assert!(report.scale_pressure_rows.iter().any(|row| {
        row.sweep_label() == "wire_membership_detach"
            && row.edit_families().contains(
                &crate::topology_operators::TopologyEditFamily::DetachShellOrWireMembership,
            )
    }));
    assert!(report.scale_pressure_rows.iter().any(|row| {
        row.sweep_label() == "radial_adjacency_detach"
            && row
                .edit_families()
                .contains(&crate::topology_operators::TopologyEditFamily::DetachRadialAdjacency)
    }));
}
