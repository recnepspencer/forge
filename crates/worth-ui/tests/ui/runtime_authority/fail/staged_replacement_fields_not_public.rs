use worth_ui::facade::WorthUiStagedReplacement;

fn main() {
    let _staged = WorthUiStagedReplacement {
        frame_epoch: missing(),
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        admitted_candidate: missing(),
        impact: missing(),
        narrowing: missing(),
        node_plan: missing(),
        reconciliation_plan: missing(),
        query_rebind_plan: missing(),
        pending_execution_plan_lowering_input: missing(),
    };
}

fn missing<T>() -> T {
    loop {}
}
