use worth_ui::facade::{
    WorthUiMountedProductViewCounters, WorthUiMountedProductViewReceipt,
    WorthUiMountedProductViewSemanticSlice,
};

fn main() {
    let _forged = WorthUiMountedProductViewReceipt {
        semantic_slice: WorthUiMountedProductViewSemanticSlice::LiveView,
        mounted_view: todo!(),
        consumed_facts: Vec::new(),
        graph_obligation_execution_digests: Vec::new(),
        counters: WorthUiMountedProductViewCounters::default(),
        receipt_digest: 1,
    };
}
