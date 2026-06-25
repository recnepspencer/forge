use worth_ui::facade::{
    WorthUiCompositionRootReceipt, WorthUiMountedCompositionTraversalCounters,
    WorthUiMountedCompositionTreeReceipt,
};

fn main() {
    let _forged = WorthUiMountedCompositionTreeReceipt {
        root: forged_root(),
        children_by_parent: std::collections::BTreeMap::new(),
        counters: WorthUiMountedCompositionTraversalCounters {
            mounted_node_index_entry_count: 0,
            child_index_entry_count: 0,
            flat_node_scan_count: 0,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        },
        receipt_digest: 0,
    };
}

fn forged_root() -> WorthUiCompositionRootReceipt {
    panic!("compile-fail fixture never runs")
}
