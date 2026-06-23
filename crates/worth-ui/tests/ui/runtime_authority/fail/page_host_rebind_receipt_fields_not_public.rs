use worth_ui::facade::{WorthUiPageHostRebindReceipt, WorthUiPageHostRebindStatus};

fn main() {
    let _forged = WorthUiPageHostRebindReceipt {
        status: WorthUiPageHostRebindStatus::EquivalentAfterActivation,
        previous_frame_digest: 11,
        rebound_frame_digest: 17,
        projection_batch: forged_projection_batch(),
        projection_rebuild_count: 1,
    };
}

fn forged_projection_batch() -> worth_ui::facade::WorthUiProjectionRebindBatchReceipt {
    panic!("compile-fail fixture should never execute");
}
