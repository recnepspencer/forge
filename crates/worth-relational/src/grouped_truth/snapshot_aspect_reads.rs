use worth_foundational::facade::AspectValue;
use worth_runtime_bridge::facade::{SnapshotReadRecord, SnapshotReadValue};

use super::grouped_projection::RelationalGroupedTruthError;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SnapshotAspectReadValue {
    value: SnapshotReadValue,
}

impl SnapshotAspectReadValue {
    pub(super) fn value(&self) -> &SnapshotReadValue {
        &self.value
    }
}

pub(super) fn decode_snapshot_aspect_read_value(
    record: &SnapshotReadRecord,
) -> Result<SnapshotAspectReadValue, RelationalGroupedTruthError> {
    let value = record.read_value().clone();
    Ok(SnapshotAspectReadValue { value })
}

pub fn encode_snapshot_aspect_read_value(value: &AspectValue) -> AspectValue {
    value.clone()
}
