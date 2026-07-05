use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanOverlapCellClassificationCounters;
use super::denial::{
    PlanarBooleanOverlapCellClassificationDenial, PlanarBooleanOverlapCellClassificationDenialKind,
};
use super::input::PlanarBooleanOverlapCellWindingFieldInput;
use super::lookup::boundary_segments_by_identity;
use super::product::PlanarBooleanOverlapCellWindingField;
use super::rows::{
    PlanarBooleanOverlapCellContainmentEvidenceKind::Inside,
    PlanarBooleanOverlapCellWindingEvidenceKind::{
        BoundaryTopology, BoundaryTopologyAndSupportingIslandTopology, NoTopologySupport,
        SupportingIslandTopology,
    },
    PlanarBooleanOverlapCellWindingRow,
};
use super::validation::validate_winding_input;
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::{
    PlanarBooleanOverlapArrangementBoundarySegmentRow, PlanarBooleanOverlapArrangementCellRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperandLocalTopologyContribution {
    source_loop_identity: String,
    winding_sign: i8,
}

pub(crate) fn build_winding_field(
    input: PlanarBooleanOverlapCellWindingFieldInput<'_>,
) -> Result<PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellClassificationDenial> {
    let mut counters = PlanarBooleanOverlapCellClassificationCounters::default();
    validate_winding_input(&input, &mut counters)?;
    let graph = input.arrangement_graph();
    let containment_map = input.containment_map();
    let segments_by_identity = boundary_segments_by_identity(graph.boundary_segments());
    let mut rows = Vec::new();

    for containment_row in containment_map.rows() {
        let cell = graph
            .cell_set()
            .cells()
            .iter()
            .find(|cell| cell.cell_identity() == containment_row.cell_identity())
            .expect("validated winding input should reference admitted arrangement cells");
        let boundary_supporting_loops =
            boundary_supporting_loops(cell, containment_row.operand_side(), &segments_by_identity);
        let boundary_supporting_loop_identities = boundary_supporting_loops
            .iter()
            .map(|contribution| contribution.source_loop_identity.as_str())
            .collect::<BTreeSet<_>>();
        let hidden_supporting_loops = hidden_supporting_loops(
            cell,
            containment_row.operand_side(),
            &boundary_supporting_loop_identities,
        );
        let unique_hidden_supporting_loop_count = hidden_supporting_loops
            .iter()
            .map(|contribution| contribution.source_loop_identity.as_str())
            .collect::<BTreeSet<_>>()
            .len();

        if cell.supporting_island_identity().is_some() && unique_hidden_supporting_loop_count > 1 {
            counters.denied_input();
            return Err(PlanarBooleanOverlapCellClassificationDenial::new(
                PlanarBooleanOverlapCellClassificationDenialKind::NoOperandLocalWindingEvidenceDenied,
                cell.cell_identity(),
                counters,
                "overlap cell winding denies cells whose operand-local winding would require more than one hidden same-operand supporting loop without certified boundary topology",
            ));
        }

        let evidence_kind = match (
            boundary_supporting_loops.is_empty(),
            hidden_supporting_loops.is_empty(),
        ) {
            (true, true) => NoTopologySupport,
            (false, true) => BoundaryTopology,
            (true, false) => SupportingIslandTopology,
            (false, false) => BoundaryTopologyAndSupportingIslandTopology,
        };
        let boundary_winding = boundary_supporting_loops
            .iter()
            .map(|contribution| contribution.winding_sign)
            .sum::<i8>();
        let hidden_winding = hidden_supporting_loops
            .iter()
            .map(|contribution| contribution.winding_sign)
            .sum::<i8>();
        let winding_number = match containment_row.evidence_kind() {
            Inside => boundary_winding + hidden_winding,
            _ => boundary_winding,
        };

        rows.push(PlanarBooleanOverlapCellWindingRow::new(
            containment_row.cell_identity().to_string(),
            containment_row.arrangement_identity().to_string(),
            containment_row.neighborhood_identity().to_string(),
            containment_row.operand_side(),
            containment_row
                .supporting_island_identity()
                .map(str::to_string),
            containment_row.source_loop_identities().to_vec(),
            evidence_kind,
            winding_number,
        ));
        counters.emitted_winding_row();
    }

    Ok(PlanarBooleanOverlapCellWindingField::new(
        winding_field_identity(graph.request_identity(), graph.arrangement_graph_identity()),
        graph.request_identity().to_string(),
        graph.arrangement_graph_identity().to_string(),
        graph.cell_set().cell_set_identity().to_string(),
        graph.ordering_basis_identity().to_string(),
        rows,
        counters,
    ))
}

fn winding_field_identity(request_identity: &str, arrangement_graph_identity: &str) -> String {
    format!("overlap-arrangement:winding-field:{request_identity}:{arrangement_graph_identity}")
}

fn boundary_supporting_loops(
    cell: &PlanarBooleanOverlapArrangementCellRow,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    boundary_segments_by_identity: &BTreeMap<
        &str,
        &PlanarBooleanOverlapArrangementBoundarySegmentRow,
    >,
) -> Vec<OperandLocalTopologyContribution> {
    cell.boundary_segment_identities()
        .iter()
        .filter_map(|segment_identity| {
            boundary_segments_by_identity
                .get(segment_identity.as_str())
                .copied()
        })
        .filter(|segment| segment.operand_side() == operand_side)
        .map(|segment| OperandLocalTopologyContribution {
            source_loop_identity: segment.source_loop_identity().to_string(),
            winding_sign: segment.source_loop_winding_sign(),
        })
        .collect()
}

fn hidden_supporting_loops(
    cell: &PlanarBooleanOverlapArrangementCellRow,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    boundary_supporting_loop_identities: &BTreeSet<&str>,
) -> Vec<OperandLocalTopologyContribution> {
    cell.supporting_island_member_source_loop_identities()
        .iter()
        .cloned()
        .zip(
            cell.supporting_island_member_source_loop_operand_sides()
                .iter()
                .copied(),
        )
        .zip(
            cell.supporting_island_member_source_loop_winding_signs()
                .iter()
                .copied(),
        )
        .filter(|((_, side), _)| *side == operand_side)
        .map(
            |((source_loop_identity, _), winding_sign)| OperandLocalTopologyContribution {
                source_loop_identity,
                winding_sign,
            },
        )
        .filter(|contribution| {
            !boundary_supporting_loop_identities
                .contains(contribution.source_loop_identity.as_str())
        })
        .collect()
}
