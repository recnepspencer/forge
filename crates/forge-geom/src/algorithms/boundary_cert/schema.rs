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
        Self { drop_axis, u_axis, v_axis, orientation_sign }
    }

    /// The dropped axis index (0=X, 1=Y, 2=Z).
    pub fn get_drop_axis(&self) -> usize { self.drop_axis }

    /// The u-axis index in the original 3D space.
    pub fn get_u_axis(&self) -> usize { self.u_axis }

    /// The v-axis index in the original 3D space.
    pub fn get_v_axis(&self) -> usize { self.v_axis }

    /// Orientation sign (+1.0 or -1.0).
    pub fn get_orientation_sign(&self) -> f64 { self.orientation_sign }
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
        Self { start, end, provenance }
    }

    /// Start point of the segment.
    pub fn get_start(&self) -> [f64; 2] { self.start }

    /// End point of the segment.
    pub fn get_end(&self) -> [f64; 2] { self.end }

    /// Provenance identifier.
    pub fn get_provenance(&self) -> u64 { self.provenance }

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
    pub fn get_segments(&self) -> &[Segment2D] { &self.segments }

    /// The projection frame.
    pub fn get_frame(&self) -> &ProjectionFrame2D { &self.frame }

    /// Number of segments in this boundary.
    pub fn segment_count(&self) -> usize { self.segments.len() }
}

/// Classifies a boundary interaction event between two segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryEventKind {
    /// Two non-adjacent segments cross transversally.
    ProperCrossing,
    /// A segment endpoint touches a non-incident segment interior.
    EndpointTouch,
    /// Two collinear segments begin overlapping.
    OverlapStart,
    /// Two collinear segments stop overlapping.
    OverlapEnd,
    /// A segment has zero projected length (degenerate).
    DegenerateSegment,
}

/// A classified event at a point in the boundary arrangement.
#[derive(Debug, Clone)]
pub struct BoundaryEvent {
    /// What kind of interaction this is.
    kind: BoundaryEventKind,
    /// Location in 2D projected space.
    location: [f64; 2],
    /// Indices of the two involved segments (self-referencing for degenerate).
    segments: [usize; 2],
}

impl BoundaryEvent {
    /// Construct a boundary event.
    pub fn new(kind: BoundaryEventKind, location: [f64; 2], segments: [usize; 2]) -> Self {
        Self { kind, location, segments }
    }

    /// The event kind.
    pub fn get_kind(&self) -> BoundaryEventKind { self.kind }

    /// The 2D location of the event.
    pub fn get_location(&self) -> [f64; 2] { self.location }

    /// The segment indices involved.
    pub fn get_segments(&self) -> [usize; 2] { self.segments }
}

/// Event-based classification of boundary segment interactions.
///
/// This is NOT a planar subdivision — segments are stored as-is from the
/// input boundary. Events classify pairwise interactions (crossing, touch,
/// overlap, degeneracy) for weakly-simple recognition per Akitaya et al.
/// No segment splitting occurs; the events are sufficient for certification.
#[derive(Debug, Clone)]
pub struct BoundaryArrangement {
    /// Original input segments (not split at events).
    segments: Vec<Segment2D>,
    /// Classified interaction events, in deterministic order.
    events: Vec<BoundaryEvent>,
}

impl BoundaryArrangement {
    /// Construct an arrangement from input segments and classified events.
    pub fn new(segments: Vec<Segment2D>, events: Vec<BoundaryEvent>) -> Self {
        Self { segments, events }
    }

    /// The input segments (original, not split).
    pub fn get_segments(&self) -> &[Segment2D] { &self.segments }

    /// The classified events.
    pub fn get_events(&self) -> &[BoundaryEvent] { &self.events }
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
