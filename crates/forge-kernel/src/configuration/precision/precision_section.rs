//! Precision section of the unified configuration.
//!
//! DOMAIN: Settings for automatic scaling to arbitrary precision and default constant.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::super::kernel_config::ConfigSection;

// ── Default constants ────────────────────────────────────────────────

/// Bit-length threshold for precision escalation.
pub const BIT_LENGTH_THRESHOLD: u32 = 512;

// ── Precision section ────────────────────────────────────────────────

/// Settings for automatic scaling to arbitrary precision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionSection {
    pub bit_length_threshold: u32,
}

impl ConfigSection for PrecisionSection {
    fn defaults() -> Self {
        Self {
            bit_length_threshold: BIT_LENGTH_THRESHOLD,
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for PrecisionSection {
    fn default() -> Self {
        Self::defaults()
    }
}
