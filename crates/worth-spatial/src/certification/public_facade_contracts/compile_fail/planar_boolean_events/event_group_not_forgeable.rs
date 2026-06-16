use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventGroup, PlanarBooleanEventGroupInput, PlanarBooleanEventGroupKind,
};

fn main() {
    let _ = PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity: String::from("forged"),
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
        canonical_group_key: String::from("forged"),
        point_event_identities: Vec::new(),
        interval_event_identities: Vec::new(),
        segment_pair_identities: Vec::new(),
        participating_carrier_identities: Vec::new(),
        source_endpoint_identities: Vec::new(),
        source_interval_identities: Vec::new(),
    });
}
