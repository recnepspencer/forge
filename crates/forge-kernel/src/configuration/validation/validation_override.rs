//! Sparse validation configuration overrides.
//!
//! DOMAIN: Partial overrides for the validation section of the kernel configuration.

use forge_core::ValidationCheckpoint;
use serde::{Deserialize, Serialize};

use super::group_policy::GroupPolicyConfig;

/// Sparse overrides for `ValidationSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationOverride {
    pub checkpoints: Option<Vec<ValidationCheckpoint>>,
    pub include_geometric: Option<bool>,
    pub entity_limit: Option<usize>,
    pub group_policy: Option<GroupPolicyConfig>,
}
