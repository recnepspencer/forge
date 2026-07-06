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
    let ordering_basis_identity = input
        .overlap_islands()
        .ordering_basis_identity()
        .to_string();

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
    let area_cells_by_island = input.area_overlap_components().rows().iter().fold(
        BTreeMap::<&str, BTreeSet<&str>>::new(),
        |mut grouped, row| {
            let entry = grouped.entry(row.island_identity()).or_default();
            for cell_identity in row.cell_identities() {
                entry.insert(cell_identity);
            }
            grouped
        },
    );

    let pure_boundary_rows = input
        .overlap_islands()
        .rows()
        .iter()
        .filter_map(|island| {
            let mut boundary_contact_component_identities = BTreeSet::new();
            let mut cell_identities = BTreeSet::new();
            let mut boundary_component_identities = BTreeSet::new();
            let mut boundary_segment_identities = BTreeSet::new();
            let mut source_loop_identities = BTreeSet::new();
            let area_cells = area_cells_by_island.get(island.island_identity());
            for component_identity in island.boundary_contact_component_identities() {
                if let Some(shared_boundary_row) =
                    shared_boundary_by_component.get(component_identity.as_str())
                {
                    if !boundary_component_is_pure_boundary_only(shared_boundary_row, area_cells) {
                        continue;
                    }
                    boundary_contact_component_identities.insert(
                        shared_boundary_row
                            .boundary_contact_component_identity()
                            .to_string(),
                    );
                    cell_identities.extend(shared_boundary_row.cell_identities().iter().cloned());
                    boundary_component_identities.extend(
                        shared_boundary_row
                            .boundary_component_identities()
                            .iter()
                            .cloned(),
                    );
                    boundary_segment_identities.extend(
                        shared_boundary_row
                            .boundary_segment_identities()
                            .iter()
                            .cloned(),
                    );
                    source_loop_identities
                        .extend(shared_boundary_row.source_loop_identities().iter().cloned());
                }
            }
            if boundary_contact_component_identities.is_empty() {
                return None;
            }
            counters.admitted_pure_boundary_only_outcome();
            Some(PlanarBooleanPureBoundaryOnlyOutcomeRow::new(
                pure_boundary_only_outcome_identity(&request_identity, island.island_identity()),
                island.island_identity().to_string(),
                island.neighborhood_identity().to_string(),
                boundary_contact_component_identities.into_iter().collect(),
                cell_identities.into_iter().collect(),
                boundary_component_identities.into_iter().collect(),
                boundary_segment_identities.into_iter().collect(),
                source_loop_identities.into_iter().collect(),
            ))
        })
        .collect::<Vec<_>>();

    let shared_boundary_contact_outcomes = PlanarBooleanSharedBoundaryContactOutcomeSet::new(
        outcome_set_identity(
            &request_identity,
            "shared-boundary",
            shared_boundary_rows.len(),
        ),
        request_identity.clone(),
        arrangement_graph_identity.clone(),
        cell_set_identity.clone(),
        ordering_basis_identity.clone(),
        shared_boundary_rows,
    );
    let pure_boundary_only_outcomes = PlanarBooleanPureBoundaryOnlyOutcomeSet::new(
        outcome_set_identity(
            &request_identity,
            "pure-boundary-only",
            pure_boundary_rows.len(),
        ),
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

fn boundary_component_is_pure_boundary_only(
    row: &PlanarBooleanSharedBoundaryContactOutcomeRow,
    area_cells: Option<&BTreeSet<&str>>,
) -> bool {
    area_cells.is_none_or(|cells| {
        row.cell_identities()
            .iter()
            .all(|cell_identity| !cells.contains(cell_identity.as_str()))
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::boundary_component_is_pure_boundary_only;
    use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanSharedBoundaryContactOutcomeRow;

    fn boundary_row(cell_identities: &[&str]) -> PlanarBooleanSharedBoundaryContactOutcomeRow {
        PlanarBooleanSharedBoundaryContactOutcomeRow::new(
            "boundary-outcome".to_string(),
            "island".to_string(),
            "neighborhood".to_string(),
            "boundary-component".to_string(),
            cell_identities
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            vec!["boundary".to_string()],
            vec!["segment".to_string()],
            vec!["loop".to_string()],
        )
    }

    #[test]
    fn boundary_component_remains_pure_when_island_area_cells_are_disjoint() {
        let row = boundary_row(&["boundary-a", "boundary-b"]);
        let area_cells = BTreeSet::from(["area-a", "area-b"]);

        assert!(boundary_component_is_pure_boundary_only(
            &row,
            Some(&area_cells)
        ));
    }

    #[test]
    fn boundary_component_is_not_pure_when_cells_overlap_area_locality() {
        let row = boundary_row(&["shared-cell", "boundary-b"]);
        let area_cells = BTreeSet::from(["shared-cell", "area-b"]);

        assert!(!boundary_component_is_pure_boundary_only(
            &row,
            Some(&area_cells)
        ));
    }
}
