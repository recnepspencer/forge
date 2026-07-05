use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::{
    FullOverlapSpan, OverlapEndBoundary, OverlapInteriorFragment, OverlapStartBoundary,
};

use super::counters::PlanarBooleanOverlapArrangementGraphCounters;
use super::denial::{
    PlanarBooleanOverlapArrangementGraphDenial,
    PlanarBooleanOverlapArrangementGraphDenialKind as Kind,
};
use super::lookup::{
    ValidatedArrangementBoundaryComponent, ValidatedArrangementBoundarySegment,
    ValidatedArrangementCell,
};

pub(super) fn validate_boundary_components<'a>(
    neighborhood_identity: &str,
    segments: &[ValidatedArrangementBoundarySegment<'a>],
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
) -> Result<
    Vec<ValidatedArrangementBoundaryComponent<'a>>,
    PlanarBooleanOverlapArrangementGraphDenial,
> {
    let mut components = Vec::new();
    let mut current_component = Vec::new();
    let mut component_ordinal = 0usize;

    for segment in segments.iter().cloned() {
        match segment.boundary_role {
            FullOverlapSpan => {
                if !current_component.is_empty() {
                    return Err(deny(
                        Kind::DisconnectedArrangementNeighborhoodDenied,
                        neighborhood_identity,
                        counters,
                        "overlap arrangement denies neighborhoods whose canonical segment walk opens one boundary component and then forks into another before closure",
                    ));
                }
                components.push(ValidatedArrangementBoundaryComponent {
                    source_loop_identities: sorted_unique_loop_identities([
                        segment.source_loop_identity
                    ]),
                    segments: vec![segment],
                    ordinal: component_ordinal,
                });
                component_ordinal += 1;
            }
            OverlapStartBoundary => {
                if !current_component.is_empty() {
                    return Err(deny(
                        Kind::AmbiguousArrangementSegmentOrderingDenied,
                        neighborhood_identity,
                        counters,
                        "overlap arrangement denies a canonical segment walk that opens a second boundary component before the current one closes",
                    ));
                }
                current_component.push(segment);
            }
            OverlapInteriorFragment => {
                if current_component.is_empty() {
                    return Err(deny(
                        Kind::DisconnectedArrangementNeighborhoodDenied,
                        neighborhood_identity,
                        counters,
                        "overlap arrangement denies interior boundary fragments that are not enclosed by an admitted start and end boundary",
                    ));
                }
                current_component.push(segment);
            }
            OverlapEndBoundary => {
                if current_component.is_empty() {
                    return Err(deny(
                        Kind::DisconnectedArrangementNeighborhoodDenied,
                        neighborhood_identity,
                        counters,
                        "overlap arrangement denies end boundaries that do not close an admitted boundary component",
                    ));
                }
                current_component.push(segment);
                components.push(ValidatedArrangementBoundaryComponent {
                    source_loop_identities: sorted_unique_loop_identities(
                        current_component
                            .iter()
                            .map(|component_segment| component_segment.source_loop_identity),
                    ),
                    segments: std::mem::take(&mut current_component),
                    ordinal: component_ordinal,
                });
                component_ordinal += 1;
            }
        }
    }

    if !current_component.is_empty() {
        return Err(deny(
            Kind::DisconnectedArrangementNeighborhoodDenied,
            neighborhood_identity,
            counters,
            "overlap arrangement denies neighborhoods whose canonical segment walk leaves an unclosed boundary component",
        ));
    }
    if components.is_empty() {
        return Err(deny(
            Kind::NoConcreteCellSubstrateDenied,
            neighborhood_identity,
            counters,
            "overlap arrangement denies neighborhoods that cannot lower any boundary component from the admitted segment substrate",
        ));
    }
    Ok(canonicalize_components(components))
}

pub(super) fn validate_cells<'a>(
    row: &'a crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapAdjacencyRow,
    neighborhood_identity: &str,
    components: &[ValidatedArrangementBoundaryComponent<'a>],
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
) -> Result<Vec<ValidatedArrangementCell<'a>>, PlanarBooleanOverlapArrangementGraphDenial> {
    if components.is_empty() {
        return Err(deny(
            Kind::NoConcreteCellSubstrateDenied,
            neighborhood_identity,
            counters,
            "overlap arrangement denies neighborhoods that do not lower any arrangement cell from admitted boundary components",
        ));
    }

    let mut cells = components
        .iter()
        .filter(|component| is_full_overlap_component(component))
        .cloned()
        .enumerate()
        .map(|(ordinal, component)| ValidatedArrangementCell {
            source_loop_identities: component.source_loop_identities.clone(),
            supporting_island_identity: None,
            supporting_island_member_source_loop_identities: Vec::new(),
            supporting_island_member_source_loop_operand_sides: Vec::new(),
            supporting_island_member_source_loop_winding_signs: Vec::new(),
            components: vec![component],
            ordinal,
        })
        .collect::<Vec<_>>();

    let walk_components = components
        .iter()
        .filter(|component| !is_full_overlap_component(component))
        .cloned()
        .collect::<Vec<_>>();
    let face_groups = group_walk_components_by_island_witness(
        row,
        neighborhood_identity,
        &walk_components,
        counters,
    )?;
    for (
        supporting_island_identity,
        supporting_island_member_source_loop_identities,
        supporting_island_member_source_loop_operand_sides,
        supporting_island_member_source_loop_winding_signs,
        grouped_components,
    ) in face_groups
    {
        let source_loop_identities = sorted_unique_loop_identities(
            grouped_components
                .iter()
                .flat_map(|component| component.source_loop_identities.iter().copied()),
        );
        cells.push(ValidatedArrangementCell {
            source_loop_identities,
            supporting_island_identity: Some(supporting_island_identity),
            supporting_island_member_source_loop_identities,
            supporting_island_member_source_loop_operand_sides,
            supporting_island_member_source_loop_winding_signs,
            components: grouped_components,
            ordinal: 0,
        });
    }
    cells.sort_by(|left, right| cell_order_key(left).cmp(&cell_order_key(right)));
    for (ordinal, cell) in cells.iter_mut().enumerate() {
        cell.ordinal = ordinal;
    }
    Ok(cells)
}

