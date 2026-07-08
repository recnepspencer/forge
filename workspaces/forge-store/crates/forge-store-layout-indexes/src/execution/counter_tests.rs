use crate::{
    access_lowering, S8AccessLoweringOutcome, S8CostEnvelopeViolationOutcome,
};
use forge_store_budgets::S8PreExecutionPlanBinding;

#[test]
fn observed_counters_above_admitted_envelope_produce_typed_violation() {
    let ready = super::tests_support::ready_exact_point_plan(33);
    let observed = super::tests_support::observed_from_snapshot(
        &ready,
        ready.selected().planned_counter_envelope().lookup(),
    );
    let oversized = super::tests_support::observed_from_snapshot(
        &ready,
        observed
            .snapshot()
            .with_page_touches(9)
            .with_read_amplification(9)
            .with_bytes_read(9 * 4_096),
    );

    let executed = match access_lowering().execute_ready(ready, oversized) {
        S8AccessLoweringOutcome::Executed(executed) => executed,
        other => panic!("expected executed outcome, got {other:?}"),
    };

    let receipt = executed.planned_vs_observed();
    assert!(!receipt.parity_holds());
    assert!(matches!(
        receipt.violation_outcome(),
        Some(S8CostEnvelopeViolationOutcome::ObservedExceededPlanned { .. })
    ));
    assert!(receipt.observed().page_touches() > receipt.planned().page_touches());
}

#[test]
fn hidden_broad_scan_fails_exact_counter_contract() {
    let ready = super::tests_support::ready_exact_point_plan(35);
    let observed = super::tests_support::observed_from_snapshot(
        &ready,
        ready.selected().planned_counter_envelope().lookup(),
    );
    let broad_scan = super::tests_support::observed_from_snapshot(
        &ready,
        observed
            .snapshot()
            .with_page_touches(32)
            .with_range_steps(32)
            .with_read_amplification(32)
            .with_bytes_read(32 * 4_096),
    );

    let executed = match access_lowering().execute_ready(ready, broad_scan) {
        S8AccessLoweringOutcome::Executed(executed) => executed,
        other => panic!("expected executed outcome, got {other:?}"),
    };

    assert!(matches!(
        executed.planned_vs_observed().violation_outcome(),
        Some(S8CostEnvelopeViolationOutcome::ObservedExceededPlanned { .. })
    ));
    assert!(
        executed.planned_vs_observed().observed().page_touches()
            > executed.planned_vs_observed().planned().page_touches()
    );
}

#[test]
fn observed_counters_with_mismatched_ready_basis_are_denied() {
    let first_ready = super::tests_support::ready_exact_point_plan(37);
    let second_ready = super::tests_support::ready_exact_prefix_plan(37);
    let observed = super::tests_support::observed_from_snapshot(
        &second_ready,
        super::tests_support::observed_from_snapshot(
            &first_ready,
            first_ready.selected().planned_counter_envelope().lookup(),
        )
        .snapshot(),
    );

    let denial = access_lowering().execute_ready(first_ready, observed);
    assert!(matches!(
        denial,
        S8AccessLoweringOutcome::Denied(crate::S8AccessLoweringDenied::ObservedCounterBasisMismatch { .. })
    ));
    if let S8AccessLoweringOutcome::Denied(denial) = denial {
        assert!(matches!(
            denial.spent_cost_receipt(),
            crate::S8AccessAttemptCostReceipt::DeniedObservedExecutionCost { .. }
        ));
    }
}

#[test]
fn witness_admission_rejects_wrong_access_path_kind_with_observed_cost_receipt() {
    let ready = super::tests_support::ready_exact_point_plan(39);
    let wrong_ready = super::tests_support::ready_exact_range_plan(39);
    let witness = super::counter_witness::TestExecutedCounterWitness::new(
        ready.selected().budget_receipt().plan_binding(),
        wrong_ready.path_kind(),
        wrong_ready.selected().planned_counter_envelope().lookup(),
    );
    let denial = access_lowering()
        .admit_executed_counters(&ready, &witness)
        .expect_err("range witness should not satisfy point-read ready basis");

    assert!(matches!(
        denial,
        crate::S8AccessLoweringDenied::ExecutedCounterWitnessPathMismatch { .. }
    ));
    assert!(matches!(
        denial.spent_cost_receipt(),
        crate::S8AccessAttemptCostReceipt::DeniedObservedExecutionCost { .. }
    ));
}

#[test]
fn witness_admission_rejects_wrong_plan_binding_even_when_shape_matches() {
    let ready = super::tests_support::ready_exact_point_plan(41);
    let expected_binding = ready.selected().budget_receipt().plan_binding();
    let wrong_binding = S8PreExecutionPlanBinding::new(
        expected_binding.identity_word().wrapping_add(1),
        expected_binding.lookup_word(),
        expected_binding.publication_word(),
        expected_binding.recovery_word(),
        expected_binding.budget_rows(),
    );
    let witness = super::counter_witness::TestExecutedCounterWitness::new(
        wrong_binding,
        ready.path_kind(),
        ready.selected().planned_counter_envelope().lookup(),
    );
    let denial = access_lowering()
        .admit_executed_counters(&ready, &witness)
        .expect_err("witness from another admitted plan binding should fail closed");

    assert!(matches!(
        denial,
        crate::S8AccessLoweringDenied::ExecutedCounterWitnessPlanBindingMismatch { .. }
    ));
    assert!(matches!(
        denial.spent_cost_receipt(),
        crate::S8AccessAttemptCostReceipt::DeniedObservedExecutionCost { .. }
    ));
}
