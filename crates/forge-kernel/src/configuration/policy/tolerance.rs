//! Spatial tolerance policy for coincidence detection.
//!
//! DOMAIN: Groups spatial and angular tolerances for lower-layer callers.

use super::super::data::defaults;

/// Spatial tolerance policy for coincidence detection.
#[derive(Debug, Clone)]
pub struct TolerancePolicy {
    spatial_tolerance: f64,
    angular_tolerance: f64,
}

impl TolerancePolicy {
    /// Build from a resolved config.
    pub fn from_config(config: &super::super::data::KernelConfig) -> Self {
        Self {
            spatial_tolerance: config.tolerance.spatial_tolerance,
            angular_tolerance: config.tolerance.angular_tolerance,
        }
    }

    /// Create a tolerance policy with explicit values.
    pub fn new(spatial_tolerance: f64, angular_tolerance: f64) -> Self {
        Self {
            spatial_tolerance,
            angular_tolerance,
        }
    }

    /// Distance below which two points are considered coincident (meters).
    pub fn get_spatial_tolerance(&self) -> f64 {
        self.spatial_tolerance
    }

    /// Set the spatial coincidence tolerance.
    pub fn set_spatial_tolerance(&mut self, value: f64) {
        self.spatial_tolerance = value;
    }

    /// Angular tolerance for direction comparisons (radians).
    pub fn get_angular_tolerance(&self) -> f64 {
        self.angular_tolerance
    }

    /// Set the angular comparison tolerance.
    pub fn set_angular_tolerance(&mut self, value: f64) {
        self.angular_tolerance = value;
    }
}

impl Default for TolerancePolicy {
    fn default() -> Self {
        Self {
            spatial_tolerance: defaults::SPATIAL_TOLERANCE,
            angular_tolerance: defaults::ANGULAR_TOLERANCE,
        }
    }
}
