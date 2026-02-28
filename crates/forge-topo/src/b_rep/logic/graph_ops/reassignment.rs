//! Entity reassignment methods for cross-container moves.
//!
//! DOMAIN: Move faces between shells, halfedges between faces/vertices,
//! updating both entity data and reverse indexes atomically.

use forge_core::KernelError;

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, ShellId, VertexId};

impl TopologyArena {
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
}
