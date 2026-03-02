//! Policy for precision escalation.
//!
//! DOMAIN: Groups precision escalation thresholds for lower-layer callers.

use super::precision_section::{self, PrecisionSection};

/// Policy for precision escalation.
#[derive(Debug, Clone)]
pub struct PrecisionEscalationPolicy {
    bit_length_threshold: u32,
}

impl PrecisionEscalationPolicy {
    /// Build from a precision section.
    pub fn from_section(section: &PrecisionSection) -> Self {
        Self {
            bit_length_threshold: section.bit_length_threshold,
        }
    }

    /// Create a precision escalation policy with explicit value.
    pub fn new(bit_length_threshold: u32) -> Self {
        Self {
            bit_length_threshold,
        }
    }

    /// Bit-length threshold before escalating.
    pub fn get_bit_length_threshold(&self) -> u32 {
        self.bit_length_threshold
    }

    /// Set the bit-length threshold.
    pub fn set_bit_length_threshold(&mut self, value: u32) {
        self.bit_length_threshold = value;
    }
}

impl Default for PrecisionEscalationPolicy {
    fn default() -> Self {
        Self {
            bit_length_threshold: precision_section::BIT_LENGTH_THRESHOLD,
        }
    }
}
