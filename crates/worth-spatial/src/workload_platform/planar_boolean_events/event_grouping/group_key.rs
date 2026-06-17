use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEvent, PlanarBooleanPointEvent,
};

pub(crate) fn point_group_key(event: &PlanarBooleanPointEvent) -> String {
    format!(
        "point:{}",
        event.coordinate_fact().coordinate_fact_identity()
    )
}

pub(crate) fn interval_group_key(event: &PlanarBooleanIntervalEvent) -> String {
    format!(
        "interval:{}",
        event.normalized_interval().normalized_interval_identity()
    )
}
