use forge_foundational::facade::AspectValue;
use forge_runtime_bridge::facade::SnapshotReadRecord;

use super::grouped_projection::RelationalGroupedTruthError;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SnapshotAspectReadValue {
    value: AspectValue,
}

impl SnapshotAspectReadValue {
    pub(super) fn value(&self) -> &AspectValue {
        &self.value
    }
}

pub(super) fn decode_snapshot_aspect_read_value(
    record: &SnapshotReadRecord,
) -> Result<SnapshotAspectReadValue, RelationalGroupedTruthError> {
    let value = crate::aspect_wire::decode_aspect_value(record.aspect_bytes()).map_err(|_| {
        RelationalGroupedTruthError::AspectValueDecodeFailure {
            request_key: record.request_key().to_string(),
        }
    })?;
    Ok(SnapshotAspectReadValue { value })
}

pub fn encode_snapshot_aspect_read_value(
    value: &AspectValue,
) -> Result<Vec<u8>, RelationalGroupedTruthError> {
    crate::aspect_wire::encode_aspect_value(value).map_err(|error| {
        RelationalGroupedTruthError::AspectValueEncodeFailure {
            detail: error.detail().to_string(),
        }
    })
}
