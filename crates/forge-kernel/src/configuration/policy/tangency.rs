//! Policy for handling near-tangent surface intersections.
//!
//! DOMAIN: Groups tangency-related thresholds for lower-layer callers.

use super::super::data::defaults;

/// Policy for handling near-tangent surface intersections.
#[derive(Debug, Clone)]
pub struct TangencyPolicy {
    min_transversal_angle: f64,
    max_tangent_gap: f64,
}

impl TangencyPolicy {
    /// Build from a resolved config.
    pub fn from_config(config: &super::super::data::KernelConfig) -> Self {
        Self {
            min_transversal_angle: config.tolerance.min_transversal_angle,
            max_tangent_gap: config.tolerance.max_tangent_gap,
        }
    }

    /// Create a tangency policy with explicit values.
    pub fn new(min_transversal_angle: f64, max_tangent_gap: f64) -> Self {
        Self {
            min_transversal_angle,
            max_tangent_gap,
        }
    }

    /// Minimum angle between surfaces to classify as transversal (radians).
    pub fn get_min_transversal_angle(&self) -> f64 {
        self.min_transversal_angle
    }

    /// Set the minimum transversal angle.
    pub fn set_min_transversal_angle(&mut self, value: f64) {
        self.min_transversal_angle = value;
    }

    /// Maximum gap in near-tangent regions before escalating to policy decision.
    pub fn get_max_tangent_gap(&self) -> f64 {
        self.max_tangent_gap
    }

    /// Set the maximum tangent gap.
    pub fn set_max_tangent_gap(&mut self, value: f64) {
        self.max_tangent_gap = value;
    }
}

impl Default for TangencyPolicy {
    fn default() -> Self {
        Self {
            min_transversal_angle: defaults::MIN_TRANSVERSAL_ANGLE,
            max_tangent_gap: defaults::MAX_TANGENT_GAP,
        }
    }
}
