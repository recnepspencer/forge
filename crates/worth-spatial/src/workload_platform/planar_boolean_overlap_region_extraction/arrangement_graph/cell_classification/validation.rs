use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};

use super::counters::PlanarBooleanOverlapCellClassificationCounters;
use super::denial::{
    PlanarBooleanOverlapCellClassificationDenial,
    PlanarBooleanOverlapCellClassificationDenialKind as Kind,
};
use super::input::{
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellWindingFieldInput,
};
use super::lookup::{
    boundary_segments_by_identity, ValidatedCellClassification, ValidatedCellClassificationLookup,
};

pub(crate) fn validate_containment_input<'a>(
    input: &'a PlanarBooleanOverlapCellContainmentInput<'a>,
    counters: &mut PlanarBooleanOverlapCellClassificationCounters,
) -> Result<ValidatedCellClassificationLookup<'a>, PlanarBooleanOverlapCellClassificationDenial> {
    let graph = input.arrangement_graph();
    let boundary_segments_by_identity = boundary_segments_by_identity(graph.boundary_segments());
    let mut validated_cells = Vec::new();

    for cell in graph.cell_set().cells() {
        let boundary_segments = cell
            .boundary_segment_identities()
            .iter()
            .map(|segment_identity| {
                boundary_segments_by_identity
                    .get(segment_identity.as_str())
                    .copied()
                    .ok_or_else(|| {
                        deny(
                            Kind::MissingOperandContainmentEvidenceDenied,
                            cell.cell_identity(),
                            counters,
                            "overlap cell containment requires every arrangement cell to resolve its certified boundary segments",
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut boundary_operand_sides = boundary_segments
            .iter()
            .map(
                |segment: &&crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::PlanarBooleanOverlapArrangementBoundarySegmentRow| {
                    segment.operand_side()
                },
            )
            .collect::<Vec<_>>();
        boundary_operand_sides.sort_by_key(|side: &crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide| side.query_key());
        boundary_operand_sides.dedup();
        let left_boundary_winding_sum = boundary_segments
            .iter()
            .filter(|segment| segment.operand_side() == Left)
            .map(|segment| i16::from(segment.source_loop_winding_sign()))
            .sum::<i16>();
        let right_boundary_winding_sum = boundary_segments
            .iter()
            .filter(|segment| segment.operand_side() == Right)
            .map(|segment| i16::from(segment.source_loop_winding_sign()))
            .sum::<i16>();

        let supporting_island_operand_sides = cell
            .supporting_island_member_source_loop_operand_sides()
            .to_vec();
        let supporting_island_winding_signs = cell
            .supporting_island_member_source_loop_winding_signs()
            .to_vec();
        let left_supporting_winding_sum = supporting_island_operand_sides
            .iter()
            .copied()
            .zip(supporting_island_winding_signs.iter().copied())
            .filter(|(side, _)| *side == Left)
            .map(|(_, winding_sign)| i16::from(winding_sign))
            .sum::<i16>();
        let right_supporting_winding_sum = supporting_island_operand_sides
            .iter()
            .copied()
            .zip(supporting_island_winding_signs.iter().copied())
            .filter(|(side, _)| *side == Right)
            .map(|(_, winding_sign)| i16::from(winding_sign))
            .sum::<i16>();

        if cell.supporting_island_identity().is_some()
            && cell
                .supporting_island_member_source_loop_identities()
                .is_empty()
        {
            return Err(deny(
                Kind::MissingOperandContainmentEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell containment denies area-bearing cells whose certified supporting-island witness carries no member source-loop authority",
            ));
        }
        if cell.supporting_island_identity().is_some()
            && cell.supporting_island_member_source_loop_identities().len()
                != supporting_island_operand_sides.len()
        {
            return Err(deny(
                Kind::ContradictoryOperandContainmentEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell containment denies supporting-island witnesses whose member source-loop identities and operand sides are not aligned",
            ));
        }
        if cell.supporting_island_identity().is_some()
            && cell.supporting_island_member_source_loop_identities().len()
                != supporting_island_winding_signs.len()
        {
            return Err(deny(
                Kind::ContradictoryOperandContainmentEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell containment denies supporting-island witnesses whose member source-loop identities and winding signs are not aligned",
            ));
        }
        if cell.supporting_island_identity().is_some()
            && left_supporting_winding_sum == 0
            && right_supporting_winding_sum == 0
        {
            return Err(deny(
                Kind::MissingOperandContainmentEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell containment denies area-bearing cells whose supporting-island witness carries no operand-local winding membership",
            ));
        }
        if cell.supporting_island_identity().is_some()
            && supporting_island_operand_sides.contains(&Left)
            && left_supporting_winding_sum == 0
            && left_boundary_winding_sum == 0
        {
            return Err(deny(
                Kind::ContradictoryOperandContainmentEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell containment denies left-operand hidden support whose certified signed winding cancels to zero without boundary topology",
            ));
        }
        if cell.supporting_island_identity().is_some()
            && supporting_island_operand_sides.contains(&Right)
            && right_supporting_winding_sum == 0
            && right_boundary_winding_sum == 0
        {
            return Err(deny(
                Kind::ContradictoryOperandContainmentEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell containment denies right-operand hidden support whose certified signed winding cancels to zero without boundary topology",
            ));
        }

        validated_cells.push(ValidatedCellClassification {
            cell,
            boundary_operand_sides,
            left_boundary_winding_sum,
            right_boundary_winding_sum,
            left_supporting_winding_sum,
            right_supporting_winding_sum,
        });
    }

    validated_cells
        .sort_by(|left, right| left.cell.cell_identity().cmp(right.cell.cell_identity()));
    Ok(ValidatedCellClassificationLookup::new(validated_cells))
}

pub(crate) fn validate_winding_input<'a>(
    input: &'a PlanarBooleanOverlapCellWindingFieldInput<'a>,
    counters: &mut PlanarBooleanOverlapCellClassificationCounters,
) -> Result<(), PlanarBooleanOverlapCellClassificationDenial> {
    let graph = input.arrangement_graph();
    let containment_map = input.containment_map();

    if containment_map.arrangement_graph_identity() != graph.arrangement_graph_identity()
        || containment_map.request_identity() != graph.request_identity()
        || containment_map.cell_set_identity() != graph.cell_set().cell_set_identity()
    {
        return Err(deny(
            Kind::WindingFieldInputMismatchDenied,
            graph.arrangement_graph_identity(),
            counters,
            "overlap cell winding requires containment evidence admitted for the same arrangement graph and cell set",
        ));
    }

    let expected_row_count = graph.cell_set().cells().len() * 2;
    if containment_map.rows().len() != expected_row_count {
        return Err(deny(
            Kind::NoOperandLocalWindingEvidenceDenied,
            containment_map.containment_map_identity(),
            counters,
            "overlap cell winding requires exactly one containment row per cell and operand side before winding truth can admit",
        ));
    }

    for cell in graph.cell_set().cells() {
        for operand_side in [Left, Right] {
            let count = containment_map
                .rows()
                .iter()
                .filter(|row| {
                    row.cell_identity() == cell.cell_identity()
                        && row.operand_side() == operand_side
                })
                .count();
            if count != 1 {
                return Err(deny(
                    Kind::NoOperandLocalWindingEvidenceDenied,
                    cell.cell_identity(),
                    counters,
                    "overlap cell winding denies containment maps that do not carry one deterministic containment row per operand side",
                ));
            }
        }
    }

    Ok(())
}

fn deny(
    kind: Kind,
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapCellClassificationCounters,
    human_reason: &'static str,
) -> PlanarBooleanOverlapCellClassificationDenial {
    counters.denied_input();
    PlanarBooleanOverlapCellClassificationDenial::new(
        kind,
        rejected_identity,
        *counters,
        human_reason,
    )
}
