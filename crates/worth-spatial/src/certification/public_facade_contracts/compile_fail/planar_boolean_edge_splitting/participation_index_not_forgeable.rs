use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitEventParticipationCounters, PlanarBooleanSplitEventParticipationIndex,
    PlanarBooleanSplitEventParticipationRow,
};

fn main() {
    let _ = PlanarBooleanSplitEventParticipationIndex {
        index_identity: String::from("forged"),
        event_ledger_identity: String::from("synthetic ledger"),
        recovered_carrier_set_identity: String::from("synthetic recovered carriers"),
        rows: Vec::<PlanarBooleanSplitEventParticipationRow>::new(),
        carrier_row_offsets: unavailable_offsets(),
        point_events_by_identity: unavailable_point_events(),
        interval_events_by_identity: unavailable_interval_events(),
        counters: PlanarBooleanSplitEventParticipationCounters::default(),
    };
}

fn unavailable_offsets() -> std::collections::BTreeMap<String, usize> {
    panic!("compile-fail fixture must never construct row offsets")
}

fn unavailable_point_events() -> std::collections::BTreeMap<
    String,
    worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEvent,
> {
    panic!("compile-fail fixture must never construct point events")
}

fn unavailable_interval_events() -> std::collections::BTreeMap<
    String,
    worth_spatial::facade::planar_boolean_events::PlanarBooleanIntervalEvent,
> {
    panic!("compile-fail fixture must never construct interval events")
}
