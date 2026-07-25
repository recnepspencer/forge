use worth_store_security::StoreAuthorityBoundSecurityScopeReceipt;

use super::{
    effect_contract::require_effect_contract, PhysicalWorkDeclarationDenial,
    PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass, PhysicalWorkOperationFamily,
    PhysicalWorkRecoveryDisposition, PhysicalWorkResourceDemand, PhysicalWorkScope,
};
use crate::physical_runtime::work::{
    PhysicalSignalProfileIdentity, PhysicalWorkIdentity, PhysicalWorkSemanticBasis,
};

/// Immutable Store-owned declaration crossing mechanism boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkIntent {
    identity: PhysicalWorkIdentity,
    operation: PhysicalWorkOperationFamily,
    scope: PhysicalWorkScope,
    semantic_basis: PhysicalWorkSemanticBasis,
    security: StoreAuthorityBoundSecurityScopeReceipt,
    resources: PhysicalWorkResourceDemand,
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    signal_profile: PhysicalSignalProfileIdentity,
    recovery: PhysicalWorkRecoveryDisposition,
}

pub(in crate::physical_runtime) struct PhysicalWorkIntentParts {
    pub(in crate::physical_runtime) identity: PhysicalWorkIdentity,
    pub(in crate::physical_runtime) operation: PhysicalWorkOperationFamily,
    pub(in crate::physical_runtime) scope: PhysicalWorkScope,
    pub(in crate::physical_runtime) semantic_basis: PhysicalWorkSemanticBasis,
    pub(in crate::physical_runtime) security: StoreAuthorityBoundSecurityScopeReceipt,
    pub(in crate::physical_runtime) effect: PhysicalWorkEffectClass,
    pub(in crate::physical_runtime) durability: PhysicalWorkDurabilityRequirement,
    pub(in crate::physical_runtime) signal_profile: PhysicalSignalProfileIdentity,
    pub(in crate::physical_runtime) recovery: PhysicalWorkRecoveryDisposition,
}

impl PhysicalWorkIntent {
    pub(in crate::physical_runtime) fn from_instance_owner(
        parts: PhysicalWorkIntentParts,
    ) -> Result<Self, PhysicalWorkDeclarationDenial> {
        require_effect_contract(
            parts.operation,
            parts.effect,
            parts.durability,
            parts.recovery,
        )?;
        let resources =
            PhysicalWorkResourceDemand::derive(&parts.scope, parts.operation, parts.durability);
        Ok(Self {
            identity: parts.identity,
            operation: parts.operation,
            scope: parts.scope,
            semantic_basis: parts.semantic_basis,
            security: parts.security,
            resources,
            effect: parts.effect,
            durability: parts.durability,
            signal_profile: parts.signal_profile,
            recovery: parts.recovery,
        })
    }

    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }
    pub const fn operation(&self) -> PhysicalWorkOperationFamily {
        self.operation
    }
    pub const fn scope(&self) -> &PhysicalWorkScope {
        &self.scope
    }
    pub const fn semantic_basis(&self) -> &PhysicalWorkSemanticBasis {
        &self.semantic_basis
    }
    pub const fn security(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.security.receipt().identity()
    }
    pub(in crate::physical_runtime) const fn security_authority(
        &self,
    ) -> StoreAuthorityBoundSecurityScopeReceipt {
        self.security
    }
    pub const fn resources(&self) -> PhysicalWorkResourceDemand {
        self.resources
    }
    pub const fn effect(&self) -> PhysicalWorkEffectClass {
        self.effect
    }
    pub const fn durability(&self) -> PhysicalWorkDurabilityRequirement {
        self.durability
    }
    pub const fn signal_profile(&self) -> PhysicalSignalProfileIdentity {
        self.signal_profile
    }
    pub const fn recovery(&self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }
}
