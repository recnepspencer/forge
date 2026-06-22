use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet;

fn main() {
    let _ = PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
        schedule_set_identity: "forged".to_string(),
        normalized_schedule_set_identity: "normalized-set".to_string(),
        schedules: Vec::new(),
        counters: Default::default(),
    };
}
