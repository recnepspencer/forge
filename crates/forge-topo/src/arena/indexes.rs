//! O(1) reverse indexes for parent→child entity lookups.
//!
//! DOMAIN: Maintains `BTreeMap` indexes that map container entities
//! (Shell, Face, Vertex) to their contained entities (Face→HalfEdges,
//! Shell→Faces, Vertex→HalfEdges). These are derived data, rebuilt
//! from entity fields on deserialization.
//!
//! INVARIANTS:
//! - Indexes are always in sync with entity data fields
//! - Indexes are `#[serde(skip)]` and rebuilt via `rebuild_indexes()`
//! - Reassignment methods update both entity data and indexes atomically

use std::collections::BTreeMap;

use forge_core::KernelError;

use crate::arena::core::TopologyArena;
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

    /// Move a face from one shell to another, updating both the entity and the index.
    pub fn reassign_face_shell(&mut self, face: FaceId, new_shell: ShellId) -> Result<(), KernelError> {
        let old_shell = self.get_face(face)?.shell();
        self.index_remove_face(face, old_shell);
        self.get_face_mut(face)?.set_shell(new_shell);
        self.index_add_face(face, new_shell);
        Ok(())
    }

    /// Move a halfedge to a different face, updating both the entity and the index.
    pub fn reassign_halfedge_face(&mut self, he: HalfEdgeId, new_face: FaceId) -> Result<(), KernelError> {
        let old_face = self.get_half_edge(he)?.face();
        if let Some(hes) = self.face_halfedges.get_mut(&old_face) {
            if let Some(pos) = hes.iter().position(|&h| h == he) {
                hes.swap_remove(pos);
            }
        }
        self.get_half_edge_mut(he)?.set_face(new_face);
        self.face_halfedges.entry(new_face).or_default().push(he);
        Ok(())
    }

    /// Move a halfedge to a different origin vertex, updating both the entity and the index.
    pub fn reassign_halfedge_origin(&mut self, he: HalfEdgeId, new_origin: VertexId) -> Result<(), KernelError> {
        let old_origin = self.get_half_edge(he)?.origin();
        if let Some(hes) = self.vertex_halfedges.get_mut(&old_origin) {
            if let Some(pos) = hes.iter().position(|&h| h == he) {
                hes.swap_remove(pos);
            }
        }
        self.get_half_edge_mut(he)?.set_origin(new_origin);
        self.vertex_halfedges.entry(new_origin).or_default().push(he);
        Ok(())
    }

    /// Rebuild all indexes from entity data. Called after deserialization.
    pub fn rebuild_indexes(&mut self) {
        self.shell_faces.clear();
        self.face_halfedges.clear();
        self.vertex_halfedges.clear();

        for (face_id, face_data) in self.iter_faces_raw() {
            self.shell_faces
                .entry(face_data.shell())
                .or_default()
                .push(face_id);
        }

        for (he_id, he_data) in self.iter_half_edges_raw() {
            self.face_halfedges
                .entry(he_data.face())
                .or_default()
                .push(he_id);
            self.vertex_halfedges
                .entry(he_data.origin())
                .or_default()
                .push(he_id);
        }
    }

    /// Raw face iteration for index rebuilding (avoids borrow conflicts).
    fn iter_faces_raw(&self) -> Vec<(FaceId, crate::arena::mesh_schema::FaceData)> {
        self.face_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((FaceId::new(i as u32, slot.generation), data.clone()))
        }).collect()
    }

    /// Raw halfedge iteration for index rebuilding (avoids borrow conflicts).
    fn iter_half_edges_raw(&self) -> Vec<(HalfEdgeId, crate::arena::mesh_schema::HalfEdgeData)> {
        self.half_edge_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((HalfEdgeId::new(i as u32, slot.generation), data.clone()))
        }).collect()
    }
}
