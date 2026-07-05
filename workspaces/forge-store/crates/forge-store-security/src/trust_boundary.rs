use forge_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeIdentity,
};

pub use crate::trust_boundary_category::{
    StoreBackupRestoreAfterKeyRotationBoundaryEvidence, StoreBackupRestoreBoundaryFactInput,
    StoreCustodyDomainBoundaryEvidence, StoreCustodyDomainBoundaryFactInput,
    StoreDeploymentBoundaryFact, StoreDifferentDeploymentBoundaryEvidence,
    StoreDifferentStoreInstanceBoundaryEvidence, StoreKeyScopeGenerationBoundaryEvidence,
    StoreKeyScopeGenerationBoundaryFactInput, StoreOfflineExportImportBoundaryEvidence,
    StoreOfflineTransferBoundaryFact, StoreStoreInstanceBoundaryFact,
    StoreTenantScopeAuthorityBoundaryEvidence, StoreTenantScopeAuthorityBoundaryFactInput,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreTrustBoundaryCrossing {
    DifferentDeployment,
    DifferentStoreInstance,
    KeyScopeGenerationChanged,
    TenantScopeAuthorityChanged,
    CustodyDomainChanged,
    OfflineExportImport,
    BackupRestoreAfterKeyRotation,
}

impl StoreTrustBoundaryCrossing {
    pub const fn requires_security_scope_readmission(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreTrustBoundaryEvidence {
    exported_identity: StoreSecurityScopeIdentity,
    current_identity: StoreSecurityScopeIdentity,
}

impl StoreTrustBoundaryEvidence {
    pub fn from_security_scope_readmission_candidate(
        declaration: StoreRawSecurityScopeDeclaration,
        current_authority: &StoreCurrentAuthorityWitness,
        expectation: StoreSecurityScopeAdmissionExpectation,
    ) -> Result<Self, StoreTrustBoundaryEvidenceDenial> {
        let exported_identity = declaration_identity(declaration)?;
        let current_identity = StoreSecurityScopeIdentity::from_physical_security_scope(
            current_authority.physical_witness(),
            expectation.key_scope(),
            StoreKeyVersionPosture::Current,
            expectation.tenant_scope(),
            expectation.authenticity_requirement(),
            expectation.custody_posture(),
        );

        if exported_identity == current_identity {
            Err(StoreTrustBoundaryEvidenceDenial::MissingBoundaryChange)
        } else {
            Ok(Self {
                exported_identity,
                current_identity,
            })
        }
    }

    pub const fn exported_identity(self) -> StoreSecurityScopeIdentity {
        self.exported_identity
    }

    pub const fn current_identity(self) -> StoreSecurityScopeIdentity {
        self.current_identity
    }
}

macro_rules! define_trust_boundary_fact {
    ($fact:ident, $category:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $fact {
            candidate_evidence: StoreTrustBoundaryEvidence,
            category_evidence: $category,
        }

        impl $fact {
            pub fn from_readmission_candidate(
                category_evidence: $category,
                declaration: StoreRawSecurityScopeDeclaration,
                current_authority: &StoreCurrentAuthorityWitness,
                expectation: StoreSecurityScopeAdmissionExpectation,
            ) -> Result<Self, StoreTrustBoundaryEvidenceDenial> {
                Ok(Self {
                    candidate_evidence:
                        StoreTrustBoundaryEvidence::from_security_scope_readmission_candidate(
                            declaration,
                            current_authority,
                            expectation,
                        )?,
                    category_evidence,
                })
            }

            const fn candidate_evidence(&self) -> StoreTrustBoundaryEvidence {
                self.candidate_evidence
            }
        }
    };
}

define_trust_boundary_fact!(
    StoreDifferentDeploymentBoundaryFact,
    StoreDifferentDeploymentBoundaryEvidence
);
define_trust_boundary_fact!(
    StoreDifferentStoreInstanceBoundaryFact,
    StoreDifferentStoreInstanceBoundaryEvidence
);
define_trust_boundary_fact!(
    StoreKeyScopeGenerationBoundaryFact,
    StoreKeyScopeGenerationBoundaryEvidence
);
define_trust_boundary_fact!(
    StoreTenantScopeAuthorityBoundaryFact,
    StoreTenantScopeAuthorityBoundaryEvidence
);
define_trust_boundary_fact!(
    StoreCustodyDomainBoundaryFact,
    StoreCustodyDomainBoundaryEvidence
);
define_trust_boundary_fact!(
    StoreOfflineExportImportBoundaryFact,
    StoreOfflineExportImportBoundaryEvidence
);
define_trust_boundary_fact!(
    StoreBackupRestoreAfterKeyRotationBoundaryFact,
    StoreBackupRestoreAfterKeyRotationBoundaryEvidence
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreTrustBoundaryCrossingEvidence {
    DifferentDeployment(StoreDifferentDeploymentBoundaryFact),
    DifferentStoreInstance(StoreDifferentStoreInstanceBoundaryFact),
    KeyScopeGenerationChanged(StoreKeyScopeGenerationBoundaryFact),
    TenantScopeAuthorityChanged(StoreTenantScopeAuthorityBoundaryFact),
    CustodyDomainChanged(StoreCustodyDomainBoundaryFact),
    OfflineExportImport(StoreOfflineExportImportBoundaryFact),
    BackupRestoreAfterKeyRotation(StoreBackupRestoreAfterKeyRotationBoundaryFact),
}

impl StoreTrustBoundaryCrossingEvidence {
    pub const fn crossing(&self) -> StoreTrustBoundaryCrossing {
        match self {
            Self::DifferentDeployment(_) => StoreTrustBoundaryCrossing::DifferentDeployment,
            Self::DifferentStoreInstance(_) => StoreTrustBoundaryCrossing::DifferentStoreInstance,
            Self::KeyScopeGenerationChanged(_) => {
                StoreTrustBoundaryCrossing::KeyScopeGenerationChanged
            }
            Self::TenantScopeAuthorityChanged(_) => {
                StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged
            }
            Self::CustodyDomainChanged(_) => StoreTrustBoundaryCrossing::CustodyDomainChanged,
            Self::OfflineExportImport(_) => StoreTrustBoundaryCrossing::OfflineExportImport,
            Self::BackupRestoreAfterKeyRotation(_) => {
                StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation
            }
        }
    }

    pub const fn candidate_evidence(&self) -> StoreTrustBoundaryEvidence {
        match self {
            Self::DifferentDeployment(fact) => fact.candidate_evidence(),
            Self::DifferentStoreInstance(fact) => fact.candidate_evidence(),
            Self::KeyScopeGenerationChanged(fact) => fact.candidate_evidence(),
            Self::TenantScopeAuthorityChanged(fact) => fact.candidate_evidence(),
            Self::CustodyDomainChanged(fact) => fact.candidate_evidence(),
            Self::OfflineExportImport(fact) => fact.candidate_evidence(),
            Self::BackupRestoreAfterKeyRotation(fact) => fact.candidate_evidence(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreTrustBoundaryEvidenceDenial {
    MissingAuthenticityRequirement,
    MissingBoundaryChange,
    MissingCategoryBoundaryChange,
    MissingCustodyPosture,
    WrongTrustBoundaryCategory,
    CrossingEvidenceMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreTrustBoundaryReadmissionTrigger {
    crossing_evidence: StoreTrustBoundaryCrossingEvidence,
}

impl StoreTrustBoundaryReadmissionTrigger {
    pub const fn different_deployment(fact: StoreDifferentDeploymentBoundaryFact) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::DifferentDeployment(fact),
        }
    }

    pub const fn different_store_instance(fact: StoreDifferentStoreInstanceBoundaryFact) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::DifferentStoreInstance(fact),
        }
    }

    pub const fn key_scope_generation_changed(fact: StoreKeyScopeGenerationBoundaryFact) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::KeyScopeGenerationChanged(fact),
        }
    }

    pub const fn tenant_scope_authority_changed(
        fact: StoreTenantScopeAuthorityBoundaryFact,
    ) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::TenantScopeAuthorityChanged(
                fact,
            ),
        }
    }

    pub const fn custody_domain_changed(fact: StoreCustodyDomainBoundaryFact) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::CustodyDomainChanged(fact),
        }
    }

    pub const fn offline_export_import(fact: StoreOfflineExportImportBoundaryFact) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::OfflineExportImport(fact),
        }
    }

    pub const fn backup_restore_after_key_rotation(
        fact: StoreBackupRestoreAfterKeyRotationBoundaryFact,
    ) -> Self {
        Self {
            crossing_evidence: StoreTrustBoundaryCrossingEvidence::BackupRestoreAfterKeyRotation(
                fact,
            ),
        }
    }

    pub const fn crossing(&self) -> StoreTrustBoundaryCrossing {
        self.crossing_evidence.crossing()
    }

    pub const fn evidence(&self) -> StoreTrustBoundaryEvidence {
        self.crossing_evidence.candidate_evidence()
    }

    pub const fn crossing_evidence(&self) -> &StoreTrustBoundaryCrossingEvidence {
        &self.crossing_evidence
    }

    pub const fn requires_security_scope_readmission(&self) -> bool {
        self.crossing().requires_security_scope_readmission()
    }

    pub(crate) fn bind_to_readmission_candidate(
        &self,
        declaration: StoreRawSecurityScopeDeclaration,
        current_authority: &StoreCurrentAuthorityWitness,
        expectation: StoreSecurityScopeAdmissionExpectation,
    ) -> Result<(), StoreTrustBoundaryEvidenceDenial> {
        let evidence = StoreTrustBoundaryEvidence::from_security_scope_readmission_candidate(
            declaration,
            current_authority,
            expectation,
        )?;
        if self.evidence() == evidence {
            Ok(())
        } else {
            Err(StoreTrustBoundaryEvidenceDenial::CrossingEvidenceMismatch)
        }
    }
}

fn declaration_identity(
    declaration: StoreRawSecurityScopeDeclaration,
) -> Result<StoreSecurityScopeIdentity, StoreTrustBoundaryEvidenceDenial> {
    Ok(StoreSecurityScopeIdentity::from_physical_security_scope(
        declaration.physical_witness(),
        declaration.key_scope(),
        declaration.key_version_posture(),
        declaration.tenant_scope(),
        declaration
            .authenticity_requirement()
            .ok_or(StoreTrustBoundaryEvidenceDenial::MissingAuthenticityRequirement)?,
        declaration
            .custody_posture()
            .ok_or(StoreTrustBoundaryEvidenceDenial::MissingCustodyPosture)?,
    ))
}
