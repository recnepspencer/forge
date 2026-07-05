use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAreaOverlapComponentSet, PlanarBooleanBoundaryContactComponentSet,
    PlanarBooleanOverlapIslandSet,
};

use super::counters::PlanarBooleanBoundaryContactClassificationCounters;
use super::denial::{
    PlanarBooleanBoundaryContactClassificationDenial,
    PlanarBooleanBoundaryContactClassificationDenialKind as Kind,
};
use super::input::PlanarBooleanBoundaryContactClassificationInput;

pub(super) fn validate_input_identities(
    input: PlanarBooleanBoundaryContactClassificationInput<'_>,
    counters: &mut PlanarBooleanBoundaryContactClassificationCounters,
) -> Result<(), PlanarBooleanBoundaryContactClassificationDenial> {
    let overlap_islands = input.overlap_islands();
    let boundary_components = input.boundary_contact_components();
    let area_components = input.area_overlap_components();

    if overlap_islands.request_identity() != boundary_components.request_identity()
        || overlap_islands.request_identity() != area_components.request_identity()
        || overlap_islands.arrangement_graph_identity()
            != boundary_components.arrangement_graph_identity()
        || overlap_islands.arrangement_graph_identity()
            != area_components.arrangement_graph_identity()
        || overlap_islands.cell_set_identity() != boundary_components.cell_set_identity()
        || overlap_islands.cell_set_identity() != area_components.cell_set_identity()
        || overlap_islands.ordering_basis_identity() != boundary_components.ordering_basis_identity()
        || overlap_islands.ordering_basis_identity() != area_components.ordering_basis_identity()
    {
        counters.denied_classification();
        return Err(PlanarBooleanBoundaryContactClassificationDenial::new(
            Kind::InputIdentityMismatchDenied,
            overlap_islands.request_identity(),
            *counters,
            "boundary contact classification denies island and component products that do not share one admitted identity basis",
        ));
    }
    Ok(())
}

pub(super) fn validate_component_membership(
    overlap_islands: &PlanarBooleanOverlapIslandSet,
    boundary_components: &PlanarBooleanBoundaryContactComponentSet,
    area_components: &PlanarBooleanAreaOverlapComponentSet,
    counters: &mut PlanarBooleanBoundaryContactClassificationCounters,
) -> Result<(), PlanarBooleanBoundaryContactClassificationDenial> {
    let mut boundary_by_island = BTreeMap::<&str, BTreeSet<&str>>::new();
    for row in boundary_components.rows() {
        boundary_by_island
            .entry(row.island_identity())
            .or_default()
            .insert(row.component_identity());
    }

    let mut area_by_island = BTreeMap::<&str, BTreeSet<&str>>::new();
    for row in area_components.rows() {
        area_by_island
            .entry(row.island_identity())
            .or_default()
            .insert(row.component_identity());
    }

    for island in overlap_islands.rows() {
        let expected_boundary = island
            .boundary_contact_component_identities()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let actual_boundary = boundary_by_island
            .get(island.island_identity())
            .cloned()
            .unwrap_or_default();
        if expected_boundary != actual_boundary {
            return Err(contradictory(island.island_identity(), counters));
        }

        let expected_area = island
            .area_overlap_component_identities()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let actual_area = area_by_island
            .get(island.island_identity())
            .cloned()
            .unwrap_or_default();
        if expected_area != actual_area {
            return Err(mixed(island.island_identity(), counters));
        }
    }

    Ok(())
}

fn contradictory(
    island_identity: &str,
    counters: &mut PlanarBooleanBoundaryContactClassificationCounters,
) -> PlanarBooleanBoundaryContactClassificationDenial {
    counters.denied_classification();
    PlanarBooleanBoundaryContactClassificationDenial::new(
        Kind::ContradictoryIslandComponentMembershipDenied,
        island_identity,
        *counters,
        "boundary contact classification denies islands whose component membership does not agree with the admitted phase-seven partition",
    )
}

pub(super) fn mixed(
    island_identity: &str,
    counters: &mut PlanarBooleanBoundaryContactClassificationCounters,
) -> PlanarBooleanBoundaryContactClassificationDenial {
    counters.denied_classification();
    PlanarBooleanBoundaryContactClassificationDenial::new(
        Kind::MixedBoundaryAreaRequiresCellDecompositionDenied,
        island_identity,
        *counters,
        "boundary contact classification denies any path that would hide a mixed boundary-and-area island behind a pure-boundary-only outcome",
    )
}
