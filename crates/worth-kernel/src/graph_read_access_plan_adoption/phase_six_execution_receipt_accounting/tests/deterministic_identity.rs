use super::{production_phase_six_closeout, production_phase_six_seed};
use crate::graph_read_access_plan_adoption::current_worth_graph_read_access_execution_receipt_accounting_closeout;

#[test]
fn same_canonical_inputs_produce_same_receipt_identity() {
    let first = production_phase_six_closeout();
    let second = current_worth_graph_read_access_execution_receipt_accounting_closeout(
        &production_phase_six_seed(),
    )
    .expect("same seed should close deterministically");

    assert_eq!(
        first.receipt_accounting_report().report_digest(),
        second.receipt_accounting_report().report_digest()
    );
    assert_eq!(first.closeout_digest(), second.closeout_digest());
}
