use worth_ui::facade::WorthUiActivationStagingCounters;

fn main() {
    let _counters = WorthUiActivationStagingCounters {
        verified_input_count: 6,
        digest_comparison_count: 12,
        staged_reconciliation_receipt_count: 7,
        staged_query_binding_count: 1,
        rejected_missing_input_count: 0,
        rejected_mismatched_input_count: 0,
        receipt_verification_count: 1,
        active_mutation_observed_count: 0,
    };
}
