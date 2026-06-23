use worth_ui::facade::WorthUiHeaderFrameReceipt;

fn main() {
    let _forged = WorthUiHeaderFrameReceipt {
        groups: forged_groups(),
        projected_command_count: 2,
        source_parse_count: 0,
        registry_lookup_count: 0,
        artifact_tree_scan_count: 0,
    };
}

fn forged_groups() -> Vec<worth_ui::facade::WorthUiHeaderMenuGroup> {
    panic!("compile-fail fixture should never execute");
}
