use super::state::NativeExternalRow;
use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::foundation::WorthQueryWorkspaceError;
use worth_query::facade::runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectMutation};

pub(super) fn external_row_from_aspects(
    aspects: &[WorthQueryAuthoredAspectMutation],
) -> Result<NativeExternalRow, WorthQueryWorkspaceError> {
    let mut external_row = NativeExternalRow::new();
    apply_aspects_to_external_row(&mut external_row, aspects)?;
    Ok(external_row)
}

pub(super) fn apply_aspects_to_external_row(
    external_row: &mut NativeExternalRow,
    aspects: &[WorthQueryAuthoredAspectMutation],
) -> Result<(), WorthQueryWorkspaceError> {
    for aspect in aspects {
        let aspect_touch = aspect.aspect_touch();
        if aspect.clears_existing_value() {
            clear_external_row_touch(external_row, &aspect_touch)?;
        } else if let Some(value) = aspect.foundational_value() {
            set_external_row_touch(external_row, &aspect_touch, value.clone())?;
        }
    }
    Ok(())
}

fn set_external_row_touch(
    external_row: &mut NativeExternalRow,
    aspect_touch: &WorthQueryAspectTouch,
    value: AspectValue,
) -> Result<(), WorthQueryWorkspaceError> {
    external_row.insert(native_external_field_path_for_touch(aspect_touch)?, value);
    Ok(())
}

fn clear_external_row_touch(
    external_row: &mut NativeExternalRow,
    aspect_touch: &WorthQueryAspectTouch,
) -> Result<(), WorthQueryWorkspaceError> {
    external_row.remove(&native_external_field_path_for_touch(aspect_touch)?);
    Ok(())
}

fn native_external_field_path_for_touch(
    aspect_touch: &WorthQueryAspectTouch,
) -> Result<CanonicalFieldPath, WorthQueryWorkspaceError> {
    let mut fields = vec![
        FieldKey::new(aspect_touch.native_aspect_key().as_str()).ok_or_else(|| {
            WorthQueryWorkspaceError::new(format!(
                "public bridge could not use native aspect `{}` as an external field",
                aspect_touch.native_aspect_key().as_str()
            ))
        })?,
    ];
    if let Some(field_path) = aspect_touch.native_field_path() {
        fields.extend(field_path.fields().iter().cloned());
    }
    CanonicalFieldPath::new(fields).ok_or_else(|| {
        WorthQueryWorkspaceError::new(format!(
            "public bridge could not derive external field path for native aspect `{}`",
            aspect_touch.native_aspect_key().as_str()
        ))
    })
}
