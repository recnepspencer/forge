use crate::workload_platform::planar_boolean_edge_splitting::{
    point_split_posture::PlanarBooleanPointSplitPosture,
    raw_edge_split_schedule::counters::PlanarBooleanRawEdgeSplitScheduleCounters,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanRawEdgeSplitScheduleEntryKind {
    Point(PlanarBooleanPointSplitPosture),
    Interval,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanRawEdgeSplitScheduleEntry {
    entry_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    candidate_identity: String,
    event_identity: String,
    parameter_fact_identity: Option<String>,
    parameter: f64,
    parameter_range: Option<[f64; 2]>,
    local_frame_identity: String,
    precision_basis_identity: String,
    kind: PlanarBooleanRawEdgeSplitScheduleEntryKind,
    segment_pair_identities: Vec<String>,
    predicate_receipt_identities: Vec<String>,
    event_group_identities: Vec<String>,
    exact_endpoint_source_identity: Option<String>,
    exact_projected_endpoint_fact_identity: Option<String>,
    shared_endpoint_source_identities: Vec<String>,
    shared_endpoint_projection_fact_digests: Vec<String>,
    interval_authority: Option<PlanarBooleanRawIntervalAuthority>,
}

impl PlanarBooleanRawEdgeSplitScheduleEntry {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        entry_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        candidate_identity: String,
        event_identity: String,
        parameter_fact_identity: Option<String>,
        parameter: f64,
        parameter_range: Option<[f64; 2]>,
        local_frame_identity: String,
        precision_basis_identity: String,
        kind: PlanarBooleanRawEdgeSplitScheduleEntryKind,
        segment_pair_identities: Vec<String>,
        predicate_receipt_identities: Vec<String>,
        event_group_identities: Vec<String>,
        endpoint_authority: PlanarBooleanRawPointEndpointAuthority,
        interval_authority: Option<PlanarBooleanRawIntervalAuthority>,
    ) -> Self {
        Self {
            entry_identity,
            source_edge_identity,
            carrier_identity,
            candidate_identity,
            event_identity,
            parameter_fact_identity,
            parameter,
            parameter_range,
            local_frame_identity,
            precision_basis_identity,
            kind,
            segment_pair_identities,
            predicate_receipt_identities,
            event_group_identities: canonical_values(event_group_identities),
            exact_endpoint_source_identity: endpoint_authority.exact_endpoint_source_identity,
            exact_projected_endpoint_fact_identity: endpoint_authority
                .exact_projected_endpoint_fact_identity,
            shared_endpoint_source_identities: canonical_values(
                endpoint_authority.shared_endpoint_source_identities,
            ),
            shared_endpoint_projection_fact_digests: canonical_values(
                endpoint_authority.shared_endpoint_projection_fact_digests,
            ),
            interval_authority,
        }
    }

    pub fn entry_identity(&self) -> &str {
        &self.entry_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }
    pub fn event_identity(&self) -> &str {
        &self.event_identity
    }
    pub fn parameter_fact_identity(&self) -> Option<&str> {
        self.parameter_fact_identity.as_deref()
    }
    pub fn parameter(&self) -> f64 {
        self.parameter
    }
    pub fn parameter_range(&self) -> Option<[f64; 2]> {
        self.parameter_range
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn kind(&self) -> PlanarBooleanRawEdgeSplitScheduleEntryKind {
        self.kind
    }
    pub fn segment_pair_identities(&self) -> &[String] {
        &self.segment_pair_identities
    }
    pub fn predicate_receipt_identities(&self) -> &[String] {
        &self.predicate_receipt_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn exact_endpoint_source_identity(&self) -> Option<&str> {
        self.exact_endpoint_source_identity.as_deref()
    }
    pub fn exact_projected_endpoint_fact_identity(&self) -> Option<&str> {
        self.exact_projected_endpoint_fact_identity.as_deref()
    }
    pub fn shared_endpoint_source_identities(&self) -> &[String] {
        &self.shared_endpoint_source_identities
    }
    pub fn shared_endpoint_projection_fact_digests(&self) -> &[String] {
        &self.shared_endpoint_projection_fact_digests
    }
    pub fn interval_authority(&self) -> Option<&PlanarBooleanRawIntervalAuthority> {
        self.interval_authority.as_ref()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PlanarBooleanRawPointEndpointAuthority {
    pub(crate) exact_endpoint_source_identity: Option<String>,
    pub(crate) exact_projected_endpoint_fact_identity: Option<String>,
    pub(crate) shared_endpoint_source_identities: Vec<String>,
    pub(crate) shared_endpoint_projection_fact_digests: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanRawIntervalAuthority {
    interval_event_kind: PlanarBooleanIntervalEventKind,
    source_interval_identity: String,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    normalized_interval_identity: String,
    normalized_parameter_range: [f64; 2],
    participation_row_identity: String,
}

impl PlanarBooleanRawIntervalAuthority {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        interval_event_kind: PlanarBooleanIntervalEventKind,
        source_interval_identity: String,
        source_parameter_range: [f64; 2],
        source_sense: PlanarBooleanSourceIntervalSense,
        normalized_interval_identity: String,
        normalized_parameter_range: [f64; 2],
        participation_row_identity: String,
    ) -> Self {
        Self {
            interval_event_kind,
            source_interval_identity,
            source_parameter_range,
            source_sense,
            normalized_interval_identity,
            normalized_parameter_range,
            participation_row_identity,
        }
    }

    pub fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
    }
    pub fn source_interval_identity(&self) -> &str {
        &self.source_interval_identity
    }
    pub fn source_parameter_range(&self) -> [f64; 2] {
        self.source_parameter_range
    }
    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }
    pub fn normalized_interval_identity(&self) -> &str {
        &self.normalized_interval_identity
    }
    pub fn normalized_parameter_range(&self) -> [f64; 2] {
        self.normalized_parameter_range
    }
    pub fn participation_row_identity(&self) -> &str {
        &self.participation_row_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanRawEdgeSplitSchedule {
    schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
}

fn canonical_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

impl PlanarBooleanRawEdgeSplitSchedule {
    pub(crate) fn new(
        schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
    ) -> Self {
        Self {
            schedule_identity,
            source_edge_identity,
            carrier_identity,
            entries,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn entries(&self) -> &[PlanarBooleanRawEdgeSplitScheduleEntry] {
        &self.entries
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanRawEdgeSplitScheduleSet {
    schedule_set_identity: String,
    point_posture_set_identity: String,
    interval_candidate_set_identity: String,
    schedules: Vec<PlanarBooleanRawEdgeSplitSchedule>,
    counters: PlanarBooleanRawEdgeSplitScheduleCounters,
}

impl PlanarBooleanRawEdgeSplitScheduleSet {
    pub(crate) fn new(
        schedule_set_identity: String,
        point_posture_set_identity: String,
        interval_candidate_set_identity: String,
        schedules: Vec<PlanarBooleanRawEdgeSplitSchedule>,
        counters: PlanarBooleanRawEdgeSplitScheduleCounters,
    ) -> Self {
        Self {
            schedule_set_identity,
            point_posture_set_identity,
            interval_candidate_set_identity,
            schedules,
            counters,
        }
    }

    pub fn schedule_set_identity(&self) -> &str {
        &self.schedule_set_identity
    }
    pub fn point_posture_set_identity(&self) -> &str {
        &self.point_posture_set_identity
    }
    pub fn interval_candidate_set_identity(&self) -> &str {
        &self.interval_candidate_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanRawEdgeSplitSchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanRawEdgeSplitScheduleCounters {
        self.counters
    }
}
