//! Radial ring wiring for halfedge pairs.
//!
//! DOMAIN: Insert a pair of halfedges and wire their radial_next
//! fields reciprocally for manifold edge creation.

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::b_rep::data::mesh::half_edge::HalfEdgeData;
use crate::handles::HalfEdgeId;

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
}
