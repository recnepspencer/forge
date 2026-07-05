use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};

use super::counters::PlanarBooleanOverlapCellClassificationCounters;
use super::denial::PlanarBooleanOverlapCellClassificationDenial;
use super::input::PlanarBooleanOverlapCellContainmentInput;
use super::product::PlanarBooleanOverlapCellContainmentMap;
use super::rows::{
    PlanarBooleanOverlapCellContainmentEvidenceKind::{BoundaryOnly, Inside, Outside},
    PlanarBooleanOverlapCellContainmentRow,
};
use super::validation::validate_containment_input;

pub(crate) fn build_containment_map(
    input: PlanarBooleanOverlapCellContainmentInput<'_>,
) -> Result<PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellClassificationDenial> {
    let mut counters = PlanarBooleanOverlapCellClassificationCounters::default();
    let lookup = validate_containment_input(&input, &mut counters)?;
    let graph = input.arrangement_graph();
    let mut rows = Vec::new();

    for validated in lookup.cells() {
        counters.classified_cell();
        for operand_side in [Left, Right] {
            let boundary_winding_sum = match operand_side {
                Left => validated.left_boundary_winding_sum,
                Right => validated.right_boundary_winding_sum,
            };
            let supporting_winding_sum = match operand_side {
                Left => validated.left_supporting_winding_sum,
                Right => validated.right_supporting_winding_sum,
            };
            let evidence_kind = if validated.cell.supporting_island_identity().is_some() {
                if boundary_winding_sum + supporting_winding_sum != 0 {
                    Inside
                } else if boundary_winding_sum != 0
                    || validated.boundary_operand_sides.contains(&operand_side)
                {
                    BoundaryOnly
                } else {
                    Outside
                }
            } else if boundary_winding_sum != 0
                || validated.boundary_operand_sides.contains(&operand_side)
            {
                BoundaryOnly
            } else {
                Outside
            };
            rows.push(PlanarBooleanOverlapCellContainmentRow::new(
                validated.cell.cell_identity().to_string(),
                validated.cell.arrangement_identity().to_string(),
                validated.cell.neighborhood_identity().to_string(),
                operand_side,
                validated
                    .cell
                    .supporting_island_identity()
                    .map(str::to_string),
                validated.cell.source_loop_identities().to_vec(),
                evidence_kind,
            ));
            counters.emitted_containment_row();
        }
    }

    Ok(PlanarBooleanOverlapCellContainmentMap::new(
        containment_map_identity(graph.request_identity(), graph.arrangement_graph_identity()),
        graph.request_identity().to_string(),
        graph.arrangement_graph_identity().to_string(),
        graph.cell_set().cell_set_identity().to_string(),
        graph.ordering_basis_identity().to_string(),
        rows,
        counters,
    ))
}

fn containment_map_identity(request_identity: &str, arrangement_graph_identity: &str) -> String {
    format!("overlap-arrangement:containment-map:{request_identity}:{arrangement_graph_identity}")
}
