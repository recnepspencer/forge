use forge_query::facade::PreviewSubscriptionDiscardCloseout;

fn main() {
    let _ = PreviewSubscriptionDiscardCloseout {
        active_lane_digest: todo!(),
        attachment_digest: todo!(),
        preview_epoch_digest: "epoch".to_string(),
        residue_report_digest: "residue".to_string(),
        counters: todo!(),
        closeout_digest: "closeout".to_string(),
    };
}
