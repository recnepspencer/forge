//! Centralized default constants for all kernel configuration.
//!
//! DOMAIN: Every magic number in the kernel lives here — the one source of truth.
//! INVARIANTS: All defaults are suitable for unit-scale CAD (meters).
//!
//! When adding a new threshold or policy default, put it here and reference
//! it from the corresponding struct's `Default` impl.

/// Spatial coincidence tolerance: 1 micron.
pub const SPATIAL_TOLERANCE: f64 = 1e-6;

/// Angular tolerance for direction comparisons: ~0.00006 degrees.
pub const ANGULAR_TOLERANCE: f64 = 1e-6;

/// Minimum angle between surfaces to classify as transversal: ~0.06 degrees.
pub const MIN_TRANSVERSAL_ANGLE: f64 = 1e-3;

/// Maximum gap in near-tangent regions before policy escalation: 0.1mm.
pub const MAX_TANGENT_GAP: f64 = 1e-4;

/// Minimum face area (m²) — faces below this are slivers.
pub const MIN_FACE_AREA: f64 = 1e-10;

/// Maximum number of slivers an operation may create.
pub const MAX_SLIVERS_PER_OP: usize = 3;

/// Maximum gap for automatic gap closure: 0.1mm.
pub const GAP_CLOSURE_MAX: f64 = 1e-4;

/// Bit-length threshold for precision escalation.
pub const BIT_LENGTH_THRESHOLD: u32 = 512;

/// Residual tolerance for intersection computations.
pub const RESIDUAL_TOLERANCE: f64 = 1e-8;

/// Degeneracy threshold for near-zero denominators.
pub const DEGENERACY_THRESHOLD: f64 = 1e-12;

/// Inward offset for classification sample points.
pub const SAMPLE_INWARD_OFFSET: f64 = 1e-6;

/// Maximum ray extent for ray-casting operations.
pub const RAY_EXTENT: f64 = 1e6;

/// Angular epsilon for coplanar plane detection.
pub const COPLANAR_ANGLE_EPSILON: f64 = 1e-20;

/// Offset epsilon for coplanar plane detection.
pub const COPLANAR_OFFSET_EPSILON: f64 = 1e-12;

/// Degeneracy threshold for edge splitting.
pub const EDGE_SPLIT_DEGENERACY: f64 = 1e-30;

/// Minimum edge length — edges shorter than this are degenerate.
pub const MIN_EDGE_LENGTH: f64 = 1e-9;

/// Dot-product tolerance for collinearity detection.
pub const COLLINEARITY_DOT_TOLERANCE: f64 = 1e-8;

/// AABB inflation factor for bounding box overlap tests.
pub const AABB_INFLATION: f64 = 1e-7;

/// Multiplier applied to spatial_tolerance to define the ambiguity band.
pub const AMBIGUITY_BAND_FACTOR: f64 = 10.0;

/// Maximum iterations for iterative numeric solvers (e.g. Newton-Raphson).
pub const MAX_ITERATIONS: usize = 50;

/// Target numeric residual for solver convergence.
pub const CONVERGENCE_TOLERANCE: f64 = 1e-10;

/// Bail-out threshold if an iterative solver is diverging.
pub const DIVERGENCE_THRESHOLD: f64 = 1e2;
