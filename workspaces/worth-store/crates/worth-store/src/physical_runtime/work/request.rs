use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_security::StoreAuthorityBoundSecurityScopeReceipt;

use super::{
    PhysicalWorkEffectClass, PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition,
    PhysicalWorkScope, PhysicalWorkSemanticBasis, PhysicalWorkSemanticPosture,
    PhysicalWorkSubmissionDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalReadWorkRequest {
    pub(super) scope: PhysicalWorkScope,
    pub(super) semantic_basis: PhysicalWorkSemanticBasis,
    pub(super) security: StoreAuthorityBoundSecurityScopeReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMetadataReadWorkRequest {
    pub(super) scope: PhysicalWorkScope,
    pub(super) semantic_basis: PhysicalWorkSemanticBasis,
    pub(super) security: StoreAuthorityBoundSecurityScopeReceipt,
}

impl PhysicalMetadataReadWorkRequest {
    pub fn new(
        artifact: worth_store_physical_format::RecordArtifactFile,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        if semantic_basis.posture() != PhysicalWorkSemanticPosture::Projection {
            return Err(PhysicalWorkSubmissionDenial::SemanticPostureMismatch);
        }
        require_security_witness(&semantic_basis, security)?;
        Ok(Self {
            scope: PhysicalWorkScope::artifact(artifact),
            semantic_basis,
            security,
        })
    }
}

impl PhysicalReadWorkRequest {
    pub fn new(
        scope: PhysicalWorkScope,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        if semantic_basis.posture() != PhysicalWorkSemanticPosture::Projection {
            return Err(PhysicalWorkSubmissionDenial::SemanticPostureMismatch);
        }
        require_security_witness(&semantic_basis, security)?;
        Ok(Self {
            scope,
            semantic_basis,
            security,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMutationWorkRequest {
    pub(super) operation: PhysicalWorkOperationFamily,
    pub(super) scope: PhysicalWorkScope,
    pub(super) semantic_basis: PhysicalWorkSemanticBasis,
    pub(super) security: StoreAuthorityBoundSecurityScopeReceipt,
    pub(super) effect: PhysicalWorkEffectClass,
    pub(super) durability: super::PhysicalWorkDurabilityRequirement,
    pub(super) recovery: PhysicalWorkRecoveryDisposition,
}

impl PhysicalMutationWorkRequest {
    pub fn exact_write(
        scope: PhysicalWorkScope,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        Self::new(
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            scope,
            semantic_basis,
            security,
            PhysicalWorkEffectClass::IdempotentExactWrite,
            super::PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(durability),
            PhysicalWorkRecoveryDisposition::RetryExact,
        )
    }

    pub fn publication(
        scope: PhysicalWorkScope,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        Self::new(
            PhysicalWorkOperationFamily::ArtifactPublication,
            scope,
            semantic_basis,
            security,
            PhysicalWorkEffectClass::PublicationBoundary,
            super::PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(durability),
            PhysicalWorkRecoveryDisposition::ContinueSettlement,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        operation: PhysicalWorkOperationFamily,
        scope: PhysicalWorkScope,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
        effect: PhysicalWorkEffectClass,
        durability: super::PhysicalWorkDurabilityRequirement,
        recovery: PhysicalWorkRecoveryDisposition,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        if semantic_basis.posture() != PhysicalWorkSemanticPosture::Mutation {
            return Err(PhysicalWorkSubmissionDenial::SemanticPostureMismatch);
        }
        require_security_witness(&semantic_basis, security)?;
        Ok(Self {
            operation,
            scope,
            semantic_basis,
            security,
            effect,
            durability,
            recovery,
        })
    }

    pub(in crate::physical_runtime) fn wal_append(
        scope: super::PhysicalWalAppendScope,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        Self::new(
            PhysicalWorkOperationFamily::WalAppend,
            PhysicalWorkScope::wal_append(scope),
            semantic_basis,
            security,
            PhysicalWorkEffectClass::ReversibleBeforePublication,
            super::PhysicalWorkDurabilityRequirement::WalAppend,
            PhysicalWorkRecoveryDisposition::InspectionRequired,
        )
    }

    pub(in crate::physical_runtime) fn wal_durability_barrier(
        scope: super::PhysicalWalBarrierScope,
        semantic_basis: PhysicalWorkSemanticBasis,
        security: StoreAuthorityBoundSecurityScopeReceipt,
    ) -> Result<Self, PhysicalWorkSubmissionDenial> {
        Self::new(
            PhysicalWorkOperationFamily::DurabilityBarrier,
            PhysicalWorkScope::wal_barrier(scope),
            semantic_basis,
            security,
            PhysicalWorkEffectClass::PublicationBoundary,
            super::PhysicalWorkDurabilityRequirement::WalDurabilityBarrier,
            PhysicalWorkRecoveryDisposition::InspectionRequired,
        )
    }
}

fn require_security_witness(
    semantic_basis: &PhysicalWorkSemanticBasis,
    security: StoreAuthorityBoundSecurityScopeReceipt,
) -> Result<(), PhysicalWorkSubmissionDenial> {
    if semantic_basis.physical_witness() == security.receipt().identity().physical_witness() {
        Ok(())
    } else {
        Err(PhysicalWorkSubmissionDenial::SecurityScopeWitnessMismatch)
    }
}
