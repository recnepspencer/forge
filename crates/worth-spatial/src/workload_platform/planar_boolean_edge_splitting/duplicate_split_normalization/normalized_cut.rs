use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntryKind;

use super::counters::PlanarBooleanNormalizedEdgeSplitScheduleCounters;
use super::retained_interval_entry::PlanarBooleanRetainedIntervalSplitEntry;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedSplitCut {
    cut_identity: String,
    duplicate_report_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    parameter: f64,
    parameter_bits: u64,
    kind: PlanarBooleanRawEdgeSplitScheduleEntryKind,
    local_frame_identity: String,
    precision_basis_identity: String,
    provenance_entry_identities: Vec<String>,
    event_identities: Vec<String>,
    parameter_fact_identities: Vec<String>,
    event_group_identities: Vec<String>,
    segment_pair_identities: Vec<String>,
    predicate_receipt_identities: Vec<String>,
    exact_endpoint_source_identity: Option<String>,
    exact_projected_endpoint_fact_identity: Option<String>,
    shared_endpoint_source_identities: Vec<String>,
    shared_endpoint_projection_fact_digests: Vec<String>,
}

impl PlanarBooleanNormalizedSplitCut {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        cut_identity: String,
        duplicate_report_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        parameter: f64,
        parameter_bits: u64,
        kind: PlanarBooleanRawEdgeSplitScheduleEntryKind,
        local_frame_identity: String,
        precision_basis_identity: String,
        provenance_entry_identities: Vec<String>,
        event_identities: Vec<String>,
        parameter_fact_identities: Vec<String>,
        event_group_identities: Vec<String>,
        segment_pair_identities: Vec<String>,
        predicate_receipt_identities: Vec<String>,
        endpoint_authority: PlanarBooleanNormalizedEndpointAuthority,
    ) -> Self {
        Self {
            cut_identity,
            duplicate_report_identity,
            source_edge_identity,
            carrier_identity,
            parameter,
            parameter_bits,
            kind,
            local_frame_identity,
            precision_basis_identity,
            provenance_entry_identities,
            event_identities,
            parameter_fact_identities,
            event_group_identities,
            segment_pair_identities,
            predicate_receipt_identities,
            exact_endpoint_source_identity: endpoint_authority.exact_endpoint_source_identity,
            exact_projected_endpoint_fact_identity: endpoint_authority
                .exact_projected_endpoint_fact_identity,
            shared_endpoint_source_identities: endpoint_authority.shared_endpoint_source_identities,
            shared_endpoint_projection_fact_digests: endpoint_authority
                .shared_endpoint_projection_fact_digests,
        }
    }

    pub fn cut_identity(&self) -> &str {
        &self.cut_identity
    }
    pub fn duplicate_report_identity(&self) -> &str {
        &self.duplicate_report_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn parameter(&self) -> f64 {
        self.parameter
    }
    pub fn parameter_bits(&self) -> u64 {
        self.parameter_bits
    }
    pub fn kind(&self) -> PlanarBooleanRawEdgeSplitScheduleEntryKind {
        self.kind
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn provenance_entry_identities(&self) -> &[String] {
        &self.provenance_entry_identities
    }
    pub fn event_identities(&self) -> &[String] {
        &self.event_identities
    }
    pub fn parameter_fact_identities(&self) -> &[String] {
        &self.parameter_fact_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn segment_pair_identities(&self) -> &[String] {
        &self.segment_pair_identities
    }
    pub fn predicate_receipt_identities(&self) -> &[String] {
        &self.predicate_receipt_identities
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
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PlanarBooleanNormalizedEndpointAuthority {
    pub(crate) exact_endpoint_source_identity: Option<String>,
    pub(crate) exact_projected_endpoint_fact_identity: Option<String>,
    pub(crate) shared_endpoint_source_identities: Vec<String>,
    pub(crate) shared_endpoint_projection_fact_digests: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedEdgeSplitSchedule {
    schedule_identity: String,
    ordered_schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    cuts: Vec<PlanarBooleanNormalizedSplitCut>,
    retained_interval_entries: Vec<PlanarBooleanRetainedIntervalSplitEntry>,
    retained_interval_entry_identities: Vec<String>,
}

impl PlanarBooleanNormalizedEdgeSplitSchedule {
    pub(crate) fn new(
        schedule_identity: String,
        ordered_schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        cuts: Vec<PlanarBooleanNormalizedSplitCut>,
        retained_interval_entries: Vec<PlanarBooleanRetainedIntervalSplitEntry>,
    ) -> Self {
        let retained_interval_entry_identities = retained_interval_entries
            .iter()
            .map(|entry| entry.entry_identity().to_string())
            .collect();
        Self {
            schedule_identity,
            ordered_schedule_identity,
            source_edge_identity,
            carrier_identity,
            cuts,
            retained_interval_entries,
            retained_interval_entry_identities,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn ordered_schedule_identity(&self) -> &str {
        &self.ordered_schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn cuts(&self) -> &[PlanarBooleanNormalizedSplitCut] {
        &self.cuts
    }
    pub fn retained_interval_entries(&self) -> &[PlanarBooleanRetainedIntervalSplitEntry] {
        &self.retained_interval_entries
    }
    pub fn retained_interval_entry_identities(&self) -> &[String] {
        &self.retained_interval_entry_identities
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedEdgeSplitScheduleSet {
    schedule_set_identity: String,
    ordered_schedule_set_identity: String,
    schedules: Vec<PlanarBooleanNormalizedEdgeSplitSchedule>,
    counters: PlanarBooleanNormalizedEdgeSplitScheduleCounters,
}

impl PlanarBooleanNormalizedEdgeSplitScheduleSet {
    pub(crate) fn new(
        schedule_set_identity: String,
        ordered_schedule_set_identity: String,
        schedules: Vec<PlanarBooleanNormalizedEdgeSplitSchedule>,
        counters: PlanarBooleanNormalizedEdgeSplitScheduleCounters,
    ) -> Self {
        Self {
            schedule_set_identity,
            ordered_schedule_set_identity,
            schedules,
            counters,
        }
    }

    pub fn schedule_set_identity(&self) -> &str {
        &self.schedule_set_identity
    }
    pub fn ordered_schedule_set_identity(&self) -> &str {
        &self.ordered_schedule_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanNormalizedEdgeSplitSchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanNormalizedEdgeSplitScheduleCounters {
        self.counters
    }
}
