//! Data shape for the Edge entity.
//!
//! DOMAIN: An undirected geometric edge shared by a radial ring of halfedges.

use serde::{Deserialize, Serialize};

use crate::handles::{CurveRef, HalfEdgeId};

/// Data stored for each undirected edge — owns a representative halfedge.
///
/// All halfedges around this geometric edge form a radial ring linked
/// via `radial_next`. The representative halfedge provides an entry point.
/// Edge-level attributes (fillet radius, crease angle, seam) live here.
///
/// Geometric data (3D curve + tolerance tube) lives in `forge-geom::CurveGeom`
/// and is referenced via the opaque `curve` handle. `EdgeData` never owns
/// or compares `f64` values (Doctrine D3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    half_edge: HalfEdgeId,
    /// Opaque reference to the edge's 3D curve in `forge-geom::CurveGeom`.
    /// `None` for planar edges (the edge is an implicit plane-plane intersection).
    /// `Some` for curved edges, populated by the kernel in Phase 4+.
    pub curve: Option<CurveRef>,
}

impl EdgeData {
    /// Construct a new edge from one halfedge of the pair.
    pub fn new(half_edge: HalfEdgeId) -> Self {
        Self {
            half_edge,
            curve: None,
        }
    }

    /// Representative halfedge of the radial ring.
    pub fn half_edge(&self) -> HalfEdgeId {
        self.half_edge
    }

    /// The opaque curve reference for this edge (None = planar).
    pub fn curve_ref(&self) -> Option<CurveRef> {
        self.curve
    }

    /// Set the representative halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) {
        self.half_edge = id;
    }

    /// Set the curve reference (populated by the kernel for curved edges).
    pub fn set_curve_ref(&mut self, id: Option<CurveRef>) {
        self.curve = id;
    }
}
