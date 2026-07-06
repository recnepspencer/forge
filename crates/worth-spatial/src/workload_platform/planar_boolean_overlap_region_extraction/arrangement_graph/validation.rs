use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanOverlapArrangementGraphCounters;
use super::denial::{
    PlanarBooleanOverlapArrangementGraphDenial,
    PlanarBooleanOverlapArrangementGraphDenialKind as Kind,
};
use super::input::PlanarBooleanOverlapArrangementGraphInput;
use super::lookup::{
    ValidatedArrangementBoundarySegment, ValidatedArrangementNeighborhood,
    ValidatedOverlapArrangementLookup,
};
use super::topology_validation::{validate_boundary_components, validate_cells};

pub(crate) fn validate_input<'a>(
    input: &'a PlanarBooleanOverlapArrangementGraphInput<'a>,
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
) -> Result<ValidatedOverlapArrangementLookup<'a>, PlanarBooleanOverlapArrangementGraphDenial> {
    if input.adjacency_index().request_identity() != input.ordering_basis().request_identity()
        || input.adjacency_index().adjacency_index_identity()
            != input.ordering_basis().adjacency_index_identity()
    {
        return Err(deny(
            Kind::ArrangementOrderingBasisMismatchDenied,
            input.adjacency_index().adjacency_index_identity(),
            counters,
            "overlap arrangement requires an ordering basis admitted for the same adjacency index and overlap request",
        ));
    }

    let mut adjacency_rows_by_neighborhood = BTreeMap::new();
    for row in input.adjacency_index().rows() {
        if adjacency_rows_by_neighborhood
            .insert(row.neighborhood_identity(), row)
            .is_some()
        {
            return Err(deny(
                Kind::ContradictoryArrangementNeighborhoodDenied,
                row.neighborhood_identity(),
                counters,
                "overlap arrangement denies duplicate adjacency neighborhoods inside one admitted adjacency index",
            ));
        }
    }

    let ordered_neighborhood_identities = input
        .ordering_basis()
        .ordered_neighborhood_identities()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let ordered_set = ordered_neighborhood_identities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let adjacency_set = adjacency_rows_by_neighborhood
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    if ordered_set != adjacency_set
        || ordered_neighborhood_identities.len() != adjacency_rows_by_neighborhood.len()
    {
        return Err(deny(
            Kind::ArrangementOrderingBasisMismatchDenied,
            input.ordering_basis().basis_identity(),
            counters,
            "overlap arrangement denies adjacency neighborhoods whose ordering-basis coverage does not exactly match the admitted adjacency index",
        ));
    }

    let mut ordered_neighborhoods = Vec::new();
    for identity in ordered_neighborhood_identities {
        let row = adjacency_rows_by_neighborhood
            .get(identity)
            .copied()
            .expect("validated ordered neighborhood identity should resolve");
        ordered_neighborhoods.push(validate_adjacency_row(
            row,
            input.adjacency_index().source_only_boundary_lane(),
            counters,
        )?);
    }

    Ok(ValidatedOverlapArrangementLookup::new(
        ordered_neighborhoods,
    ))
}

