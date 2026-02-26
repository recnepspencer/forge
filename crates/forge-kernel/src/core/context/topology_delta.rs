//! Arena snapshot and topology delta computation.
//!
//! DOMAIN: Captures arena slot counts at a point in time and computes
//! the set of entities created between two snapshots.

use forge_core::TopologyDelta;
use forge_topo::arena::TopologyArena;

/// A lightweight snapshot of arena slot counts at a point in time.
///
/// Used to compute `TopologyDelta` — the set of entities created
/// between two snapshots (pre-op and post-op).
#[derive(Debug, Clone)]
pub struct ArenaSnapshot {
    face_slots: usize,
    half_edge_slots: usize,
    vertex_slots: usize,
}

impl ArenaSnapshot {
    /// Capture the current slot counts of an arena.
    pub fn capture(arena: &TopologyArena) -> Self {
        Self {
            face_slots: arena.face_slot_count(),
            half_edge_slots: arena.half_edge_slot_count(),
            vertex_slots: arena.vertex_slot_count(),
        }
    }
}

/// Compute the topology delta between a pre-operation snapshot and the
/// current arena state.
///
/// Any slot indices in `[snapshot.X_slots .. arena.X_slot_count())` are
/// entities created since the snapshot was taken.
pub fn compute_topology_delta(snapshot: &ArenaSnapshot, arena: &TopologyArena) -> TopologyDelta {
    let created_faces: Vec<u32> = (snapshot.face_slots..arena.face_slot_count())
        .filter(|&i| arena.face_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    let created_halfedges: Vec<u32> = (snapshot.half_edge_slots..arena.half_edge_slot_count())
        .filter(|&i| arena.half_edge_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    let created_vertices: Vec<u32> = (snapshot.vertex_slots..arena.vertex_slot_count())
        .filter(|&i| arena.vertex_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    TopologyDelta {
        created_faces,
        created_halfedges,
        created_vertices,
        deleted_faces: Vec::new(),
        deleted_halfedges: Vec::new(),
        deleted_vertices: Vec::new(),
    }
}
