use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanPointSplitPosture;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct PlanarBooleanDuplicateSplitCutKey {
    source_edge_identity: String,
    carrier_identity: String,
    parameter_bits: u64,
    posture_rank: u8,
    local_frame_identity: String,
    precision_basis_identity: String,
}

impl PlanarBooleanDuplicateSplitCutKey {
    pub(super) fn from_point_entry(entry: &PlanarBooleanRawEdgeSplitScheduleEntry) -> Option<Self> {
        let posture_rank = match entry.kind() {
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_) => {
                duplicate_cut_kind_rank(entry.kind())
            }
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => return None,
        };
        Some(Self {
            source_edge_identity: entry.source_edge_identity().to_string(),
            carrier_identity: entry.carrier_identity().to_string(),
            parameter_bits: crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits(entry.parameter()),
            posture_rank,
            local_frame_identity: entry.local_frame_identity().to_string(),
            precision_basis_identity: entry.precision_basis_identity().to_string(),
        })
    }
}

pub(super) fn duplicate_cut_kind_rank(kind: PlanarBooleanRawEdgeSplitScheduleEntryKind) -> u8 {
    match kind {
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) => match posture {
            PlanarBooleanPointSplitPosture::InteriorSplit => 0,
            PlanarBooleanPointSplitPosture::TJunctionPromotion => 1,
            PlanarBooleanPointSplitPosture::SharedEndpoint => 2,
            PlanarBooleanPointSplitPosture::EndpointNoOp => 3,
        },
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => 4,
    }
}
