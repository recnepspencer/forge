use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEndpointBoundaryNormalizedSplitSchedule;

fn main() {
    let _ = PlanarBooleanEndpointBoundaryNormalizedSplitSchedule {
        schedule_identity: "forged".to_string(),
        normalized_schedule_identity: "normalized".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        fragment_cuts: Vec::new(),
        endpoint_contact_decisions: Vec::new(),
        retained_interval_entries: Vec::new(),
        retained_interval_entry_identities: Vec::new(),
    };
}
