use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::PlanarBooleanNormalizedSplitCut;
use crate::workload_platform::planar_boolean_edge_splitting::endpoint_boundary_normalization::PlanarBooleanEndpointContactDecision;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::action::PlanarBooleanMicroIntervalAction;
use super::counters::PlanarBooleanIntervalSubdivisionNormalizationCounters;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedIntervalSubdivisionRow {
    subdivision_identity: String,
    interval_event_identity: String,
    candidate_identities: Vec<String>,
    source_edge_identity: String,
    carrier_identity: String,
    admitted_parameter_range: [f64; 2],
    source_interval_identity: String,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    normalized_interval_identity: String,
    normalized_parameter_range: [f64; 2],
    interval_event_kind: PlanarBooleanIntervalEventKind,
    local_frame_identity: String,
    precision_basis_identity: String,
    action: PlanarBooleanMicroIntervalAction,
    provenance_entry_identities: Vec<String>,
    event_group_identities: Vec<String>,
}

impl PlanarBooleanNormalizedIntervalSubdivisionRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        subdivision_identity: String,
        interval_event_identity: String,
        candidate_identities: Vec<String>,
        source_edge_identity: String,
        carrier_identity: String,
        admitted_parameter_range: [f64; 2],
        source_interval_identity: String,
        source_parameter_range: [f64; 2],
        source_sense: PlanarBooleanSourceIntervalSense,
        normalized_interval_identity: String,
        normalized_parameter_range: [f64; 2],
        interval_event_kind: PlanarBooleanIntervalEventKind,
        local_frame_identity: String,
        precision_basis_identity: String,
        action: PlanarBooleanMicroIntervalAction,
        provenance_entry_identities: Vec<String>,
        event_group_identities: Vec<String>,
    ) -> Self {
        Self {
            subdivision_identity,
            interval_event_identity,
            candidate_identities,
            source_edge_identity,
            carrier_identity,
            admitted_parameter_range,
            source_interval_identity,
            source_parameter_range,
            source_sense,
            normalized_interval_identity,
            normalized_parameter_range,
            interval_event_kind,
            local_frame_identity,
            precision_basis_identity,
            action,
            provenance_entry_identities,
            event_group_identities,
        }
    }

    pub fn subdivision_identity(&self) -> &str {
        &self.subdivision_identity
    }
    pub fn interval_event_identity(&self) -> &str {
        &self.interval_event_identity
    }
    pub fn candidate_identities(&self) -> &[String] {
        &self.candidate_identities
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn admitted_parameter_range(&self) -> [f64; 2] {
        self.admitted_parameter_range
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
    pub fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn action(&self) -> PlanarBooleanMicroIntervalAction {
        self.action
    }
    pub fn provenance_entry_identities(&self) -> &[String] {
        &self.provenance_entry_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalSubdivisionNormalizedSchedule {
    schedule_identity: String,
    endpoint_boundary_schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    fragment_cuts: Vec<PlanarBooleanNormalizedSplitCut>,
    endpoint_contact_decisions: Vec<PlanarBooleanEndpointContactDecision>,
    interval_subdivisions: Vec<PlanarBooleanNormalizedIntervalSubdivisionRow>,
}

impl PlanarBooleanIntervalSubdivisionNormalizedSchedule {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        schedule_identity: String,
        endpoint_boundary_schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        fragment_cuts: Vec<PlanarBooleanNormalizedSplitCut>,
        endpoint_contact_decisions: Vec<PlanarBooleanEndpointContactDecision>,
        interval_subdivisions: Vec<PlanarBooleanNormalizedIntervalSubdivisionRow>,
    ) -> Self {
        Self {
            schedule_identity,
            endpoint_boundary_schedule_identity,
            source_edge_identity,
            carrier_identity,
            fragment_cuts,
            endpoint_contact_decisions,
            interval_subdivisions,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn endpoint_boundary_schedule_identity(&self) -> &str {
        &self.endpoint_boundary_schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn fragment_cuts(&self) -> &[PlanarBooleanNormalizedSplitCut] {
        &self.fragment_cuts
    }
    pub fn endpoint_contact_decisions(&self) -> &[PlanarBooleanEndpointContactDecision] {
        &self.endpoint_contact_decisions
    }
    pub fn interval_subdivisions(&self) -> &[PlanarBooleanNormalizedIntervalSubdivisionRow] {
        &self.interval_subdivisions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    schedule_set_identity: String,
    endpoint_boundary_schedule_set_identity: String,
    schedules: Vec<PlanarBooleanIntervalSubdivisionNormalizedSchedule>,
    counters: PlanarBooleanIntervalSubdivisionNormalizationCounters,
}

impl PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    pub(crate) fn new(
        schedule_set_identity: String,
        endpoint_boundary_schedule_set_identity: String,
        schedules: Vec<PlanarBooleanIntervalSubdivisionNormalizedSchedule>,
        counters: PlanarBooleanIntervalSubdivisionNormalizationCounters,
    ) -> Self {
        Self {
            schedule_set_identity,
            endpoint_boundary_schedule_set_identity,
            schedules,
            counters,
        }
    }

    pub fn schedule_set_identity(&self) -> &str {
        &self.schedule_set_identity
    }
    pub fn endpoint_boundary_schedule_set_identity(&self) -> &str {
        &self.endpoint_boundary_schedule_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanIntervalSubdivisionNormalizedSchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanIntervalSubdivisionNormalizationCounters {
        self.counters
    }
}
