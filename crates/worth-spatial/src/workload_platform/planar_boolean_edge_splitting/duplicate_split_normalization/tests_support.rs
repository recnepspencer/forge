use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleCounters,
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawEdgeSplitScheduleSet, PlanarBooleanRawIntervalAuthority,
    PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanPointSplitPosture;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

pub(crate) fn raw_set_from_schedules(
    schedules: Vec<PlanarBooleanRawEdgeSplitSchedule>,
) -> PlanarBooleanRawEdgeSplitScheduleSet {
    let entry_count = schedules
        .iter()
        .map(|schedule| schedule.entries().len())
        .sum();
    let schedule_count = schedules.len();
    PlanarBooleanRawEdgeSplitScheduleSet::new(
        "raw schedule set".to_string(),
        "point posture set".to_string(),
        "interval set".to_string(),
        schedules,
        PlanarBooleanRawEdgeSplitScheduleCounters::new(
            schedule_count,
            entry_count,
            0,
            0,
            0,
            0,
            entry_count,
        ),
    )
}

pub(crate) fn raw_schedule(
    schedule_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> PlanarBooleanRawEdgeSplitSchedule {
    PlanarBooleanRawEdgeSplitSchedule::new(
        schedule_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        entries,
    )
}

pub(crate) fn raw_point_entry(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identity: &str,
    parameter: f64,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    raw_point_entry_with_posture(
        entry_identity,
        source_edge_identity,
        carrier_identity,
        event_identity,
        parameter,
        PlanarBooleanPointSplitPosture::InteriorSplit,
    )
}

pub(crate) fn raw_point_entry_with_posture(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identity: &str,
    parameter: f64,
    posture: PlanarBooleanPointSplitPosture,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    raw_entry(
        entry_identity,
        source_edge_identity,
        carrier_identity,
        event_identity,
        parameter,
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture),
        "local frame",
        "precision basis",
    )
}

pub(crate) fn raw_point_entry_with_frame_precision(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identity: &str,
    parameter: f64,
    local_frame_identity: &str,
    precision_basis_identity: &str,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    raw_entry(
        entry_identity,
        source_edge_identity,
        carrier_identity,
        event_identity,
        parameter,
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::InteriorSplit,
        ),
        local_frame_identity,
        precision_basis_identity,
    )
}

pub(crate) fn raw_interval_entry(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identity: &str,
    parameter: f64,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    raw_entry(
        entry_identity,
        source_edge_identity,
        carrier_identity,
        event_identity,
        parameter,
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval,
        "local frame",
        "precision basis",
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn raw_entry(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identity: &str,
    parameter: f64,
    kind: PlanarBooleanRawEdgeSplitScheduleEntryKind,
    local_frame_identity: &str,
    precision_basis_identity: &str,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    let parameter_range = match kind {
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_) => None,
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => Some([parameter, parameter + 0.25]),
    };
    let interval_authority = match kind {
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_) => None,
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => {
            Some(PlanarBooleanRawIntervalAuthority::new(
                PlanarBooleanIntervalEventKind::PartialOverlap,
                format!("source-interval:{entry_identity}"),
                [parameter, parameter + 0.25],
                PlanarBooleanSourceIntervalSense::Forward,
                format!("normalized-interval:{entry_identity}"),
                [parameter, parameter + 0.25],
                format!("participation-row:{entry_identity}"),
            ))
        }
    };
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        format!("candidate:{entry_identity}"),
        event_identity.to_string(),
        match kind {
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_) => {
                Some(format!("parameter-fact:{entry_identity}"))
            }
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => None,
        },
        parameter,
        parameter_range,
        local_frame_identity.to_string(),
        precision_basis_identity.to_string(),
        kind,
        vec![format!("segment-pair:{event_identity}")],
        vec![format!("predicate:{event_identity}")],
        vec![format!("event-group:{event_identity}")],
        PlanarBooleanRawPointEndpointAuthority::default(),
        interval_authority,
    )
}
