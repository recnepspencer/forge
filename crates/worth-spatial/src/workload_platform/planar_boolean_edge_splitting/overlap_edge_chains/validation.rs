use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanNormalizedIntervalSubdivisionRow,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::PlanarBooleanSplitEdgeFragmentSet;

use super::denial::{PlanarBooleanOverlapEdgeChainDenial, PlanarBooleanOverlapEdgeChainDenialKind};

pub(super) fn reject_foreign_fragment_set(
    schedules: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) -> Result<(), PlanarBooleanOverlapEdgeChainDenial> {
    if fragments.interval_subdivision_schedule_set_identity() == schedules.schedule_set_identity() {
        return Ok(());
    }
    Err(PlanarBooleanOverlapEdgeChainDenial::new(
        PlanarBooleanOverlapEdgeChainDenialKind::ForeignFragmentSet,
        fragments.fragment_set_identity(),
        "overlap chain construction requires fragments built from the same interval-subdivision schedule set",
    ))
}

pub(super) fn reject_ambiguous_chain_basis(
    subdivisions: &[&PlanarBooleanNormalizedIntervalSubdivisionRow],
) -> Result<(), PlanarBooleanOverlapEdgeChainDenial> {
    let Some(first) = subdivisions.first() else {
        return Ok(());
    };
    if subdivisions.iter().all(|subdivision| {
        subdivision.interval_event_kind() == first.interval_event_kind()
            && subdivision.local_frame_identity() == first.local_frame_identity()
            && subdivision.precision_basis_identity() == first.precision_basis_identity()
    }) {
        return Ok(());
    }
    Err(PlanarBooleanOverlapEdgeChainDenial::new(
        PlanarBooleanOverlapEdgeChainDenialKind::AmbiguousOverlapChainBasis,
        first.interval_event_identity(),
        "overlap chain construction requires one event kind, frame, and precision basis per chain",
    ))
}
