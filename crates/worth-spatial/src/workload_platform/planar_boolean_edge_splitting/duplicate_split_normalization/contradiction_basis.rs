use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::canonical_schedule_ordering::PlanarBooleanOrderedEdgeSplitScheduleEntry;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};

use super::denial::{
    PlanarBooleanDuplicateSplitNormalizationDenial,
    PlanarBooleanDuplicateSplitNormalizationDenialKind,
};

pub(super) fn reject_contradictory_same_parameter_points(
    entries: &[PlanarBooleanOrderedEdgeSplitScheduleEntry],
) -> Result<(), PlanarBooleanDuplicateSplitNormalizationDenial> {
    let mut by_parameter =
        BTreeMap::<SameParameterPointAuthority, PointSplitCompatibilityBasis>::new();
    for ordered_entry in entries {
        let raw_entry = ordered_entry.raw_entry();
        if matches!(
            raw_entry.kind(),
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval
        ) {
            continue;
        }
        let authority = SameParameterPointAuthority::from_entry(raw_entry);
        let compatibility = PointSplitCompatibilityBasis::from_entry(raw_entry);
        if let Some(existing) = by_parameter.get(&authority) {
            if existing != &compatibility {
                return Err(PlanarBooleanDuplicateSplitNormalizationDenial::new(
                    PlanarBooleanDuplicateSplitNormalizationDenialKind::ContradictoryDuplicateSplitPoint,
                    raw_entry.entry_identity(),
                    "same source edge, carrier, and parameter cannot normalize contradictory frame or precision facts",
                ));
            }
        } else {
            by_parameter.insert(authority, compatibility);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SameParameterPointAuthority {
    source_edge_identity: String,
    carrier_identity: String,
    parameter_bits: u64,
}

impl SameParameterPointAuthority {
    fn from_entry(entry: &PlanarBooleanRawEdgeSplitScheduleEntry) -> Self {
        Self {
            source_edge_identity: entry.source_edge_identity().to_string(),
            carrier_identity: entry.carrier_identity().to_string(),
            parameter_bits: canonical_parameter_bits(entry.parameter()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PointSplitCompatibilityBasis {
    local_frame_identity: String,
    precision_basis_identity: String,
}

impl PointSplitCompatibilityBasis {
    fn from_entry(entry: &PlanarBooleanRawEdgeSplitScheduleEntry) -> Self {
        Self {
            local_frame_identity: entry.local_frame_identity().to_string(),
            precision_basis_identity: entry.precision_basis_identity().to_string(),
        }
    }
}
