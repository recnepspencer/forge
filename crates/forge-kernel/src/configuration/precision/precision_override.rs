//! Sparse precision configuration overrides.
//!
//! DOMAIN: Partial overrides for the precision section of the kernel configuration.

use serde::{Deserialize, Serialize};

/// Sparse overrides for `PrecisionSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrecisionOverride {
    pub bit_length_threshold: Option<u32>,
}
