use worth_ui::facade::WorthUiDurableStateReconciliationCounters;

fn main() {
    let _ = WorthUiDurableStateReconciliationCounters {
        reconciled_family_count: 1,
        reconciled_node_count: 1,
        receipt_count: 1,
        carry_forward_count: 1,
        replacement_count: 0,
        drop_count: 0,
        recreate_count: 0,
        orphan_removal_count: 0,
        incompatible_shape_count: 0,
        query_posture_required_count: 0,
        rejected_reconciliation_count: 0,
    };
}
