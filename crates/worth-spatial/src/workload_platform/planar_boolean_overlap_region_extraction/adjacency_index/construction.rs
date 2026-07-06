use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

use super::counters::PlanarBooleanOverlapAdjacencyIndexCounters;
use super::denial::{
    PlanarBooleanOverlapAdjacencyIndexDenial, PlanarBooleanOverlapAdjacencyIndexDenialKind as Kind,
};
use super::identity::{
    adjacency_index_identity, adjacency_neighborhood_identity, adjacency_row_identity,
};
use super::input::PlanarBooleanOverlapAdjacencyIndexInput;
use super::lookup::sorted_unique;
use super::ordering::{
    adjacency_order_key, canonicalize_adjacency_rows, island_order_key, loop_order_key,
    PlanarBooleanOverlapAdjacencyOrderingBasis,
};
use super::product::PlanarBooleanOverlapRegionAdjacencyIndex;
use super::row::PlanarBooleanOverlapAdjacencyRow;
use super::validation::validate_input;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanLoopIslandOverlapParticipationRow, PlanarBooleanLoopOverlapParticipationRow,
    PlanarBooleanOverlapChainRegionLineageRow,
};

#[derive(Clone)]
struct CanonicalAdjacencySegment {
    source_loop_identity: String,
    source_loop_operand_side: PlanarBooleanCommonPlaneOperandSide,
    source_loop_winding_sign: i8,
    source_edge_identity: String,
    fragment_identity: String,
    boundary_role:
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
}

pub(crate) fn build_adjacency_index(
    input: PlanarBooleanOverlapAdjacencyIndexInput<'_>,
) -> Result<PlanarBooleanOverlapRegionAdjacencyIndex, PlanarBooleanOverlapAdjacencyIndexDenial> {
    let mut counters = PlanarBooleanOverlapAdjacencyIndexCounters::default();
    let lookup = validate_input(&input, &mut counters)?;

    let mut adjacency_rows = Vec::new();
    for component in lookup.neighborhood_components() {
        for _ in component.lineage_rows() {
            counters.consumed_chain_lineage_row();
        }

        let lineage_identities = sorted_unique(
            component
                .lineage_rows()
                .iter()
                .map(|row| row.lineage_identity().to_string()),
        );
        let chain_identities = vec![component.chain_identity().to_string()];
        let participating_loop_identities = component.participating_loop_identities().to_vec();
        let participating_island_identities = component.participating_island_identities().to_vec();

        let loop_order_seed = component.connectivity_identity();
        let mut ordered_loop_rows = participating_loop_identities
            .iter()
            .map(|identity| {
                lookup
                    .loop_row(identity)
                    .expect("validated loop identity should resolve")
            })
            .collect::<Vec<_>>();
        ordered_loop_rows.sort_by_key(|row| loop_order_key(row, &loop_order_seed));

        let mut ordered_island_rows = participating_island_identities
            .iter()
            .map(|identity| {
                lookup
                    .island_row(identity)
                    .expect("validated island identity should resolve")
            })
            .collect::<Vec<_>>();
        ordered_island_rows.sort_by_key(|row| island_order_key(row, &loop_order_seed));

        let neighborhood_identity = adjacency_neighborhood_identity(
            input.chain_lineage_map().request_identity(),
            &chain_identities,
            &lineage_identities,
            &participating_loop_identities,
            &participating_island_identities,
        );
        let canonical_segments = canonical_adjacency_segments(component.lineage_rows());

        let row = PlanarBooleanOverlapAdjacencyRow::new(
            adjacency_row_identity(&neighborhood_identity),
            neighborhood_identity,
            chain_identities,
            lineage_identities,
            ordered_loop_rows
                .iter()
                .map(|row| row.participation_identity().to_string())
                .collect(),
            participating_loop_identities,
            ordered_loop_rows
                .iter()
                .map(|row| row.loop_role())
                .collect(),
            ordered_island_rows
                .iter()
                .map(|row| row.participation_identity().to_string())
                .collect(),
            participating_island_identities,
            ordered_island_rows
                .iter()
                .map(|row| row.island_origin_loop_identity().to_string())
                .collect(),
            ordered_island_rows
                .iter()
                .map(|row| row.island_kind())
                .collect(),
            ordered_island_rows
                .iter()
                .map(|row| row.member_source_loop_identities().to_vec())
                .collect(),
            ordered_island_rows
                .iter()
                .map(|row| row.member_source_loop_operand_sides().to_vec())
                .collect(),
            ordered_island_rows
                .iter()
                .map(|row| row.member_source_loop_winding_signs().to_vec())
                .collect(),
            canonical_segments
                .iter()
                .map(|segment| segment.source_loop_identity.clone())
                .collect(),
            canonical_segments
                .iter()
                .map(|segment| segment.source_loop_operand_side)
                .collect(),
            canonical_segments
                .iter()
                .map(|segment| segment.source_loop_winding_sign)
                .collect(),
            canonical_segments
                .iter()
                .map(|segment| segment.source_edge_identity.clone())
                .collect(),
            canonical_segments
                .iter()
                .map(|segment| segment.fragment_identity.clone())
                .collect(),
            canonical_segments
                .iter()
                .map(|segment| segment.boundary_role)
                .collect(),
            aggregate_persistent_name_identities(
                component.lineage_rows(),
                &ordered_loop_rows,
                &ordered_island_rows,
            ),
        );
        counters.indexed_neighborhood();
        counters.emitted_row();
        adjacency_rows.push(row);
    }

    reject_duplicate_row_keys(&adjacency_rows, &mut counters)?;
    let mut ordered_rows = adjacency_rows;
    let ordered_neighborhood_identities = canonicalize_adjacency_rows(&mut ordered_rows);
    let row_identities = ordered_rows
        .iter()
        .map(|row| row.adjacency_identity().to_string())
        .collect::<Vec<_>>();
    let index_identity = adjacency_index_identity(
        input.chain_lineage_map().request_identity(),
        &row_identities,
    );
    let ordering_basis = PlanarBooleanOverlapAdjacencyOrderingBasis::new(
        input.chain_lineage_map().request_identity(),
        &index_identity,
        ordered_neighborhood_identities,
    );
    let source_only_boundary_lane = input.loop_participation_map().rows().is_empty()
        && input.island_participation_map().rows().is_empty();
    Ok(
        PlanarBooleanOverlapRegionAdjacencyIndex::new_with_source_only_boundary_lane(
            index_identity,
            input.chain_lineage_map().request_identity().to_string(),
            input.loop_participation_map().map_identity().to_string(),
            input.island_participation_map().map_identity().to_string(),
            input.chain_lineage_map().map_identity().to_string(),
            ordered_rows,
            ordering_basis,
            source_only_boundary_lane,
            counters,
        ),
    )
}

