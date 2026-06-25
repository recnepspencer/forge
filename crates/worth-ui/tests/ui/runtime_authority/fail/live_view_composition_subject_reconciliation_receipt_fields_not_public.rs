use worth_ui::facade::{
    WorthUiLiveViewCompositionSubjectReconciliationReceipt,
    WorthUiMountedGraphChildSelectionCounters,
};

fn main() {
    let counters = WorthUiMountedGraphChildSelectionCounters {
        graph_child_row_count: 0,
        control_payload_lookup_count: 0,
        interaction_payload_lookup_count: 0,
        projection_control_scan_count: 0,
        projection_interaction_scan_count: 0,
        mounted_subject_count: 0,
        declared_unmounted_count: 0,
        missing_payload_count: 0,
        duplicate_subject_count: 0,
    };
    let _receipt = WorthUiLiveViewCompositionSubjectReconciliationReceipt {
        rows: Vec::new(),
        counters,
        consumed_facts: Vec::new(),
        receipt_digest: 0,
    };
}
