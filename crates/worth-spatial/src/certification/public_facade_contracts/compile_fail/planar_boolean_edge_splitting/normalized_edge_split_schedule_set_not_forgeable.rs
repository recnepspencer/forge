use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanNormalizedEdgeSplitScheduleCounters, PlanarBooleanNormalizedEdgeSplitScheduleSet,
};

fn main() {
    let _ = PlanarBooleanNormalizedEdgeSplitScheduleSet {
        schedule_set_identity: "schedule-set".to_string(),
        ordered_schedule_set_identity: "ordered-set".to_string(),
        schedules: Vec::new(),
        counters: PlanarBooleanNormalizedEdgeSplitScheduleCounters::default(),
    };
}
