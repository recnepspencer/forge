use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellContainmentEvidenceKind::BoundaryOnly;

use super::counters::PlanarBooleanOverlapIslandComponentCounters;
use super::identity::{candidate_identity, candidate_set_identity, island_identity};
use super::input::PlanarBooleanOverlapIslandCandidateInput;
use super::lookup::PlanarBooleanOverlapCellEvidenceLookup;
use super::product::PlanarBooleanOverlapIslandCandidateSet;
use super::rows::{PlanarBooleanOverlapIslandCandidateKind as Kind, PlanarBooleanOverlapIslandCandidateRow};
use super::validation::{contradictory, unsupported, validate_cell_overlap_signal, validate_input_identities};
use super::PlanarBooleanOverlapIslandComponentDenial;

pub(super) fn build_island_candidate_set(
    input: PlanarBooleanOverlapIslandCandidateInput<'_>,
) -> Result<PlanarBooleanOverlapIslandCandidateSet, PlanarBooleanOverlapIslandComponentDenial> {
    let mut counters = PlanarBooleanOverlapIslandComponentCounters::default();
    validate_input_identities(input, &mut counters)?;
    let arrangement = input.arrangement_graph();
    let lookup = PlanarBooleanOverlapCellEvidenceLookup::default()
        .with_containment_rows(input.containment_map().rows())
        .with_winding_rows(input.winding_field().rows());

    let mut rows = Vec::new();
    for cell in arrangement.cell_set().cells() {
        validate_cell_overlap_signal(cell.cell_identity(), &lookup, &mut counters)?;
        let left_containment = lookup
            .containment_row(cell.cell_identity(), Left)
            .expect("validated left containment row should exist");
        let right_containment = lookup
            .containment_row(cell.cell_identity(), Right)
            .expect("validated right containment row should exist");
        let left_winding = lookup
            .winding_row(cell.cell_identity(), Left)
            .expect("validated left winding row should exist");
        let right_winding = lookup
            .winding_row(cell.cell_identity(), Right)
            .expect("validated right winding row should exist");
        let left_inside = left_winding.winding_number() != 0;
        let right_inside = right_winding.winding_number() != 0;
        let has_boundary_contact = left_containment.evidence_kind() == BoundaryOnly
            || right_containment.evidence_kind() == BoundaryOnly;

        let kind = match (left_inside, right_inside, has_boundary_contact) {
            (true, true, false) | (true, true, true) => Kind::AreaOverlap,
            (false, false, true) => Kind::BoundaryContact,
            (true, false, true) | (false, true, true) => Kind::BoundaryContact,
            (true, false, false) | (false, true, false) => {
                return Err(contradictory(cell.cell_identity(), &mut counters));
            }
            (false, false, false) => {
                return Err(unsupported(cell.cell_identity(), &mut counters));
            }
        };

        rows.push(PlanarBooleanOverlapIslandCandidateRow::new(
            candidate_identity(cell.cell_identity(), kind),
            island_identity(cell.neighborhood_identity()),
            cell.cell_identity().to_string(),
            cell.neighborhood_identity().to_string(),
            cell.boundary_component_identities().to_vec(),
            cell.boundary_segment_identities().to_vec(),
            cell.source_loop_identities().to_vec(),
            cell.propagated_persistent_name_identities().to_vec(),
            kind,
        ));
        counters.admitted_candidate();
    }

    Ok(PlanarBooleanOverlapIslandCandidateSet::new(
        candidate_set_identity(
            arrangement.request_identity(),
            rows.iter().map(|row| row.candidate_identity().to_string()),
        ),
        arrangement.request_identity().to_string(),
        arrangement.arrangement_graph_identity().to_string(),
        arrangement.cell_set().cell_set_identity().to_string(),
        arrangement.ordering_basis_identity().to_string(),
        rows,
        counters,
    ))
}
