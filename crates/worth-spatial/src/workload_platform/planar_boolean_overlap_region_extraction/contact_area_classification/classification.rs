use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanBoundaryContactClassificationCounters;
use super::denial::PlanarBooleanBoundaryContactClassificationDenial;
use super::identity::{
    outcome_set_identity, pure_boundary_only_outcome_identity,
    shared_boundary_contact_outcome_identity,
};
use super::input::PlanarBooleanBoundaryContactClassificationInput;
use super::product::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanPureBoundaryOnlyOutcomeSet,
    PlanarBooleanSharedBoundaryContactOutcomeSet,
};
use super::rows::{
    PlanarBooleanPureBoundaryOnlyOutcomeRow, PlanarBooleanSharedBoundaryContactOutcomeRow,
};
use super::validation::{validate_component_membership, validate_input_identities};

pub(super) fn build_boundary_contact_classification_bundle(
    input: PlanarBooleanBoundaryContactClassificationInput<'_>,
) -> Result<
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanBoundaryContactClassificationDenial,
> {
    let mut counters = PlanarBooleanBoundaryContactClassificationCounters::default();
    validate_input_identities(input, &mut counters)?;
    validate_component_membership(
        input.overlap_islands(),
        input.boundary_contact_components(),
        input.area_overlap_components(),
        &mut counters,
    )?;

    let request_identity = input.overlap_islands().request_identity().to_string();
    let arrangement_graph_identity = input
        .overlap_islands()
        .arrangement_graph_identity()
        .to_string();
    let cell_set_identity = input.overlap_islands().cell_set_identity().to_string();
    let ordering_basis_identity = input.overlap_islands().ordering_basis_identity().to_string();

    let shared_boundary_rows = input
        .boundary_contact_components()
        .rows()
        .iter()
        .map(|component| {
            counters.admitted_shared_boundary_contact_outcome();
            PlanarBooleanSharedBoundaryContactOutcomeRow::new(
                shared_boundary_contact_outcome_identity(
                    &request_identity,
                    component.component_identity(),
                ),
                component.island_identity().to_string(),
                component.neighborhood_identity().to_string(),
                component.component_identity().to_string(),
                component.cell_identities().to_vec(),
                component.boundary_component_identities().to_vec(),
                component.boundary_segment_identities().to_vec(),
                component.source_loop_identities().to_vec(),
            )
        })
        .collect::<Vec<_>>();
    let shared_boundary_by_component = shared_boundary_rows
        .iter()
        .map(|row| (row.boundary_contact_component_identity(), row))
        .collect::<BTreeMap<_, _>>();

    let pure_boundary_rows = input
        .overlap_islands()
        .rows()
        .iter()
        .filter(|island| {
            !island.boundary_contact_component_identities().is_empty()
                && island.area_overlap_component_identities().is_empty()
        })
        .map(|island| {
            let mut boundary_component_identities = BTreeSet::new();
            let mut boundary_segment_identities = BTreeSet::new();
            let mut source_loop_identities = BTreeSet::new();
            for component_identity in island.boundary_contact_component_identities() {
                if let Some(shared_boundary_row) =
                    shared_boundary_by_component.get(component_identity.as_str())
                {
                    boundary_component_identities
                        .extend(shared_boundary_row.boundary_component_identities().iter().cloned());
                    boundary_segment_identities
                        .extend(shared_boundary_row.boundary_segment_identities().iter().cloned());
                    source_loop_identities
                        .extend(shared_boundary_row.source_loop_identities().iter().cloned());
                }
            }
            counters.admitted_pure_boundary_only_outcome();
            PlanarBooleanPureBoundaryOnlyOutcomeRow::new(
                pure_boundary_only_outcome_identity(&request_identity, island.island_identity()),
                island.island_identity().to_string(),
                island.neighborhood_identity().to_string(),
                island.boundary_contact_component_identities().to_vec(),
                island.cell_identities().to_vec(),
                boundary_component_identities.into_iter().collect(),
                boundary_segment_identities.into_iter().collect(),
                source_loop_identities.into_iter().collect(),
            )
        })
        .collect::<Vec<_>>();

    let shared_boundary_contact_outcomes = PlanarBooleanSharedBoundaryContactOutcomeSet::new(
        outcome_set_identity(&request_identity, "shared-boundary", shared_boundary_rows.len()),
        request_identity.clone(),
        arrangement_graph_identity.clone(),
        cell_set_identity.clone(),
        ordering_basis_identity.clone(),
        shared_boundary_rows,
    );
    let pure_boundary_only_outcomes = PlanarBooleanPureBoundaryOnlyOutcomeSet::new(
        outcome_set_identity(&request_identity, "pure-boundary-only", pure_boundary_rows.len()),
        request_identity.clone(),
        arrangement_graph_identity,
        cell_set_identity,
        ordering_basis_identity,
        pure_boundary_rows,
    );

    Ok(PlanarBooleanBoundaryContactClassificationBundle::new(
        format!(
            "boundary-contact-classification:{}:{}",
            shared_boundary_contact_outcomes.rows().len(),
            pure_boundary_only_outcomes.rows().len()
        ),
        shared_boundary_contact_outcomes,
        pure_boundary_only_outcomes,
        input.area_overlap_components().clone(),
        counters,
    ))
}
