use worth_ui::facade::WorthUiPageHostFrameReceipt;

fn main() {
    let _forged = WorthUiPageHostFrameReceipt {
        page_name: "HeaderProofPage".to_owned(),
        slots: forged_slots(),
        frame_digest: 17,
    };
}

fn forged_slots() -> Vec<worth_ui::facade::WorthUiPageHostSlotReceipt> {
    panic!("compile-fail fixture should never execute");
}
