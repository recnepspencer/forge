use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanSharedAreaAdmissionCounters;
use super::denial::PlanarBooleanSharedAreaAdmissionDenial;
use super::identity::{
    mixed_boundary_area_outcome_identity, outcome_set_identity,
    shared_area_admission_outcome_identity,
};
use super::input::PlanarBooleanSharedAreaAdmissionInput;
use super::product::{
    PlanarBooleanMixedBoundaryAreaOutcomeSet, PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanSharedAreaAdmissionOutcomeSet,
};
use super::rows::{
    PlanarBooleanMixedBoundaryAreaOutcomeRow, PlanarBooleanSharedAreaAdmissionOutcomeRow,
};
use super::validation::{
    deny_mixed_island, mixed_boundary_components_by_island, validate_area_component_cell_proof,
    validate_input_identities, validate_pure_boundary_absence,
};

pub(super) fn build_shared_area_admission_bundle(
    input: PlanarBooleanSharedAreaAdmissionInput<'_>,
) -> Result<PlanarBooleanSharedAreaAdmissionBundle, PlanarBooleanSharedAreaAdmissionDenial> {
    let mut counters = PlanarBooleanSharedAreaAdmissionCounters::default();
    validate_input_identities(input, &mut counters)?;
    validate_pure_boundary_absence(input.boundary_contact_classification(), &mut counters)?;
    validate_area_component_cell_proof(
        input.boundary_contact_classification(),
        input.containment_map(),
        input.winding_field(),
        &mut counters,
    )?;

    let boundary = input.boundary_contact_classification();
    let request_identity = boundary.request_identity().to_string();
    let arrangement_graph_identity = boundary.arrangement_graph_identity().to_string();
    let cell_set_identity = boundary.cell_set_identity().to_string();
    let ordering_basis_identity = boundary.ordering_basis_identity().to_string();
    let mixed_boundary_by_island = mixed_boundary_components_by_island(boundary);
    let boundary_cells_by_island = boundary
        .shared_boundary_contact_outcomes()
        .rows()
        .iter()
        .fold(BTreeMap::<&str, BTreeSet<&str>>::new(), |mut acc, row| {
            let entry = acc.entry(row.island_identity()).or_default();
            for cell_identity in row.cell_identities() {
                entry.insert(cell_identity);
            }
            acc
        });

    let mut shared_area_rows = Vec::new();
    let mut mixed_rows = Vec::new();
    let mut emitted_mixed_islands = BTreeSet::new();

    for component in boundary.area_overlap_components().rows() {
        let island_identity = component.island_identity();
        if let Some(boundary_component_identities) = mixed_boundary_by_island.get(island_identity) {
            let area_cells = component
                .cell_identities()
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>();
            if boundary_cells_by_island
                .get(island_identity)
                .into_iter()
                .flatten()
                .any(|cell_identity| area_cells.contains(cell_identity))
            {
                return Err(deny_mixed_island(island_identity, &mut counters));
            }

            if emitted_mixed_islands.insert(island_identity.to_string()) {
                counters.admitted_mixed_boundary_area_outcome();
                let component_ids = boundary
                    .area_overlap_components()
                    .rows()
                    .iter()
                    .filter(|row| row.island_identity() == island_identity)
                    .map(|row| row.component_identity().to_string())
                    .collect::<Vec<_>>();
                let cell_identities = boundary
                    .area_overlap_components()
                    .rows()
                    .iter()
                    .filter(|row| row.island_identity() == island_identity)
                    .flat_map(|row| row.cell_identities().iter().cloned())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                mixed_rows.push(PlanarBooleanMixedBoundaryAreaOutcomeRow::new(
                    mixed_boundary_area_outcome_identity(&request_identity, island_identity),
                    island_identity.to_string(),
                    component.neighborhood_identity().to_string(),
                    component_ids,
                    boundary_component_identities
                        .iter()
                        .map(|identity| (*identity).to_string())
                        .collect(),
                    cell_identities,
                ));
            }
            continue;
        }

        counters.admitted_shared_area_outcome();
        shared_area_rows.push(PlanarBooleanSharedAreaAdmissionOutcomeRow::new(
            shared_area_admission_outcome_identity(&request_identity, component.component_identity()),
            component.island_identity().to_string(),
            component.neighborhood_identity().to_string(),
            component.component_identity().to_string(),
            component.cell_identities().to_vec(),
            component.boundary_component_identities().to_vec(),
            component.boundary_segment_identities().to_vec(),
            component.source_loop_identities().to_vec(),
        ));
    }

    let shared_area_admission_outcomes = PlanarBooleanSharedAreaAdmissionOutcomeSet::new(
        outcome_set_identity(&request_identity, "shared-area", shared_area_rows.len()),
        request_identity.clone(),
        arrangement_graph_identity.clone(),
        cell_set_identity.clone(),
        ordering_basis_identity.clone(),
        shared_area_rows,
    );
    let mixed_boundary_area_outcomes = PlanarBooleanMixedBoundaryAreaOutcomeSet::new(
        outcome_set_identity(&request_identity, "mixed-boundary-area", mixed_rows.len()),
        request_identity.clone(),
        arrangement_graph_identity,
        cell_set_identity,
        ordering_basis_identity,
        mixed_rows,
    );

    Ok(PlanarBooleanSharedAreaAdmissionBundle::new(
        format!(
            "shared-area-admission:{}:{}",
            shared_area_admission_outcomes.rows().len(),
            mixed_boundary_area_outcomes.rows().len()
        ),
        shared_area_admission_outcomes,
        mixed_boundary_area_outcomes,
        boundary.pure_boundary_only_outcomes().clone(),
        counters,
    ))
}
