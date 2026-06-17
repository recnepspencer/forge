use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPointEvent, PlanarBooleanPointEventKind,
};

use super::point_event_deduplication_key::PlanarBooleanPointEventDeduplicationKey;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PlanarBooleanDeduplicatedPointEventSet {
    point_events: Vec<PlanarBooleanPointEvent>,
    duplicate_point_reports_suppressed: usize,
    high_valence_point_groups_detected: usize,
}

impl PlanarBooleanDeduplicatedPointEventSet {
    pub(crate) fn from_point_reports(point_reports: Vec<PlanarBooleanPointEvent>) -> Self {
        let mut grouped_events =
            BTreeMap::<PlanarBooleanPointEventDeduplicationKey, PlanarBooleanPointEvent>::new();
        let mut duplicate_point_reports_suppressed = 0;
        for point_report in point_reports {
            let key = PlanarBooleanPointEventDeduplicationKey::from_point_report(&point_report);
            if let Some(existing) = grouped_events.get_mut(&key) {
                existing.merge_duplicate_report(point_report);
                duplicate_point_reports_suppressed += 1;
            } else {
                grouped_events.insert(key, point_report);
            }
        }
        let mut point_events = grouped_events.into_values().collect::<Vec<_>>();
        for event in &mut point_events {
            if event.kind() == PlanarBooleanPointEventKind::SharedEndpoint {
                event.canonicalize_deduplicated_identity();
            }
        }
        point_events.sort_by(|left, right| left.event_identity().cmp(right.event_identity()));
        let high_valence_point_groups_detected = point_events
            .iter()
            .filter(|event| event.participating_carrier_identities().len() > 2)
            .count();
        Self {
            point_events,
            duplicate_point_reports_suppressed,
            high_valence_point_groups_detected,
        }
    }

    pub(crate) fn point_events(self) -> Vec<PlanarBooleanPointEvent> {
        self.point_events
    }

    pub(crate) fn duplicate_point_reports_suppressed(&self) -> usize {
        self.duplicate_point_reports_suppressed
    }

    pub(crate) fn high_valence_point_groups_detected(&self) -> usize {
        self.high_valence_point_groups_detected
    }
}
