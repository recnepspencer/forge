use forge_proof::{Recipe, Unresolved};

use crate::pressure_harness::fixtures::{
    lower_io_pressure_plan, replay_bundle_with_shortcut_observation,
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
    IoPressureHarnessScenario, PhysicalSimulationProfile, SyntheticHarnessShortcutDenialReceipt,
    SyntheticHarnessShortcutRejectionReport,
};
use crate::{IoPressureHarnessEvidence, IoPressureHarnessEvidenceDenial};

#[test]
fn io_pressure_publication_rejects_forbidden_shortcuts_before_evidence_can_be_minted() {
    let report = SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(
        io_pressure_shortcut_receipts(),
    )
    .unwrap()
    .require_all_certification_shortcuts_denied()
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
fn io_pressure_evidence_publication_denies_shortcut_contaminated_replay_bundles() {
    for shortcut in [
        crate::ShortcutRejectionObservation::private_mutation_denied(),
        crate::ShortcutRejectionObservation::json_authority_denied(),
        crate::ShortcutRejectionObservation::same_run_self_comparison_denied(),
    ] {
        let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure();
        let replay = replay_bundle_with_shortcut_observation(
            scenario.clone(),
            PhysicalSimulationProfile::DeveloperSmoke,
            shortcut,
        );

        assert_eq!(
            IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap_err(),
            IoPressureHarnessEvidenceDenial::ForbiddenShortcutEvidencePresent
        );
    }
}

fn io_pressure_shortcut_receipts() -> Vec<SyntheticHarnessShortcutDenialReceipt> {
    let io_pressure_plan = lower_io_pressure_plan(
        IoPressureHarnessScenario::deterministic_read_under_repair_pressure(),
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
            reject_unresolved_simulation_plan_recipe(Recipe::<Unresolved, _>::new(
                io_pressure_plan,
            ))
            .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(test_support_oracle_verdict_attempt().unwrap_err())
            .unwrap(),
    ]
}
