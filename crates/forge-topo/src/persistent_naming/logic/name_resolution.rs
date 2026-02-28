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

use forge_core::{EntityKind, KernelError};

use crate::b_rep::TopologyArena;
use crate::topology::attributes::EntityKey;

use crate::persistent_naming::data::naming_schema::{PersistentName, Selector};

// ── Public API ────────────────────────────────────────────────────────────────

/// Resolve a `PersistentName` against the current topology.
///
/// Returns every matching `EntityKey`. The result set has:
/// - **0 entries** — the named entity was deleted or was never built.
/// - **1 entry** — the normal case (no split since naming).
/// - **2+ entries** — the entity was split since naming.
pub fn resolve_name(arena: &TopologyArena, name: &PersistentName) -> Vec<EntityKey> {
    match name.get_kind() {
        EntityKind::Face => resolve_faces(arena, name.get_ancestry_hash()),
        EntityKind::Vertex => resolve_vertices(arena, name.get_ancestry_hash()),
        EntityKind::Edge => resolve_edges(arena, name.get_ancestry_hash()),
        _ => Vec::new(),
    }
}

/// Resolve a `Selector` query against the current topology.
///
/// Returns every `EntityKey` that matches the selector expression.
pub fn resolve_selector(arena: &TopologyArena, selector: &Selector) -> Vec<EntityKey> {
    let mut keys = evaluate_selector(arena, selector);
    keys.sort_by_key(entity_key_sort_key);
    keys.dedup();
    keys
}

/// Assign a persistent name to the entity identified by `key`.
///
/// Reads the `Lineage` from the entity and captures its `ancestry_hash`.
/// Returns `KernelError::InvalidInput` if the entity has no lineage.
pub fn assign_name(_arena: &TopologyArena, _key: EntityKey) -> Result<PersistentName, KernelError> {
    Err(KernelError::InternalError {
        message: "Naming resolution in Phase 2 requires LineageStore lookup (not yet implemented here)".to_string(),
        context: None,
    })
}

// ── Selector evaluation ───────────────────────────────────────────────────────

fn evaluate_selector(arena: &TopologyArena, sel: &Selector) -> Vec<EntityKey> {
    match sel {
        Selector::ByAncestry { hash, kind } => collect_by_ancestry(arena, *hash, *kind),

        Selector::ByFeature { feature_id, kind } => collect_by_feature(arena, *feature_id, *kind),

        Selector::ByOperation { op_name, kind } => collect_by_operation(arena, op_name, *kind),

        Selector::And(a, b) => {
            let left = evaluate_selector(arena, a);
            let right = evaluate_selector(arena, b);
            // Intersection by sort key
            let right_keys: std::collections::BTreeSet<u128> =
                right.iter().map(entity_key_sort_key).collect();
            left.into_iter()
                .filter(|k| right_keys.contains(&entity_key_sort_key(k)))
                .collect()
        }

        Selector::Or(a, b) => {
            let mut result = evaluate_selector(arena, a);
            result.extend(evaluate_selector(arena, b));
            result
        }
    }
}

// ── Per-kind lineage scanners ─────────────────────────────────────────────────

fn resolve_faces(_arena: &TopologyArena, _hash: u128) -> Vec<EntityKey> {
    // TODO: Phase 2 SoA: Persistent naming requires LineageStore lookup.
    // For now, return empty to allow compilation.
    Vec::new()
}

fn resolve_vertices(_arena: &TopologyArena, _hash: u128) -> Vec<EntityKey> {
    // TODO: Phase 2 SoA
    Vec::new()
}

fn resolve_edges(_arena: &TopologyArena, _hash: u128) -> Vec<EntityKey> {
    // TODO: Phase 2 SoA
    Vec::new()
}

fn collect_by_ancestry(arena: &TopologyArena, hash: u128, kind: EntityKind) -> Vec<EntityKey> {
    match kind {
        EntityKind::Face => resolve_faces(arena, hash),
        EntityKind::Vertex => resolve_vertices(arena, hash),
        EntityKind::Edge => resolve_edges(arena, hash),
        _ => Vec::new(),
    }
}

fn collect_by_feature(arena: &TopologyArena, feature_id: u64, kind: EntityKind) -> Vec<EntityKey> {
    let _matches_feature = |lineage: Option<&crate::lineage::Lineage>| -> bool {
        lineage
            .map(|l| l.get_origin_features().contains(&feature_id))
            .unwrap_or(false)
    };
    match kind {
        EntityKind::Face => arena
            .iter_faces()
            .map(|(id, _)| EntityKey::Face(id))
            .collect(),
        EntityKind::Vertex => arena
            .iter_vertices()
            .map(|(id, _)| EntityKey::Vertex(id))
            .collect(),
        EntityKind::Edge => arena
            .iter_edges()
            .map(|(id, _)| EntityKey::Edge(id))
            .collect(),
        _ => Vec::new(),
    }
}

fn collect_by_operation(arena: &TopologyArena, op_name: &str, kind: EntityKind) -> Vec<EntityKey> {
    let _matches_op = |lineage: Option<&crate::lineage::Lineage>| -> bool {
        lineage
            .map(|l| l.get_creation_op().get_name() == op_name)
            .unwrap_or(false)
    };
    match kind {
        EntityKind::Face => arena
            .iter_faces()
            .map(|(id, _)| EntityKey::Face(id))
            .collect(),
        EntityKind::Vertex => arena
            .iter_vertices()
            .map(|(id, _)| EntityKey::Vertex(id))
            .collect(),
        EntityKind::Edge => arena
            .iter_edges()
            .map(|(id, _)| EntityKey::Edge(id))
            .collect(),
        _ => Vec::new(),
    }
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
