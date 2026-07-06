use std::collections::BTreeSet;

use super::counters::PlanarBooleanOverlapAdjacencyIndexCounters;
use super::denial::{
    PlanarBooleanOverlapAdjacencyIndexDenial, PlanarBooleanOverlapAdjacencyIndexDenialKind as Kind,
};
use super::input::PlanarBooleanOverlapAdjacencyIndexInput;
use super::lookup::ValidatedOverlapAdjacencyLookup;
use super::ordering::{island_order_key, loop_order_key};

pub(crate) fn validate_input<'a>(
    input: &'a PlanarBooleanOverlapAdjacencyIndexInput<'a>,
    counters: &mut PlanarBooleanOverlapAdjacencyIndexCounters,
) -> Result<ValidatedOverlapAdjacencyLookup<'a>, PlanarBooleanOverlapAdjacencyIndexDenial> {
    if input.loop_participation_map().request_identity()
        != input.island_participation_map().request_identity()
        || input.loop_participation_map().request_identity()
            != input.chain_lineage_map().request_identity()
    {
        return Err(deny(
            Kind::ContradictoryAdjacencyNeighborhoodDenied,
            input.chain_lineage_map().request_identity(),
            counters,
            "overlap adjacency requires loop, island, and chain participation from the same admitted overlap request",
        ));
    }

    let mut lookup = ValidatedOverlapAdjacencyLookup::new();
    for row in input.loop_participation_map().rows() {
        if !lookup.insert_loop_row(row) {
            return Err(deny(
                Kind::ContradictoryAdjacencyNeighborhoodDenied,
                row.canonical_loop_identity(),
                counters,
                "overlap adjacency denies duplicate loop participation rows for one canonical loop identity",
            ));
        }
    }
    for row in input.island_participation_map().rows() {
        if !lookup.insert_island_row(row) {
            return Err(deny(
                Kind::ContradictoryAdjacencyNeighborhoodDenied,
                row.island_identity(),
                counters,
                "overlap adjacency denies duplicate island participation rows for one island identity",
            ));
        }
    }

    let promised_lineage_ids = input
        .loop_participation_map()
        .rows()
        .iter()
        .flat_map(|row| row.overlap_chain_lineage_identities().iter().cloned())
        .collect::<BTreeSet<_>>();
    let source_only_boundary_lane = input.loop_participation_map().rows().is_empty()
        && input.island_participation_map().rows().is_empty();

    for lineage_row in input.chain_lineage_map().rows() {
        if !lookup.insert_lineage_row(lineage_row) {
            return Err(deny(
                Kind::ContradictoryAdjacencyNeighborhoodDenied,
                lineage_row.lineage_identity(),
                counters,
                "overlap adjacency denies duplicate chain-lineage identities inside one admitted chain-lineage map",
            ));
        }
        let source_only_boundary_lineage =
            source_only_boundary_lane && is_source_only_boundary_lineage(lineage_row);
        if !promised_lineage_ids.contains(lineage_row.lineage_identity())
            && !source_only_boundary_lineage
        {
            return Err(deny(
                Kind::DanglingAdjacencyLineageDenied,
                lineage_row.lineage_identity(),
                counters,
                "overlap adjacency denies chain lineage that is not promised by the recovered loop participation surface",
            ));
        }
        if lineage_row.participating_loop_identities().is_empty()
            || lineage_row.participating_island_identities().is_empty()
        {
            if !source_only_boundary_lineage {
                return Err(deny(
                    Kind::UnindexedOverlapNeighborhoodDiscoveryDenied,
                    lineage_row.lineage_identity(),
                    counters,
                    "overlap adjacency denies overlap-chain lineage that would require unindexed neighborhood discovery",
                ));
            }
        }
        for loop_identity in lineage_row.participating_loop_identities() {
            if lookup.loop_row(loop_identity).is_none() {
                return Err(deny(
                    Kind::DanglingAdjacencyLineageDenied,
                    lineage_row.lineage_identity(),
                    counters,
                    "overlap adjacency denies chain lineage that references a participating loop outside the recovered participation map",
                ));
            }
        }
        for island_identity in lineage_row.participating_island_identities() {
            if lookup.island_row(island_identity).is_none() {
                return Err(deny(
                    Kind::DanglingAdjacencyLineageDenied,
                    lineage_row.lineage_identity(),
                    counters,
                    "overlap adjacency denies chain lineage that references a participating island outside the recovered participation map",
                ));
            }
        }
        for loop_identity in lineage_row.participating_loop_identities() {
            let loop_row = lookup
                .loop_row(loop_identity)
                .expect("validated loop identity should resolve");
            if !loop_row
                .overlap_chain_lineage_identities()
                .contains(&lineage_row.lineage_identity().to_string())
            {
                return Err(deny(
                    Kind::DanglingAdjacencyLineageDenied,
                    lineage_row.lineage_identity(),
                    counters,
                    "overlap adjacency denies chain lineage that is not certified by every participating loop row that claims the neighborhood",
                ));
            }
        }
    }

    for promised_lineage_identity in &promised_lineage_ids {
        if !lookup.has_lineage_identity(promised_lineage_identity) {
            return Err(deny(
                Kind::UnindexedOverlapNeighborhoodDiscoveryDenied,
                promised_lineage_identity,
                counters,
                "overlap adjacency denies participation that promises overlap-chain lineage the adjacency index cannot consume from the admitted chain-lineage map",
            ));
        }
    }

    for component in lookup.neighborhood_components() {
        let connectivity_identity = component.connectivity_identity();
        for lineage_row in component.lineage_rows() {
            if !matches_canonical_membership(
                lineage_row.participating_loop_identities(),
                component.participating_loop_identities(),
            ) || !matches_canonical_membership(
                lineage_row.participating_island_identities(),
                component.participating_island_identities(),
            ) {
                return Err(deny(
                    Kind::ContradictoryAdjacencyNeighborhoodDenied,
                    lineage_row.lineage_identity(),
                    counters,
                    "overlap adjacency denies one chain neighborhood whose admitted lineage rows disagree about certified participating loops or islands",
                ));
            }
        }
        validate_neighborhood_orderability(&lookup, &component, &connectivity_identity, counters)?;
    }

    Ok(lookup)
}

