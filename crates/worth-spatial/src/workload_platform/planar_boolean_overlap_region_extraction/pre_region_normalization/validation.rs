use super::counters::PlanarBooleanPreRegionNormalizationCounters;
use super::denial::{
    PlanarBooleanPreRegionNormalizationDenial, PlanarBooleanPreRegionNormalizationDenialKind,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapChainRegionLineageMap,
    PlanarBooleanOverlapChainRegionLineageRow, PlanarBooleanPreRegionNormalizationInput,
    PlanarBooleanSharedAreaAdmissionOutcomeRow,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan;

pub(super) fn validate_input_identities(
    input: PlanarBooleanPreRegionNormalizationInput<'_>,
    counters: &mut PlanarBooleanPreRegionNormalizationCounters,
) -> Result<(), PlanarBooleanPreRegionNormalizationDenial> {
    if input.shared_area_admission().request_identity()
        != input.chain_lineage_map().request_identity()
    {
        counters.denied_normalization();
        return Err(PlanarBooleanPreRegionNormalizationDenial::new(
            PlanarBooleanPreRegionNormalizationDenialKind::InputIdentityMismatchDenied,
            input.shared_area_admission().request_identity(),
            *counters,
            "pre-region normalization requires admitted shared-area outcomes and chain lineage from the same overlap extraction request",
        ));
    }
    Ok(())
}

pub(super) fn relevant_lineage_rows<'a>(
    row: &PlanarBooleanSharedAreaAdmissionOutcomeRow,
    chain_lineage_map: &'a PlanarBooleanOverlapChainRegionLineageMap,
    counters: &mut PlanarBooleanPreRegionNormalizationCounters,
) -> Result<
    Vec<&'a PlanarBooleanOverlapChainRegionLineageRow>,
    PlanarBooleanPreRegionNormalizationDenial,
> {
    let relevant = chain_lineage_map
        .rows()
        .iter()
        .filter(|lineage| lineage_binds_shared_area_row(lineage, row))
        .collect::<Vec<_>>();
    if relevant.is_empty() {
        counters.denied_normalization();
        return Err(PlanarBooleanPreRegionNormalizationDenial::new(
            PlanarBooleanPreRegionNormalizationDenialKind::MissingChainLineageForSharedAreaOutcomeDenied,
            row.area_overlap_component_identity(),
            *counters,
            "pre-region normalization denies admitted shared-area outcomes that cannot be bound to matching overlap-chain lineage proof",
        ));
    }
    Ok(relevant)
}

fn lineage_binds_shared_area_row(
    lineage: &PlanarBooleanOverlapChainRegionLineageRow,
    row: &PlanarBooleanSharedAreaAdmissionOutcomeRow,
) -> bool {
    let source_loop_matches = lineage
        .source_loop_identities()
        .iter()
        .any(|identity| row.source_loop_identities().contains(identity));
    if !source_loop_matches {
        return false;
    }

    let island_matches = lineage
        .participating_island_identities()
        .iter()
        .any(|identity| identity == row.island_identity());
    let source_edge_matches = lineage
        .source_edge_identities()
        .iter()
        .any(|identity| row.boundary_segment_identities().contains(identity));
    let source_only_full_span = lineage.participating_island_identities().is_empty()
        && lineage
            .boundary_roles()
            .iter()
            .any(|role| *role == FullOverlapSpan);

    (island_matches && source_edge_matches) || source_only_full_span
}

pub(super) fn ambiguous_ordering(
    row: &PlanarBooleanSharedAreaAdmissionOutcomeRow,
    counters: &mut PlanarBooleanPreRegionNormalizationCounters,
) -> PlanarBooleanPreRegionNormalizationDenial {
    counters.denied_normalization();
    PlanarBooleanPreRegionNormalizationDenial::new(
        PlanarBooleanPreRegionNormalizationDenialKind::AmbiguousOppositeSenseOverlapOrderingDenied,
        row.area_overlap_component_identity(),
        *counters,
        "pre-region normalization denies opposite-sense coincidence whose boundary ordering remains ambiguous before region promotion",
    )
}

pub(super) fn unstable_tie_breaker(
    row: &PlanarBooleanSharedAreaAdmissionOutcomeRow,
    counters: &mut PlanarBooleanPreRegionNormalizationCounters,
) -> PlanarBooleanPreRegionNormalizationDenial {
    counters.denied_normalization();
    PlanarBooleanPreRegionNormalizationDenial::new(
        PlanarBooleanPreRegionNormalizationDenialKind::UnstableOrientationTieBreakerDenied,
        row.area_overlap_component_identity(),
        *counters,
        "pre-region normalization denies admitted shared-area outcomes whose operand-side or winding tie breaker is unstable across opposite-sense coincidence",
    )
}
