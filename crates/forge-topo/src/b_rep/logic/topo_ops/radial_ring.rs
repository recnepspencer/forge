//! Radial ring wiring for halfedge pairs.
//!
//! DOMAIN: Insert a pair of halfedges and wire their radial_next
//! fields reciprocally for manifold edge creation.
//!
//! Uses the two-phase reserve/populate API to eliminate sentinel handles:
//! both IDs are known before either halfedge's data is populated.

use crate::b_rep::data::mesh::half_edge::HalfEdgeData;
use crate::b_rep::data::mesh::EdgeRadialClass;
use crate::b_rep::data::storage::arena::TopologyArena;
use crate::handles::HalfEdgeId;

impl TopologyArena {
    /// Insert a pair of radial halfedges with their `radial_next` fields
    /// correctly wired from the start — no sentinel handles needed.
    ///
    /// Returns `(he_a, he_b)` where `he_a.radial_next == he_b` and vice versa.
    pub(crate) fn insert_radial_pair(
        &mut self,
        mut data_a: HalfEdgeData,
        mut data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        // Phase 1: Reserve both slots to learn their IDs
        let he_a_id = self.reserve_half_edge();
        let he_b_id = self.reserve_half_edge();

        // Phase 2: Wire radial_next with real IDs, then populate
        data_a.set_radial_next(he_b_id);
        data_b.set_radial_next(he_a_id);

        self.populate_half_edge(he_a_id, data_a);
        self.populate_half_edge(he_b_id, data_b);

        (he_a_id, he_b_id)
    }

    /// Returns the twin halfedge if and only if the edge is manifold
    /// (exactly 2 uses sharing the same geometric edge).
    ///
    /// Returns `None` for:
    /// - Boundary edges (`radial_next == self`)
    /// - NMT edges (3+ halfedges in the radial ring)
    pub fn twin_if_manifold(&self, he: HalfEdgeId) -> Option<HalfEdgeId> {
        if !matches!(self.classify_half_edge(he).ok()?, EdgeRadialClass::Manifold) {
            return None;
        }
        let data = self.get_half_edge(he).ok()?;
        let partner = data.radial_next();
        Some(partner)
    }

    /// Iterate all halfedges in the radial ring around the same geometric edge.
    ///
    /// For manifold edges, returns `[he, twin]`.
    /// For boundary edges, returns `[he]`.
    /// For NMT edges, returns the full ring.
    pub fn radial_ring(&self, he: HalfEdgeId) -> Vec<HalfEdgeId> {
        let mut ring = vec![he];
        let mut current = match self.get_half_edge(he) {
            Ok(d) => d.radial_next(),
            Err(_) => return ring,
        };
        while current != he {
            ring.push(current);
            current = match self.get_half_edge(current) {
                Ok(d) => d.radial_next(),
                Err(_) => break,
            };
        }
        ring
    }
}
