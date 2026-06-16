use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEvent;

use super::counters::PlanarBooleanEventGroupingCounters;
use super::group::{
    PlanarBooleanEventGroup, PlanarBooleanEventGroupInput, PlanarBooleanEventGroupKind,
};
use super::group_key::point_group_key;
use super::identity::{event_group_identity, EventGroupIdentityBasis};

pub(crate) fn group_point_events(
    point_events: &[PlanarBooleanPointEvent],
) -> (
    Vec<PlanarBooleanEventGroup>,
    PlanarBooleanEventGroupingCounters,
) {
    let mut counters = PlanarBooleanEventGroupingCounters::default();
    let mut buckets = BTreeMap::<String, Vec<&PlanarBooleanPointEvent>>::new();
    for event in point_events {
        counters.inspect_point_event();
        buckets
            .entry(point_group_key(event))
            .or_default()
            .push(event);
    }
    let mut groups = buckets
        .into_iter()
        .map(|(key, events)| point_group_from_bucket(key, events, &mut counters))
        .collect::<Vec<_>>();
    groups.sort_by(|left, right| left.group_identity().cmp(right.group_identity()));
    (groups, counters)
}

fn point_group_from_bucket(
    canonical_group_key: String,
    events: Vec<&PlanarBooleanPointEvent>,
    counters: &mut PlanarBooleanEventGroupingCounters,
) -> PlanarBooleanEventGroup {
    let point_event_identities = canonical_values(
        events
            .iter()
            .map(|event| event.event_identity().to_string()),
    );
    let segment_pair_identities = canonical_values(
        events
            .iter()
            .flat_map(|event| event.segment_pair_identities().iter().cloned()),
    );
    let participating_carrier_identities = canonical_values(
        events
            .iter()
            .flat_map(|event| event.participating_carrier_identities().iter().cloned()),
    );
    let source_endpoint_identities = canonical_values(
        events
            .iter()
            .flat_map(|event| event.source_endpoint_identities().iter().cloned()),
    );
    let interval_event_identities = Vec::new();
    let source_interval_identities = Vec::new();
    let group_identity = event_group_identity(EventGroupIdentityBasis {
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
        canonical_group_key: &canonical_group_key,
        point_event_identities: &point_event_identities,
        interval_event_identities: &interval_event_identities,
        segment_pair_identities: &segment_pair_identities,
        participating_carrier_identities: &participating_carrier_identities,
        source_endpoint_identities: &source_endpoint_identities,
        source_interval_identities: &source_interval_identities,
    });
    counters.emit_point_group(events.len().saturating_sub(1));
    counters.retain_group_provenance(
        participating_carrier_identities.len(),
        segment_pair_identities.len(),
    );
    PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity,
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
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
