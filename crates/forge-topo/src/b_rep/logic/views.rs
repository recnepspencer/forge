//! Read-only entity views that unify connectivity + side-car metadata.
//!
//! DOMAIN: Ergonomic access layer for topology consumers. Each view
//! borrows both the entity struct (connectivity) and the arena (side-car
//! metadata), exposing a unified API. This establishes the canonical
//! access pattern so all future callers go through views from day one.

use crate::b_rep::data::mesh::edge::EdgeData;
use crate::b_rep::data::mesh::half_edge::HalfEdgeData;
use crate::b_rep::data::mesh::vertex::VertexData;
use crate::b_rep::data::mesh::CoedgeInfo;
use crate::b_rep::data::storage::arena::TopologyArena;
use crate::handles::{CurveRef, EdgeId, FaceId, HalfEdgeId, VertexId};

// ── HalfEdgeView ────────────────────────────────────────────────────

/// Read-only view of a halfedge: connectivity pointers + side-car metadata.
pub struct HalfEdgeView<'a> {
    id: HalfEdgeId,
    data: &'a HalfEdgeData,
    arena: &'a TopologyArena,
}

impl<'a> HalfEdgeView<'a> {
    pub(crate) fn new(id: HalfEdgeId, data: &'a HalfEdgeData, arena: &'a TopologyArena) -> Self {
        Self { id, data, arena }
    }

    /// The handle of this halfedge.
    pub fn id(&self) -> HalfEdgeId {
        self.id
    }

    // ── Connectivity (from struct) ──────────────────────────────

    /// Next halfedge in the radial ring around the same geometric edge.
    pub fn radial_next(&self) -> HalfEdgeId {
        self.data.radial_next()
    }

    /// The next halfedge in the face loop.
    pub fn next(&self) -> HalfEdgeId {
        self.data.next()
    }

    /// The previous halfedge in the face loop.
    pub fn prev(&self) -> HalfEdgeId {
        self.data.prev()
    }

    /// The face this halfedge borders.
    pub fn face(&self) -> FaceId {
        self.data.face()
    }

    /// The origin vertex.
    pub fn origin(&self) -> VertexId {
        self.data.origin()
    }

    /// The owning undirected edge.
    pub fn edge(&self) -> EdgeId {
        self.data.edge()
    }

    // ── Side-car metadata (from arena) ──────────────────────────

    /// Whether this halfedge is a synthetic bridge (inserted by `BridgeEdge`).
    pub fn is_bridge(&self) -> bool {
        self.arena
            .metadata
            .bridge_flags
            .get(self.id.index() as usize)
            .copied()
            .unwrap_or(false)
    }

    /// Coedge metadata (UV trim curve + direction sense), if present.
    pub fn coedge_info(&self) -> Option<&'a CoedgeInfo> {
        self.arena
            .metadata
            .coedge_data
            .get(self.id.index() as usize)
            .and_then(|opt| opt.as_ref())
    }
}

// ── VertexView ──────────────────────────────────────────────────────

/// Read-only view of a vertex: connectivity pointer + side-car metadata.
pub struct VertexView<'a> {
    id: VertexId,
    data: &'a VertexData,
    arena: &'a TopologyArena,
}

impl<'a> VertexView<'a> {
    pub(crate) fn new(id: VertexId, data: &'a VertexData, arena: &'a TopologyArena) -> Self {
        Self { id, data, arena }
    }

    /// The handle of this vertex.
    pub fn id(&self) -> VertexId {
        self.id
    }

    // ── Connectivity (from struct) ──────────────────────────────

    /// Representative halfedge for the primary disk entry.
    pub fn primary_disk(&self) -> HalfEdgeId {
        self.data.primary_disk()
    }

    // ── Side-car metadata (from arena) ──────────────────────────

    /// The 3-plane intersection provenance (sorted plane indices).
    pub fn provenance(&self) -> Option<&'a [usize; 3]> {
        self.arena
            .metadata
            .vertex_provenance
            .get(self.id.index() as usize)
            .and_then(|opt| opt.as_ref())
    }
}

// ── EdgeView ────────────────────────────────────────────────────────

/// Read-only view of an edge: connectivity pointer + side-car metadata.
pub struct EdgeView<'a> {
    id: EdgeId,
    data: &'a EdgeData,
    arena: &'a TopologyArena,
}

impl<'a> EdgeView<'a> {
    pub(crate) fn new(id: EdgeId, data: &'a EdgeData, arena: &'a TopologyArena) -> Self {
        Self { id, data, arena }
    }

    /// The handle of this edge.
    pub fn id(&self) -> EdgeId {
        self.id
    }

    // ── Connectivity (from struct) ──────────────────────────────

    /// Representative halfedge of the radial ring.
    pub fn half_edge(&self) -> HalfEdgeId {
        self.data.half_edge()
    }

    // ── Side-car metadata (from arena) ──────────────────────────

    /// Opaque reference to this edge's 3D curve in the geometry store.
    pub fn curve(&self) -> Option<CurveRef> {
        self.arena
            .metadata
            .edge_curves
            .get(self.id.index() as usize)
            .and_then(|opt| *opt)
    }
}
