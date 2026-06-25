use worth_ui::facade::WorthUiMountedControlNodeReceipt;

fn main() {
    let _forged = WorthUiMountedControlNodeReceipt {
        composition_child_binding: panic!("fixture only checks receipt field privacy"),
        state_binding: panic!("fixture only checks receipt field privacy"),
        host_frame: panic!("fixture only checks receipt field privacy"),
        consumed_facts: Vec::new(),
        receipt_digest: 1,
    };
}
