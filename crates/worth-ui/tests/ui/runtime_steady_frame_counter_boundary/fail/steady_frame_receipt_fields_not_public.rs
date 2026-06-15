use worth_ui::facade::{
    WorthUiFrameExecutionReceipt, WorthUiMeasurementCounterPacket, WorthUiSteadyFrameCounters,
    WorthUiSteadyFrameDiagnosticPolicy,
};

fn main() {
    let _receipt = WorthUiFrameExecutionReceipt {
        active_plan_digest: 1,
        diagnostic_policy: WorthUiSteadyFrameDiagnosticPolicy::Minimal,
        counters: WorthUiSteadyFrameCounters::default(),
        lane_receipts: Vec::new(),
        aggregate_packet: packet(),
    };
}

fn packet() -> WorthUiMeasurementCounterPacket {
    panic!()
}
