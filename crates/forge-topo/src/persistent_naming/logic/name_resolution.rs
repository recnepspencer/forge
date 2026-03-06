//! Persistent name resolution logic.
//!
//! DOMAIN: Query the live topology arena to find entities that match a
//! `PersistentName` or a `Selector` expression.
//!
//! INVARIANTS:
//! - Resolution is read-only — never mutates the arena.
//! - Multiple matches are valid after a split (name → many entities).
//! - A result set of zero means the entity was deleted or never created.
//!
//! DEPENDENCIES: `arena::TopologyArena`, `schema::{PersistentName, Selector}`,
//! `attributes::EntityKey`, `forge_core::KernelError`

use crate::semantic_attributes::EntityKey;
use forge_core::{EntityKind, KernelError};

use crate::persistent_naming::data::naming_schema::{PersistentName, Selector};

// ── Public API ────────────────────────────────────────────────────────────────

use crate::provenance::LineageStore;
use forge_core::EntityRef;

/// Resolve a `PersistentName` against the current topology.
///
/// Returns every matching `EntityKey`. The result set has:
/// - **0 entries** — the named entity was deleted or was never built.
/// - **1 entry** — the normal case (no split since naming).
/// - **2+ entries** — the entity was split since naming.
pub fn resolve_name(store: &LineageStore, name: &PersistentName) -> Vec<EntityKey> {
    let mut keys = match name.get_kind() {
        EntityKind::Face => resolve_faces(store, name.get_ancestry_hash()),
        EntityKind::Vertex => resolve_vertices(store, name.get_ancestry_hash()),
        EntityKind::Edge => resolve_edges(store, name.get_ancestry_hash()),
        _ => Vec::new(),
    };
    keys.sort_by_key(entity_key_sort_key);
    keys.dedup();
    keys
}

/// Resolve a `Selector` query against the current topology.
///
/// Returns every `EntityKey` that matches the selector expression.
pub fn resolve_selector(store: &LineageStore, selector: &Selector) -> Vec<EntityKey> {
    let mut keys = evaluate_selector(store, selector);
    keys.sort_by_key(entity_key_sort_key);
    keys.dedup();
    keys
}

// ... assign_name is already here ... // Oops I replaced it with a comment literally. Let's put it back.

/// Assign a persistent name to the entity identified by `key`.
///
/// Reads the `Lineage` from the entity and captures its `ancestry_hash`.
/// Returns `KernelError::InvalidInput` if the entity has no lineage.
pub fn assign_name(store: &LineageStore, key: EntityKey) -> Result<PersistentName, KernelError> {
    let eref = match key {
        EntityKey::Face(id) => EntityRef::new(EntityKind::Face, id.index(), id.generation()),
        EntityKey::Vertex(id) => EntityRef::new(EntityKind::Vertex, id.index(), id.generation()),
        EntityKey::Edge(id) => EntityRef::new(EntityKind::Edge, id.index(), id.generation()),
        EntityKey::Shell(id) => EntityRef::new(EntityKind::Shell, id.index(), id.generation()),
    };

    let lineage = store
        .get_lineage(&eref)
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("Cannot assign name: no lineage found for entity {:?}", key),
            context: None,
        })?;

    Ok(PersistentName::new(
        lineage.get_ancestry_hash(),
        eref.kind(),
        0,
    ))
}

// ── Selector evaluation ───────────────────────────────────────────────────────

fn evaluate_selector(store: &LineageStore, sel: &Selector) -> Vec<EntityKey> {
    match sel {
        Selector::ByAncestry { hash, kind } => collect_by_ancestry(store, *hash, *kind),

        Selector::ByFeature { feature_id, kind } => collect_by_feature(store, *feature_id, *kind),

        Selector::ByOperation { op_name, kind } => collect_by_operation(store, op_name, *kind),

        Selector::And(a, b) => {
            let left = evaluate_selector(store, a);
            let right = evaluate_selector(store, b);
            // Intersection by sort key
            let right_keys: std::collections::BTreeSet<u128> =
                right.iter().map(entity_key_sort_key).collect();
            left.into_iter()
                .filter(|k| right_keys.contains(&entity_key_sort_key(k)))
                .collect()
        }

        Selector::Or(a, b) => {
            let mut result = evaluate_selector(store, a);
            result.extend(evaluate_selector(store, b));
            result
        }
    }
}

