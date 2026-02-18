//! Topology hashing for the change firewall.
//!
//! DOMAIN: Structural hashing of topological entities and solids.
//!
//! INVARIANTS:
//! - Same topology → same hash (D1 determinism)
//! - Geometry-only changes do NOT alter the topology hash
//! - Entity hashes are order-independent (sorted before aggregation)
//! - Hashes are **index-independent**: isomorphic topologies with different
//!   arena slot assignments produce the same hash (permutation invariance)
//!
//! DEPENDENCIES: `arena` (entity data), `lineage` (inline provenance)

use std::collections::HashMap;

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

/// Compute the aggregate topology hash from an arena using **structural
/// descriptors** rather than raw slot indices.
///
/// This is the foundation of the signal engine's change firewall:
/// if this hash doesn't change, topology-dependent signals stay `Clean`.
///
/// # Permutation invariance
///
/// Instead of hashing raw entity indices (which differ when the same
/// operations are applied in different orders), we hash *structural
/// neighbourhood descriptors*: vertex degree, face size, and the
/// multisets of those values around each entity.  Two isomorphic
/// topologies therefore produce the same hash regardless of arena
/// slot assignment (D1).
///
/// Lineage is intentionally **excluded** from the structural hash
/// because it encodes *provenance*, not *connectivity*.  Two meshes
/// with identical wiring but different operation histories must hash
/// identically for the change firewall to work.
pub fn compute_arena_topology_hash(arena: &TopologyArena) -> u128 {
    // ── Step 1: Precompute structural descriptors ───────────────
    //
    // vertex_degree[v] = number of halfedges originating from v
    // face_size[f]     = number of halfedges belonging to f
    //
    // Both are topological invariants independent of slot indices.

    let mut vertex_degree: HashMap<u32, u64> = HashMap::new();
    let mut face_size: HashMap<u32, u64> = HashMap::new();
    let mut face_he_origins: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut vertex_he_faces: HashMap<u32, Vec<u32>> = HashMap::new();

    for (_he_id, he) in arena.iter_half_edges() {
        *vertex_degree.entry(he.origin.index()).or_insert(0) += 1;
        *face_size.entry(he.face.index()).or_insert(0) += 1;
        face_he_origins
            .entry(he.face.index())
            .or_default()
            .push(he.origin.index());
        vertex_he_faces
            .entry(he.origin.index())
            .or_default()
            .push(he.face.index());
    }

    let mut entity_hashes = Vec::new();

    // ── Step 2: Face hashes ─────────────────────────────────────
    //
    // Signature = sorted multiset of vertex degrees around the face.
    // e.g. a triangle with one degree-4 and two degree-3 vertices → [3, 3, 4]

    for (face_id, _) in arena.iter_faces() {
        let mut sig: Vec<u64> = face_he_origins
            .get(&face_id.index())
            .map(|origins| {
                origins
                    .iter()
                    .map(|v| *vertex_degree.get(v).unwrap_or(&0))
                    .collect()
            })
            .unwrap_or_default();
        sig.sort_unstable();
        let hash = compute_entity_hash(EntityKind::Face, &sig, None);
        entity_hashes.push(hash);
    }

    // ── Step 3: Halfedge hashes ─────────────────────────────────
    //
    // Signature = (origin_degree, twin_origin_degree, face_size, twin_face_size).
    // All four values are index-free structural properties.

    for (_he_id, he) in arena.iter_half_edges() {
        let origin_deg = *vertex_degree.get(&he.origin.index()).unwrap_or(&0);
        let my_face_sz = *face_size.get(&he.face.index()).unwrap_or(&0);
        let (twin_origin_deg, twin_face_sz) = match arena.get_half_edge(he.twin) {
            Ok(twin) => (
                *vertex_degree.get(&twin.origin.index()).unwrap_or(&0),
                *face_size.get(&twin.face.index()).unwrap_or(&0),
            ),
            Err(_) => (0, 0),
        };
        let connectivity = [origin_deg, twin_origin_deg, my_face_sz, twin_face_sz];
        let hash = compute_entity_hash(EntityKind::HalfEdge, &connectivity, None);
        entity_hashes.push(hash);
    }

    // ── Step 4: Vertex hashes ───────────────────────────────────
    //
    // Signature = sorted multiset of incident face sizes.
    // e.g. a vertex touching a triangle and a quad → [3, 4]

    for (vtx_id, _) in arena.iter_vertices() {
        let mut sig: Vec<u64> = vertex_he_faces
            .get(&vtx_id.index())
            .map(|faces| {
                faces
                    .iter()
                    .map(|f| *face_size.get(f).unwrap_or(&0))
                    .collect()
            })
            .unwrap_or_default();
        sig.sort_unstable();
        let hash = compute_entity_hash(EntityKind::Vertex, &sig, None);
        entity_hashes.push(hash);
    }

    compute_solid_hash(&entity_hashes)
}

/// Extract the canonical loop of halfedge indices for a face.
/// 
/// Traverses the face loop, finds the halfedge with the minimum index,
/// and returns the sequence starting from there. This ensures that
/// the hash is independent of which halfedge is the "first" in the list.
#[allow(dead_code)]
fn canonical_face_loop(arena: &TopologyArena, face_id: crate::handles::FaceId) -> Vec<u64> {
     match crate::traverse::face_edges(arena, face_id) {
        Ok(edges) => {
            if edges.is_empty() {
                return Vec::new();
            }
            // Find the index of the halfedge with the minimum ID
            let (min_pos, _) = edges.iter()
                .enumerate()
                .min_by_key(|(_, he)| he.index())
                .unwrap(); // edges is not empty
            
            // Reorder: elements from min_pos to end, then 0 to min_pos
            let mut canonical = Vec::with_capacity(edges.len());
            for i in 0..edges.len() {
                canonical.push(edges[(min_pos + i) % edges.len()].index() as u64);
            }
            canonical
        },
        Err(_) => Vec::new(),
    }
}

/// Extract the canonical ring of outgoing halfedges for a vertex.
/// 
/// Traverses the vertex star, finds the halfedge with the minimum index,
/// and returns the sequence starting from there.
#[allow(dead_code)]
fn canonical_vertex_ring(arena: &TopologyArena, vertex_id: crate::handles::VertexId) -> Vec<u64> {
     match crate::traverse::vertex_ring(arena, vertex_id) {
        Ok(edges) => {
            if edges.is_empty() {
                return Vec::new();
            }
             // Find the index of the halfedge with the minimum ID
             let (min_pos, _) = edges.iter()
             .enumerate()
             .min_by_key(|(_, he)| he.index())
             .unwrap(); // edges is not empty
         
             // Reorder: elements from min_pos to end, then 0 to min_pos
             let mut canonical = Vec::with_capacity(edges.len());
             for i in 0..edges.len() {
                 canonical.push(edges[(min_pos + i) % edges.len()].index() as u64);
             }
             canonical
        },
        Err(_) => Vec::new(),
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