fn validate_adjacency_row<'a>(
    row: &'a crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapAdjacencyRow,
    source_only_boundary_lane: bool,
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
) -> Result<ValidatedArrangementNeighborhood<'a>, PlanarBooleanOverlapArrangementGraphDenial> {
    if row.chain_identities().is_empty() || row.lineage_identities().is_empty() {
        return Err(deny(
            Kind::ContradictoryArrangementNeighborhoodDenied,
            row.neighborhood_identity(),
            counters,
            "overlap arrangement denies adjacency neighborhoods missing certified participation or lineage authority",
        ));
    }

    let segment_count = row.source_loop_identities().len();
    if segment_count == 0
        || row.source_loop_operand_sides().len() != segment_count
        || row.source_loop_winding_signs().len() != segment_count
        || row.source_edge_identities().len() != segment_count
        || row.fragment_identities().len() != segment_count
        || row.boundary_roles().len() != segment_count
    {
        return Err(deny(
            Kind::NoConcreteCellSubstrateDenied,
            row.neighborhood_identity(),
            counters,
            "overlap arrangement denies neighborhoods that do not carry one aligned segment substrate across source loops, source edges, fragments, and boundary roles",
        ));
    }
    let certified_source_loop_identities = row
        .participating_loop_identities()
        .iter()
        .map(String::as_str)
        .chain(
            row.island_origin_loop_identities()
                .iter()
                .map(String::as_str),
        )
        .collect::<BTreeSet<_>>();
    if !source_only_boundary_lane
        && row
            .source_loop_identities()
            .iter()
            .any(|identity| !certified_source_loop_identities.contains(identity.as_str()))
    {
        return Err(deny(
            Kind::ContradictoryArrangementNeighborhoodDenied,
            row.neighborhood_identity(),
            counters,
            "overlap arrangement denies segment source-loop identities that are not certified by the admitted adjacency-carried loop provenance",
        ));
    }
    if !source_only_boundary_lane
        && (row.participating_loop_identities().is_empty()
            || row.participating_island_identities().is_empty())
    {
        return Err(deny(
            Kind::ContradictoryArrangementNeighborhoodDenied,
            row.neighborhood_identity(),
            counters,
            "overlap arrangement denies adjacency neighborhoods missing certified participation or lineage authority",
        ));
    }
    if row.participating_island_identities().len() != row.island_origin_loop_identities().len()
        || row.participating_island_identities().len() != row.island_kinds().len()
        || row.participating_island_identities().len()
            != row.island_member_source_loop_identities().len()
        || row.participating_island_identities().len()
            != row.island_member_source_loop_operand_sides().len()
        || row.participating_island_identities().len()
            != row.island_member_source_loop_winding_signs().len()
    {
        return Err(deny(
            Kind::ContradictoryArrangementNeighborhoodDenied,
            row.neighborhood_identity(),
            counters,
            "overlap arrangement denies island-backed face witnesses whose identities, origins, kinds, and member source-loop operand and winding groups are not aligned",
        ));
    }
    if row.source_loop_winding_signs().len() != segment_count
        || row
            .island_member_source_loop_identities()
            .iter()
            .zip(row.island_member_source_loop_operand_sides())
            .zip(row.island_member_source_loop_winding_signs())
            .any(|((identities, sides), winding_signs)| {
                identities.len() != sides.len() || identities.len() != winding_signs.len()
            })
    {
        return Err(deny(
            Kind::ContradictoryArrangementNeighborhoodDenied,
            row.neighborhood_identity(),
            counters,
            "overlap arrangement denies neighborhoods whose carried source-loop winding witness is not aligned with the admitted segment and island provenance substrate",
        ));
    }

    let segments = row
        .source_loop_identities()
        .iter()
        .zip(row.source_loop_operand_sides().iter().copied())
        .zip(row.source_loop_winding_signs().iter().copied())
        .zip(row.source_edge_identities().iter())
        .zip(row.fragment_identities().iter())
        .zip(row.boundary_roles().iter().copied())
        .enumerate()
        .map(
            |(
                ordinal,
                (
                    (
                        (
                            ((source_loop_identity, operand_side), source_loop_winding_sign),
                            source_edge_identity,
                        ),
                        fragment_identity,
                    ),
                    boundary_role,
                ),
            )| {
                ValidatedArrangementBoundarySegment {
                    source_loop_identity,
                    operand_side,
                    source_loop_winding_sign,
                    source_edge_identity,
                    fragment_identity,
                    boundary_role,
                    ordinal,
                }
            },
        )
        .collect::<Vec<_>>();
    let components = validate_boundary_components(
        row.neighborhood_identity(),
        &segments,
        source_only_boundary_lane,
        counters,
    )?;
    let cells = validate_cells(row, row.neighborhood_identity(), &components, counters)?;

    Ok(ValidatedArrangementNeighborhood::new(
        row, segments, components, cells,
    ))
}

fn deny(
    kind: Kind,
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
    human_reason: &'static str,
) -> PlanarBooleanOverlapArrangementGraphDenial {
    counters.denied_neighborhood();
    PlanarBooleanOverlapArrangementGraphDenial::new(
        kind,
        rejected_identity,
        *counters,
        human_reason,
    )
}
