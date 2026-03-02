//! Precision section of the unified configuration.
//!
//! DOMAIN: Settings for automatic scaling to arbitrary precision.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::defaults;
use super::kernel_config::ConfigSection;

/// Settings for automatic scaling to arbitrary precision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionSection {
    pub bit_length_threshold: u32,
}

impl ConfigSection for PrecisionSection {
    fn defaults() -> Self {
        Self {
            bit_length_threshold: defaults::BIT_LENGTH_THRESHOLD,
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
