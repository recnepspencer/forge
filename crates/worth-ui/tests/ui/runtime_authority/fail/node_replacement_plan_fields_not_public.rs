use worth_ui::facade::{WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan};

fn main() {
    let _ = WorthUiNodeReplacementPlan {
        active_artifact_digest: 0,
        candidate_artifact_digest: 0,
        classifications: Vec::new(),
        counters: WorthUiNodeReplacementCounters::default(),
    };
}
