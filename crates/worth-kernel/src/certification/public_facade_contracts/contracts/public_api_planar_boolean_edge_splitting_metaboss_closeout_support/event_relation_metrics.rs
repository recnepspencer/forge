use std::collections::HashSet;

use super::super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanPointEventKind,
};

#[derive(Clone, Copy, Default)]
pub(crate) struct PointKindCounts {
    pub(crate) proper_crossings: usize,
    pub(crate) a_endpoint_on_b_interior: usize,
    pub(crate) b_endpoint_on_a_interior: usize,
    pub(crate) shared_endpoints: usize,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct IntervalKindCounts {
    pub(crate) partial_overlaps: usize,
    pub(crate) containment_overlaps: usize,
    pub(crate) identical_same_direction: usize,
    pub(crate) identical_anti_parallel: usize,
}

pub(crate) fn event_bearing_pair_count(subject: &MetabossEventExtractionSubject) -> usize {
    let mut identities = HashSet::new();
    identities.extend(
        subject
            .ledger()
            .point_events()
            .iter()
            .map(|event| event.segment_pair_identity()),
    );
    identities.extend(
        subject
            .ledger()
            .interval_events()
            .iter()
            .map(|event| event.segment_pair_identity()),
    );
    identities.len()
}

pub(crate) fn point_kind_counts(subject: &MetabossEventExtractionSubject) -> PointKindCounts {
    let mut counts = PointKindCounts::default();
    for event in subject.ledger().point_events() {
        match event.kind() {
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing => {
                counts.proper_crossings += 1;
            }
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior => {
                counts.a_endpoint_on_b_interior += 1;
            }
            PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior => {
                counts.b_endpoint_on_a_interior += 1;
            }
            PlanarBooleanPointEventKind::SharedEndpoint => counts.shared_endpoints += 1,
        }
    }
    counts
}

pub(crate) fn interval_kind_counts(subject: &MetabossEventExtractionSubject) -> IntervalKindCounts {
    let mut counts = IntervalKindCounts::default();
    for event in subject.ledger().interval_events() {
        match event.kind() {
            PlanarBooleanIntervalEventKind::PartialOverlap => counts.partial_overlaps += 1,
            PlanarBooleanIntervalEventKind::ContainmentOverlap => counts.containment_overlaps += 1,
            PlanarBooleanIntervalEventKind::IdenticalSameDirection => {
                counts.identical_same_direction += 1;
            }
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel => {
                counts.identical_anti_parallel += 1;
            }
        }
    }
    counts
}
