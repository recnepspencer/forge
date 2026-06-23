use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventGroup, PlanarBooleanIntervalEvent, PlanarBooleanPointEvent,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOrderedEventSet {
    point_event_identities: Vec<String>,
    interval_event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    relation_diagnostic_identities: Vec<String>,
}

impl PlanarBooleanOrderedEventSet {
    pub(crate) fn from_events_and_groups(
        point_events: &[PlanarBooleanPointEvent],
        interval_events: &[PlanarBooleanIntervalEvent],
        event_groups: &[PlanarBooleanEventGroup],
        relation_diagnostic_identities: Vec<String>,
    ) -> Self {
        Self {
            point_event_identities: canonical_values(
                point_events
                    .iter()
                    .map(|event| event.event_identity().to_string()),
            ),
            interval_event_identities: canonical_values(
                interval_events
                    .iter()
                    .map(|event| event.event_identity().to_string()),
            ),
            event_group_identities: canonical_values(
                event_groups
                    .iter()
                    .map(|group| group.group_identity().to_string()),
            ),
            relation_diagnostic_identities: canonical_values(relation_diagnostic_identities),
        }
    }

    pub fn point_event_identities(&self) -> &[String] {
        &self.point_event_identities
    }

    pub fn interval_event_identities(&self) -> &[String] {
        &self.interval_event_identities
    }

    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }

    pub fn relation_diagnostic_identities(&self) -> &[String] {
        &self.relation_diagnostic_identities
    }
}

fn canonical_values(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}
