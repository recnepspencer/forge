use forge_runtime_bridge::facade::{
    BridgeSymbolicTargetReferenceBundle, BridgeSymbolicTargetReferenceFamily,
    BridgeSymbolicTargetReferenceOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQuerySymbolicTargetReferenceOutcome {
    SameBatchSymbolicTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySymbolicTargetReferenceEvidence {
    family: crate::runtime::ForgeQuerySymbolicTargetReferenceFamily,
    outcome: ForgeQuerySymbolicTargetReferenceOutcome,
    symbol: String,
    resolved_entity_identity: String,
    target_collection: Option<String>,
}

impl ForgeQuerySymbolicTargetReferenceEvidence {
    pub(in crate::runtime) fn from_bridge(reference: &BridgeSymbolicTargetReferenceBundle) -> Self {
        Self {
            family: match reference.family() {
                BridgeSymbolicTargetReferenceFamily::SameBatchDeclaredTarget => {
                    crate::runtime::ForgeQuerySymbolicTargetReferenceFamily::SameBatchDeclaredTarget
                }
            },
            outcome: match reference.outcome() {
                BridgeSymbolicTargetReferenceOutcome::SameBatchSymbolicTarget => {
                    ForgeQuerySymbolicTargetReferenceOutcome::SameBatchSymbolicTarget
                }
            },
            symbol: reference.symbol().to_string(),
            resolved_entity_identity: reference.resolved_entity_identity().to_string(),
            target_collection: reference.target_collection().map(str::to_string),
        }
    }

    pub(in crate::runtime) fn from_reference(
        reference: &crate::runtime::ForgeQuerySymbolicTargetReference,
        resolved_entity_identity: &str,
    ) -> Self {
        Self {
            family: reference.family(),
            outcome: ForgeQuerySymbolicTargetReferenceOutcome::SameBatchSymbolicTarget,
            symbol: reference.symbol().to_string(),
            resolved_entity_identity: resolved_entity_identity.to_string(),
            target_collection: reference.target_collection().map(str::to_string),
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQuerySymbolicTargetReferenceFamily {
        self.family
    }

    pub fn outcome(&self) -> ForgeQuerySymbolicTargetReferenceOutcome {
        self.outcome
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn resolved_entity_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        symbol: impl Into<String>,
        resolved_entity_identity: impl Into<String>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family:
                crate::runtime::ForgeQuerySymbolicTargetReferenceFamily::SameBatchDeclaredTarget,
            outcome: ForgeQuerySymbolicTargetReferenceOutcome::SameBatchSymbolicTarget,
            symbol: symbol.into(),
            resolved_entity_identity: resolved_entity_identity.into(),
            target_collection: target_collection.map(str::to_string),
        }
    }
}
