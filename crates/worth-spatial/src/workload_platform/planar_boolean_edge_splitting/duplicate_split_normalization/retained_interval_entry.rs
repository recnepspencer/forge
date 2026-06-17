use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::denial::{
    PlanarBooleanDuplicateSplitNormalizationDenial,
    PlanarBooleanDuplicateSplitNormalizationDenialKind,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanRetainedIntervalSplitEntry {
    entry_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    candidate_identity: String,
    event_identity: String,
    interval_event_kind: PlanarBooleanIntervalEventKind,
    admitted_parameter_range: [f64; 2],
    source_interval_identity: String,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    normalized_interval_identity: String,
    normalized_parameter_range: [f64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    participation_row_identity: String,
    event_group_identities: Vec<String>,
}

impl PlanarBooleanRetainedIntervalSplitEntry {
    pub(crate) fn from_raw_interval_entry(
        raw_entry: &PlanarBooleanRawEdgeSplitScheduleEntry,
    ) -> Result<Self, PlanarBooleanDuplicateSplitNormalizationDenial> {
        if raw_entry.kind() != PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval {
            return Err(PlanarBooleanDuplicateSplitNormalizationDenial::new(
                PlanarBooleanDuplicateSplitNormalizationDenialKind::MalformedRetainedIntervalEntry,
                raw_entry.entry_identity(),
                "retained interval entries must come from interval raw schedule entries",
            ));
        }
        let admitted_parameter_range = raw_entry.parameter_range().ok_or_else(|| {
            PlanarBooleanDuplicateSplitNormalizationDenial::new(
                PlanarBooleanDuplicateSplitNormalizationDenialKind::MalformedRetainedIntervalEntry,
                raw_entry.entry_identity(),
                "retained interval entry is missing admitted parameter range",
            )
        })?;
        let authority = raw_entry.interval_authority().ok_or_else(|| {
            PlanarBooleanDuplicateSplitNormalizationDenial::new(
                PlanarBooleanDuplicateSplitNormalizationDenialKind::MalformedRetainedIntervalEntry,
                raw_entry.entry_identity(),
                "retained interval entry is missing interval authority",
            )
        })?;
        Ok(Self {
            entry_identity: raw_entry.entry_identity().to_string(),
            source_edge_identity: raw_entry.source_edge_identity().to_string(),
            carrier_identity: raw_entry.carrier_identity().to_string(),
            candidate_identity: raw_entry.candidate_identity().to_string(),
            event_identity: raw_entry.event_identity().to_string(),
            interval_event_kind: authority.interval_event_kind(),
            admitted_parameter_range,
            source_interval_identity: authority.source_interval_identity().to_string(),
            source_parameter_range: authority.source_parameter_range(),
            source_sense: authority.source_sense(),
            normalized_interval_identity: authority.normalized_interval_identity().to_string(),
            normalized_parameter_range: authority.normalized_parameter_range(),
            local_frame_identity: raw_entry.local_frame_identity().to_string(),
            precision_basis_identity: raw_entry.precision_basis_identity().to_string(),
            participation_row_identity: authority.participation_row_identity().to_string(),
            event_group_identities: raw_entry.event_group_identities().to_vec(),
        })
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
    pub fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
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
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
    pub fn participation_row_identity(&self) -> &str {
        &self.participation_row_identity
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
}
