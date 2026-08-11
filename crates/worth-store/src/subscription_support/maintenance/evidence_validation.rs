use super::super::classification_error;
use crate::failure::StoreError;

pub(super) fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support maintenance {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
