use worth_ui::facade::{
    WorthUiAccessibilityHostInspectionCounters, WorthUiAccessibilityHostInspectionPosture,
    WorthUiAccessibilityHostInspectionReceipt,
};

fn main() {
    let _receipt = WorthUiAccessibilityHostInspectionReceipt {
        participation_digest: 0,
        posture: WorthUiAccessibilityHostInspectionPosture::UnsupportedHostApi,
        consumed_facts: Vec::new(),
        counters: WorthUiAccessibilityHostInspectionCounters::default(),
        receipt_digest: 0,
    };
}
