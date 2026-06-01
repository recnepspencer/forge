use forge_query::facade::{ForgeQueryAspectValue, ForgeQueryWorkspaceError};
use serde_json::{Map, Value};

pub(super) fn external_row_from_aspects(
    aspects: &[ForgeQueryAspectValue],
) -> Result<Value, ForgeQueryWorkspaceError> {
    let mut external_row = Value::Object(Map::new());
    apply_aspects_to_external_row(&mut external_row, aspects)?;
    Ok(external_row)
}

pub(super) fn apply_aspects_to_external_row(
    external_row: &mut Value,
    aspects: &[ForgeQueryAspectValue],
) -> Result<(), ForgeQueryWorkspaceError> {
    for aspect in aspects {
        if aspect.clears_existing_value() {
            clear_external_row_path(external_row, aspect.aspect_path())?;
        } else {
            set_external_row_path(external_row, aspect.aspect_path(), aspect.value().clone())?;
        }
    }
    Ok(())
}

fn set_external_row_path(
    external_row: &mut Value,
    dotted_path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut current = external_row;
    let mut parts = dotted_path.split('.').peekable();
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            return match current {
                Value::Object(object) => {
                    object.insert(part.to_string(), value);
                    Ok(())
                }
                _ => Err(non_object_boundary_error(dotted_path)),
            };
        }
        current = match current {
            Value::Object(object) => object
                .entry(part.to_string())
                .or_insert_with(|| Value::Object(Map::new())),
            _ => return Err(non_object_boundary_error(dotted_path)),
        };
    }
    Ok(())
}

fn clear_external_row_path(
    external_row: &mut Value,
    dotted_path: &str,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut current = external_row;
    let mut parts = dotted_path.split('.').peekable();
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            return match current {
                Value::Object(object) => {
                    object.remove(part);
                    Ok(())
                }
                _ => Ok(()),
            };
        }
        current = match current {
            Value::Object(object) => match object.get_mut(part) {
                Some(next) => next,
                None => return Ok(()),
            },
            _ => return Ok(()),
        };
    }
    Ok(())
}

fn non_object_boundary_error(dotted_path: &str) -> ForgeQueryWorkspaceError {
    ForgeQueryWorkspaceError::new(format!(
        "public bridge external row path `{dotted_path}` crossed a non-object boundary"
    ))
}
