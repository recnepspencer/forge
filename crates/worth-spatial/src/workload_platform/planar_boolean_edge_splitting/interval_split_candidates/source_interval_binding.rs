use crate::workload_platform::planar_boolean_edge_splitting::event_participation_index::PlanarBooleanSplitEventParticipationRow;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEvent, PlanarBooleanIntervalEventKind, PlanarBooleanNormalizedInterval,
    PlanarBooleanSourceInterval,
};

use super::denial::PlanarBooleanIntervalSplitCandidateDenial;

pub(crate) struct BoundIntervalSourceRange<'a> {
    interval_event_identity: &'a str,
    interval_event_kind: PlanarBooleanIntervalEventKind,
    source_edge_identity: &'a str,
    participation_row_identity: &'a str,
    normalized_interval: &'a PlanarBooleanNormalizedInterval,
    source_interval: &'a PlanarBooleanSourceInterval,
    local_frame_identity: &'a str,
    precision_basis_identity: &'a str,
    event_group_identities: &'a [String],
}

impl<'a> BoundIntervalSourceRange<'a> {
    pub(crate) fn bind(
        participation_row: &'a PlanarBooleanSplitEventParticipationRow,
        event: &'a PlanarBooleanIntervalEvent,
    ) -> Result<Self, PlanarBooleanIntervalSplitCandidateDenial> {
        let matching_source_interval =
            matching_source_interval_for_row_carrier(participation_row, event)?;
        Ok(Self {
            interval_event_identity: event.event_identity(),
            interval_event_kind: event.kind(),
            source_edge_identity: participation_row.source_edge_identity(),
            participation_row_identity: participation_row.participation_row_identity(),
            normalized_interval: event.normalized_interval(),
            source_interval: matching_source_interval,
            local_frame_identity: event.local_frame_identity(),
            precision_basis_identity: event.precision_basis_identity(),
            event_group_identities: participation_row.event_group_identities(),
        })
    }

    pub(crate) fn interval_event_identity(&self) -> &str {
        self.interval_event_identity
    }

    pub(crate) fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
    }

    pub(crate) fn source_edge_identity(&self) -> &str {
        self.source_edge_identity
    }

    pub(crate) fn participation_row_identity(&self) -> &str {
        self.participation_row_identity
    }

    pub(crate) fn normalized_interval(&self) -> &PlanarBooleanNormalizedInterval {
        self.normalized_interval
    }

    pub(crate) fn source_interval(&self) -> &PlanarBooleanSourceInterval {
        self.source_interval
    }

    pub(crate) fn local_frame_identity(&self) -> &str {
        self.local_frame_identity
    }

    pub(crate) fn precision_basis_identity(&self) -> &str {
        self.precision_basis_identity
    }

    pub(crate) fn event_group_identities(&self) -> &[String] {
        self.event_group_identities
    }
}

fn matching_source_interval_for_row_carrier<'a>(
    participation_row: &PlanarBooleanSplitEventParticipationRow,
    event: &'a PlanarBooleanIntervalEvent,
) -> Result<&'a PlanarBooleanSourceInterval, PlanarBooleanIntervalSplitCandidateDenial> {
    let row_carrier_identity = participation_row.carrier_identity();
    let mut matches = [
        matching_side_source_interval(
            row_carrier_identity,
            event.left_carrier_identity(),
            event.left_source_interval(),
        ),
        matching_side_source_interval(
            row_carrier_identity,
            event.right_carrier_identity(),
            event.right_source_interval(),
        ),
    ]
    .into_iter()
    .flatten();
    match (matches.next(), matches.next()) {
        (Some(source_interval), None) => Ok(source_interval),
        _ => Err(
            PlanarBooleanIntervalSplitCandidateDenial::missing_source_interval_for_row_carrier(
                event.event_identity(),
                "interval split candidate requires exactly one source interval for the participation row carrier",
            ),
        ),
    }
}

fn matching_side_source_interval<'a>(
    row_carrier_identity: &str,
    event_side_carrier_identity: &str,
    source_interval: &'a PlanarBooleanSourceInterval,
) -> Option<&'a PlanarBooleanSourceInterval> {
    if row_carrier_identity == event_side_carrier_identity
        && source_interval.carrier_identity() == event_side_carrier_identity
    {
        Some(source_interval)
    } else {
        None
    }
}
