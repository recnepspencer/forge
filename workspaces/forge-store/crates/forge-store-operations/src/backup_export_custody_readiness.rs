use forge_store_readiness::{S51AdmittedSecurityScopeReadiness, S51SecurityScopeReadinessFamily};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{
    backup_export_custody_declaration::backup_capsule_authenticity, BackupExportCustodyAdmission,
    BackupExportCustodyCounterSnapshot, BackupExportCustodyDenial, S10BackupExportCustodyHandoff,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportCustodyReadiness {
    identity: StoreSecurityScopeIdentity,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: BackupExportCustodyCounterSnapshot,
}

impl BackupExportCustodyReadiness {
    pub fn from_admitted_custody(
        admission: BackupExportCustodyAdmission,
    ) -> Result<Self, BackupExportCustodyDenial> {
        let counters = admission.counters();
        let readiness = admission.into_readiness();
        Self::from_admitted_readiness(readiness, counters)
    }

    pub fn from_s10_handoff(handoff: S10BackupExportCustodyHandoff) -> Self {
        handoff.into_readiness()
    }

    pub(crate) fn from_admitted_readiness(
        readiness: S51AdmittedSecurityScopeReadiness,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Result<Self, BackupExportCustodyDenial> {
        reject_wrong_family(&readiness, counters)?;
        let identity = readiness.receipt().identity();
        reject_wrong_key_scope(identity, counters)?;
        reject_wrong_tenant_scope(identity, counters)?;
        reject_wrong_authenticity(identity, counters)?;
        reject_wrong_custody(identity, counters)?;

        Ok(Self {
            identity,
            receipt: readiness.receipt(),
            counters,
        })
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.identity.custody_posture()
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }
}

fn reject_wrong_family(
    readiness: &S51AdmittedSecurityScopeReadiness,
    counters: BackupExportCustodyCounterSnapshot,
) -> Result<(), BackupExportCustodyDenial> {
    let actual = readiness.reservation().family();
    if actual == S51SecurityScopeReadinessFamily::BackupExportCustody {
        Ok(())
    } else {
        Err(BackupExportCustodyDenial::WrongReadinessFamily {
            actual,
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_key_scope(
    identity: StoreSecurityScopeIdentity,
    counters: BackupExportCustodyCounterSnapshot,
) -> Result<(), BackupExportCustodyDenial> {
    if identity.key_scope() == StoreKeyScope::BackupExportEnvelope {
        Ok(())
    } else {
        Err(BackupExportCustodyDenial::WrongKeyScope {
            actual: identity.key_scope(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_tenant_scope(
    identity: StoreSecurityScopeIdentity,
    counters: BackupExportCustodyCounterSnapshot,
) -> Result<(), BackupExportCustodyDenial> {
    match identity.tenant_scope() {
        StoreTenantScope::BackupRestoreBoundary | StoreTenantScope::ImportReadmissionBoundary => {
            Ok(())
        }
        actual => Err(BackupExportCustodyDenial::WrongTenantScope {
            actual,
            counters: counters.denied(),
        }),
    }
}

fn reject_wrong_authenticity(
    identity: StoreSecurityScopeIdentity,
    counters: BackupExportCustodyCounterSnapshot,
) -> Result<(), BackupExportCustodyDenial> {
    let expected: StoreAuthenticityRequirement = backup_capsule_authenticity();
    if identity.authenticity_requirement() == expected {
        Ok(())
    } else {
        Err(BackupExportCustodyDenial::WrongAuthenticityRequirement {
            actual: identity.authenticity_requirement(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_custody(
    identity: StoreSecurityScopeIdentity,
    counters: BackupExportCustodyCounterSnapshot,
) -> Result<(), BackupExportCustodyDenial> {
    match identity.custody_posture() {
        StoreCustodyPosture::ExportPrepared | StoreCustodyPosture::Readmitted => Ok(()),
        StoreCustodyPosture::CustodyUnavailable => {
            Err(BackupExportCustodyDenial::WrongCustodyPosture {
                actual: identity.custody_posture(),
                counters: counters.record_unavailable_custody_evidence().denied(),
            })
        }
        StoreCustodyPosture::CustodyUnsupported => {
            Err(BackupExportCustodyDenial::WrongCustodyPosture {
                actual: identity.custody_posture(),
                counters: counters.record_unsupported_secure_posture().denied(),
            })
        }
        actual => Err(BackupExportCustodyDenial::WrongCustodyPosture {
            actual,
            counters: counters.record_custody_denied().denied(),
        }),
    }
}
