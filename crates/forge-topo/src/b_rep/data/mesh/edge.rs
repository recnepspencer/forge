//! Data shape for the Edge entity.
//!
//! DOMAIN: An undirected geometric edge shared by a radial ring of halfedges.
//!
//! Connectivity only — the 3D curve reference lives in a slot-parallel
//! side-car vector on `TopologyArena`.

use serde::{Deserialize, Serialize};

use crate::handles::HalfEdgeId;

/// Data stored for each undirected edge — 1 connectivity pointer, nothing else.
///
/// All halfedges around this geometric edge form a radial ring linked
/// via `radial_next`. The representative halfedge provides an entry point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    half_edge: HalfEdgeId,
}

impl EdgeData {
    /// Construct a new edge from one halfedge of the pair.
    pub fn new(half_edge: HalfEdgeId) -> Self {
        Self { half_edge }
    }

    /// Representative halfedge of the radial ring.
    pub fn half_edge(&self) -> HalfEdgeId {
        self.half_edge
    }

    /// Set the representative halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) {
        self.half_edge = id;
    }
}
