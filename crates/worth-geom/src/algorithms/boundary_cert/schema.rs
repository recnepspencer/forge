//! Data shapes for boundary certification.
//!
//! DOMAIN: Stateless 2D boundary segment representations and certification
//! result types for weakly-simple polygon recognition.
//!
//! DEPENDENCIES: None (pure data shapes).
//! INVARIANTS: All types are value-only — no topology handles, no policy.

/// Deterministic 2D projection frame for planar boundaries.
///
/// Encodes which 3D axis was dropped and the resulting u/v mapping
/// with an orientation sign for consistent winding direction.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionFrame2D {
    /// Which 3D axis was dropped (0=X, 1=Y, 2=Z).
    drop_axis: usize,
    /// Index of the 3D axis mapped to the u direction.
    u_axis: usize,
    /// Index of the 3D axis mapped to the v direction.
    v_axis: usize,
    /// Sign correction (+1.0 or -1.0) to preserve winding orientation.
    orientation_sign: f64,
}

impl ProjectionFrame2D {
    /// Construct a projection frame from axis indices and orientation sign.
    pub fn new(drop_axis: usize, u_axis: usize, v_axis: usize, orientation_sign: f64) -> Self {
        Self {
            drop_axis,
            u_axis,
            v_axis,
            orientation_sign,
        }
    }

    /// The dropped axis index (0=X, 1=Y, 2=Z).
    pub fn get_drop_axis(&self) -> usize {
        self.drop_axis
    }

    /// The u-axis index in the original 3D space.
    pub fn get_u_axis(&self) -> usize {
        self.u_axis
    }

    /// The v-axis index in the original 3D space.
    pub fn get_v_axis(&self) -> usize {
        self.v_axis
    }

    /// Orientation sign (+1.0 or -1.0).
    pub fn get_orientation_sign(&self) -> f64 {
        self.orientation_sign
    }
}

/// A 2D segment in projected space with provenance tracking.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment2D {
    /// Start point in projected 2D space.
    start: [f64; 2],
    /// End point in projected 2D space.
    end: [f64; 2],
    /// Stable provenance identifier for diagnostics/tracing.
    provenance: u64,
}

impl Segment2D {
    /// Construct a segment from endpoints and provenance.
    pub fn new(start: [f64; 2], end: [f64; 2], provenance: u64) -> Self {
        Self {
            start,
            end,
            provenance,
        }
    }

    /// Start point of the segment.
    pub fn get_start(&self) -> [f64; 2] {
        self.start
    }

    /// End point of the segment.
    pub fn get_end(&self) -> [f64; 2] {
        self.end
    }

    /// Provenance identifier.
    pub fn get_provenance(&self) -> u64 {
        self.provenance
    }

    /// Squared length of this segment in 2D.
    pub fn length_sq(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        dx * dx + dy * dy
    }
}

/// A projected boundary composed of ordered 2D segments.
#[derive(Debug, Clone)]
pub struct ProjectedBoundary2D {
    /// Ordered segments forming the boundary cycle.
    segments: Vec<Segment2D>,
    /// The projection frame used to create these segments.
    frame: ProjectionFrame2D,
}

impl ProjectedBoundary2D {
    /// Construct from segments and frame.
    pub fn new(segments: Vec<Segment2D>, frame: ProjectionFrame2D) -> Self {
        Self { segments, frame }
    }

    /// The boundary segments.
    pub fn get_segments(&self) -> &[Segment2D] {
        &self.segments
    }

    /// The projection frame.
    pub fn get_frame(&self) -> &ProjectionFrame2D {
        &self.frame
    }

    /// Number of segments in this boundary.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

use super::split::{ArrangementVertex, AtomicSegment2D};

/// Planar arrangement: boundary segments subdivided at all event points.
///
/// Every atomic segment contains no interior event points.
/// Vertices carry exact rational positions for identity.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryArrangement {
    /// Original input segments (not split at events).
    source_segments: Vec<Segment2D>,
    /// Subdivided arrangement edges containing no interior events.
    atomic_segments: Vec<AtomicSegment2D>,
    /// Exact vertices in the arrangement graph, capturing incidence.
    vertices: Vec<ArrangementVertex>,
}

impl BoundaryArrangement {
    /// Construct an arrangement from input segments, computed splits and exact vertices.
    pub fn new(
        source_segments: Vec<Segment2D>,
        atomic_segments: Vec<AtomicSegment2D>,
        vertices: Vec<ArrangementVertex>,
    ) -> Self {
        Self {
            source_segments,
            atomic_segments,
            vertices,
        }
    }

    /// Original source segments.
    pub fn get_source_segments(&self) -> &[Segment2D] {
        &self.source_segments
    }

    /// The resulting atomic segments.
    pub fn get_atomic_segments(&self) -> &[AtomicSegment2D] {
        &self.atomic_segments
    }

    /// The exact topological vertices grouping the atomic segments.
    pub fn get_vertices(&self) -> &[ArrangementVertex] {
        &self.vertices
    }
}

/// Reason for rejecting a boundary as non-mergeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryRejectReason {
    /// Non-adjacent segments cross transversally (proper crossing).
    SelfCrossing,
    /// Collinear segments overlap (policy may later accept this).
    OverlappingSegments,
    /// Boundary is degenerate (zero-length segments, insufficient vertices).
    DegenerateBoundary,
}

/// Errors raised during exact boundary certification due to degenerate input
/// that cannot be robustly classified.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundaryCertError {
    /// Exact geometric predicate failed to evaluate (e.g., NaN coordinates).
    PredicateFailure,
    /// Vector direction evaluates to exactly zero.
    DegenerateVector,
    /// A computed intersection parameter fell outside `[0, 1]`, indicating
    /// a logic defect in the caller (not a recoverable input condition).
    OutOfRangeParameter,
    /// Two non-adjacent source segments share a collinear overlapping 1D interval.
    /// The `f64` coordinates are the approximate midpoint of the overlap for diagnostics.
    OverlapDetected([f64; 2]),
}

/// Result of boundary certification for merge eligibility.
///
/// A boundary is **weakly simple** if ∀ε > 0, its vertices can be
/// perturbed by ≤ ε to obtain a simple polygon (Akitaya et al., SoCG 2016).
#[derive(Debug, Clone, PartialEq)]
pub enum WeakSimpleCertificate {
    /// Boundary is strictly simple (no self-intersections, no touches).
    Simple,
    /// Boundary is weakly simple with endpoint-touch contacts.
    WeaklySimple {
        /// Number of distinct endpoint-touch events.
        touch_count: usize,
    },
    /// Boundary is rejected — contains a proper crossing or overlap.
    Rejected {
        /// Why the boundary was rejected.
        reason: BoundaryRejectReason,
        /// Witness point in 2D where the violation occurs.
        witness: [f64; 2],
    },
}
