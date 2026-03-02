//! Sparse policy configuration overrides.
//!
//! DOMAIN: Partial overrides for the policy section of the kernel configuration.

use forge_core::PolicyKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Sparse overrides for `PolicySection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyOverride {
    pub fallback_rules: Option<BTreeMap<PolicyKind, bool>>,
}
