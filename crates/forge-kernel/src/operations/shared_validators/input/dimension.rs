//! Dimension and coordinate validation.
//!
//! DOMAIN: Numeric input validation ensuring dimensions are finite,
//! positive, and large enough relative to tolerances for reliable topology.
//! Coordinates must be finite (no NaN or Inf).
//!
//! CONSUMED BY: primitives (make_cube, make_prism, etc.), booleans,
//!              any operation accepting user-supplied numeric parameters.

use crate::configuration::facade::ResolvedConfig;
use forge_core::KernelError;

/// Safety margin: dimensions smaller than this multiple of the spatial
/// tolerance will be rejected as too small to produce reliable topology.
const DIMENSION_TOLERANCE_SAFETY_FACTOR: f64 = 10.0;

/// Validate a primitive dimension is usable.
///
/// - **Hard reject**: NaN, Inf, or ≤ 0.
/// - **Policy reject**: Finite positive but smaller than
///   `vertex_tolerance × DIMENSION_TOLERANCE_SAFETY_FACTOR`. The BSP
///   would merge all vertices and produce degenerate topology.
pub fn validate_dimension(
    value: f64,
    name: &str,
    config: &crate::configuration::facade::KernelConfig,
) -> Result<(), KernelError> {
    if value.is_nan() || value.is_infinite() {
        return Err(KernelError::InvalidInput {
            message: format!("{name} must be finite, got {value}"),
            context: None,
        });
    }
    if value <= 0.0 {
        return Err(KernelError::InvalidInput {
            message: format!("{name} must be > 0, got {value}"),
            context: None,
        });
    }

    let min_usable = config.tolerance.spatial_tolerance * DIMENSION_TOLERANCE_SAFETY_FACTOR;
    if value < min_usable {
        return Err(KernelError::InvalidInput {
            message: format!(
                "{name} = {value:.2e} is smaller than the minimum usable dimension \
                 ({min_usable:.2e} = {DIMENSION_TOLERANCE_SAFETY_FACTOR}× vertex tolerance). \
                 BSP would produce degenerate topology."
            ),
            context: None,
        });
    }
    Ok(())
}

/// Validate that a coordinate is finite (not NaN or ±Inf).
pub fn validate_coordinate(value: f64, name: &str) -> Result<(), KernelError> {
    if value.is_nan() || value.is_infinite() {
        return Err(KernelError::InvalidInput {
            message: format!("{name} must be finite, got {value}"),
            context: None,
        });
    }
    Ok(())
}

/// Validate center coordinates and a single size dimension.
pub fn validate_center_and_size(
    center: [f64; 3],
    size: f64,
    config: &crate::configuration::facade::KernelConfig,
) -> Result<(), KernelError> {
    validate_coordinate(center[0], "center[0]")?;
    validate_coordinate(center[1], "center[1]")?;
    validate_coordinate(center[2], "center[2]")?;
    validate_dimension(size, "size", config)
}
