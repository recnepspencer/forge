use worth_ui::facade::{
    WorthUiFrameExecutionReceipt, WorthUiSteadyFrameFoundationalBridge,
};

fn wants_foundational_lowering(receipt: &WorthUiFrameExecutionReceipt) {
    let _ = WorthUiSteadyFrameFoundationalBridge::lower_counter_receipts(receipt);
}

fn main() {}