fn aggregate_persistent_name_identities(
    lineage_rows: &[&PlanarBooleanOverlapChainRegionLineageRow],
    ordered_loop_rows: &[&PlanarBooleanLoopOverlapParticipationRow],
    ordered_island_rows: &[&PlanarBooleanLoopIslandOverlapParticipationRow],
) -> Vec<String> {
    sorted_unique(
        lineage_rows
            .iter()
            .flat_map(|row| row.propagated_persistent_name_identities().iter().cloned())
            .chain(
                ordered_loop_rows
                    .iter()
                    .flat_map(|row| row.propagated_persistent_name_identities().iter().cloned()),
            )
            .chain(
                ordered_island_rows
                    .iter()
                    .flat_map(|row| row.propagated_persistent_name_identities().iter().cloned()),
            ),
    )
}

fn canonical_adjacency_segments(
    lineage_rows: &[&PlanarBooleanOverlapChainRegionLineageRow],
) -> Vec<CanonicalAdjacencySegment> {
    let mut segments = lineage_rows
        .iter()
        .flat_map(|row| {
            row.source_loop_identities()
                .iter()
                .zip(row.source_loop_operand_sides().iter().copied())
                .zip(row.source_loop_winding_signs().iter().copied())
                .zip(row.source_edge_identities().iter())
                .zip(row.fragment_identities().iter())
                .zip(row.boundary_roles().iter().copied())
                .map(
                    |(
                        (
                            (
                                (
                                    (source_loop_identity, source_loop_operand_side),
                                    source_loop_winding_sign,
                                ),
                                source_edge_identity,
                            ),
                            fragment_identity,
                        ),
                        boundary_role,
                    )| CanonicalAdjacencySegment {
                        source_loop_identity: source_loop_identity.clone(),
                        source_loop_operand_side,
                        source_loop_winding_sign,
                        source_edge_identity: source_edge_identity.clone(),
                        fragment_identity: fragment_identity.clone(),
                        boundary_role,
                    },
                )
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    segments.sort_by(|left, right| {
        canonical_adjacency_segment_key(left).cmp(&canonical_adjacency_segment_key(right))
    });
    segments
}

fn canonical_adjacency_segment_key(segment: &CanonicalAdjacencySegment) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        segment.source_loop_identity,
        segment.source_loop_operand_side.query_key(),
        segment.source_loop_winding_sign,
        segment.source_edge_identity,
        segment.fragment_identity,
        boundary_role_rank(segment.boundary_role)
    )
}

fn boundary_role_rank(
    role: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
) -> u8 {
    use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::{
        FullOverlapSpan, OverlapEndBoundary, OverlapInteriorFragment, OverlapStartBoundary,
    };

    match role {
        FullOverlapSpan => 0,
        OverlapStartBoundary => 1,
        OverlapInteriorFragment => 2,
        OverlapEndBoundary => 3,
    }
}

fn reject_duplicate_row_keys(
    rows: &[PlanarBooleanOverlapAdjacencyRow],
    counters: &mut PlanarBooleanOverlapAdjacencyIndexCounters,
) -> Result<(), PlanarBooleanOverlapAdjacencyIndexDenial> {
    let mut seen = BTreeSet::new();
    for row in rows {
        let key = adjacency_order_key(row);
        if !seen.insert(key) {
            counters.denied_neighborhood();
            return Err(PlanarBooleanOverlapAdjacencyIndexDenial::new(
                Kind::IncidentalIterationOrderTieBreakDenied,
                row.neighborhood_identity(),
                *counters,
                "overlap adjacency denies neighborhoods whose canonical ordering would collapse to an incidental iteration-order tie",
            ));
        }
    }
    Ok(())
}
