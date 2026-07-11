use forge_foundational::aspects;
use forge_store_aspect_native::{
    StoreAspectBoundaryFact, StoreAspectIdentity, StorePhysicalBoundaryWitness,
};

use crate::StoreTrustBoundaryEvidenceDenial;

const DEPLOYMENT_BOUNDARY_ASPECT: &str = "store.trust_boundary.deployment";
const STORE_INSTANCE_BOUNDARY_ASPECT: &str = "store.trust_boundary.store_instance";
const KEY_SCOPE_GENERATION_BOUNDARY_ASPECT: &str = "store.trust_boundary.key_scope_generation";
const TENANT_SCOPE_AUTHORITY_BOUNDARY_ASPECT: &str = "store.trust_boundary.tenant_scope_authority";
const CUSTODY_DOMAIN_BOUNDARY_ASPECT: &str = "store.trust_boundary.custody_domain";
const OFFLINE_TRANSFER_BOUNDARY_ASPECT: &str = "store.trust_boundary.offline_transfer";
const BACKUP_RESTORE_BOUNDARY_ASPECT: &str = "store.trust_boundary.backup_restore";

macro_rules! define_trust_boundary_category {
    ($source:ident, $evidence:ident, $constructor:ident, $aspect:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $source {
            fact: StoreAspectBoundaryFact,
        }

        impl $source {
            pub fn $constructor(
                fact: StoreAspectBoundaryFact,
            ) -> Result<Self, StoreTrustBoundaryEvidenceDenial> {
                if !store_aspect_fact_matches_category(&fact, $aspect) {
                    return Err(StoreTrustBoundaryEvidenceDenial::WrongTrustBoundaryCategory);
                }

                Ok(Self { fact })
            }

            pub const fn identity(&self) -> &StoreAspectIdentity {
                self.fact.identity()
            }

            pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
                self.fact.authority_input().physical_witness()
            }

            pub const fn store_aspect_fact(&self) -> &StoreAspectBoundaryFact {
                &self.fact
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $evidence {
            exported_fact: $source,
            current_fact: $source,
        }

        impl $evidence {
            pub fn from_category_facts(
                exported_fact: $source,
                current_fact: $source,
            ) -> Result<Self, StoreTrustBoundaryEvidenceDenial> {
                if exported_fact == current_fact {
                    Err(StoreTrustBoundaryEvidenceDenial::MissingCategoryBoundaryChange)
                } else {
                    Ok(Self {
                        exported_fact,
                        current_fact,
                    })
                }
            }

            pub const fn exported_fact(&self) -> &$source {
                &self.exported_fact
            }

            pub const fn current_fact(&self) -> &$source {
                &self.current_fact
            }
        }
    };
}

define_trust_boundary_category!(
    StoreDeploymentBoundaryFact,
    StoreDifferentDeploymentBoundaryEvidence,
    observed_deployment,
    DEPLOYMENT_BOUNDARY_ASPECT
);
define_trust_boundary_category!(
    StoreStoreInstanceBoundaryFact,
    StoreDifferentStoreInstanceBoundaryEvidence,
    observed_store_instance,
    STORE_INSTANCE_BOUNDARY_ASPECT
);
define_trust_boundary_category!(
    StoreKeyScopeGenerationBoundaryFactInput,
    StoreKeyScopeGenerationBoundaryEvidence,
    observed_key_scope_generation,
    KEY_SCOPE_GENERATION_BOUNDARY_ASPECT
);
define_trust_boundary_category!(
    StoreTenantScopeAuthorityBoundaryFactInput,
    StoreTenantScopeAuthorityBoundaryEvidence,
    observed_tenant_scope_authority,
    TENANT_SCOPE_AUTHORITY_BOUNDARY_ASPECT
);
define_trust_boundary_category!(
    StoreCustodyDomainBoundaryFactInput,
    StoreCustodyDomainBoundaryEvidence,
    observed_custody_domain,
    CUSTODY_DOMAIN_BOUNDARY_ASPECT
);
define_trust_boundary_category!(
    StoreOfflineTransferBoundaryFact,
    StoreOfflineExportImportBoundaryEvidence,
    observed_offline_transfer,
    OFFLINE_TRANSFER_BOUNDARY_ASPECT
);
define_trust_boundary_category!(
    StoreBackupRestoreBoundaryFactInput,
    StoreBackupRestoreAfterKeyRotationBoundaryEvidence,
    observed_backup_restore,
    BACKUP_RESTORE_BOUNDARY_ASPECT
);

fn store_aspect_fact_matches_category(fact: &StoreAspectBoundaryFact, category: &str) -> bool {
    let Ok(category_key) = aspects().vocabulary().key(category) else {
        return false;
    };
    fact.identity() == &StoreAspectIdentity::from_aspect_key(category_key)
}
