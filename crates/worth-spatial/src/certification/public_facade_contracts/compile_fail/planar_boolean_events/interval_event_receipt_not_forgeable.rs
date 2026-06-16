use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEventExtractionCounters, PlanarBooleanIntervalEventExtractionReceipt,
};

fn main() {
    let _ = PlanarBooleanIntervalEventExtractionReceipt {
        collinear_relation_receipt_identity: String::from("forged"),
        interval_events: Vec::new(),
        counters: PlanarBooleanIntervalEventExtractionCounters::default(),
        extraction_identity: String::from("forged"),
    };
}
