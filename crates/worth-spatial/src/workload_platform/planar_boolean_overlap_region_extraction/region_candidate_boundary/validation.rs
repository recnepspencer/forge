use std::collections::BTreeSet;

use super::counters::PlanarBooleanOverlapRegionCandidateBoundaryCounters;
use super::denial::{
    PlanarBooleanOverlapRegionCandidateBoundaryDenial,
    PlanarBooleanOverlapRegionCandidateBoundaryDenialKind,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOppositeSenseOverlapNormalizationRow, PlanarBooleanOverlapRegionCandidateBoundaryInput,
};

pub(super) fn validate_input_identities(
    input: PlanarBooleanOverlapRegionCandidateBoundaryInput<'_>,
) -> Result<(), PlanarBooleanOverlapRegionCandidateBoundaryDenial> {
    let shared_area = input.shared_area_admission();
    let normalization = input.pre_region_normalization().opposite_sense_overlap_normalizations();
    if shared_area.request_identity() != normalization.request_identity()
        || shared_area.arrangement_graph_identity() != normalization.arrangement_graph_identity()
        || shared_area.cell_set_identity() != normalization.cell_set_identity()
        || shared_area.ordering_basis_identity() != normalization.ordering_basis_identity()
    {
        return Err(PlanarBooleanOverlapRegionCandidateBoundaryDenial::new(
            PlanarBooleanOverlapRegionCandidateBoundaryDenialKind::InputIdentityMismatchDenied,
            shared_area.request_identity(),
            PlanarBooleanOverlapRegionCandidateBoundaryCounters::default(),
            "overlap-region candidate promotion requires shared-area admission and pre-region normalization from the same proof chain",
        ));
    }
    Ok(())
}

pub(super) fn validate_normalization_coverage(
    input: PlanarBooleanOverlapRegionCandidateBoundaryInput<'_>,
) -> Result<(), PlanarBooleanOverlapRegionCandidateBoundaryDenial> {
    let shared_area_outcome_ids = input
        .shared_area_admission()
        .shared_area_admission_outcomes()
        .rows()
        .iter()
        .map(|row| row.outcome_identity())
        .collect::<BTreeSet<_>>();
    for normalization in input
        .pre_region_normalization()
        .opposite_sense_overlap_normalizations()
        .rows()
    {
        if !shared_area_outcome_ids.contains(normalization.shared_area_admission_outcome_identity()) {
            return Err(orphan_normalization_denial(normalization));
        }
    }
    Ok(())
}

fn orphan_normalization_denial(
    normalization: &PlanarBooleanOppositeSenseOverlapNormalizationRow,
) -> PlanarBooleanOverlapRegionCandidateBoundaryDenial {
    PlanarBooleanOverlapRegionCandidateBoundaryDenial::new(
        PlanarBooleanOverlapRegionCandidateBoundaryDenialKind::NormalizationSharedAreaMismatchDenied,
        normalization.normalization_identity(),
        PlanarBooleanOverlapRegionCandidateBoundaryCounters::default(),
        "overlap-region candidate promotion denies normalization rows that cannot be bound back to admitted shared-area outcomes",
    )
}
