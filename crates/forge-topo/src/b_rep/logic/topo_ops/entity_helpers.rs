//! Miscellaneous entity helper methods on TopologyArena.
//!
//! DOMAIN: Face version bumping, edge endpoint queries, and slot counts.

use forge_core::KernelError;

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::b_rep::data::storage::slot::{validate_generation, cold_err_bounds};
use crate::handles::{FaceId, VertexId};

impl TopologyArena {
    /// Bump the version of a face slot without requiring mutable data access.
    ///
    /// Used by operators to mark a face as "dirty" when its boundary
    /// half-edges are rewired. This enables the diff engine to detect
    /// transitive face modifications even when `FaceData` fields are unchanged.
    pub fn bump_face_version(&mut self, id: FaceId) -> Result<(), KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Face", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        slot.version += 1;
        Ok(())
    }

    /// Helper to fetch topological endpoints of an undirected edge.
    pub fn get_edge_endpoints(
        &self,
        edge_id: crate::handles::EdgeId,
    ) -> Result<(VertexId, VertexId), KernelError> {
        let he_id = self.get_edge(edge_id)?.half_edge();
        let he = self.get_half_edge(he_id)?;
        let origin = he.origin();
        let dest = self.get_half_edge(he.next())?.origin();
        Ok((origin, dest))
    }

    /// Total face slot count (including vacant slots).
    pub fn face_slot_count(&self) -> usize { self.face_slots.len() }

    /// Total halfedge slot count (including vacant slots).
    pub fn half_edge_slot_count(&self) -> usize { self.half_edge_slots.len() }

    /// Total vertex slot count (including vacant slots).
    pub fn vertex_slot_count(&self) -> usize { self.vertex_slots.len() }
}
