use worth_proof::{Recipe, Unresolved};

use crate::s6_io_pressure_test_support::{
    lower_s6_pressure_plan, replay_bundle_with_shortcut_observation,
};
use crate::{
    fixture_label_oracle_attempt, reject_copied_transcript_fields,
    reject_loose_log_evidence_attempt, reject_raw_json_scenario_authority_attempt,
    reject_same_run_self_comparison_evidence_attempt, reject_terminal_json_evidence_attempt,
    reject_unresolved_simulation_plan_recipe, shortcut_denial_from_evidence_bundle_denial,
    shortcut_denial_from_fault_delivery_denial, shortcut_denial_from_oracle_denial,
    shortcut_denial_from_plan_denial, shortcut_denial_from_scenario_denial,
    shortcut_denial_from_terminal_projection_denial, shortcut_denial_from_transcript_denial,
    test_support_oracle_verdict_attempt, FaultDeliveryAttempt, ForbiddenShortcutKind,
    PhysicalSimulationProfile, S6IoPressureHarnessScenario, SyntheticHarnessShortcutDenialReceipt,
    SyntheticHarnessShortcutRejectionReport,
};
use crate::{S6IoPressureHarnessEvidence, S6IoPressureHarnessEvidenceDenial};

#[test]
fn s6_pressure_publication_rejects_forbidden_shortcuts_before_evidence_can_be_minted() {
    let report =
        SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(s6_shortcut_receipts())
            .unwrap()
            .require_all_roadmap2_shortcuts_denied()
            .unwrap();

    assert!(report.all_required_shortcuts_denied());
    assert!(report.all_required_boundaries_denied());
    assert_eq!(report.receipts().len(), 9);
    for required in [
        ForbiddenShortcutKind::PrivateMutation,
        ForbiddenShortcutKind::SameRunSelfComparison,
        ForbiddenShortcutKind::LogsAsProof,
        ForbiddenShortcutKind::JsonScenarioAuthority,
        ForbiddenShortcutKind::TestSupportVerdictAuthority,
    ] {
        assert!(report
            .receipts()
            .iter()
            .any(|receipt| receipt.shortcut() == required));
    }
}

#[test]
fn s6_evidence_publication_denies_shortcut_contaminated_replay_bundles() {
    for shortcut in [
        crate::ShortcutRejectionObservation::private_mutation_denied(),
        crate::ShortcutRejectionObservation::json_authority_denied(),
        crate::ShortcutRejectionObservation::same_run_self_comparison_denied(),
    ] {
        let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure();
        let replay = replay_bundle_with_shortcut_observation(
            scenario.clone(),
            PhysicalSimulationProfile::DeveloperSmoke,
            shortcut,
        );

        assert_eq!(
            S6IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap_err(),
            S6IoPressureHarnessEvidenceDenial::ForbiddenShortcutEvidencePresent
        );
    }
}

fn s6_shortcut_receipts() -> Vec<SyntheticHarnessShortcutDenialReceipt> {
    let s6_plan = lower_s6_pressure_plan(
        S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure(),
        PhysicalSimulationProfile::DeveloperSmoke,
    );
    vec![
        shortcut_denial_from_evidence_bundle_denial(
            reject_loose_log_evidence_attempt().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_scenario_denial(
            reject_raw_json_scenario_authority_attempt(r#"{"scenario":"fake"}"#).unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_terminal_projection_denial(
            reject_terminal_json_evidence_attempt().unwrap_err(),
        ),
        shortcut_denial_from_evidence_bundle_denial(
            reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_fault_delivery_denial(
            FaultDeliveryAttempt::private_mutation()
                .admit()
                .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(fixture_label_oracle_attempt().unwrap_err()).unwrap(),
        shortcut_denial_from_transcript_denial(reject_copied_transcript_fields().unwrap_err())
            .unwrap(),
        shortcut_denial_from_plan_denial(
            reject_unresolved_simulation_plan_recipe(Recipe::<Unresolved, _>::new(s6_plan))
                .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(test_support_oracle_verdict_attempt().unwrap_err())
            .unwrap(),
    ]
}
