use super::phase_chain_fixture::{production_phase_eight_closeout, production_phase_eight_seed};

#[test]
fn closeout_counters_match_source_phase_eight_reports() {
    let seed = production_phase_eight_seed();
    let closeout = production_phase_eight_closeout();
    let counters = closeout.counters();

    assert_eq!(
        counters.executed_receipt_count(),
        seed.receipt_accounting_report().executed_receipt_count()
    );
    assert_eq!(
        counters.receipt_row_count(),
        seed.receipt_accounting_report().rows().len()
    );
    assert_eq!(
        counters.admitted_plan_count(),
        closeout.receipts().executed_receipt_count()
            + closeout.receipts().admitted_plan_requires_receipt_count()
    );
    assert_eq!(
        counters.required_future_receipt_count(),
        seed.receipt_accounting_report()
            .required_future_receipt_count()
    );
    assert_eq!(
        counters.admitted_plan_requires_receipt_count(),
        closeout.receipts().admitted_plan_requires_receipt_count()
    );
    assert_eq!(
        counters.required_posture_count(),
        closeout.receipts().required_posture_count()
    );
    assert_eq!(
        counters.denied_posture_count(),
        closeout.receipts().denied_posture_count()
    );
    assert_eq!(
        counters.carried_gap_count(),
        closeout.receipts().carried_gap_count()
    );
    assert_eq!(
        counters.visible_non_executed_posture_count(),
        closeout.receipts().visible_non_executed_posture_count()
    );
    assert_eq!(
        counters.no_receipt_posture_count(),
        seed.receipt_accounting_report().no_receipt_posture_count()
    );
    assert_eq!(
        counters.accounted_counter_row_count(),
        seed.counter_accounting_report()
            .accounted_counter_row_count()
    );
    assert_eq!(
        counters.explicit_counter_gap_count(),
        seed.counter_accounting_report()
            .explicit_counter_gap_count()
    );
    assert_eq!(
        counters.no_execution_counter_required_count(),
        seed.counter_accounting_report()
            .no_execution_counter_required_count()
    );
    assert_eq!(
        counters.batch_row_count(),
        seed.batch_accounting_report().rows().len()
    );
    assert_eq!(counters.caller_owned_graph_work_count(), 0);
}

#[test]
fn closeout_counters_carry_deletion_residue_and_firewall_proof_counts() {
    let seed = production_phase_eight_seed();
    let closeout = production_phase_eight_closeout();
    let counters = closeout.counters();

    assert_eq!(
        counters.deleted_path_count(),
        seed.deletion_proof_report().deleted_count()
    );
    assert_eq!(
        counters.capped_residue_count(),
        seed.capped_residue_report().residue_count()
    );
    assert_eq!(
        counters.uncapped_residue_count(),
        seed.capped_residue_report().uncapped_residue_count()
    );
    assert_eq!(
        counters.source_firewall_region_count(),
        seed.source_firewall_report().scanned_region_count()
    );
    assert_eq!(
        counters.source_firewall_source_count(),
        seed.source_firewall_report().scanned_source_count()
    );
    assert_eq!(
        counters.source_firewall_violation_count(),
        seed.source_firewall_report().violation_count()
    );
}
