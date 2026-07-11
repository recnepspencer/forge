use forge_store_live_query::{
    ContinuationRetentionStatus, CursorContinuationPlan, StableBasisId,
};

fn main() {
    let _ = CursorContinuationPlan::new(
        StableBasisId(41),
        4,
        ContinuationRetentionStatus::RetentionRebindRequired,
    );
}
