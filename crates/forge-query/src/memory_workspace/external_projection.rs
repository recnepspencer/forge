use super::*;
use serde_json::Map;

pub(super) fn set_json_path(
    target: &mut Value,
    path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut parts = path.split('.').peekable();
    let mut current = target;
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            let object = current.as_object_mut().ok_or_else(|| {
                ForgeQueryWorkspaceError::new("target external projection is not an object")
            })?;
            object.insert(part.to_string(), value);
            return Ok(());
        }
        let object = current.as_object_mut().ok_or_else(|| {
            ForgeQueryWorkspaceError::new("target external projection is not an object")
        })?;
        current = object
            .entry(part.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    Err(ForgeQueryWorkspaceError::new(
        "empty external projection path",
    ))
}
