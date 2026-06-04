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
    let value = record.scalar_aspect_value().cloned().ok_or_else(|| {
        RelationalGroupedTruthError::AspectValueDecodeFailure {
            request_key: record.correlation_id().as_str().to_string(),
        }
    })?;
    Ok(SnapshotAspectReadValue { value })
}

pub fn encode_snapshot_aspect_read_value(value: &AspectValue) -> AspectValue {
    value.clone()
}
