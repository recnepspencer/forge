//! Policy for automatic gap closure during sewing.
//!
//! DOMAIN: Groups gap-closure thresholds for lower-layer callers.

use super::tolerance_section::{self, ToleranceSection};

/// Policy for automatic gap closure during sewing.
#[derive(Debug, Clone)]
pub struct GapClosurePolicy {
    max_gap: f64,
}

impl GapClosurePolicy {
    /// Build from a tolerance section.
    pub fn from_section(section: &ToleranceSection) -> Self {
        Self {
            max_gap: section.max_gap_closure,
        }
    }

    /// Create a gap closure policy with explicit value.
    pub fn new(max_gap: f64) -> Self {
        Self { max_gap }
    }

    /// Maximum gap that will be automatically closed (meters).
    pub fn get_max_gap(&self) -> f64 {
        self.max_gap
    }

    /// Set the maximum gap for automatic closure.
    pub fn set_max_gap(&mut self, value: f64) {
        self.max_gap = value;
    }
}

impl Default for GapClosurePolicy {
    fn default() -> Self {
        Self {
            max_gap: tolerance_section::GAP_CLOSURE_MAX,
        }
    }
}
