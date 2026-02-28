//! Non-generic arena methods that don't fit the uniform CRUD pattern.
//!
//! DOMAIN: Specialized operations on Face (bump_face_version),
//! HalfEdge (insert_radial_pair), and Edge (get_edge_endpoints).

use forge_core::KernelError;

use crate::arena::core::TopologyArena;
use crate::arena::slot::{validate_generation, cold_err_bounds};
use crate::arena::mesh_schema::HalfEdgeData;
use crate::handles::{FaceId, HalfEdgeId, VertexId};

impl TopologyArena {
    /// Insert a pair of radial halfedges and wire their `radial_next` fields reciprocally.
    ///
    /// Returns `(he_a, he_b)` where `he_a.radial_next == he_b` and `he_b.radial_next == he_a`.
    pub(crate) fn insert_radial_pair(
        &mut self,
        mut data_a: HalfEdgeData,
        mut data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        data_a.set_radial_next(HalfEdgeId::new(u32::MAX, 0));
        data_b.set_radial_next(HalfEdgeId::new(u32::MAX, 0));

        let he_a_id = self.insert_half_edge(data_a);
        let he_b_id = self.insert_half_edge(data_b);

        if let Some(he_a) = self.half_edge_slots[he_a_id.index() as usize].data.as_mut() {
            he_a.set_radial_next(he_b_id);
        }
        if let Some(he_b) = self.half_edge_slots[he_b_id.index() as usize].data.as_mut() {
            he_b.set_radial_next(he_a_id);
        }

        (he_a_id, he_b_id)
    }

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
