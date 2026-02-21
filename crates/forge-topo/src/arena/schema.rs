//! Data shapes for arena-stored topology entities.
//!
//! DOMAIN: Defines the per-entity data structs (Face, HalfEdge, Vertex, Loop)
//! and the generational `Slot` wrapper.
//!
//! DEPENDENCIES: `handles` (typed IDs), `lineage` (inline provenance)

use serde::{Deserialize, Serialize};

use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use crate::lineage::Lineage;

/// A slot in the arena that may be occupied or vacant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Slot<T: Clone> {
    /// The current generation of this slot.
    pub(crate) generation: u32,
    /// The current version of the data in this slot (increments on mutation).
    pub(crate) version: u32,
    /// The data, if the slot is occupied.
    pub(crate) data: Option<T>,
}

impl<T: Clone> Slot<T> {
    /// Create a new empty slot at generation 0.
    pub(crate) fn empty() -> Self {
        Self {
            generation: 0,
            version: 0,
            data: None,
        }
    }

    /// Occupy this slot with data, returning the current generation.
    /// Resets version to 0.
    pub(crate) fn occupy(&mut self, data: T) -> u32 {
        self.data = Some(data);
        self.version = 0;
        self.generation
    }
}

/// Data stored for each face.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    outer_loop: LoopId,
    inner_loops: Vec<LoopId>,
    lineage: Option<Lineage>,
}

impl FaceData {
    /// Construct a new face with the given outer loop.
    pub fn new(outer_loop: LoopId) -> Self {
        Self { outer_loop, inner_loops: Vec::new(), lineage: None }
    }

    /// Construct a new face with lineage.
    pub fn with_lineage(outer_loop: LoopId, lineage: Option<Lineage>) -> Self {
        Self { outer_loop, inner_loops: Vec::new(), lineage }
    }

    /// The outer boundary loop of this face.
    pub fn outer_loop(&self) -> LoopId { self.outer_loop }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// Set the outer boundary loop.
    pub fn set_outer_loop(&mut self, id: LoopId) { self.outer_loop = id; }

    /// Inner loops (holes) on this face.
    pub fn inner_loops(&self) -> &[LoopId] { &self.inner_loops }

    /// Add an inner loop (hole boundary) to this face.
    pub fn add_inner_loop(&mut self, id: LoopId) { self.inner_loops.push(id); }

    /// Remove an inner loop from this face.
    ///
    /// Returns `true` if the loop was found and removed, `false` otherwise.
    pub fn remove_inner_loop(&mut self, id: LoopId) -> bool {
        if let Some(pos) = self.inner_loops.iter().position(|&l| l == id) {
            self.inner_loops.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of inner loops (rings) on this face.
    pub fn inner_loop_count(&self) -> usize { self.inner_loops.len() }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }
}

/// Data stored for each halfedge.
///
/// Twin, next, and prev are all explicit pointers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalfEdgeData {
    twin: HalfEdgeId,
    next: HalfEdgeId,
    prev: HalfEdgeId,
    face: FaceId,
    origin: VertexId,
    lineage: Option<Lineage>,
}

impl HalfEdgeData {
    /// Construct a new halfedge with all connectivity fields.
    pub fn new(
        twin: HalfEdgeId,
        next: HalfEdgeId,
        prev: HalfEdgeId,
        face: FaceId,
        origin: VertexId,
    ) -> Self {
        Self { twin, next, prev, face, origin, lineage: None }
    }

    /// Construct a new halfedge with lineage.
    pub fn with_lineage(
        twin: HalfEdgeId,
        next: HalfEdgeId,
        prev: HalfEdgeId,
        face: FaceId,
        origin: VertexId,
        lineage: Option<Lineage>,
    ) -> Self {
        Self { twin, next, prev, face, origin, lineage }
    }

    /// The twin halfedge (on the adjacent face).
    pub fn twin(&self) -> HalfEdgeId { self.twin }

    /// The next halfedge in the face loop.
    pub fn next(&self) -> HalfEdgeId { self.next }

    /// The previous halfedge in the face loop.
    pub fn prev(&self) -> HalfEdgeId { self.prev }

    /// The face this halfedge borders.
    pub fn face(&self) -> FaceId { self.face }

    /// The origin vertex.
    pub fn origin(&self) -> VertexId { self.origin }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// Set the twin halfedge.
    pub fn set_twin(&mut self, id: HalfEdgeId) { self.twin = id; }

    /// Set the next halfedge.
    pub fn set_next(&mut self, id: HalfEdgeId) { self.next = id; }

    /// Set the previous halfedge.
    pub fn set_prev(&mut self, id: HalfEdgeId) { self.prev = id; }

    /// Set the face this halfedge borders.
    pub fn set_face(&mut self, id: FaceId) { self.face = id; }

    /// Set the origin vertex.
    pub fn set_origin(&mut self, id: VertexId) { self.origin = id; }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }
}

/// Data stored for each vertex.
///
/// The optional `provenance` field stores 3 sorted plane indices
/// that define this vertex as a 3-plane intersection. This survives
/// across chained boolean operations so that cross-solid vertex
/// welding can identify geometrically coincident vertices without
/// re-deriving keys from (potentially changed) face adjacency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    outgoing: HalfEdgeId,
    lineage: Option<Lineage>,
    provenance: Option<[usize; 3]>,
}

impl VertexData {
    /// Construct a new vertex with the given outgoing halfedge.
    pub fn new(outgoing: HalfEdgeId) -> Self {
        Self { outgoing, lineage: None, provenance: None }
    }

    /// Construct a new vertex with lineage.
    pub fn with_lineage(outgoing: HalfEdgeId, lineage: Option<Lineage>) -> Self {
        Self { outgoing, lineage, provenance: None }
    }

    /// One outgoing halfedge (for traversal entry).
    pub fn outgoing(&self) -> HalfEdgeId { self.outgoing }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// The 3-plane intersection provenance (sorted plane indices).
    pub fn provenance(&self) -> Option<&[usize; 3]> { self.provenance.as_ref() }

    /// Set the outgoing halfedge.
    pub fn set_outgoing(&mut self, id: HalfEdgeId) { self.outgoing = id; }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }

    /// Set the 3-plane intersection provenance.
    pub fn set_provenance(&mut self, provenance: Option<[usize; 3]>) { self.provenance = provenance; }
}

/// Data stored for each loop (boundary of a face).
///
/// Each face has at least one loop (outer boundary).
/// Future: inner loops represent holes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopData {
    half_edge: HalfEdgeId,
    face: FaceId,
}

impl LoopData {
    /// Construct a new loop.
    pub fn new(half_edge: HalfEdgeId, face: FaceId) -> Self {
        Self { half_edge, face }
    }

    /// One halfedge on this loop (entry point for traversal).
    pub fn half_edge(&self) -> HalfEdgeId { self.half_edge }

    /// The face this loop belongs to.
    pub fn face(&self) -> FaceId { self.face }

    /// Set the entry halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) { self.half_edge = id; }

    /// Set the owning face.
    pub fn set_face(&mut self, id: FaceId) { self.face = id; }
}
