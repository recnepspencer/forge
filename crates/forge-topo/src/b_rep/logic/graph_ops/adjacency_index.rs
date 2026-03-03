//! O(1) reverse indexes for parent→child entity lookups.
//!
//! DOMAIN: Maintains IndexMap indexes that map container entities
//! to their contained entities. These are derived data, rebuilt
//! from entity fields on deserialization.
//!
//! INVARIANTS:
//! - Indexes are always in sync with entity data fields
//! - Indexes are `#[serde(skip)]` and rebuilt via `rebuild_indexes()`

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, ShellId, VertexId};

impl TopologyArena {
    /// Faces belonging to a given shell. Returns empty slice if shell unknown.
    pub fn faces_of_shell(&self, shell: ShellId) -> &[FaceId] {
        self.shell_faces
            .get(&shell)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Halfedges belonging to a given face. Returns empty slice if face unknown.
    pub fn halfedges_of_face(&self, face: FaceId) -> &[HalfEdgeId] {
        self.face_halfedges
            .get(&face)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Halfedges originating from a given vertex. Returns empty slice if vertex unknown.
    pub fn halfedges_from_vertex(&self, vertex: VertexId) -> &[HalfEdgeId] {
        self.vertex_halfedges
            .get(&vertex)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Register a face in the shell→faces index (called from insert_face).
    pub(crate) fn index_add_face(&mut self, face: FaceId, shell: ShellId) {
        self.shell_faces.entry(shell).or_default().push(face);
    }

    /// Remove a face from the shell→faces index (called from remove_face).
    pub(crate) fn index_remove_face(&mut self, face: FaceId, shell: ShellId) {
        if let Some(faces) = self.shell_faces.get_mut(&shell) {
            if let Some(pos) = faces.iter().position(|&f| f == face) {
                faces.swap_remove(pos);
            }
        }
    }

    /// Register a halfedge in both face→halfedges and vertex→halfedges indexes.
    pub(crate) fn index_add_halfedge(&mut self, he: HalfEdgeId, face: FaceId, origin: VertexId) {
        self.face_halfedges.entry(face).or_default().push(he);
        self.vertex_halfedges.entry(origin).or_default().push(he);
    }

    /// Remove a halfedge from both face→halfedges and vertex→halfedges indexes.
    pub(crate) fn index_remove_halfedge(&mut self, he: HalfEdgeId, face: FaceId, origin: VertexId) {
        if let Some(hes) = self.face_halfedges.get_mut(&face) {
            if let Some(pos) = hes.iter().position(|&h| h == he) {
                hes.swap_remove(pos);
            }
        }
        if let Some(hes) = self.vertex_halfedges.get_mut(&origin) {
            if let Some(pos) = hes.iter().position(|&h| h == he) {
                hes.swap_remove(pos);
            }
        }
    }

    /// Rebuild all indexes from entity data. Called after deserialization.
    ///
    /// Collects only the needed fields (not full entity clones) to avoid
    /// unnecessary allocation at scale.
    pub fn rebuild_indexes(&mut self) {
        self.shell_faces.clear();
        self.face_halfedges.clear();
        self.vertex_halfedges.clear();

        // Collect (face_id, shell) — no FaceData clone
        let face_shells: Vec<_> = self.face_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((FaceId::new(i as u32, slot.generation), data.shell()))
        }).collect();

        for (face_id, shell) in face_shells {
            self.shell_faces.entry(shell).or_default().push(face_id);
        }

        // Collect (he_id, face, origin) — no HalfEdgeData clone
        let he_refs: Vec<_> = self.half_edge_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((HalfEdgeId::new(i as u32, slot.generation), data.face(), data.origin()))
        }).collect();

        for (he_id, face, origin) in he_refs {
            self.face_halfedges.entry(face).or_default().push(he_id);
            self.vertex_halfedges.entry(origin).or_default().push(he_id);
        }
    }
}
