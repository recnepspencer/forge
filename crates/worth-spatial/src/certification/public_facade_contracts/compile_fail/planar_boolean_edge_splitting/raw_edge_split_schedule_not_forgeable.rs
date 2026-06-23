use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanRawEdgeSplitSchedule;

fn main() {
    let _ = PlanarBooleanRawEdgeSplitSchedule {
        schedule_identity: "forged".to_string(),
        source_edge_identity: "source".to_string(),
        entries: Vec::new(),
    };
}
