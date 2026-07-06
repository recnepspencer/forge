use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanOverlapCellContainmentEvidenceKind, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingEvidenceKind, PlanarBooleanOverlapCellWindingField,
};

use super::counters::PlanarBooleanSharedAreaAdmissionCounters;
use super::denial::{
    PlanarBooleanSharedAreaAdmissionDenial, PlanarBooleanSharedAreaAdmissionDenialKind as Kind,
};
use super::input::PlanarBooleanSharedAreaAdmissionInput;

pub(super) fn validate_input_identities(
    input: PlanarBooleanSharedAreaAdmissionInput<'_>,
    counters: &mut PlanarBooleanSharedAreaAdmissionCounters,
) -> Result<(), PlanarBooleanSharedAreaAdmissionDenial> {
    let boundary = input.boundary_contact_classification();
    let containment = input.containment_map();
    let winding = input.winding_field();

    if boundary.request_identity() != containment.request_identity()
        || boundary.request_identity() != winding.request_identity()
        || boundary.arrangement_graph_identity() != containment.arrangement_graph_identity()
        || boundary.arrangement_graph_identity() != winding.arrangement_graph_identity()
        || boundary.cell_set_identity() != containment.cell_set_identity()
        || boundary.cell_set_identity() != winding.cell_set_identity()
        || boundary.ordering_basis_identity() != containment.ordering_basis_identity()
        || boundary.ordering_basis_identity() != winding.ordering_basis_identity()
    {
        counters.denied_admission();
        return Err(PlanarBooleanSharedAreaAdmissionDenial::new(
            Kind::InputIdentityMismatchDenied,
            boundary.request_identity(),
            *counters,
            "shared area admission denies any boundary-contact bundle, containment map, and winding field that do not share one admitted identity basis",
        ));
    }

    Ok(())
}

