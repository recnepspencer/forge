use std::collections::BTreeMap;

use crate::identity::data::{KindId, PartitionId, VersionId};
use crate::storage::data::RecordLifecycleState;
use crate::symbols::data::Symbol;

use super::{RecordArena, RecordKind};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RecordArenaAllocationInventory {
    pub(crate) authoritative_bytes: u64,
    pub(crate) diagnostic_bytes: u64,
    pub(crate) retention_metadata_bytes: u64,
    pub(crate) allocator_bookkeeping_bytes: u64,
}

impl RecordArenaAllocationInventory {
    pub(crate) fn saturating_add(self, other: Self) -> Self {
        Self {
            authoritative_bytes: self
                .authoritative_bytes
                .saturating_add(other.authoritative_bytes),
            diagnostic_bytes: self.diagnostic_bytes.saturating_add(other.diagnostic_bytes),
            retention_metadata_bytes: self
                .retention_metadata_bytes
                .saturating_add(other.retention_metadata_bytes),
            allocator_bookkeeping_bytes: self
                .allocator_bookkeeping_bytes
                .saturating_add(other.allocator_bookkeeping_bytes),
        }
    }
}

impl<K: RecordKind> RecordArena<K> {
    pub(crate) fn allocation_inventory(&self) -> RecordArenaAllocationInventory {
        let authoritative_bytes = [
            self.slots.allocation_bytes(),
            vector_bytes::<PartitionId>(&self.partition_ids),
            vector_bytes::<u32>(&self.generations),
            vector_bytes::<RecordLifecycleState>(&self.lifecycle),
            vector_bytes::<Option<KindId>>(&self.kind_ids),
            vector_bytes::<Vec<K::Meta>>(&self.metadata_history),
            vector_bytes::<VersionId>(&self.created_at),
            vector_bytes::<Option<VersionId>>(&self.retired_at),
            vector_bytes::<K::Extra>(&self.extra),
            vector_bytes::<BTreeMap<Symbol, u64>>(&self.aspect_versions),
            self.live_bitset.authoritative_allocation_bytes(),
            self.reclaimable_bitset.authoritative_allocation_bytes(),
            self.metadata_history
                .iter()
                .map(vector_bytes::<K::Meta>)
                .sum(),
            self.metadata_history
                .iter()
                .flatten()
                .map(K::metadata_owned_allocation_bytes)
                .sum(),
            self.extra.iter().map(K::extra_owned_allocation_bytes).sum(),
            self.aspect_versions
                .iter()
                .map(|versions| map_entry_bytes::<Symbol, u64>(versions.len()))
                .sum(),
        ]
        .into_iter()
        .fold(0_u64, u64::saturating_add);
        let diagnostic_bytes =
            vector_bytes::<BTreeMap<Symbol, String>>(&self.diagnostics_enrichment).saturating_add(
                self.diagnostics_enrichment
                    .iter()
                    .map(|entries| {
                        map_entry_bytes::<Symbol, String>(entries.len()).saturating_add(
                            entries.values().map(|value| value.capacity() as u64).sum(),
                        )
                    })
                    .sum(),
            );
        let retention_metadata_bytes = [
            vector_bytes::<u32>(&self.branch_pins),
            vector_bytes::<u32>(&self.replay_pins),
            vector_bytes::<u32>(&self.snapshot_pins),
        ]
        .into_iter()
        .fold(0_u64, u64::saturating_add);
        RecordArenaAllocationInventory {
            authoritative_bytes,
            diagnostic_bytes,
            retention_metadata_bytes,
            allocator_bookkeeping_bytes: 0,
        }
    }
}

fn vector_bytes<T>(values: &Vec<T>) -> u64 {
    (values.capacity() as u64).saturating_mul(std::mem::size_of::<T>() as u64)
}

fn map_entry_bytes<K, V>(entry_count: usize) -> u64 {
    (entry_count as u64).saturating_mul(std::mem::size_of::<(K, V)>() as u64)
}
