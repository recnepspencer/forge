use worth_ui::facade::{WorthUiHeaderFrameRebindReceipt, WorthUiHeaderFrameRebindStatus};

fn main() {
    let _forged = WorthUiHeaderFrameRebindReceipt {
        status: WorthUiHeaderFrameRebindStatus::ReboundAfterActivation,
        previous_frame_digest: 1,
        rebound_frame_digest: 2,
        source_parse_count: 0,
        registry_lookup_count: 0,
        artifact_tree_scan_count: 0,
        projection_rebuild_count: 1,
    };
}
