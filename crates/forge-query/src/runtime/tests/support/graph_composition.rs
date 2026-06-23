use super::*;

pub(in crate::runtime::tests) type GraphCompositionResolutionEntrySnapshot =
    (usize, Option<String>, String, String);

pub(in crate::runtime::tests) fn graph_composition_resolution_snapshot(
    map: &ForgeQueryGraphCompositionResolutionMap,
) -> Vec<GraphCompositionResolutionEntrySnapshot> {
    map.entries()
        .iter()
        .map(|entry| {
            (
                entry.component_index(),
                entry.aspect_path().map(str::to_string),
                entry.symbol().as_str().to_string(),
                entry
                    .resolved_entity_identity()
                    .terminal_projection_for_reporting(),
            )
        })
        .collect()
}

pub(in crate::runtime::tests) fn assert_graph_composition_resolution_snapshot(
    map: &ForgeQueryGraphCompositionResolutionMap,
    expected: &[GraphCompositionResolutionEntrySnapshot],
) {
    assert_eq!(graph_composition_resolution_snapshot(map), expected);
}

pub(in crate::runtime::tests) fn assert_graph_composition_resolution_maps_match(
    left: &ForgeQueryGraphCompositionResolutionMap,
    right: &ForgeQueryGraphCompositionResolutionMap,
) {
    assert_eq!(
        graph_composition_resolution_snapshot(left),
        graph_composition_resolution_snapshot(right)
    );
}
