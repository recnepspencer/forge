use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan;

use super::counters::PlanarBooleanOverlapArrangementGraphCounters;
use super::denial::{
    PlanarBooleanOverlapArrangementGraphDenial,
    PlanarBooleanOverlapArrangementGraphDenialKind as Kind,
};
use super::lookup::{ValidatedArrangementBoundaryComponent, ValidatedArrangementBoundarySegment};

pub(super) fn validate_source_only_boundary_components<'a>(
    neighborhood_identity: &str,
    segments: &[ValidatedArrangementBoundarySegment<'a>],
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
) -> Result<
    Vec<ValidatedArrangementBoundaryComponent<'a>>,
    PlanarBooleanOverlapArrangementGraphDenial,
> {
    if segments.is_empty() {
        counters.denied_neighborhood();
        return Err(PlanarBooleanOverlapArrangementGraphDenial::new(
            Kind::NoConcreteCellSubstrateDenied,
            neighborhood_identity,
            *counters,
            "overlap arrangement denies source-only boundary neighborhoods without a concrete segment substrate",
        ));
    }

    let mut components = segments
        .iter()
        .cloned()
        .enumerate()
        .map(|(ordinal, segment)| ValidatedArrangementBoundaryComponent {
            source_loop_identities: sorted_unique_loop_identities([segment.source_loop_identity]),
            segments: vec![segment],
            ordinal,
        })
        .collect::<Vec<_>>();
    components.sort_by(|left, right| component_order_key(left).cmp(&component_order_key(right)));
    for (ordinal, component) in components.iter_mut().enumerate() {
        component.ordinal = ordinal;
    }
    Ok(components)
}

pub(super) struct SourceOnlyAreaWitness<'a> {
    pub(super) island_identity: &'a str,
    pub(super) source_loop_identities: Vec<&'a str>,
    pub(super) source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    pub(super) source_loop_winding_signs: Vec<i8>,
}

pub(super) fn source_only_area_witness<'a>(
    row: &'a crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapAdjacencyRow,
    components: &[ValidatedArrangementBoundaryComponent<'a>],
) -> Option<SourceOnlyAreaWitness<'a>> {
    if components.is_empty() || !has_source_only_area_substrate(row) {
        return None;
    }

    let mut source_loop_identities = Vec::new();
    let mut source_loop_operand_sides = Vec::new();
    let mut source_loop_winding_signs = Vec::new();
    let mut seen = BTreeSet::new();

    for ((source_loop_identity, operand_side), winding_sign) in row
        .source_loop_identities()
        .iter()
        .zip(row.source_loop_operand_sides())
        .zip(row.source_loop_winding_signs())
    {
        if *winding_sign == 0 {
            continue;
        }
        let key = format!(
            "{}:{}:{}",
            source_loop_identity,
            operand_side.query_key(),
            winding_sign
        );
        if seen.insert(key) {
            source_loop_identities.push(source_loop_identity.as_str());
            source_loop_operand_sides.push(*operand_side);
            source_loop_winding_signs.push(*winding_sign);
        }
    }

    let has_left = source_loop_operand_sides.iter().any(|side| *side == Left);
    let has_right = source_loop_operand_sides.iter().any(|side| *side == Right);
    if !has_left || !has_right {
        return None;
    }

    Some(SourceOnlyAreaWitness {
        island_identity: row.neighborhood_identity(),
        source_loop_identities,
        source_loop_operand_sides,
        source_loop_winding_signs,
    })
}

fn has_source_only_area_substrate(
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapAdjacencyRow,
) -> bool {
    row.boundary_roles()
        .iter()
        .all(|role| *role == FullOverlapSpan)
        && row
            .source_loop_operand_sides()
            .iter()
            .any(|side| *side == Left)
        && row
            .source_loop_operand_sides()
            .iter()
            .any(|side| *side == Right)
        && row
            .source_loop_winding_signs()
            .iter()
            .filter(|winding| **winding != 0)
            .count()
            >= 2
}

fn sorted_unique_loop_identities<'a>(values: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

fn component_order_key(component: &ValidatedArrangementBoundaryComponent<'_>) -> String {
    let segment_key = component
        .segments
        .iter()
        .map(|segment| {
            format!(
                "{}|{}|{}|{:?}",
                segment.source_loop_identity,
                segment.source_edge_identity,
                segment.fragment_identity,
                segment.boundary_role
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    format!(
        "{}|{}",
        component.source_loop_identities.join("|"),
        segment_key
    )
}
