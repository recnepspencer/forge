use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPointEvent, PlanarBooleanPointEventKind,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PlanarBooleanPointEventDeduplicationKey(String);

impl PlanarBooleanPointEventDeduplicationKey {
    pub(crate) fn from_point_report(point_report: &PlanarBooleanPointEvent) -> Self {
        if point_report.kind() == PlanarBooleanPointEventKind::SharedEndpoint {
            return Self(shared_endpoint_point_key(point_report));
        }
        Self(point_report.event_identity().to_string())
    }
}

fn shared_endpoint_point_key(point_report: &PlanarBooleanPointEvent) -> String {
    format!(
        "shared-endpoint:{}",
        point_report.coordinate_fact().coordinate_fact_identity()
    )
}
