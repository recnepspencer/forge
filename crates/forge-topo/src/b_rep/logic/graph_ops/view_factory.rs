//! Factory methods on TopologyArena for creating entity views.
//!
//! DOMAIN: Constructs read-only view wrappers that unify connectivity
//! (from entity structs) with side-car metadata (from arena vectors).

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::b_rep::logic::views::{EdgeView, HalfEdgeView, VertexView};
use crate::handles::{EdgeId, HalfEdgeId, VertexId};
use forge_core::KernelError;

impl TopologyArena {
    /// Get a read-only view of a halfedge (connectivity + side-car metadata).
    ///
    /// This is the preferred access pattern for code that needs both
    /// connectivity pointers and metadata like `is_bridge` or `coedge_info`.
    #[inline]
    pub fn view_half_edge(&self, id: HalfEdgeId) -> Result<HalfEdgeView<'_>, KernelError> {
        let data = self.get_half_edge(id)?;
        Ok(HalfEdgeView::new(id, data, self))
    }

    /// Get a read-only view of a vertex (connectivity + side-car metadata).
    #[inline]
    pub fn view_vertex(&self, id: VertexId) -> Result<VertexView<'_>, KernelError> {
        let data = self.get_vertex(id)?;
        Ok(VertexView::new(id, data, self))
    }

    /// Get a read-only view of an edge (connectivity + side-car metadata).
    #[inline]
    pub fn view_edge(&self, id: EdgeId) -> Result<EdgeView<'_>, KernelError> {
        let data = self.get_edge(id)?;
        Ok(EdgeView::new(id, data, self))
    }
}
