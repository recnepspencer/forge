use worth_ui::facade::graph::{
    UiGraphCoreIndexes, UiGraphMountedReceiptAuthoritySeedStore, UiGraphNode,
    UiGraphNodeTopology, UiGraphSnapshot, UiGraphTopology,
};

fn main() {
    let _ = std::mem::MaybeUninit::<UiGraphSnapshot>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphNode>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphNodeTopology>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphTopology>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphCoreIndexes>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphMountedReceiptAuthoritySeedStore>::uninit();
}
