use worth_query::facade::{WorthQueryAuthorityLane, WorthQueryRuntimeDeliveryBatch, QueryPatchGroupKind};

fn main() {
    let _ = WorthQueryRuntimeDeliveryBatch {
        view_name: String::new(),
        authority_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        delivery_batch_digest: String::new(),
        delivery_window_digest: String::new(),
        consumer_attachment_digest: String::new(),
        sequence: 0,
        patch_group_kind: QueryPatchGroupKind::DetailFieldPatchGroup,
        patch_group_digest: String::new(),
        patch_group_width: 0,
    };
}
