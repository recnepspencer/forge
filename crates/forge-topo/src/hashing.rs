//! Topology hashing for the change firewall.
//!
//! DOMAIN: Structural hashing of topological entities and solids.
//!
//! INVARIANTS:
//! - Same topology → same hash (D1 determinism)
//! - Geometry-only changes do NOT alter the topology hash
//! - Entity hashes are order-independent (sorted before aggregation)
//!
//! DEPENDENCIES: `arena` (entity data), `lineage` (provenance)

use crate::arena::TopologyArena;
use crate::lineage::{EntityKind, Lineage};

/// FNV-1a constants for hash computation.
const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// Compute a hash for a single topology entity.
///
/// The hash is based on entity kind and connectivity (the list
/// of adjacent entity indices), NOT on geometric positions. This
/// ensures that geometry-only changes don't alter the topology hash.
pub fn compute_entity_hash(
    entity_kind: EntityKind,
    connectivity: &[u64],
    lineage: Option<&Lineage>,
) -> u64 {
    let kind_hash = fnv_mix(FNV_OFFSET, entity_kind_discriminant(entity_kind));

    let connectivity_hash = connectivity.iter().fold(kind_hash, |h, &conn| {
        fnv_mix(h, conn)
    });

    match lineage {
        Some(lin) => {
            let with_lo = fnv_mix(connectivity_hash, lin.get_ancestry_hash() as u64);
            fnv_mix(with_lo, (lin.get_ancestry_hash() >> 64) as u64)
        }
        None => connectivity_hash,
    }
}

/// Compute a Merkle-style aggregate hash for an entire solid.
///
/// Takes per-entity hashes, sorts them for order-independence,
/// and chain-hashes them. Two topologically identical solids
/// produce the same aggregate hash regardless of entity insertion order.
pub fn compute_solid_hash(entity_hashes: &[u64]) -> u128 {
    let mut sorted = entity_hashes.to_vec();
    sorted.sort_unstable();

    let hash_lo = sorted.iter().fold(FNV_OFFSET, |h, &entity_hash| {
        fnv_mix(h, entity_hash)
    });

    let hash_hi = sorted.iter().fold(0x517cc1b727220a95_u64, |h, &entity_hash| {
        fnv_mix(h, entity_hash.rotate_left(17))
    });

    ((hash_hi as u128) << 64) | (hash_lo as u128)
}

/// Compute the aggregate topology hash from an arena.
///
/// Walks all active entities, computes per-entity hashes based on
/// connectivity and lineage, and aggregates into a solid-level hash.
///
/// This is the foundation of the signal engine's change firewall:
/// if this hash doesn't change, topology-dependent signals stay `Clean`.
pub fn compute_arena_topology_hash(arena: &TopologyArena) -> u128 {
    let mut entity_hashes = Vec::new();

    for (face_id, face) in arena.iter_faces() {
        let edge_count = count_face_edges(arena, face_id) as u64;
        let connectivity = [edge_count];
        let hash = compute_entity_hash(
            EntityKind::Face,
            &connectivity,
            face.lineage.as_ref(),
        );
        entity_hashes.push(hash);
    }

    for (_id, he) in arena.iter_half_edges() {
        let origin_degree = count_vertex_degree(arena, he.origin) as u64;
        let face_valence = count_face_edges(arena, he.face) as u64;
        let connectivity = [
            origin_degree,
            face_valence,
        ];
        let hash = compute_entity_hash(
            EntityKind::HalfEdge,
            &connectivity,
            he.lineage.as_ref(),
        );
        entity_hashes.push(hash);
    }

    for (_id, vtx) in arena.iter_vertices() {
        let degree = count_vertex_degree(arena, _id) as u64;
        let connectivity = [degree];
        let hash = compute_entity_hash(
            EntityKind::Vertex,
            &connectivity,
            vtx.lineage.as_ref(),
        );
        entity_hashes.push(hash);
    }

    compute_solid_hash(&entity_hashes)
}

/// Count the number of edges in a face loop.
fn count_face_edges(arena: &TopologyArena, face_id: crate::handles::FaceId) -> usize {
    match crate::traverse::face_edges(arena, face_id) {
        Ok(edges) => edges.len(),
        Err(_) => 0,
    }
}

/// Count the degree of a vertex (number of outgoing halfedges).
fn count_vertex_degree(arena: &TopologyArena, vertex_id: crate::handles::VertexId) -> usize {
    match crate::traverse::vertex_ring(arena, vertex_id) {
        Ok(edges) => edges.len(),
        Err(_) => 0,
    }
}

/// FNV-1a mixing step.
fn fnv_mix(hash: u64, value: u64) -> u64 {
    value.to_le_bytes().iter().fold(hash, |h, &byte| {
        (h ^ byte as u64).wrapping_mul(FNV_PRIME)
    })
}

/// Map entity kind to a discriminant for hashing.
fn entity_kind_discriminant(kind: EntityKind) -> u64 {
    match kind {
        EntityKind::Face => 1,
        EntityKind::HalfEdge => 2,
        EntityKind::Vertex => 3,
        EntityKind::Loop => 4,
        EntityKind::Solid => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lineage::{Lineage, OpSignature};

    #[test]
    fn same_entity_produces_same_hash() {
        let h1 = compute_entity_hash(EntityKind::Face, &[1, 2, 3], None);
        let h2 = compute_entity_hash(EntityKind::Face, &[1, 2, 3], None);
        assert_eq!(h1, h2);
    }

    #[test]
    fn different_connectivity_produces_different_hash() {
        let h1 = compute_entity_hash(EntityKind::Face, &[1, 2, 3], None);
        let h2 = compute_entity_hash(EntityKind::Face, &[1, 2, 4], None);
        assert_ne!(h1, h2);
    }

    #[test]
    fn different_entity_kind_produces_different_hash() {
        let h1 = compute_entity_hash(EntityKind::Face, &[1, 2, 3], None);
        let h2 = compute_entity_hash(EntityKind::Vertex, &[1, 2, 3], None);
        assert_ne!(h1, h2);
    }

    #[test]
    fn lineage_affects_hash() {
        let lineage = Lineage::root(1, OpSignature::new("test"));
        let h1 = compute_entity_hash(EntityKind::Face, &[1], None);
        let h2 = compute_entity_hash(EntityKind::Face, &[1], Some(&lineage));
        assert_ne!(h1, h2);
    }

    #[test]
    fn solid_hash_is_order_independent() {
        let h1 = compute_solid_hash(&[100, 200, 300]);
        let h2 = compute_solid_hash(&[300, 100, 200]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn solid_hash_is_deterministic() {
        let h1 = compute_solid_hash(&[42, 99, 7]);
        let h2 = compute_solid_hash(&[42, 99, 7]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn different_entity_sets_produce_different_solid_hash() {
        let h1 = compute_solid_hash(&[1, 2, 3]);
        let h2 = compute_solid_hash(&[1, 2, 4]);
        assert_ne!(h1, h2);
    }

    #[test]
    fn empty_arena_hash_is_deterministic() {
        let arena = TopologyArena::new();
        let h1 = compute_arena_topology_hash(&arena);
        let h2 = compute_arena_topology_hash(&arena);
        assert_eq!(h1, h2);
    }
}
