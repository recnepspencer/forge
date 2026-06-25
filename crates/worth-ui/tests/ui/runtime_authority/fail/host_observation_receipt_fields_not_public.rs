use worth_ui::facade::{
    WorthUiAdmittedHostFrameObservationReceipt, WorthUiHostMeasurementReadinessPosture,
};

fn main() {
    let _receipt = WorthUiAdmittedHostFrameObservationReceipt {
        basis: panic!("basis is runtime-admitted"),
        readiness: WorthUiHostMeasurementReadinessPosture::Ready,
        available_bounds: Vec::new(),
        consumed_facts: Vec::new(),
        counters: panic!("counters are runtime-admitted"),
        receipt_digest: 1,
    };
}
