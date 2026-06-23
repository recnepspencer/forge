use worth_ui::facade::WorthUiComponentReloadReceipt;

fn main() {
    let _forged = WorthUiComponentReloadReceipt {
        component_ids: Vec::new(),
        compatibility: forged_compatibility(),
    };
}

fn forged_compatibility() -> worth_ui::facade::WorthUiComponentCompatibility {
    panic!("compile-fail fixture should never execute");
}
