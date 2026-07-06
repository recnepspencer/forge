use super::lookup::{ValidatedArrangementBoundaryComponent, ValidatedArrangementCell};

pub(super) fn sorted_unique_loop_identities<'a>(
    values: impl IntoIterator<Item = &'a str>,
) -> Vec<&'a str> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

pub(super) fn canonicalize_components<'a>(
    mut components: Vec<ValidatedArrangementBoundaryComponent<'a>>,
) -> Vec<ValidatedArrangementBoundaryComponent<'a>> {
    components.sort_by(|left, right| component_order_key(left).cmp(&component_order_key(right)));
    for (ordinal, component) in components.iter_mut().enumerate() {
        component.ordinal = ordinal;
    }
    components
}

pub(super) fn component_order_key(component: &ValidatedArrangementBoundaryComponent<'_>) -> String {
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

pub(super) fn cell_order_key(cell: &ValidatedArrangementCell<'_>) -> String {
    let island_key = cell.supporting_island_identity.unwrap_or("");
    let component_key = cell
        .components
        .iter()
        .map(component_order_key)
        .collect::<Vec<_>>()
        .join("||");
    format!("{island_key}|{component_key}")
}
