use topology::derived_invalidation_migrated_products::{
    TraversalViewsReadStageReceipt, TraversalViewsSourceRow,
};

fn main() {
    let _ = TraversalViewsReadStageReceipt {
        selected_plan_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        traversal_views_selected_row_digest: String::new(),
        native_query_read_receipt_digest: String::new(),
        selected_legality_receipt_digest: String::new(),
        read_source_digest: String::new(),
        touched_closure_traversal_bound: 0,
        selected_traversal_count: 0,
        available_traversal_count: 0,
        selected_rows: fake_selected_rows(),
        receipt_digest: String::new(),
    };
}

fn fake_selected_rows() -> Vec<TraversalViewsSourceRow> {
    panic!("compile-fail fixture does not execute")
}