fn is_source_only_boundary_lineage(
    lineage_row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapChainRegionLineageRow,
) -> bool {
    let segment_count = lineage_row.source_loop_identities().len();
    lineage_row.participating_loop_identities().is_empty()
        && lineage_row.participating_island_identities().is_empty()
        && segment_count > 0
        && lineage_row.source_loop_operand_sides().len() == segment_count
        && lineage_row.source_loop_winding_signs().len() == segment_count
        && lineage_row.source_edge_identities().len() == segment_count
        && lineage_row.fragment_identities().len() == segment_count
        && lineage_row.boundary_roles().len() == segment_count
}

fn matches_canonical_membership(
    lineage_membership: &[String],
    component_membership: &[String],
) -> bool {
    let canonical_lineage_membership = lineage_membership
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    canonical_lineage_membership == component_membership
}

fn validate_neighborhood_orderability(
    lookup: &ValidatedOverlapAdjacencyLookup<'_>,
    component: &super::lookup::OverlapAdjacencyNeighborhoodComponent<'_>,
    connectivity_identity: &str,
    counters: &mut PlanarBooleanOverlapAdjacencyIndexCounters,
) -> Result<(), PlanarBooleanOverlapAdjacencyIndexDenial> {
    let loop_rows = component
        .participating_loop_identities()
        .iter()
        .map(|identity| {
            lookup
                .loop_row(identity)
                .expect("validated component loop identity should resolve")
        })
        .collect::<Vec<_>>();
    let island_rows = component
        .participating_island_identities()
        .iter()
        .map(|identity| {
            lookup
                .island_row(identity)
                .expect("validated component island identity should resolve")
        })
        .collect::<Vec<_>>();

    reject_duplicate_keys(
        loop_rows
            .iter()
            .map(|row| loop_order_key(row, connectivity_identity)),
        component.chain_identity(),
        counters,
        "overlap adjacency denies loop neighborhoods whose canonical precedence would depend on incidental iteration order",
    )?;
    reject_duplicate_keys(
        island_rows
            .iter()
            .map(|row| island_order_key(row, connectivity_identity)),
        component.chain_identity(),
        counters,
        "overlap adjacency denies island neighborhoods whose canonical precedence would depend on incidental iteration order",
    )?;
    Ok(())
}

fn reject_duplicate_keys(
    keys: impl Iterator<Item = String>,
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapAdjacencyIndexCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanOverlapAdjacencyIndexDenial> {
    let mut seen = BTreeSet::new();
    for key in keys {
        if !seen.insert(key) {
            return Err(deny(
                Kind::IncidentalIterationOrderTieBreakDenied,
                rejected_identity,
                counters,
                human_reason,
            ));
        }
    }
    Ok(())
}

fn deny(
    kind: Kind,
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapAdjacencyIndexCounters,
    human_reason: &'static str,
) -> PlanarBooleanOverlapAdjacencyIndexDenial {
    counters.denied_neighborhood();
    PlanarBooleanOverlapAdjacencyIndexDenial::new(kind, rejected_identity, *counters, human_reason)
}
