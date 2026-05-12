use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSymbolicTargetReferenceFamily {
    SameBatchDeclaredTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSymbolicTargetReferenceOutcome {
    SameBatchSymbolicTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSymbolicTargetReferenceBundle {
    family: BridgeSymbolicTargetReferenceFamily,
    outcome: BridgeSymbolicTargetReferenceOutcome,
    symbol: Arc<str>,
    resolved_entity_identity: Arc<str>,
    target_collection: Option<Arc<str>>,
}

impl BridgeSymbolicTargetReferenceBundle {
    pub fn same_batch_target(
        symbol: impl Into<Arc<str>>,
        resolved_entity_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family: BridgeSymbolicTargetReferenceFamily::SameBatchDeclaredTarget,
            outcome: BridgeSymbolicTargetReferenceOutcome::SameBatchSymbolicTarget,
            symbol: symbol.into(),
            resolved_entity_identity: resolved_entity_identity.into(),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
        }
    }

    pub fn family(&self) -> BridgeSymbolicTargetReferenceFamily {
        self.family
    }

    pub fn outcome(&self) -> BridgeSymbolicTargetReferenceOutcome {
        self.outcome
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_ref()
    }

    pub fn resolved_entity_identity(&self) -> &str {
        self.resolved_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }
}
