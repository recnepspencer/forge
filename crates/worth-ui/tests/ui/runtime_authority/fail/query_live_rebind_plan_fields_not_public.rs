use worth_ui::facade::{
    WorthUiQueryLiveRebindCounters, WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindPlan,
};

fn forged_query_live_rebind_entries() -> Vec<WorthUiQueryLiveRebindEntry> {
    panic!("fixture should not run")
}

fn forged_query_live_rebind_counters() -> WorthUiQueryLiveRebindCounters {
    panic!("fixture should not run")
}

fn main() {
    let _ = WorthUiQueryLiveRebindPlan {
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        entries: forged_query_live_rebind_entries(),
        counters: forged_query_live_rebind_counters(),
    };
}
