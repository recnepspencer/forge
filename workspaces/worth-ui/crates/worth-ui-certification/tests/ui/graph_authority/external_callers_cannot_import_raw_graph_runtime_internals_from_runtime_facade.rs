use worth_ui_runtime::facade::graph::{
    UiGraphCoreIndexes, UiGraphMountedReceiptAuthoritySeedStore, UiGraphMountedReceiptSlot,
    UiGraphNode, UiGraphNodeTopology, UiGraphSnapshot, UiGraphTopology,
};

fn main() {
    let _ = std::mem::MaybeUninit::<UiGraphSnapshot>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphNode>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphNodeTopology>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphTopology>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphCoreIndexes>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphMountedReceiptAuthoritySeedStore>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphMountedReceiptSlot>::uninit();
}
