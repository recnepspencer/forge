//! Error bounds and machine constants for expansion arithmetic.

/// Largest power of two such that `1.0 + EPSILON == 1.0` in f64 arithmetic.
///
/// Bounds the relative roundoff error. Pre-computed for IEEE 754 doubles
/// (53-bit mantissa): `2^{-53} ≈ 1.11e-16`.
pub const EPSILON: f64 = 1.110_223_024_625_156_5e-16;

/// Splitter for exact multiplication via Dekker splitting.
///
/// `2^{ceil(53/2)} + 1 = 2^{27} + 1 = 134_217_729`.
pub const SPLITTER: f64 = 134_217_729.0;

/// Error bound for the `fast` path of various predicates.
pub const RESULT_ERR_BOUND: f64 = (3.0 + 8.0 * EPSILON) * EPSILON;

/// orient2d (ccw) error bounds — stages A, B, C.
pub const CCW_ERR_BOUND_A: f64 = (3.0 + 16.0 * EPSILON) * EPSILON;
pub const CCW_ERR_BOUND_B: f64 = (2.0 + 12.0 * EPSILON) * EPSILON;
pub const CCW_ERR_BOUND_C: f64 = (9.0 + 64.0 * EPSILON) * EPSILON * EPSILON;

/// orient3d error bounds — stages A, B, C.
pub const O3D_ERR_BOUND_A: f64 = (7.0 + 56.0 * EPSILON) * EPSILON;
pub const O3D_ERR_BOUND_B: f64 = (3.0 + 28.0 * EPSILON) * EPSILON;
pub const O3D_ERR_BOUND_C: f64 = (26.0 + 288.0 * EPSILON) * EPSILON * EPSILON;

/// incircle error bounds — stages A, B, C.
pub const ICC_ERR_BOUND_A: f64 = (10.0 + 96.0 * EPSILON) * EPSILON;
pub const ICC_ERR_BOUND_B: f64 = (4.0 + 48.0 * EPSILON) * EPSILON;
pub const ICC_ERR_BOUND_C: f64 = (44.0 + 576.0 * EPSILON) * EPSILON * EPSILON;

/// insphere error bounds — stages A, B, C.
pub const ISP_ERR_BOUND_A: f64 = (16.0 + 224.0 * EPSILON) * EPSILON;
pub const ISP_ERR_BOUND_B: f64 = (5.0 + 72.0 * EPSILON) * EPSILON;
pub const ISP_ERR_BOUND_C: f64 = (71.0 + 1408.0 * EPSILON) * EPSILON * EPSILON;
