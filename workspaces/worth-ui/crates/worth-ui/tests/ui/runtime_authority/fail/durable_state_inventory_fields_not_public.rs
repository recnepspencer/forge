use worth_ui::facade::{WorthUiDurableStateInventory, WorthUiDurableStateInventoryCounters};

fn main() {
    let _ = WorthUiDurableStateInventory {
        active_artifact_digest: 0,
        candidate_artifact_digest: 0,
        families: Vec::new(),
        transient_policies: Vec::new(),
        counters: WorthUiDurableStateInventoryCounters::default(),
    };
}
