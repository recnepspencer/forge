//! Topology invariant violation error type.

use serde::{Deserialize, Serialize};

/// Specific topology invariant violations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyError {
    /// A halfedge is missing its twin (non-manifold or broken mesh)
    MissingTwin { halfedge_index: u32 },
    /// A face loop doesn't close (following `next` doesn't return to start)
    BrokenLoop {
        face_index: u32,
        starting_halfedge: u32,
    },
    /// Euler formula V - E + F ≠ 2 for a genus-0 solid
    EulerFormulaViolation {
        vertices: usize,
        edges: usize,
        faces: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    /// A non-manifold edge was detected (more than 2 faces sharing an edge)
    NonManifoldEdge { edge_index: u32, valence: usize },
    /// Generalized Euler formula violation for genus-aware validation.
    GeneralizedEulerViolation {
        shell_index: u32,
        vertices: usize,
        edges: usize,
        faces: usize,
        genus: usize,
        rings: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    /// Orientation inconsistency detected (D4 violation)
    OrientationInconsistency { face_index: u32 },
    /// An entity was referenced by a stale or invalid handle
    StaleHandle {
        entity_kind: String,
        index: u32,
        expected_generation: u32,
        actual_generation: u32,
    },
    /// A face has near-zero area (geometric degeneracy).
    ZeroAreaFace {
        face_index: u32,
        computed_area: f64,
        threshold: f64,
    },
    /// An edge has near-zero length.
    ZeroLengthEdge {
        halfedge_index: u32,
        computed_length: f64,
        threshold: f64,
    },
    /// A shell has the wrong signed volume (normals point inward).
    NegativeShellVolume {
        shell_index: u32,
        signed_volume: f64,
    },
    /// A loop has fewer than 3 distinct vertices.
    DegenerateLoop {
        face_index: u32,
        distinct_vertices: usize,
    },
    /// A topology walk exceeded its entity-count bound (corrupted next/prev chain).
    LoopCorruption {
        /// What kind of walk was being performed.
        walk_kind: String,
        /// The entity where the walk started.
        seed_index: u32,
        /// The last entity visited before the bound was hit.
        last_visited_index: u32,
        /// How many steps were taken.
        steps_taken: usize,
        /// The upper bound that was exceeded.
        entity_bound: usize,
    },
    /// A vertex referenced by a face has no geometry (position) available.
    MissingVertexPosition { vertex_index: u32, face_index: u32 },
    /// A shell has non-orientable surface topology (Möbius strip, Klein bottle).
    /// The kernel targets orientable 2-manifolds only.
    NonOrientableSurface { shell_index: u32 },
    /// A boundary edge (self-radial, no face across the gap) was found in a
    /// solid shell. Solid shells must be watertight.
    BoundaryEdgeInSolid {
        halfedge_index: u32,
        shell_index: u32,
    },
    /// An operation attempted something topologically invalid.
    InvalidOperation { detail: String },
    /// A parent-child hierarchy invariant was violated (Solid→Lump→Region→Shell).
    HierarchyViolation {
        /// The kind of the parent entity (e.g. "Region", "Lump", "Body").
        parent_kind: String,
        /// Arena index of the parent entity.
        parent_index: u32,
        /// The kind of the child entity (e.g. "Shell", "Region", "Lump").
        child_kind: String,
        /// Arena index of the child entity.
        child_index: u32,
        /// What went wrong.
        detail: String,
    },
    /// Two halfedges in the same radial ring reference different `EdgeId`s.
    /// A radial ring represents all face-uses of a single geometric edge;
    /// mixed edge entities indicate structural corruption.
    RadialEdgeInconsistency {
        /// The halfedge that disagrees with the ring seed.
        halfedge_index: u32,
        /// The edge entity found on the disagreeing halfedge.
        actual_edge: u32,
        /// The seed halfedge that defines the expected edge.
        seed_halfedge_index: u32,
        /// The edge entity on the ring seed.
        expected_edge: u32,
    },
    /// A validator detected a structural invariant violation.
    /// Generic variant for new validators that don't warrant dedicated enums.
    ValidatorFailure {
        /// Short identifier of the validator that failed.
        validator: String,
        /// Human-readable description of the violation.
        detail: String,
    },
}
