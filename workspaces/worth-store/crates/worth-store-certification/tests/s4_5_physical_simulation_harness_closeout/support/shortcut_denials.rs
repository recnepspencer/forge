use worth_proof::{Recipe, Unresolved};
use worth_store_physical_certification::{
    fixture_label_oracle_attempt, reject_copied_transcript_fields,
    reject_loose_log_evidence_attempt, reject_raw_json_scenario_authority_attempt,
    reject_same_run_self_comparison_evidence_attempt, reject_terminal_json_evidence_attempt,
    reject_unresolved_simulation_plan_recipe, shortcut_denial_from_evidence_bundle_denial,
    shortcut_denial_from_fault_delivery_denial, shortcut_denial_from_oracle_denial,
    shortcut_denial_from_plan_denial, shortcut_denial_from_scenario_denial,
    shortcut_denial_from_terminal_projection_denial, shortcut_denial_from_transcript_denial,
    test_support_oracle_verdict_attempt, FaultDeliveryAttempt,
    SyntheticHarnessShortcutDenialReceipt,
};

use super::super::coverage_support;

pub(super) fn complete_shortcut_denial_receipts() -> Vec<SyntheticHarnessShortcutDenialReceipt> {
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
                coverage_support::shortcut_plan(),
            ))
            .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(test_support_oracle_verdict_attempt().unwrap_err())
            .unwrap(),
    ]
}
