use std::cmp::Ordering;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};

use super::denial::{
    PlanarBooleanOrderedEdgeSplitScheduleDenial, PlanarBooleanOrderedEdgeSplitScheduleDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitScheduleOrderKey {
    source_edge_identity: String,
    parameter_bits: u64,
    entry_kind_rank: u8,
    event_identity: String,
    event_group_identities: Vec<String>,
    carrier_identity: String,
    candidate_identity: String,
}

impl PlanarBooleanSplitScheduleOrderKey {
    pub(crate) fn from_entry(
        entry: &PlanarBooleanRawEdgeSplitScheduleEntry,
    ) -> Result<Self, PlanarBooleanOrderedEdgeSplitScheduleDenial> {
        if !entry.parameter().is_finite() {
            return Err(PlanarBooleanOrderedEdgeSplitScheduleDenial::new(
                PlanarBooleanOrderedEdgeSplitScheduleDenialKind::NonFiniteScheduleParameter,
                entry.entry_identity(),
                "split schedule ordering requires admitted finite parameters",
            ));
        }
        if entry.event_identity().is_empty()
            || entry.carrier_identity().is_empty()
            || entry.candidate_identity().is_empty()
        {
            return Err(PlanarBooleanOrderedEdgeSplitScheduleDenial::new(
                PlanarBooleanOrderedEdgeSplitScheduleDenialKind::MissingTieBreakerIdentity,
                entry.entry_identity(),
                "split schedule ordering requires explicit tie-breaker identities",
            ));
        }
        Ok(Self {
            source_edge_identity: entry.source_edge_identity().to_string(),
            parameter_bits: canonical_parameter_bits(entry.parameter()),
            entry_kind_rank: entry_kind_rank(entry.kind()),
            event_identity: entry.event_identity().to_string(),
            event_group_identities: entry.event_group_identities().to_vec(),
            carrier_identity: entry.carrier_identity().to_string(),
            candidate_identity: entry.candidate_identity().to_string(),
        })
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn parameter_bits(&self) -> u64 {
        self.parameter_bits
    }

    pub fn entry_kind_rank(&self) -> u8 {
        self.entry_kind_rank
    }

    pub fn event_identity(&self) -> &str {
        &self.event_identity
    }

    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }

    pub(crate) fn append_digest_parts(&self, parts: &mut Vec<String>) {
        parts.push(format!("source-edge:{}", self.source_edge_identity));
        parts.push(format!("parameter-bits:{}", self.parameter_bits));
        parts.push(format!("entry-kind-rank:{}", self.entry_kind_rank));
        parts.push(format!("event:{}", self.event_identity));
        parts.extend(
            self.event_group_identities
                .iter()
                .map(|identity| format!("event-group:{identity}")),
        );
        parts.push(format!("carrier:{}", self.carrier_identity));
        parts.push(format!("candidate:{}", self.candidate_identity));
    }
}

impl Ord for PlanarBooleanSplitScheduleOrderKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_edge_identity
            .cmp(&other.source_edge_identity)
            .then_with(|| self.parameter_bits.cmp(&other.parameter_bits))
            .then_with(|| self.entry_kind_rank.cmp(&other.entry_kind_rank))
            .then_with(|| self.event_identity.cmp(&other.event_identity))
            .then_with(|| {
                self.event_group_identities
                    .cmp(&other.event_group_identities)
            })
            .then_with(|| self.carrier_identity.cmp(&other.carrier_identity))
            .then_with(|| self.candidate_identity.cmp(&other.candidate_identity))
    }
}

impl PartialOrd for PlanarBooleanSplitScheduleOrderKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn entry_kind_rank(kind: PlanarBooleanRawEdgeSplitScheduleEntryKind) -> u8 {
    match kind {
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) => {
            if posture.produces_split_vertex() {
                0
            } else {
                1
            }
        }
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => 2,
    }
}
