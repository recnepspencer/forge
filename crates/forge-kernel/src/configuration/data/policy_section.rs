//! Policy section of the unified configuration.
//!
//! DOMAIN: Default rules for handling policy ambiguity.

use forge_core::{KernelError, PolicyKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::kernel_config::ConfigSection;

/// Default rules for handling policy ambiguity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySection {
    pub fallback_rules: BTreeMap<PolicyKind, bool>,
}

impl ConfigSection for PolicySection {
    fn defaults() -> Self {
        let mut fallback_rules = BTreeMap::new();
        // Epic A/B current product semantics: weakly-simple coplanar region boundaries
        // are accepted by default, but the decision must be traced.
        fallback_rules.insert(PolicyKind::CoincidentGeometry, true);

        Self { fallback_rules }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for PolicySection {
    fn default() -> Self {
        Self::defaults()
    }
}
