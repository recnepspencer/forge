use super::state::NativeExternalRow;
use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectTouch, ForgeQueryAspectValue, ForgeQueryWorkspaceError};

pub(super) fn external_row_from_aspects(
    aspects: &[ForgeQueryAspectValue],
) -> Result<NativeExternalRow, ForgeQueryWorkspaceError> {
    let mut external_row = NativeExternalRow::new();
    apply_aspects_to_external_row(&mut external_row, aspects)?;
    Ok(external_row)
}

pub(super) fn apply_aspects_to_external_row(
    external_row: &mut NativeExternalRow,
    aspects: &[ForgeQueryAspectValue],
) -> Result<(), ForgeQueryWorkspaceError> {
    for aspect in aspects {
        let aspect_touch = aspect.aspect_touch();
        let aspect_path = terminal_aspect_path_projection(&aspect_touch);
        if aspect.clears_existing_value() {
            clear_external_row_path(external_row, &aspect_path)?;
        } else if let Some(value) = aspect.foundational_value() {
            set_external_row_path(external_row, &aspect_path, value.clone())?;
        }
    }
    Ok(())
}

fn terminal_aspect_path_projection(touch: &ForgeQueryAspectTouch) -> String {
    match touch.native_field_path() {
        Some(path) => format!(
            "{}.{}",
            touch.native_aspect_key().as_str(),
            path.fields()
                .iter()
                .map(|field| field.as_str())
                .collect::<Vec<_>>()
                .join(".")
        ),
        None => touch.native_aspect_key().as_str().to_string(),
    }
}

fn set_external_row_path(
    external_row: &mut NativeExternalRow,
    dotted_path: &str,
    value: AspectValue,
) -> Result<(), ForgeQueryWorkspaceError> {
    external_row.insert(canonical_field_path(dotted_path)?, value);
    Ok(())
}

fn clear_external_row_path(
    external_row: &mut NativeExternalRow,
    dotted_path: &str,
) -> Result<(), ForgeQueryWorkspaceError> {
    external_row.remove(&canonical_field_path(dotted_path)?);
    Ok(())
}

pub(super) fn canonical_field_path(
    path: &str,
) -> Result<CanonicalFieldPath, ForgeQueryWorkspaceError> {
    CanonicalFieldPath::new(
        path.split('.')
            .map(|segment| FieldKey::new(segment.to_string()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "public bridge external row field `{path}` is not a canonical field path"
                ))
            })?,
    )
    .ok_or_else(|| {
        ForgeQueryWorkspaceError::new(format!(
            "public bridge external row field `{path}` is empty"
        ))
    })
}