pub(super) fn validate_pure_boundary_absence(
    boundary: &PlanarBooleanBoundaryContactClassificationBundle,
    counters: &mut PlanarBooleanSharedAreaAdmissionCounters,
) -> Result<(), PlanarBooleanSharedAreaAdmissionDenial> {
    let pure_boundary_cells_by_island = boundary.pure_boundary_only_outcomes().rows().iter().fold(
        BTreeMap::<&str, BTreeSet<&str>>::new(),
        |mut grouped, row| {
            let entry = grouped.entry(row.island_identity()).or_default();
            for cell_identity in row.cell_identities() {
                entry.insert(cell_identity);
            }
            grouped
        },
    );
    for component in boundary.area_overlap_components().rows() {
        if pure_boundary_cells_by_island
            .get(component.island_identity())
            .is_some_and(|pure_boundary_cells| {
                component
                    .cell_identities()
                    .iter()
                    .any(|cell_identity| pure_boundary_cells.contains(cell_identity.as_str()))
            })
        {
            counters.denied_admission();
            return Err(PlanarBooleanSharedAreaAdmissionDenial::new(
                Kind::ContradictoryIslandComponentMembershipDenied,
                component.island_identity(),
                *counters,
                "shared area admission denies any locality that simultaneously claims pure-boundary-only and area-overlap cell membership",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_pure_boundary_absence;
    use super::PlanarBooleanSharedAreaAdmissionCounters;
    use crate::workload_platform::planar_boolean_overlap_region_extraction::{
        PlanarBooleanAreaOverlapComponentRow, PlanarBooleanAreaOverlapComponentSet,
        PlanarBooleanBoundaryContactClassificationBundle,
        PlanarBooleanBoundaryContactClassificationCounters,
        PlanarBooleanPureBoundaryOnlyOutcomeRow, PlanarBooleanPureBoundaryOnlyOutcomeSet,
        PlanarBooleanSharedBoundaryContactOutcomeSet,
    };

    fn boundary_bundle(
        pure_boundary_cells: Vec<&str>,
        area_cells: Vec<&str>,
    ) -> PlanarBooleanBoundaryContactClassificationBundle {
        PlanarBooleanBoundaryContactClassificationBundle::new(
            "boundary-bundle".to_string(),
            PlanarBooleanSharedBoundaryContactOutcomeSet::new(
                "shared-boundary".to_string(),
                "request".to_string(),
                "arrangement".to_string(),
                "cell-set".to_string(),
                "order".to_string(),
                Vec::new(),
            ),
            PlanarBooleanPureBoundaryOnlyOutcomeSet::new(
                "pure-boundary".to_string(),
                "request".to_string(),
                "arrangement".to_string(),
                "cell-set".to_string(),
                "order".to_string(),
                vec![PlanarBooleanPureBoundaryOnlyOutcomeRow::new(
                    "pure-row".to_string(),
                    "island".to_string(),
                    "neighborhood".to_string(),
                    vec!["boundary-contact-component".to_string()],
                    pure_boundary_cells
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                    vec!["boundary-component".to_string()],
                    vec!["boundary-segment".to_string()],
                    vec!["loop".to_string()],
                )],
            ),
            PlanarBooleanAreaOverlapComponentSet::new(
                "area-components".to_string(),
                "request".to_string(),
                "arrangement".to_string(),
                "cell-set".to_string(),
                "order".to_string(),
                vec![PlanarBooleanAreaOverlapComponentRow::new(
                    "area-component".to_string(),
                    "island".to_string(),
                    "neighborhood".to_string(),
                    area_cells.into_iter().map(str::to_string).collect(),
                    vec!["boundary-component".to_string()],
                    vec!["boundary-segment".to_string()],
                    vec!["loop".to_string()],
                )],
            ),
            PlanarBooleanBoundaryContactClassificationCounters::default(),
        )
    }

    #[test]
    fn pure_boundary_locality_may_coexist_with_disjoint_area_cells() {
        let bundle = boundary_bundle(vec!["boundary-cell"], vec!["area-cell"]);
        let mut counters = PlanarBooleanSharedAreaAdmissionCounters::default();

        let result = validate_pure_boundary_absence(&bundle, &mut counters);

        assert!(result.is_ok());
    }

    #[test]
    fn overlapping_boundary_and_area_cells_still_deny() {
        let bundle = boundary_bundle(vec!["shared-cell"], vec!["shared-cell"]);
        let mut counters = PlanarBooleanSharedAreaAdmissionCounters::default();

        let result = validate_pure_boundary_absence(&bundle, &mut counters);

        assert!(result.is_err());
    }
}

pub(super) fn validate_area_component_cell_proof(
    boundary: &PlanarBooleanBoundaryContactClassificationBundle,
    containment: &PlanarBooleanOverlapCellContainmentMap,
    winding: &PlanarBooleanOverlapCellWindingField,
    counters: &mut PlanarBooleanSharedAreaAdmissionCounters,
) -> Result<(), PlanarBooleanSharedAreaAdmissionDenial> {
    let containment_by_cell =
        containment
            .rows()
            .iter()
            .fold(BTreeMap::<&str, Vec<_>>::new(), |mut acc, row| {
                acc.entry(row.cell_identity()).or_default().push(row);
                acc
            });
    let winding_by_cell =
        winding
            .rows()
            .iter()
            .fold(BTreeMap::<&str, Vec<_>>::new(), |mut acc, row| {
                acc.entry(row.cell_identity()).or_default().push(row);
                acc
            });

    for component in boundary.area_overlap_components().rows() {
        let component_loops = component
            .source_loop_identities()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for cell_identity in component.cell_identities() {
            let containment_supported = containment_by_cell
                .get(cell_identity.as_str())
                .into_iter()
                .flatten()
                .any(|row| {
                    row.evidence_kind() == PlanarBooleanOverlapCellContainmentEvidenceKind::Inside
                        && row.neighborhood_identity() == component.neighborhood_identity()
                        && row
                            .source_loop_identities()
                            .iter()
                            .any(|identity| component_loops.contains(identity.as_str()))
                });
            let winding_supported = winding_by_cell
                .get(cell_identity.as_str())
                .into_iter()
                .flatten()
                .any(|row| area_component_has_winding_support(row, component, &component_loops));

            if !containment_supported || !winding_supported {
                counters.denied_admission();
                return Err(PlanarBooleanSharedAreaAdmissionDenial::new(
                    Kind::AreaComponentMissingSupportingCellProofDenied,
                    cell_identity,
                    *counters,
                    "shared area admission denies any area-overlap component whose cells do not carry matching containment-inside and nonzero supporting-island winding proof",
                ));
            }
        }
    }

    Ok(())
}

fn area_component_has_winding_support(
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellWindingRow,
    component: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanAreaOverlapComponentRow,
    component_loops: &BTreeSet<&str>,
) -> bool {
    if row.winding_number() == 0
        || row.neighborhood_identity() != component.neighborhood_identity()
        || !row
            .source_loop_identities()
            .iter()
            .any(|identity| component_loops.contains(identity.as_str()))
    {
        return false;
    }

    matches!(
        row.evidence_kind(),
        PlanarBooleanOverlapCellWindingEvidenceKind::SupportingIslandTopology
            | PlanarBooleanOverlapCellWindingEvidenceKind::BoundaryTopologyAndSupportingIslandTopology
    ) || (row.evidence_kind() == PlanarBooleanOverlapCellWindingEvidenceKind::BoundaryTopology
        && row.supporting_island_identity().is_some())
}

pub(super) fn mixed_boundary_components_by_island(
    boundary: &PlanarBooleanBoundaryContactClassificationBundle,
) -> BTreeMap<&str, Vec<&str>> {
    boundary
        .shared_boundary_contact_outcomes()
        .rows()
        .iter()
        .fold(BTreeMap::<&str, Vec<&str>>::new(), |mut acc, row| {
            acc.entry(row.island_identity())
                .or_default()
                .push(row.boundary_contact_component_identity());
            acc
        })
}

pub(super) fn deny_mixed_island(
    island_identity: &str,
    counters: &mut PlanarBooleanSharedAreaAdmissionCounters,
) -> PlanarBooleanSharedAreaAdmissionDenial {
    counters.denied_admission();
    PlanarBooleanSharedAreaAdmissionDenial::new(
        Kind::MixedBoundaryAreaRequiresCellDecompositionDenied,
        island_identity,
        *counters,
        "shared area admission denies any mixed boundary-and-area island whose admitted cells still overlap boundary-contact locality and therefore require deeper decomposition before overlap-region promotion",
    )
}
