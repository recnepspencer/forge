use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryRuntimeDeliveryBatch, QueryPatchGroupKind};

fn main() {
    let _ = ForgeQueryRuntimeDeliveryBatch {
        view_name: String::new(),
        authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        delivery_batch_digest: String::new(),
        delivery_window_digest: String::new(),
        consumer_attachment_digest: String::new(),
        sequence: 0,
        patch_group_kind: QueryPatchGroupKind::DetailFieldPatchGroup,
        patch_group_digest: String::new(),
        patch_group_width: 0,
    };
}
