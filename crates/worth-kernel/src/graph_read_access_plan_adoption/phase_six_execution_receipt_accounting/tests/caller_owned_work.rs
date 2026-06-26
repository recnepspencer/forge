use crate::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_execution_receipt_accounting_closeout,
    WorthGraphReadAccessExecutionReceiptAccountingErrorKind,
};

use super::{production_phase_six_closeout, production_phase_six_seed};

#[test]
fn no_caller_owned_graph_work_counter_stays_zero() {
    let closeout = production_phase_six_closeout();

    assert_eq!(
        closeout
            .counter_accounting_report()
            .caller_owned_graph_work_count(),
        0
    );
    assert_eq!(
        closeout
            .batch_accounting_report()
            .caller_owned_graph_work_count(),
        0
    );
}

#[test]
fn caller_owned_graph_work_fails_closed_before_batch_closeout() {
    let seed = production_phase_six_seed();
    let adversarial_receipt = seed
        .phase_four_receipt_projection()
        .with_adversarial_caller_owned_work_for_tests();
    let adversarial_seed = seed.with_phase_four_receipt_projection_for_tests(adversarial_receipt);

    let err =
        current_worth_graph_read_access_execution_receipt_accounting_closeout(&adversarial_seed)
            .expect_err("caller-owned graph work must fail closed");

    assert_eq!(
        err.kind(),
        WorthGraphReadAccessExecutionReceiptAccountingErrorKind::CallerOwnedGraphWorkDetected
    );
}
