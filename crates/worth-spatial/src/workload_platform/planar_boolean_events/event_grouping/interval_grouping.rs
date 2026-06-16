use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_events::PlanarBooleanIntervalEvent;

use super::counters::PlanarBooleanEventGroupingCounters;
use super::group::{
    PlanarBooleanEventGroup, PlanarBooleanEventGroupInput, PlanarBooleanEventGroupKind,
};
use super::group_key::interval_group_key;
use super::identity::{event_group_identity, EventGroupIdentityBasis};

pub(crate) fn group_interval_events(
    interval_events: &[PlanarBooleanIntervalEvent],
) -> (
    Vec<PlanarBooleanEventGroup>,
    PlanarBooleanEventGroupingCounters,
) {
    let mut counters = PlanarBooleanEventGroupingCounters::default();
    let mut buckets = BTreeMap::<String, Vec<&PlanarBooleanIntervalEvent>>::new();
    for event in interval_events {
        counters.inspect_interval_event();
        buckets
            .entry(interval_group_key(event))
            .or_default()
            .push(event);
    }
    let mut groups = buckets
        .into_iter()
        .map(|(key, events)| interval_group_from_bucket(key, events, &mut counters))
        .collect::<Vec<_>>();
    groups.sort_by(|left, right| left.group_identity().cmp(right.group_identity()));
    (groups, counters)
}

fn interval_group_from_bucket(
    canonical_group_key: String,
    events: Vec<&PlanarBooleanIntervalEvent>,
    counters: &mut PlanarBooleanEventGroupingCounters,
) -> PlanarBooleanEventGroup {
    let interval_event_identities = canonical_values(
        events
            .iter()
            .map(|event| event.event_identity().to_string()),
    );
    let segment_pair_identities = canonical_values(
        events
            .iter()
            .map(|event| event.segment_pair_identity().to_string()),
    );
    let participating_carrier_identities = canonical_values(events.iter().flat_map(|event| {
        [
            event.left_carrier_identity().to_string(),
            event.right_carrier_identity().to_string(),
        ]
    }));
    let source_interval_identities = canonical_values(events.iter().flat_map(|event| {
        [
            event
                .left_source_interval()
                .source_interval_identity()
                .to_string(),
            event
                .right_source_interval()
                .source_interval_identity()
                .to_string(),
        ]
    }));
    let point_event_identities = Vec::new();
    let source_endpoint_identities = Vec::new();
    let group_identity = event_group_identity(EventGroupIdentityBasis {
        kind: PlanarBooleanEventGroupKind::CoincidentInterval,
        canonical_group_key: &canonical_group_key,
        point_event_identities: &point_event_identities,
        interval_event_identities: &interval_event_identities,
        segment_pair_identities: &segment_pair_identities,
        participating_carrier_identities: &participating_carrier_identities,
        source_endpoint_identities: &source_endpoint_identities,
        source_interval_identities: &source_interval_identities,
    });
    counters.emit_interval_group(events.len().saturating_sub(1));
    counters.retain_group_provenance(
        participating_carrier_identities.len(),
        segment_pair_identities.len(),
    );
    PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity,
        kind: PlanarBooleanEventGroupKind::CoincidentInterval,
        canonical_group_key,
        point_event_identities,
        interval_event_identities,
        segment_pair_identities,
        participating_carrier_identities,
        source_endpoint_identities,
        source_interval_identities,
    })
}

fn canonical_values(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}
