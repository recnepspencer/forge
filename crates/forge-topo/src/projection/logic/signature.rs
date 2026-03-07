use std::collections::BTreeMap;

use forge_core::EntityKind;

use crate::projection::data::ProjectedTopology;
use crate::transactions::facade::{compute_entity_hash, compute_solid_hash};

/// Compute a permutation-robust structural signature for projected topology.
///
/// This intentionally mirrors the legacy arena signature shape so parity
/// checks compare architectural equivalence rather than projection-local ids.
pub fn compute_projected_topology_hash(projected: &ProjectedTopology) -> u128 {
    let mut vertex_degree = vec![0_u64; projected.vertex_count()];
    let mut face_size = vec![0_u64; projected.face_count()];
    let mut face_he_origins = vec![Vec::new(); projected.face_count()];
    let mut vertex_he_faces = vec![Vec::new(); projected.vertex_count()];

    for half_edge in projected.half_edges() {
        let v_idx = half_edge.origin.index();
        let f_idx = half_edge.face.index();

        vertex_degree[v_idx] += 1;
        face_size[f_idx] += 1;
        face_he_origins[f_idx].push(v_idx as u32);
        vertex_he_faces[v_idx].push(f_idx as u32);
    }

    let mut entity_hashes = Vec::new();

    for face_id in 0..projected.face_count() {
        let mut sig: Vec<u64> = face_he_origins[face_id]
            .iter()
            .map(|&vertex| vertex_degree[vertex as usize])
            .collect();
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Face, &sig, None));
    }

    for half_edge in projected.half_edges() {
        let origin_deg = vertex_degree[half_edge.origin.index()];
        let my_face_sz = face_size[half_edge.face.index()];
        let twin = projected.half_edge(half_edge.radial_next);
        let twin_origin_deg = vertex_degree[twin.origin.index()];
        let twin_face_sz = face_size[twin.face.index()];
        let connectivity = [origin_deg, twin_origin_deg, my_face_sz, twin_face_sz];
        entity_hashes.push(compute_entity_hash(EntityKind::HalfEdge, &connectivity, None));
    }

    for vertex_id in 0..projected.vertex_count() {
        let mut sig: Vec<u64> = vertex_he_faces[vertex_id]
            .iter()
            .map(|&face| face_size[face as usize])
            .collect();
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Vertex, &sig, None));
    }

    let mut shell_face_sizes: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    for (face_idx, face) in projected.faces().iter().enumerate() {
        shell_face_sizes
            .entry(face.shell.index())
            .or_default()
            .push(face_size[face_idx]);
    }

    for shell_id in 0..projected.shell_count() {
        let mut sig = shell_face_sizes.remove(&shell_id).unwrap_or_default();
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Shell, &sig, None));
    }

    for edge in projected.edges() {
        let he = projected.half_edge(edge.half_edge);
        let twin = projected.half_edge(he.radial_next);
        let mut sig = vec![
            vertex_degree[he.origin.index()],
            vertex_degree[twin.origin.index()],
        ];
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Edge, &sig, None));
    }

    for region in projected.regions() {
        let mut sig = Vec::with_capacity(region.shells.len() + 1);
        sig.push(region.shells.len() as u64);
        for shell in &region.shells {
            sig.push(shell.index() as u64);
        }
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Region, &sig, None));
    }

    for lump in projected.lumps() {
        let mut sig: Vec<u64> = lump.regions.iter().map(|region| region.index() as u64).collect();
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Lump, &sig, None));
    }

    for body in projected.bodies() {
        let mut sig: Vec<u64> = body.lumps.iter().map(|lump| lump.index() as u64).collect();
        sig.sort_unstable();
        entity_hashes.push(compute_entity_hash(EntityKind::Body, &sig, None));
    }

    compute_solid_hash(&entity_hashes)
}
