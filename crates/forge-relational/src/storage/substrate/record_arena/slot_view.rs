use std::marker::PhantomData;

use crate::identity::data::{KindId, PartitionId, VersionBound, VersionId};
use crate::payloads::data::RecordPayload;
use crate::storage::data::RecordLifecycleState;

use super::{RecordArena, RecordKind};

#[allow(dead_code)]
pub(crate) struct SlotView<'a, K: RecordKind> {
    arena: &'a RecordArena<K>,
    index: usize,
    _marker: PhantomData<K>,
}

#[allow(dead_code)]
impl<'a, K: RecordKind> SlotView<'a, K> {
    pub(crate) fn new(arena: &'a RecordArena<K>, index: usize) -> Self {
        Self {
            arena,
            index,
            _marker: PhantomData,
        }
    }

    pub(crate) fn generation(&self) -> u32 { self.arena.generations[self.index] }
    pub(crate) fn partition_id(&self) -> PartitionId { self.arena.partition_ids[self.index] }
    pub(crate) fn lifecycle(&self) -> RecordLifecycleState { self.arena.lifecycle[self.index] }
    pub(crate) fn is_live(&self) -> bool { self.lifecycle() == RecordLifecycleState::Live }
    pub(crate) fn kind_id(&self) -> Option<KindId> { self.arena.kind_ids[self.index] }
    pub(crate) fn payload(&self) -> Option<&RecordPayload> { self.arena.payloads[self.index].as_ref() }
    pub(crate) fn snapshot_pins(&self) -> u32 { self.arena.snapshot_pins[self.index] }
    pub(crate) fn branch_pins(&self) -> u32 { self.arena.branch_pins[self.index] }
    pub(crate) fn replay_pins(&self) -> u32 { self.arena.replay_pins[self.index] }
    pub(crate) fn retired_at(&self) -> Option<VersionId> { self.arena.retired_at[self.index] }
    pub(crate) fn extra(&self) -> &K::Extra { &self.arena.extra[self.index] }
    pub(crate) fn is_current(&self, id: &K::Id) -> bool {
        self.generation() == K::generation_of(id) && self.is_live()
    }
    pub(crate) fn is_visible_at(&self, bound: VersionBound) -> bool {
        bound.includes_created(self.arena.created_at[self.index])
            && self.arena.retired_at[self.index].is_none_or(|retired| bound.retains_retired(retired))
    }
}
