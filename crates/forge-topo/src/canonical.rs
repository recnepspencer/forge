//! Canonical ordering helpers for deterministic outputs.
//!
//! Rule: externally observed sequences must be sorted by stable keys.

use forge_core::EntityRef;

use crate::provenance::LineageEntityRef;

#[inline]
fn kind_rank(kind: forge_core::EntityKind) -> u8 {
    match kind {
        forge_core::EntityKind::Body => 0,
        forge_core::EntityKind::Lump => 1,
        forge_core::EntityKind::Region => 2,
        forge_core::EntityKind::Shell => 3,
        forge_core::EntityKind::Face => 4,
        forge_core::EntityKind::Loop => 5,
        forge_core::EntityKind::Edge => 6,
        forge_core::EntityKind::HalfEdge => 7,
        forge_core::EntityKind::Vertex => 8,
    }
}

/// Stable tuple key for canonical entity ordering.
pub fn entity_ref_key(entity: &EntityRef) -> (u8, u32, u32) {
    (
        kind_rank(entity.kind()),
        entity.index(),
        entity.generation(),
    )
}

/// Sort entity refs by `(kind, index, generation)`.
pub fn sort_entity_refs(entities: &mut [EntityRef]) {
    entities.sort_by_key(entity_ref_key);
}

/// Return a sorted entity vector from any iterator.
pub fn sorted_entity_refs<I>(iter: I) -> Vec<EntityRef>
where
    I: IntoIterator<Item = EntityRef>,
{
    let mut v: Vec<_> = iter.into_iter().collect();
    sort_entity_refs(&mut v);
    v
}

/// Stable tuple key for canonical lineage snapshot ordering.
pub fn lineage_snapshot_key(snapshot: LineageEntityRef) -> (u8, u32, u32) {
    (
        kind_rank(snapshot.kind()),
        snapshot.index(),
        snapshot.generation(),
    )
}

/// Sort lineage snapshots by `(kind, index, generation)`.
pub fn sort_lineage_snapshots(snapshots: &mut [LineageEntityRef]) {
    snapshots.sort_by_key(|s| lineage_snapshot_key(*s));
}

/// Canonicalize key-value payload fields by field name.
pub fn sort_payload_fields(fields: &mut [(String, Vec<u8>)]) {
    fields.sort_by(|a, b| a.0.cmp(&b.0));
}
