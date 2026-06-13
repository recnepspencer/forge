use worth_ui::facade::{
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationPlan,
};

fn main() {
    let _ = WorthUiDurableStateReconciliationPlan {
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        receipts: Vec::new(),
        counters: WorthUiDurableStateReconciliationCounters::default(),
    };
}
