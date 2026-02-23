//! Structural signature computation for the change firewall.
//!
//! DOMAIN: Permutation-robust structural signatures of topological entities.
//!
//! INVARIANTS:
//! - Same topology → same hash (D1 determinism)
//! - Geometry-only changes do NOT alter the topology hash
//! - Entity hashes are order-independent (sorted before aggregation)
//! - Hashes are **index-independent**: isomorphic topologies with different
//!   arena slot assignments produce the same hash (permutation invariance)
//!
//! NOTE: This is a change-detection tool, NOT a canonical graph isomorphism
//! test. Two non-isomorphic topologies may (rarely) hash to the same value.
//!
//! DEPENDENCIES: `arena` (entity data), `lineage` (inline provenance)



use crate::arena::TopologyArena;
use crate::lineage::Lineage;
use forge_core::EntityKind;

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

/// Compute a permutation-robust structural signature from an arena.
///
/// This is the foundation of the signal engine's change firewall:
/// if this hash doesn't change, topology-dependent signals stay `Clean`.
///
/// NOTE: This is a structural signature for change detection, not a
/// canonical graph isomorphism test. Collisions are theoretically
/// possible but statistically improbable for practical topologies.
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
    //
    // OPTIMIZATION: We use Vec<T> instead of HashMap<u32, T> because
    // entity indices are dense 0..N integers. This avoids hashing overhead
    // and improves cache locality.

    // Used to expand vectors on demand (sparse map pattern)
    fn ensure_capacity<T: Default + Clone>(vec: &mut Vec<T>, index: usize) {
        if index >= vec.len() {
            vec.resize(index + 1, T::default());
        }
    }

    let mut vertex_degree: Vec<u64> = Vec::new();
    let mut face_size: Vec<u64> = Vec::new();
    let mut face_he_origins: Vec<Vec<u32>> = Vec::new();
    let mut vertex_he_faces: Vec<Vec<u32>> = Vec::new();

    for (_he_id, he) in arena.iter_half_edges() {
        let v_idx = he.origin().index() as usize;
        let f_idx = he.face().index() as usize;

        ensure_capacity(&mut vertex_degree, v_idx);
        vertex_degree[v_idx] += 1;

        ensure_capacity(&mut face_size, f_idx);
        face_size[f_idx] += 1;

        ensure_capacity(&mut face_he_origins, f_idx);
        face_he_origins[f_idx].push(v_idx as u32);

        ensure_capacity(&mut vertex_he_faces, v_idx);
        vertex_he_faces[v_idx].push(f_idx as u32);
    }

    let mut entity_hashes = Vec::new();

    // ── Step 2: Face hashes ─────────────────────────────────────
    //
    // Signature = sorted multiset of vertex degrees around the face.
    // e.g. a triangle with one degree-4 and two degree-3 vertices → [3, 3, 4]

    for (face_id, _) in arena.iter_faces() {
        let f_idx = face_id.index() as usize;
        let mut sig: Vec<u64> = if f_idx < face_he_origins.len() {
            face_he_origins[f_idx]
                .iter()
                .map(|&v| {
                    let v_idx = v as usize;
                    if v_idx < vertex_degree.len() {
                        vertex_degree[v_idx]
                    } else {
                        0
                    }
                })
                .collect()
        } else {
             Vec::new()
        };
        sig.sort_unstable();
        let hash = compute_entity_hash(EntityKind::Face, &sig, None);
        entity_hashes.push(hash);
    }

    // ── Step 3: Halfedge hashes ─────────────────────────────────
    //
    // Signature = (origin_degree, twin_origin_degree, face_size, twin_face_size).
    // All four values are index-free structural properties.

    for (_he_id, he) in arena.iter_half_edges() {
        let origin_idx = he.origin().index() as usize;
        let face_idx = he.face().index() as usize;

        let origin_deg = if origin_idx < vertex_degree.len() { vertex_degree[origin_idx] } else { 0 };
        let my_face_sz = if face_idx < face_size.len() { face_size[face_idx] } else { 0 };

        let (twin_origin_deg, twin_face_sz) = match arena.get_half_edge(he.radial_next()) {
            Ok(twin) => {
                let t_origin_idx = twin.origin().index() as usize;
                let t_face_idx = twin.face().index() as usize;
                (
                    if t_origin_idx < vertex_degree.len() { vertex_degree[t_origin_idx] } else { 0 },
                    if t_face_idx < face_size.len() { face_size[t_face_idx] } else { 0 }
                )
            },
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
        let v_idx = vtx_id.index() as usize;
        let mut sig: Vec<u64> = if v_idx < vertex_he_faces.len() {
            vertex_he_faces[v_idx]
                .iter()
                .map(|&f| {
                    let f_idx = f as usize;
                    if f_idx < face_size.len() {
                        face_size[f_idx]
                    } else {
                        0
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        sig.sort_unstable();
        let hash = compute_entity_hash(EntityKind::Vertex, &sig, None);
        entity_hashes.push(hash);
    }

    // ── Pre-compute Shell topological sizes ─────────────────────
    let mut shell_face_sizes = std::collections::BTreeMap::new();
    for (face_id, face_data) in arena.iter_faces() {
        let f_idx = face_id.index() as usize;
        let mut sz = 0;
        if f_idx < face_size.len() {
            sz = face_size[f_idx];
        }
        shell_face_sizes.entry(face_data.shell().index()).or_insert_with(Vec::new).push(sz);
    }

    // ── Step 5: Shell and Edge hashes ───────────────────────────
    for (shell_id, _) in arena.iter_shells() {
        let mut sig = shell_face_sizes.remove(&shell_id.index()).unwrap_or_default();
        sig.sort_unstable();
        let hash = compute_entity_hash(EntityKind::Shell, &sig, None);
        entity_hashes.push(hash);
    }

    for (_, edge) in arena.iter_edges() {
        let mut sig = Vec::new();
        if let Ok(he) = arena.get_half_edge(edge.half_edge()) {
            let origin_idx = he.origin().index() as usize;
            sig.push(if origin_idx < vertex_degree.len() { vertex_degree[origin_idx] } else { 0 });
            
            if let Ok(twin) = arena.get_half_edge(he.radial_next()) {
                let twin_origin_idx = twin.origin().index() as usize;
                sig.push(if twin_origin_idx < vertex_degree.len() { vertex_degree[twin_origin_idx] } else { 0 });
            }
        }
        sig.sort_unstable();
        let hash = compute_entity_hash(EntityKind::Edge, &sig, None);
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
    match crate::traverse::FaceEdgeIterator::new(arena, face_id) {
        Ok(iter) => {
            // Collect to Vec for canonical rotation
            let edges: Vec<crate::handles::HalfEdgeId> = match iter.collect() {
                Ok(e) => e,
                Err(_) => return Vec::new(),
            };

            if edges.is_empty() {
                return Vec::new();
            }
            // Find the index of the halfedge with the minimum ID
            let min_pos = match edges.iter()
                .enumerate()
                .min_by_key(|(_, he)| he.index()) {
                    Some((pos, _)) => pos,
                    None => return Vec::new(),
                };
            
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
    match crate::traverse::VertexRingIterator::new(arena, vertex_id) {
        Ok(iter) => {
            let edges: Vec<crate::handles::HalfEdgeId> = match iter.collect() {
                Ok(e) => e,
                Err(_) => return Vec::new(),
            };

            if edges.is_empty() {
                return Vec::new();
            }
             // Find the index of the halfedge with the minimum ID
             let min_pos = match edges.iter()
                .enumerate()
                .min_by_key(|(_, he)| he.index()) {
                    Some((pos, _)) => pos,
                    None => return Vec::new(),
                };
         
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
        EntityKind::Loop => 4,
        EntityKind::Solid => 5,
        EntityKind::Shell => 6,
        EntityKind::Edge => 7,
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