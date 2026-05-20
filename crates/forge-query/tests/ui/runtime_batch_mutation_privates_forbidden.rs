use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBatchWriteReceipt, ForgeQueryMutationBatchBuilder,
};

fn main() {
    let _ = ForgeQueryMutationBatchBuilder {
        commands: Vec::new(),
        error: None,
    };

    let _ = ForgeQueryBatchWriteReceipt {
        write_receipts: Vec::new(),
        authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        batch_digest: String::new(),
        touched_aspect_paths: Vec::new(),
        affected_live_view_ids: Vec::new(),
        affected_derived_view_ids: Vec::new(),
        considered_computed_view_count: 0,
        considered_effect_count: 0,
        delivered_effect_count: 0,
        pending_write_intent_count: 0,
        suppressed_effect_count: 0,
        meaningful_effect_suppression_count: 0,
        effect_expression_failure_count: 0,
        refresh_fallback: false,
    };
}
