use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanNormalizedEdgeSplitSchedule;

fn main() {
    let _ = PlanarBooleanNormalizedEdgeSplitSchedule {
        schedule_identity: "schedule".to_string(),
        ordered_schedule_identity: "ordered".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        cuts: Vec::new(),
        retained_interval_entries: Vec::new(),
        retained_interval_entry_identities: Vec::new(),
    };
}
