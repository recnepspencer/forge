use worth_ui::facade::{
    WorthUiCompositionNodeReceipt, WorthUiMountedCompositionChildReceipt,
    WorthUiMountedNodeReceipt,
};

fn main() {
    let _forged = WorthUiMountedCompositionChildReceipt {
        parent_id: String::new(),
        order: 0,
        composition_node: forged_composition_node(),
        mounted_node: forged_mounted_node(),
        receipt_digest: 0,
    };
}

fn forged_composition_node() -> WorthUiCompositionNodeReceipt {
    panic!("compile-fail fixture never runs")
}

fn forged_mounted_node() -> WorthUiMountedNodeReceipt {
    panic!("compile-fail fixture never runs")
}
