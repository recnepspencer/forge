use forge_store_live_query::{ContinuationRetentionStatus, StableBasisId, StableBasisReadPlan};

fn main() {
    let _ = StableBasisReadPlan::new(
        StableBasisId(41),
        6,
        ContinuationRetentionStatus::Retained,
    );
}
