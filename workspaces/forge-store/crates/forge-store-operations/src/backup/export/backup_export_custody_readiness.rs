use forge_store_security::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{
    backup::export::backup_capsule_authenticity, BackupExportCustodyAdmission,
    BackupExportCustodyCounterSnapshot, BackupExportCustodyDenial, S10BackupExportCustodyHandoff,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportCustodyReadiness {
    mode: Option<crate::BackupExportCustodyMode>,
    identity: StoreSecurityScopeIdentity,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: BackupExportCustodyCounterSnapshot,
}

impl BackupExportCustodyReadiness {
    pub fn from_admitted_custody(
        admission: BackupExportCustodyAdmission,
    ) -> Result<Self, BackupExportCustodyDenial> {
        let counters = admission.counters();
        let mode = admission.mode();
        let security_scope = admission.into_security_scope();
        Self::from_admitted_scope(security_scope, mode, counters)
    }

    pub fn from_backup_repair_handoff(handoff: S10BackupExportCustodyHandoff) -> Self {
        let readiness = handoff.into_readiness();
        Self {
            mode: readiness.mode(),
            identity: readiness.identity(),
            receipt: readiness.receipt(),
            counters: readiness.counters(),
        }
    }

    pub(crate) fn from_admitted_scope(
        security_scope: StoreAdmittedSecurityScope,
        mode: Option<crate::BackupExportCustodyMode>,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Result<Self, BackupExportCustodyDenial> {
        let identity = security_scope.identity();
        reject_wrong_key_scope(identity, counters)?;
        reject_wrong_tenant_scope(identity, counters)?;
        reject_wrong_authenticity(identity, counters)?;
        reject_wrong_custody(identity, counters)?;

        Ok(Self {
            mode,
            identity,
            receipt: security_scope.receipt(),
            counters,
        })
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.identity.custody_posture()
    }

    pub const fn mode(&self) -> Option<crate::BackupExportCustodyMode> {
        self.mode
    }

    pub const fn mode_label(&self) -> &'static str {
        match self.mode {
            Some(crate::BackupExportCustodyMode::Backup) => "backup",
            Some(crate::BackupExportCustodyMode::PointInTimeRecovery) => "pitr",
            Some(crate::BackupExportCustodyMode::Export) => "export",
            None => "unspecified",
        }
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
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
