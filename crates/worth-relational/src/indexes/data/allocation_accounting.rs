use std::collections::BTreeMap;

use crate::identity::data::EntityId;
use crate::storage::data::AuthoritativeFieldComparisonKey;

use super::{
    DerivedIndexArtifacts, DerivedIndexEntries, DerivedIndexGeneration, RelatedEntityOrderingEntry,
    RelationJoinEntry, RelationJoinKey,
};

impl DerivedIndexArtifacts {
    pub(super) fn recursive_owned_allocation_capacity_bytes(&self) -> u64 {
        vector_capacity_bytes::<DerivedIndexGeneration>(&self.generations).saturating_add(
            self.generations
                .iter()
                .map(DerivedIndexGeneration::owned_allocation_capacity_bytes)
                .sum::<u64>(),
        )
    }
}

impl DerivedIndexGeneration {
    fn owned_allocation_capacity_bytes(&self) -> u64 {
        (self.source_branch_id.0.capacity() as u64)
            .saturating_add(self.applicability.branch_id.0.capacity() as u64)
            .saturating_add(self.entries.owned_allocation_capacity_bytes())
    }
}

impl DerivedIndexEntries {
    fn owned_allocation_capacity_bytes(&self) -> u64 {
        match self {
            Self::EntityField(entries) => comparison_entries_bytes(entries),
            Self::RelationField(entries) => comparison_entries_bytes(entries),
            Self::RelatedEntityOrdering(entries) => ordering_entries_bytes(entries),
            Self::RelationJoin(entries) => fixed_entries_bytes(entries),
        }
    }
}

fn comparison_entries_bytes<RecordId>(
    entries: &BTreeMap<AuthoritativeFieldComparisonKey, Vec<RecordId>>,
) -> u64 {
    map_payload_bytes(entries)
        .saturating_add(
            entries
                .keys()
                .map(AuthoritativeFieldComparisonKey::owned_allocation_capacity_bytes)
                .sum(),
        )
        .saturating_add(
            entries
                .values()
                .map(vector_capacity_bytes::<RecordId>)
                .sum(),
        )
}

fn ordering_entries_bytes(entries: &BTreeMap<EntityId, Vec<RelatedEntityOrderingEntry>>) -> u64 {
    map_payload_bytes(entries).saturating_add(
        entries
            .values()
            .map(|values| {
                vector_capacity_bytes::<RelatedEntityOrderingEntry>(values).saturating_add(
                    values
                        .iter()
                        .map(RelatedEntityOrderingEntry::owned_allocation_capacity_bytes)
                        .sum(),
                )
            })
            .sum(),
    )
}

fn fixed_entries_bytes(entries: &BTreeMap<RelationJoinKey, Vec<RelationJoinEntry>>) -> u64 {
    map_payload_bytes(entries).saturating_add(
        entries
            .values()
            .map(vector_capacity_bytes::<RelationJoinEntry>)
            .sum(),
    )
}

fn map_payload_bytes<Key, Value>(entries: &BTreeMap<Key, Value>) -> u64 {
    // BTreeMap does not expose node capacity. Count every owned logical entry
    // plus all recursively visible buffers; allocator/node bookkeeping is not
    // presented as authoritative cache payload.
    (entries.len() as u64).saturating_mul(std::mem::size_of::<(Key, Value)>() as u64)
}

fn vector_capacity_bytes<Value>(values: &Vec<Value>) -> u64 {
    (values.capacity() as u64).saturating_mul(std::mem::size_of::<Value>() as u64)
}
