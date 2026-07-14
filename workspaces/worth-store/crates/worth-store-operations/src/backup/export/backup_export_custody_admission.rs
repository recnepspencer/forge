use worth_store_security::{
    StoreAdmittedSecurityScope, StoreReadmittedSecurityScope, StoreSecurityScopeIdentity,
};

use crate::{BackupExportCustodyCounterSnapshot, BackupExportCustodyMode};

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportCustodyAdmission {
    mode: Option<BackupExportCustodyMode>,
    security_scope: BackupCustodySecurityScope,
    counters: BackupExportCustodyCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
enum BackupCustodySecurityScope {
    Outbound(StoreAdmittedSecurityScope),
    Readmitted(StoreReadmittedSecurityScope),
}

impl BackupExportCustodyAdmission {
    pub(crate) const fn from_outbound_declaration(
        mode: BackupExportCustodyMode,
        security_scope: StoreAdmittedSecurityScope,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Self {
        Self {
            mode: Some(mode),
            security_scope: BackupCustodySecurityScope::Outbound(security_scope),
            counters,
        }
    }

    pub(crate) const fn from_trust_boundary_readmission(
        security_scope: StoreReadmittedSecurityScope,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Self {
        Self {
            mode: None,
            security_scope: BackupCustodySecurityScope::Readmitted(security_scope),
            counters,
        }
    }

    pub const fn mode(&self) -> Option<BackupExportCustodyMode> {
        self.mode
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        match &self.security_scope {
            BackupCustodySecurityScope::Outbound(scope) => scope.identity(),
            BackupCustodySecurityScope::Readmitted(scope) => scope.admitted().identity(),
        }
    }

    pub const fn security_scope(&self) -> &StoreAdmittedSecurityScope {
        match &self.security_scope {
            BackupCustodySecurityScope::Outbound(scope) => scope,
            BackupCustodySecurityScope::Readmitted(scope) => scope.admitted(),
        }
    }

    pub const fn readmitted_security_scope(&self) -> Option<&StoreReadmittedSecurityScope> {
        match &self.security_scope {
            BackupCustodySecurityScope::Outbound(_) => None,
            BackupCustodySecurityScope::Readmitted(scope) => Some(scope),
        }
    }

    pub fn into_readmitted_security_scope(self) -> Option<StoreReadmittedSecurityScope> {
        match self.security_scope {
            BackupCustodySecurityScope::Outbound(_) => None,
            BackupCustodySecurityScope::Readmitted(scope) => Some(scope),
        }
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }

    pub fn into_security_scope(self) -> StoreAdmittedSecurityScope {
        match self.security_scope {
            BackupCustodySecurityScope::Outbound(scope) => scope,
            BackupCustodySecurityScope::Readmitted(scope) => scope.into_admitted(),
        }
    }
}
