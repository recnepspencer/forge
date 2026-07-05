use std::collections::BTreeMap;

use super::counters::PlanarBooleanOverlapArrangementGraphCounters;
use super::graph::{
    PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow,
    PlanarBooleanOverlapArrangementBoundaryComponentRow,
    PlanarBooleanOverlapArrangementBoundarySegmentRow, PlanarBooleanOverlapArrangementCellRow,
    PlanarBooleanOverlapArrangementCellSet,
};
use super::identity::{
    arrangement_boundary_component_identity, arrangement_boundary_segment_identity,
    arrangement_cell_identity, arrangement_cell_set_identity, arrangement_graph_identity,
    arrangement_neighborhood_identity,
};
use super::input::PlanarBooleanOverlapArrangementGraphInput;
use super::product::PlanarBooleanCoplanarOverlapArrangementGraph;
use super::validation::validate_input;
use super::PlanarBooleanOverlapArrangementGraphDenial;

fn sorted_unique(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

fn sorted_unique_refs(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

pub(crate) fn build_arrangement_graph(
    input: PlanarBooleanOverlapArrangementGraphInput<'_>,
) -> Result<PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapArrangementGraphDenial>
{
    let mut counters = PlanarBooleanOverlapArrangementGraphCounters::default();
    let lookup = validate_input(&input, &mut counters)?;

    let mut graph_rows = Vec::new();
    let mut boundary_component_rows = Vec::new();
    let mut boundary_segment_rows = Vec::new();
    let mut cell_rows = Vec::new();
    let mut arrangement_neighborhood_identities = Vec::new();
    let mut cell_identities = Vec::new();

    for neighborhood in lookup.ordered_neighborhoods() {
        counters.lowered_neighborhood();
        let row = neighborhood.row();
        let provisional_neighborhood_identity = format!(
            "{}:{}",
            input.adjacency_index().request_identity(),
            row.neighborhood_identity()
        );

        let mut segment_identity_by_ordinal = BTreeMap::new();
        for segment in neighborhood.segments() {
            let segment_identity = arrangement_boundary_segment_identity(
                &provisional_neighborhood_identity,
                segment.ordinal,
                segment.source_loop_identity,
                segment.source_edge_identity,
            );
            boundary_segment_rows.push(PlanarBooleanOverlapArrangementBoundarySegmentRow::new(
                segment_identity.clone(),
                row.neighborhood_identity().to_string(),
                segment.source_loop_identity.to_string(),
                segment.operand_side,
                segment.source_loop_winding_sign,
                segment.source_edge_identity.to_string(),
                segment.fragment_identity.to_string(),
                segment.boundary_role,
                segment.ordinal,
            ));
            counters.emitted_boundary_segment();
            segment_identity_by_ordinal.insert(segment.ordinal, segment_identity);
        }

        let mut boundary_component_identities = Vec::new();
        let mut component_identity_by_ordinal = BTreeMap::<usize, String>::new();
        for component in neighborhood.components() {
            let boundary_cycle_identities = component
                .segments
                .iter()
                .map(|segment| {
                    segment_identity_by_ordinal
                        .get(&segment.ordinal)
                        .cloned()
                        .expect("validated arrangement segment ordinal should resolve")
                })
                .collect::<Vec<_>>();
            let component_identity = arrangement_boundary_component_identity(
                &provisional_neighborhood_identity,
                &component
                    .source_loop_identities
                    .iter()
                    .map(|identity| identity.to_string())
                    .collect::<Vec<_>>(),
                component.ordinal,
                &boundary_cycle_identities,
            );
            boundary_component_rows.push(PlanarBooleanOverlapArrangementBoundaryComponentRow::new(
                component_identity.clone(),
                row.neighborhood_identity().to_string(),
                component
                    .source_loop_identities
                    .iter()
                    .map(|identity| identity.to_string())
                    .collect::<Vec<_>>(),
                boundary_cycle_identities,
            ));
            counters.emitted_boundary_component();
            component_identity_by_ordinal.insert(component.ordinal, component_identity.clone());
            boundary_component_identities.push(component_identity);
        }

        let arrangement_identity = arrangement_neighborhood_identity(
            input.adjacency_index().request_identity(),
            row.neighborhood_identity(),
            &boundary_component_identities,
        );
        let canonical_source_loop_identities = neighborhood
            .segments()
            .iter()
            .map(|segment| segment.source_loop_identity.to_string())
            .collect::<Vec<_>>();
        let canonical_source_edge_identities = neighborhood
            .segments()
            .iter()
            .map(|segment| segment.source_edge_identity.to_string())
            .collect::<Vec<_>>();
        let canonical_fragment_identities = neighborhood
            .segments()
            .iter()
            .map(|segment| segment.fragment_identity.to_string())
            .collect::<Vec<_>>();
        let canonical_boundary_roles = neighborhood
            .segments()
            .iter()
            .map(|segment| segment.boundary_role)
            .collect::<Vec<_>>();
        let canonical_persistent_name_identities =
            sorted_unique(row.propagated_persistent_name_identities());

        let mut neighborhood_cell_identities = Vec::new();
        for cell in neighborhood.cells() {
            let component_identities = cell
                .components
                .iter()
                .map(|component| {
                    component_identity_by_ordinal
                        .get(&component.ordinal)
                        .cloned()
                        .expect("validated boundary component ordinal should resolve")
                })
                .collect::<Vec<_>>();
            let boundary_segment_identities =
                sorted_unique_refs(cell.components.iter().flat_map(|component| {
                    component
                        .segments
                        .iter()
                        .map(|segment| {
                            segment_identity_by_ordinal
                                .get(&segment.ordinal)
                                .cloned()
                                .expect("validated arrangement segment ordinal should resolve")
                        })
                        .collect::<Vec<_>>()
                }));
            let source_loop_identities = cell
                .source_loop_identities
                .iter()
                .map(|identity| identity.to_string())
                .collect::<Vec<_>>();
            let cell_identity = arrangement_cell_identity(
                &arrangement_identity,
                &source_loop_identities,
                &component_identities,
            );
            cell_rows.push(PlanarBooleanOverlapArrangementCellRow::new(
                cell_identity.clone(),
                arrangement_identity.clone(),
                row.neighborhood_identity().to_string(),
                source_loop_identities,
                cell.supporting_island_identity.map(str::to_string),
                cell.supporting_island_member_source_loop_identities
                    .iter()
                    .map(|identity| identity.to_string())
                    .collect(),
                cell.supporting_island_member_source_loop_operand_sides
                    .clone(),
                cell.supporting_island_member_source_loop_winding_signs
                    .clone(),
                row.chain_identities().to_vec(),
                row.lineage_identities().to_vec(),
                row.participating_loop_identities().to_vec(),
                row.participating_island_identities().to_vec(),
                component_identities,
                boundary_segment_identities,
                canonical_persistent_name_identities.clone(),
            ));
            counters.emitted_cell();
            neighborhood_cell_identities.push(cell_identity.clone());
            cell_identities.push(cell_identity);
        }

        graph_rows.push(PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow::new(
            arrangement_identity.clone(),
            row.neighborhood_identity().to_string(),
            row.chain_identities().to_vec(),
            row.lineage_identities().to_vec(),
            row.participating_loop_identities().to_vec(),
            row.participating_island_identities().to_vec(),
            boundary_component_identities,
            canonical_source_loop_identities,
            canonical_source_edge_identities,
            canonical_fragment_identities,
            canonical_boundary_roles,
            canonical_persistent_name_identities,
            neighborhood_cell_identities,
        ));
        counters.emitted_graph_row();
        arrangement_neighborhood_identities.push(arrangement_identity);
    }

    let graph_identity = arrangement_graph_identity(
        input.adjacency_index().request_identity(),
        &arrangement_neighborhood_identities,
    );
    let cell_set_identity = arrangement_cell_set_identity(
        input.adjacency_index().request_identity(),
        &graph_identity,
        &cell_identities,
    );
    let cell_set = PlanarBooleanOverlapArrangementCellSet::new(
        cell_set_identity,
        input.adjacency_index().request_identity().to_string(),
        input
            .adjacency_index()
            .adjacency_index_identity()
            .to_string(),
        input.ordering_basis().basis_identity().to_string(),
        cell_rows,
    );

    Ok(PlanarBooleanCoplanarOverlapArrangementGraph::new(
        graph_identity,
        input.adjacency_index().request_identity().to_string(),
        input
            .adjacency_index()
            .adjacency_index_identity()
            .to_string(),
        input.ordering_basis().basis_identity().to_string(),
        graph_rows,
        boundary_component_rows,
        boundary_segment_rows,
        cell_set,
        counters,
    ))
}
