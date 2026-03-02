//! Policy for sliver face detection and removal.
//!
//! DOMAIN: Groups sliver-related thresholds for lower-layer callers.

use super::super::data::defaults;

/// Policy for sliver face detection and removal.
#[derive(Debug, Clone)]
pub struct SliverPolicy {
    min_face_area: f64,
    max_slivers_per_op: usize,
}

impl SliverPolicy {
    /// Build from a resolved config.
    pub fn from_config(config: &super::super::data::KernelConfig) -> Self {
        Self {
            min_face_area: config.tolerance.min_face_area,
            max_slivers_per_op: config.tolerance.max_slivers_per_op,
        }
    }

    /// Create a sliver policy with explicit values.
    pub fn new(min_face_area: f64, max_slivers_per_op: usize) -> Self {
        Self {
            min_face_area,
            max_slivers_per_op,
        }
    }

    /// Minimum face area (m²) — faces below this are slivers.
    pub fn get_min_face_area(&self) -> f64 {
        self.min_face_area
    }

    /// Set the minimum face area.
    pub fn set_min_face_area(&mut self, value: f64) {
        self.min_face_area = value;
    }

    /// Maximum number of slivers an operation may create before requiring explicit waiver.
    pub fn get_max_slivers_per_op(&self) -> usize {
        self.max_slivers_per_op
    }

    /// Set the maximum slivers per operation.
    pub fn set_max_slivers_per_op(&mut self, value: usize) {
        self.max_slivers_per_op = value;
    }
}

impl Default for SliverPolicy {
    fn default() -> Self {
        Self {
            min_face_area: defaults::MIN_FACE_AREA,
            max_slivers_per_op: defaults::MAX_SLIVERS_PER_OP,
        }
    }
}
