//! Policy for precision escalation.
//!
//! DOMAIN: Groups precision escalation thresholds for lower-layer callers.

use super::super::data::defaults;

/// Policy for precision escalation.
#[derive(Debug, Clone)]
pub struct PrecisionEscalationPolicy {
    bit_length_threshold: u32,
}

impl PrecisionEscalationPolicy {
    /// Build from a resolved config.
    pub fn from_config(config: &super::super::data::KernelConfig) -> Self {
        Self {
            bit_length_threshold: config.precision.bit_length_threshold,
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
            bit_length_threshold: defaults::BIT_LENGTH_THRESHOLD,
        }
    }
}
