use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventLedgerCounters, PlanarBooleanEventLedgerReceipt,
    PlanarBooleanOrderedEventSet,
};

fn main() {
    let _ = PlanarBooleanEventLedgerReceipt {
        reduced_pair_identity: String::from("forged"),
        event_extraction_request_identity: String::from("forged"),
        segment_carrier_set_identity: String::from("forged"),
        segment_pair_enumeration_identity: String::from("forged"),
        predicate_binding_identity: String::from("forged"),
        point_event_extraction_identity: String::from("forged"),
        collinear_relation_receipt_identity: String::from("forged"),
        interval_event_extraction_identity: String::from("forged"),
        point_events: Vec::new(),
        interval_events: Vec::new(),
        relation_diagnostics: Vec::new(),
        event_groups: Vec::new(),
        ordered_events: forged_ordered_events(),
        counters: PlanarBooleanEventLedgerCounters::default(),
        event_ledger_identity: String::from("forged"),
        downstream_consumption_identity: String::from("forged"),
    };
}

fn forged_ordered_events() -> PlanarBooleanOrderedEventSet {
    panic!("compile-fail fixture must not construct ordered events")
}
