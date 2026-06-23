use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryMutationFamily,
};

use super::registration_denial::{
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphObligationRegistrationDenialKind,
};

pub(super) fn non_empty_selector_value(
    value: String,
    label: &'static str,
) -> Result<String, ForgeQueryGraphObligationRegistrationDenial> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(ForgeQueryGraphObligationRegistrationDenial::new(
            ForgeQueryGraphObligationRegistrationDenialKind::EmptySelectorValue,
            format!("graph obligation {label} selector value must not be empty"),
        ));
    }
    Ok(value)
}

pub(super) fn sorted_unique_operations(
    values: impl IntoIterator<Item = ForgeQueryAspectMutationOperation>,
) -> Vec<ForgeQueryAspectMutationOperation> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn sorted_unique_touches(
    values: impl IntoIterator<Item = ForgeQueryAspectTouch>,
) -> Vec<ForgeQueryAspectTouch> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn terminal_touch_digest_parts(
    touched_aspects: &[ForgeQueryAspectTouch],
) -> Vec<String> {
    touched_aspects
        .iter()
        .map(ForgeQueryAspectTouch::admitted_touch_digest_part)
        .collect()
}

pub(super) fn contains_all_operations(
    available: &[ForgeQueryAspectMutationOperation],
    required: &[ForgeQueryAspectMutationOperation],
) -> bool {
    required
        .iter()
        .all(|required| available.iter().any(|available| available == required))
}

pub(super) fn contains_all_aspect_touches(
    declared_aspect_operations: &[ForgeQueryAspectMutationOperation],
    touched_aspects: &[ForgeQueryAspectTouch],
    required: &[ForgeQueryAspectTouch],
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
    operation: &ForgeQueryAspectMutationOperation,
) -> String {
    format!(
        "{}:{}",
        operation.kind().as_str(),
        operation.aspect_touch().admitted_touch_digest_part()
    )
}

pub(super) fn terminal_mutation_family(family: ForgeQueryMutationFamily) -> String {
    family.as_str().to_string()
}
