use std::sync::Arc;

use crate::mapping::CoarseRoutingMode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeInvalidationTarget {
    signal_scope: Arc<str>,
    routing_mode: CoarseRoutingMode,
}

impl BridgeInvalidationTarget {
    pub(crate) fn new(signal_scope: Arc<str>, routing_mode: CoarseRoutingMode) -> Self {
        Self {
            signal_scope,
            routing_mode,
        }
    }

    pub fn signal_scope(&self) -> &str {
        self.signal_scope.as_ref()
    }

    pub fn routing_mode(&self) -> CoarseRoutingMode {
        self.routing_mode
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalInvalidationTargets {
    targets: Arc<[BridgeInvalidationTarget]>,
}

impl CanonicalInvalidationTargets {
    pub(crate) fn new(targets: Vec<BridgeInvalidationTarget>) -> Self {
        Self {
            targets: Arc::from(targets),
        }
    }

    pub fn targets(&self) -> &[BridgeInvalidationTarget] {
        &self.targets
    }

    pub(crate) fn shared(&self) -> &Arc<[BridgeInvalidationTarget]> {
        &self.targets
    }

    pub fn len(&self) -> usize {
        self.targets.len()
    }
}
