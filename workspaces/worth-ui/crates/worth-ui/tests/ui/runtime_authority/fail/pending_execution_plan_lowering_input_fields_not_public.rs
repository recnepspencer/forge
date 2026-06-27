use worth_ui::facade::WorthUiPendingExecutionPlanLoweringInput;

fn main() {
    let _input = WorthUiPendingExecutionPlanLoweringInput {
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        node_classification_count: 3,
        reconciliation_receipt_count: 4,
        query_rebind_entry_count: 5,
    };
}
