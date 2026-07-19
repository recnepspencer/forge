use worth_foundational::facade::AspectValue;
use worth_relational::facade::transactions::AspectFieldPatch;

use super::keys::{aspect_key, field_key, planned_single_field_locator};

pub(crate) fn single_native_aspect_field_patch(
    aspect_label: &str,
    field_label: &str,
    value: AspectValue,
) -> Result<AspectFieldPatch, String> {
    Ok(AspectFieldPatch::from_locator(
        planned_single_field_locator(aspect_key(aspect_label)?, field_key(field_label)?),
        value,
    ))
}

pub(crate) fn single_native_string_aspect_field_patch(
    aspect_label: &str,
    field_label: &str,
    value: impl Into<String>,
) -> Result<AspectFieldPatch, String> {
    single_native_aspect_field_patch(
        aspect_label,
        field_label,
        crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(value),
    )
}
