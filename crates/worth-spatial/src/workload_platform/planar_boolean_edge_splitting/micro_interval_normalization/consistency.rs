use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::PlanarBooleanRetainedIntervalSplitEntry;

use super::denial::{
    PlanarBooleanIntervalSubdivisionNormalizationDenial,
    PlanarBooleanIntervalSubdivisionNormalizationDenialKind,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct IntervalSubdivisionConsistencyKey {
    source_edge_identity: String,
    carrier_identity: String,
    range_bits: [u64; 2],
    normalized_interval_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IntervalSubdivisionConsistencyBasis {
    local_frame_identity: String,
    precision_basis_identity: String,
}

impl IntervalSubdivisionConsistencyKey {
    fn from_entry(entry: &PlanarBooleanRetainedIntervalSplitEntry) -> Self {
        let range = entry.admitted_parameter_range();
        Self {
            source_edge_identity: entry.source_edge_identity().to_string(),
            carrier_identity: entry.carrier_identity().to_string(),
            range_bits: [
                canonical_parameter_bits(range[0]),
                canonical_parameter_bits(range[1]),
            ],
            normalized_interval_identity: entry.normalized_interval_identity().to_string(),
        }
    }
}

impl IntervalSubdivisionConsistencyBasis {
    fn from_entry(entry: &PlanarBooleanRetainedIntervalSplitEntry) -> Self {
        Self {
            local_frame_identity: entry.local_frame_identity().to_string(),
            precision_basis_identity: entry.precision_basis_identity().to_string(),
        }
    }
}

pub(super) fn reject_contradictory_interval_subdivision_basis(
    entries: &[PlanarBooleanRetainedIntervalSplitEntry],
) -> Result<(), PlanarBooleanIntervalSubdivisionNormalizationDenial> {
    let mut basis_by_interval =
        BTreeMap::<IntervalSubdivisionConsistencyKey, IntervalSubdivisionConsistencyBasis>::new();
    for entry in entries {
        let key = IntervalSubdivisionConsistencyKey::from_entry(entry);
        let basis = IntervalSubdivisionConsistencyBasis::from_entry(entry);
        if let Some(existing_basis) = basis_by_interval.get(&key) {
            if existing_basis != &basis {
                return Err(PlanarBooleanIntervalSubdivisionNormalizationDenial::new(
                    PlanarBooleanIntervalSubdivisionNormalizationDenialKind::ContradictoryIntervalSubdivisionBasis,
                    entry.entry_identity(),
                    "retained interval rows for the same subdivision must agree on local frame and precision basis",
                ));
            }
        } else {
            basis_by_interval.insert(key, basis);
        }
    }
    Ok(())
}
