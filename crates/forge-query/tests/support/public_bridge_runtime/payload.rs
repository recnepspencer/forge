use forge_query::facade::{ForgeQueryAspectValue, ForgeQueryWorkspaceError};
use serde_json::{Map, Value};

pub(super) fn payload_from_aspects(
    aspects: &[ForgeQueryAspectValue],
) -> Result<Value, ForgeQueryWorkspaceError> {
    let mut payload = Value::Object(Map::new());
    apply_aspects(&mut payload, aspects)?;
    Ok(payload)
}

pub(super) fn apply_aspects(
    payload: &mut Value,
    aspects: &[ForgeQueryAspectValue],
) -> Result<(), ForgeQueryWorkspaceError> {
    for aspect in aspects {
        if aspect.clears_existing_value() {
            clear_payload_path(payload, aspect.aspect_path())?;
        } else {
            set_payload_path(payload, aspect.aspect_path(), aspect.value().clone())?;
        }
    }
    Ok(())
}

fn set_payload_path(
    payload: &mut Value,
    dotted_path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut current = payload;
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

fn clear_payload_path(
    payload: &mut Value,
    dotted_path: &str,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut current = payload;
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
        "public bridge payload path `{dotted_path}` crossed a non-object boundary"
    ))
}