fn sorted_unique_loop_identities<'a>(values: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

fn canonicalize_components<'a>(
    mut components: Vec<ValidatedArrangementBoundaryComponent<'a>>,
) -> Vec<ValidatedArrangementBoundaryComponent<'a>> {
    components.sort_by(|left, right| component_order_key(left).cmp(&component_order_key(right)));
    for (ordinal, component) in components.iter_mut().enumerate() {
        component.ordinal = ordinal;
    }
    components
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
        .join("||");
    format!(
        "{}|{}",
        component.source_loop_identities.join("|"),
        segment_key
    )
}

fn is_full_overlap_component(component: &ValidatedArrangementBoundaryComponent<'_>) -> bool {
    matches!(
        component.segments.as_slice(),
        [ValidatedArrangementBoundarySegment {
            boundary_role: FullOverlapSpan,
            ..
        }]
    )
}

fn group_walk_components_by_island_witness<'a>(
    row: &'a crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapAdjacencyRow,
    neighborhood_identity: &str,
    walk_components: &[ValidatedArrangementBoundaryComponent<'a>],
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
) -> Result<
        Vec<(
            &'a str,
            Vec<&'a str>,
            Vec<crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide>,
            Vec<i8>,
            Vec<ValidatedArrangementBoundaryComponent<'a>>,
        )>,
    PlanarBooleanOverlapArrangementGraphDenial,
>
{
    let mut grouped_components = BTreeMap::<
        &str,
        (
            Vec<&'a str>,
            Vec<
                crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide,
            >,
            Vec<i8>,
            Vec<ValidatedArrangementBoundaryComponent<'a>>,
        ),
    >::new();
    for component in walk_components {
        let matching_islands = row
            .participating_island_identities()
            .iter()
            .zip(row.island_member_source_loop_identities().iter())
            .zip(row.island_member_source_loop_operand_sides().iter())
            .zip(row.island_member_source_loop_winding_signs().iter())
            .filter(|(((_, member_source_loop_identities), _), _)| {
                component
                    .source_loop_identities
                    .iter()
                    .all(|source_loop_identity| {
                        member_source_loop_identities.contains(&source_loop_identity.to_string())
                    })
            })
            .map(
                |(
                    (
                        (island_identity, member_source_loop_identities),
                        member_source_loop_operand_sides,
                    ),
                    member_source_loop_winding_signs,
                )| {
                    (
                        island_identity.as_str(),
                        member_source_loop_identities
                            .iter()
                            .map(String::as_str)
                            .collect::<Vec<_>>(),
                        member_source_loop_operand_sides.clone(),
                        member_source_loop_winding_signs.clone(),
                    )
                },
            )
            .collect::<Vec<_>>();
        match matching_islands.as_slice() {
            [] => {
                return Err(deny(
                    Kind::NoConcreteCellSubstrateDenied,
                    neighborhood_identity,
                    counters,
                    "overlap arrangement denies boundary-walk components that cannot be assigned to any admitted island-backed face witness",
                ));
            }
            [(
                supporting_island_identity,
                supporting_island_member_source_loop_identities,
                supporting_island_member_source_loop_operand_sides,
                supporting_island_member_source_loop_winding_signs,
            )] => {
                let entry = grouped_components
                    .entry(*supporting_island_identity)
                    .or_insert_with(|| {
                        (
                            supporting_island_member_source_loop_identities.clone(),
                            supporting_island_member_source_loop_operand_sides.clone(),
                            supporting_island_member_source_loop_winding_signs.clone(),
                            Vec::new(),
                        )
                    });
                entry.3.push(component.clone());
            }
            _ => {
                return Err(deny(
                    Kind::ContradictoryArrangementNeighborhoodDenied,
                    neighborhood_identity,
                    counters,
                    "overlap arrangement denies boundary-walk components that match multiple admitted island-backed face witnesses",
                ));
            }
        }
    }
    Ok(grouped_components
        .into_iter()
        .map(
            |(
                island_identity,
                (
                    supporting_island_member_source_loop_identities,
                    supporting_island_member_source_loop_operand_sides,
                    supporting_island_member_source_loop_winding_signs,
                    components,
                ),
            )| {
                (
                    island_identity,
                    supporting_island_member_source_loop_identities,
                    supporting_island_member_source_loop_operand_sides,
                    supporting_island_member_source_loop_winding_signs,
                    components,
                )
            },
        )
        .collect())
}

fn cell_order_key(cell: &ValidatedArrangementCell<'_>) -> String {
    let island_key = cell.supporting_island_identity.unwrap_or("");
    let component_key = cell
        .components
        .iter()
        .map(component_order_key)
        .collect::<Vec<_>>()
        .join("||");
    format!("{island_key}|{component_key}")
}

fn deny(
    kind: Kind,
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapArrangementGraphCounters,
    human_reason: &'static str,
) -> PlanarBooleanOverlapArrangementGraphDenial {
    counters.denied_neighborhood();
    PlanarBooleanOverlapArrangementGraphDenial::new(
        kind,
        rejected_identity,
        *counters,
        human_reason,
    )
}
