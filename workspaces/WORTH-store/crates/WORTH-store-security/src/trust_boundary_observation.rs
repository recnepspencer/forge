use worth_store_aspect_native::StoreAspectBoundaryFact;

use crate::{
    StoreBackupRestoreBoundaryFactInput, StoreCustodyDomainBoundaryFactInput,
    StoreDeploymentBoundaryFact, StoreKeyScopeGenerationBoundaryFactInput,
    StoreOfflineTransferBoundaryFact, StoreStoreInstanceBoundaryFact,
    StoreTenantScopeAuthorityBoundaryFactInput, StoreTrustBoundaryEvidenceDenial,
};

pub fn store_deployment_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreDeploymentBoundaryFact, StoreTrustBoundaryEvidenceDenial> {
    StoreDeploymentBoundaryFact::observed_deployment(fact)
}

pub fn store_instance_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreStoreInstanceBoundaryFact, StoreTrustBoundaryEvidenceDenial> {
    StoreStoreInstanceBoundaryFact::observed_store_instance(fact)
}

pub fn store_key_scope_generation_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreKeyScopeGenerationBoundaryFactInput, StoreTrustBoundaryEvidenceDenial> {
    StoreKeyScopeGenerationBoundaryFactInput::observed_key_scope_generation(fact)
}

pub fn store_tenant_scope_authority_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreTenantScopeAuthorityBoundaryFactInput, StoreTrustBoundaryEvidenceDenial> {
    StoreTenantScopeAuthorityBoundaryFactInput::observed_tenant_scope_authority(fact)
}

pub fn store_custody_domain_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreCustodyDomainBoundaryFactInput, StoreTrustBoundaryEvidenceDenial> {
    StoreCustodyDomainBoundaryFactInput::observed_custody_domain(fact)
}

pub fn store_offline_transfer_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreOfflineTransferBoundaryFact, StoreTrustBoundaryEvidenceDenial> {
    StoreOfflineTransferBoundaryFact::observed_offline_transfer(fact)
}

pub fn store_backup_restore_boundary_fact(
    fact: StoreAspectBoundaryFact,
) -> Result<StoreBackupRestoreBoundaryFactInput, StoreTrustBoundaryEvidenceDenial> {
    StoreBackupRestoreBoundaryFactInput::observed_backup_restore(fact)
}
