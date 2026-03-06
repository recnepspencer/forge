//! Entity reassignment methods for cross-container moves.
//!
//! DOMAIN: Move faces between shells, halfedges between faces/vertices,
//! updating both entity data and reverse indexes atomically.

use forge_core::KernelError;

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, ShellId, VertexId};

impl TopologyArena {
    /// Move a face from one shell to another, updating both the entity and the index.
    pub fn reassign_face_shell(
        &mut self,
        face: FaceId,
        new_shell: ShellId,
    ) -> Result<(), KernelError> {
        let old_shell = self.get_face(face)?.shell();
        self.index_remove_face(face, old_shell);
        self.get_face_mut(face)?.set_shell(new_shell);
        self.index_add_face(face, new_shell);
        Ok(())
    }

    /// Move a halfedge to a different face, updating both the entity and the index.
    pub fn reassign_halfedge_face(
        &mut self,
        he: HalfEdgeId,
        new_face: FaceId,
    ) -> Result<(), KernelError> {
        let old_face = self.get_half_edge(he)?.face();
        let origin = self.get_half_edge(he)?.origin();
        self.index_remove_halfedge(he, old_face, origin);
        self.get_half_edge_mut(he)?.set_face(new_face);
        self.index_add_halfedge(he, new_face, origin);
        Ok(())
    }

    /// Move a halfedge to a different origin vertex, updating both the entity and the index.
    pub fn reassign_halfedge_origin(
        &mut self,
        he: HalfEdgeId,
        new_origin: VertexId,
    ) -> Result<(), KernelError> {
        let old_origin = self.get_half_edge(he)?.origin();
        let face = self.get_half_edge(he)?.face();
        self.index_remove_halfedge(he, face, old_origin);
        self.get_half_edge_mut(he)?.set_origin(new_origin);
        self.index_add_halfedge(he, face, new_origin);
        Ok(())
    }
}
