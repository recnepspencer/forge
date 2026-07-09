use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch, WorthQueryMutationFamily,
};

use super::registration_denial::{
    WorthQueryGraphObligationRegistrationDenial, WorthQueryGraphObligationRegistrationDenialKind,
};

pub(super) fn non_empty_selector_value(
    value: String,
    label: &'static str,
) -> Result<String, WorthQueryGraphObligationRegistrationDenial> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(WorthQueryGraphObligationRegistrationDenial::new(
            WorthQueryGraphObligationRegistrationDenialKind::EmptySelectorValue,
            format!("graph obligation {label} selector value must not be empty"),
        ));
    }
    Ok(value)
}

pub(super) fn sorted_unique_operations(
    values: impl IntoIterator<Item = WorthQueryAspectMutationOperation>,
) -> Vec<WorthQueryAspectMutationOperation> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn sorted_unique_touches(
    values: impl IntoIterator<Item = WorthQueryAspectTouch>,
) -> Vec<WorthQueryAspectTouch> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn terminal_touch_digest_parts(
    touched_aspects: &[WorthQueryAspectTouch],
) -> Vec<String> {
    touched_aspects
        .iter()
        .map(WorthQueryAspectTouch::admitted_touch_digest_part)
        .collect()
}

pub(super) fn contains_all_operations(
    available: &[WorthQueryAspectMutationOperation],
    required: &[WorthQueryAspectMutationOperation],
) -> bool {
    required
        .iter()
        .all(|required| available.iter().any(|available| available == required))
}

pub(super) fn contains_all_aspect_touches(
    declared_aspect_operations: &[WorthQueryAspectMutationOperation],
    touched_aspects: &[WorthQueryAspectTouch],
    required: &[WorthQueryAspectTouch],
) -> bool {
    required.iter().all(|required| {
        touched_aspects
            .iter()
            .any(|available| available == required)
            || declared_aspect_operations
                .iter()
                .any(|operation| operation.aspect_touch() == required)
    })
}

pub(super) fn terminal_declared_aspect_operation_digest_part(
    operation: &WorthQueryAspectMutationOperation,
) -> String {
    format!(
        "{}:{}",
        operation.kind().as_str(),
        operation.aspect_touch().admitted_touch_digest_part()
    )
}

pub(super) fn terminal_mutation_family(family: WorthQueryMutationFamily) -> String {
    family.as_str().to_string()
}
