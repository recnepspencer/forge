mod branch_head_residency;
mod branch_pins;
mod replay_pins;
mod snapshot_state_pins;
#[cfg(test)]
mod tests;

use crate::runtime::RelationalRuntime;
use crate::storage::overlay::SnapshotState;
use crate::storage::substrate::PinClass as SubstratePinClass;
use crate::storage::substrate::{EntityRecordKind, PinClass, RecordKind, RelationRecordKind};

pub(crate) struct VisibilityPinAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

fn adjust_entity_pin(
    runtime: &mut RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    class: SubstratePinClass,
    delta: i32,
) {
    adjust_record_pin::<EntityRecordKind>(runtime, entity_id, class, delta);
}

fn adjust_relation_pin(
    runtime: &mut RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
    class: SubstratePinClass,
    delta: i32,
) {
    adjust_record_pin::<RelationRecordKind>(runtime, relation_id, class, delta);
}

fn adjust_record_pin<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    record_id: crate::identity::data::RecordId<K::Domain>,
    class: SubstratePinClass,
    delta: i32,
) {
    let retention_fence = runtime
        .visibility
        .retention_fence_version(runtime.current_version_id());
    runtime
        .storage_authority()
        .adjust_named_pin::<K>(record_id, class, delta, retention_fence);
}

impl RelationalRuntime {
    pub(crate) fn visibility_pins(&mut self) -> VisibilityPinAuthority<'_> {
        VisibilityPinAuthority::new(self)
    }
}
