use crate::graph_read_access_plan_adoption::current_worth_graph_read_access_execution_receipt_accounting_closeout;
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessReceiptStatus;

use super::{production_phase_six_closeout, production_phase_six_seed};

#[test]
fn receipt_identity_preserves_query_owned_plan_and_receipt_boundaries() {
    let closeout = production_phase_six_closeout();
    let first_row = closeout
        .receipt_accounting_report()
        .rows()
        .iter()
        .find(|row| row.status() == WorthGraphReadAccessReceiptStatus::ExecutedThroughQueryReceipt)
        .expect("Phase 4 vertical slice receipt row should exist");

    assert!(first_row.receipt_identity().plan_digest().is_some());
    assert!(first_row.receipt_identity().receipt_digest().is_some());
    assert!(first_row
        .receipt_identity()
        .execution_counter_digest()
        .is_some());
    assert_ne!(
        first_row.receipt_identity().touched_authority_digest(),
        "none"
    );
    assert_eq!(None, first_row.receipt_identity().policy_narrowing_digest());
    assert_ne!(first_row.receipt_identity().identity_digest(), "none");
}

#[test]
fn receipt_identity_changes_when_plan_or_touch_changes() {
    let seed = production_phase_six_seed();
    let base = current_worth_graph_read_access_execution_receipt_accounting_closeout(&seed)
        .expect("base seed should close");
    let base_identity = base.receipt_accounting_report().rows()[0]
        .receipt_identity()
        .identity_digest()
        .to_string();

    let plan_seed = seed.with_phase_four_receipt_projection_for_tests(
        seed.phase_four_receipt_projection()
            .with_adversarial_plan_digest_for_tests("adversarial-plan-digest"),
    );
    let plan_identity =
        current_worth_graph_read_access_execution_receipt_accounting_closeout(&plan_seed)
            .expect("plan-mutated seed should still close")
            .receipt_accounting_report()
            .rows()[0]
            .receipt_identity()
            .identity_digest()
            .to_string();

    let touch_seed = seed.with_phase_four_receipt_projection_for_tests(
        seed.phase_four_receipt_projection()
            .with_adversarial_touched_authority_for_tests("adversarial-touched-authority"),
    );
    let touch_identity =
        current_worth_graph_read_access_execution_receipt_accounting_closeout(&touch_seed)
            .expect("touch-mutated seed should still close")
            .receipt_accounting_report()
            .rows()[0]
            .receipt_identity()
            .identity_digest()
            .to_string();

    assert_ne!(base_identity, plan_identity);
    assert_ne!(base_identity, touch_identity);
    assert_ne!(plan_identity, touch_identity);
}
