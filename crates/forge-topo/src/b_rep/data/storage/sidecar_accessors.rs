//! Typed accessors for side-car metadata on TopologyArena.
//!
//! DOMAIN: Read/write access to slot-parallel metadata vectors
//! that were stripped from entity structs (Milestone 1).

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::b_rep::data::mesh::CoedgeInfo;
use crate::handles::{HalfEdgeId, EdgeId, VertexId, CurveRef};

impl TopologyArena {
    // ── Bridge flag (HalfEdge side-car) ─────────────────────────────

    /// Whether this halfedge is a synthetic bridge (inserted by `BridgeEdge`).
    pub fn is_bridge(&self, id: HalfEdgeId) -> bool {
        self.bridge_flags
            .get(id.index() as usize)
            .copied()
            .unwrap_or(false)
    }

    /// Mark or unmark this halfedge as a synthetic bridge.
    pub fn set_bridge(&mut self, id: HalfEdgeId, value: bool) {
        let idx = id.index() as usize;
        if idx >= self.bridge_flags.len() {
            self.bridge_flags.resize(idx + 1, false);
        }
        self.bridge_flags[idx] = value;
    }

    // ── Coedge metadata (HalfEdge side-car) ─────────────────────────

    /// Coedge metadata for this halfedge (UV trim curve + direction sense).
    pub fn coedge_info(&self, id: HalfEdgeId) -> Option<&CoedgeInfo> {
        self.coedge_data
            .get(id.index() as usize)
            .and_then(|opt| opt.as_ref())
    }

    /// Set or clear coedge metadata for this halfedge.
    pub fn set_coedge_info(&mut self, id: HalfEdgeId, info: Option<CoedgeInfo>) {
        let idx = id.index() as usize;
        if idx >= self.coedge_data.len() {
            self.coedge_data.resize(idx + 1, None);
        }
        self.coedge_data[idx] = info;
    }

    // ── 3D curve reference (Edge side-car) ──────────────────────────

    /// Opaque reference to this edge's 3D curve in the geometry store.
    pub fn edge_curve(&self, id: EdgeId) -> Option<CurveRef> {
        self.edge_curves
            .get(id.index() as usize)
            .and_then(|opt| *opt)
    }

    /// Set or clear the 3D curve reference for this edge.
    pub fn set_edge_curve(&mut self, id: EdgeId, curve: Option<CurveRef>) {
        let idx = id.index() as usize;
        if idx >= self.edge_curves.len() {
            self.edge_curves.resize(idx + 1, None);
        }
        self.edge_curves[idx] = curve;
    }

    // ── 3-plane intersection provenance (Vertex side-car) ───────────

    /// The 3-plane intersection provenance for this vertex (sorted plane indices).
    pub fn vertex_provenance(&self, id: VertexId) -> Option<&[usize; 3]> {
        self.vertex_provenance
            .get(id.index() as usize)
            .and_then(|opt| opt.as_ref())
    }

    /// Set or clear the 3-plane intersection provenance for this vertex.
    pub fn set_vertex_provenance(&mut self, id: VertexId, provenance: Option<[usize; 3]>) {
        let idx = id.index() as usize;
        if idx >= self.vertex_provenance.len() {
            self.vertex_provenance.resize(idx + 1, None);
        }
        self.vertex_provenance[idx] = provenance;
    }

    // ── Lockstep growth helpers ─────────────────────────────────────

    /// Ensure the halfedge side-car vectors are at least `len` long.
    pub(crate) fn grow_halfedge_sidecars(&mut self, len: usize) {
        if self.bridge_flags.len() < len {
            self.bridge_flags.resize(len, false);
        }
        if self.coedge_data.len() < len {
            self.coedge_data.resize(len, None);
        }
    }

    /// Clear half-edge side-car data at the given slot index.
    pub(crate) fn clear_halfedge_sidecar(&mut self, index: usize) {
        if index < self.bridge_flags.len() {
            self.bridge_flags[index] = false;
        }
        if index < self.coedge_data.len() {
            self.coedge_data[index] = None;
        }
    }

    /// Ensure the edge side-car vectors are at least `len` long.
    pub(crate) fn grow_edge_sidecars(&mut self, len: usize) {
        if self.edge_curves.len() < len {
            self.edge_curves.resize(len, None);
        }
    }

    /// Clear edge side-car data at the given slot index.
    pub(crate) fn clear_edge_sidecar(&mut self, index: usize) {
        if index < self.edge_curves.len() {
            self.edge_curves[index] = None;
        }
    }

    /// Ensure the vertex side-car vectors are at least `len` long.
    pub(crate) fn grow_vertex_sidecars(&mut self, len: usize) {
        if self.vertex_provenance.len() < len {
            self.vertex_provenance.resize(len, None);
        }
    }

    /// Clear vertex side-car data at the given slot index.
    pub(crate) fn clear_vertex_sidecar(&mut self, index: usize) {
        if index < self.vertex_provenance.len() {
            self.vertex_provenance[index] = None;
        }
    }
}
