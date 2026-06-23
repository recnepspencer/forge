use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryMutationSymbolIdentity, ForgeQueryMutationTargetCollectionIdentity,
};
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
    symbol: ForgeQueryMutationSymbolIdentity,
    resolved_entity_identity: ForgeQueryEntityIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQuerySymbolicTargetReferenceEvidence {
    pub(in crate::runtime) fn from_bridge_with_query_context(
        reference: &BridgeSymbolicTargetReferenceBundle,
        _resolved_entity_identity: Option<&ForgeQueryEntityIdentity>,
        target_collection: Option<&str>,
    ) -> Self {
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
            symbol: ForgeQueryMutationSymbolIdentity::from_bridge_symbolic_target(
                "symbolic-target-reference",
                reference.symbol_handle(),
            ),
            resolved_entity_identity: ForgeQueryEntityIdentity::from_relational_record(
                reference
                    .resolved_entity_identity_handle()
                    .relational_record_parts(),
            ),
            target_collection: reference.target_collection().or(target_collection).map(
                |collection| {
                    ForgeQueryMutationTargetCollectionIdentity::new("symbolic-target", collection)
                },
            ),
        }
    }

    pub(in crate::runtime) fn from_reference(
        reference: &crate::runtime::ForgeQuerySymbolicTargetReference,
        resolved_entity_identity: &ForgeQueryEntityIdentity,
    ) -> Self {
        Self {
            family: reference.family(),
            outcome: ForgeQuerySymbolicTargetReferenceOutcome::SameBatchSymbolicTarget,
            symbol: ForgeQueryMutationSymbolIdentity::new(
                "symbolic-target-reference",
                reference.symbol(),
            ),
            resolved_entity_identity: resolved_entity_identity.clone(),
            target_collection: reference.target_collection_identity().cloned(),
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQuerySymbolicTargetReferenceFamily {
        self.family
    }

    pub fn outcome(&self) -> ForgeQuerySymbolicTargetReferenceOutcome {
        self.outcome
    }

    pub fn symbol(&self) -> &ForgeQueryMutationSymbolIdentity {
        &self.symbol
    }

    pub fn resolved_entity_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        symbol: impl Into<String>,
        resolved_entity_identity: ForgeQueryEntityIdentity,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family:
                crate::runtime::ForgeQuerySymbolicTargetReferenceFamily::SameBatchDeclaredTarget,
            outcome: ForgeQuerySymbolicTargetReferenceOutcome::SameBatchSymbolicTarget,
            symbol: ForgeQueryMutationSymbolIdentity::new("symbolic-target-reference", symbol),
            resolved_entity_identity,
            target_collection: target_collection.map(|collection| {
                ForgeQueryMutationTargetCollectionIdentity::new("symbolic-target", collection)
            }),
        }
    }
}