// ── Per-kind lineage scanners ─────────────────────────────────────────────────

fn resolve_faces(store: &LineageStore, hash: u128) -> Vec<EntityKey> {
    let mut matches = Vec::new();
    for eref in store.active_entities() {
        if eref.kind() == EntityKind::Face {
            if let Some(lineage) = store.get_lineage(eref) {
                if lineage.get_ancestry_hash() == hash {
                    matches.push(EntityKey::Face(crate::handles::FaceId::new(
                        eref.index(),
                        eref.generation(),
                    )));
                }
            }
        }
    }
    matches
}

fn resolve_vertices(store: &LineageStore, hash: u128) -> Vec<EntityKey> {
    let mut matches = Vec::new();
    for eref in store.active_entities() {
        if eref.kind() == EntityKind::Vertex {
            if let Some(lineage) = store.get_lineage(eref) {
                if lineage.get_ancestry_hash() == hash {
                    matches.push(EntityKey::Vertex(crate::handles::VertexId::new(
                        eref.index(),
                        eref.generation(),
                    )));
                }
            }
        }
    }
    matches
}

fn resolve_edges(store: &LineageStore, hash: u128) -> Vec<EntityKey> {
    let mut matches = Vec::new();
    for eref in store.active_entities() {
        if eref.kind() == EntityKind::Edge {
            if let Some(lineage) = store.get_lineage(eref) {
                if lineage.get_ancestry_hash() == hash {
                    matches.push(EntityKey::Edge(crate::handles::EdgeId::new(
                        eref.index(),
                        eref.generation(),
                    )));
                }
            }
        }
    }
    matches
}

fn collect_by_ancestry(store: &LineageStore, hash: u128, kind: EntityKind) -> Vec<EntityKey> {
    match kind {
        EntityKind::Face => resolve_faces(store, hash),
        EntityKind::Vertex => resolve_vertices(store, hash),
        EntityKind::Edge => resolve_edges(store, hash),
        _ => Vec::new(),
    }
}

fn collect_by_feature(store: &LineageStore, feature_id: u64, kind: EntityKind) -> Vec<EntityKey> {
    let mut matches = Vec::new();
    for eref in store.active_entities() {
        if eref.kind() == kind {
            if let Some(lineage) = store.get_lineage(eref) {
                if lineage.get_origin_features().contains(&feature_id) {
                    match kind {
                        EntityKind::Face => matches.push(EntityKey::Face(
                            crate::handles::FaceId::new(eref.index(), eref.generation()),
                        )),
                        EntityKind::Vertex => matches.push(EntityKey::Vertex(
                            crate::handles::VertexId::new(eref.index(), eref.generation()),
                        )),
                        EntityKind::Edge => matches.push(EntityKey::Edge(
                            crate::handles::EdgeId::new(eref.index(), eref.generation()),
                        )),
                        _ => {}
                    }
                }
            }
        }
    }
    matches
}

fn collect_by_operation(store: &LineageStore, op_name: &str, kind: EntityKind) -> Vec<EntityKey> {
    let mut matches = Vec::new();
    for eref in store.active_entities() {
        if eref.kind() == kind {
            if let Some(lineage) = store.get_lineage(eref) {
                if lineage.get_creation_op().get_name() == op_name {
                    match kind {
                        EntityKind::Face => matches.push(EntityKey::Face(
                            crate::handles::FaceId::new(eref.index(), eref.generation()),
                        )),
                        EntityKind::Vertex => matches.push(EntityKey::Vertex(
                            crate::handles::VertexId::new(eref.index(), eref.generation()),
                        )),
                        EntityKind::Edge => matches.push(EntityKey::Edge(
                            crate::handles::EdgeId::new(eref.index(), eref.generation()),
                        )),
                        _ => {}
                    }
                }
            }
        }
    }
    matches
}

/// Map an `EntityKey` to a u128 for deterministic dedup/intersection.
///
/// Encodes kind (high bits) + index (low 32 bits).
fn entity_key_sort_key(key: &EntityKey) -> u128 {
    let (kind_tag, idx) = match key {
        EntityKey::Shell(id) => (0u128, id.index()),
        EntityKey::Face(id) => (1u128, id.index()),
        EntityKey::Edge(id) => (2u128, id.index()),
        EntityKey::Vertex(id) => (3u128, id.index()),
    };
    (kind_tag << 32) | (idx as u128)
}
