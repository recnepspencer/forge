use worth_store_security::{
    S51SecurityScopeReadinessFamily, StoreAuthenticityRequirement, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeAdmissionDenial, StoreTenantScope,
};

use crate::{BackupExportCustodyCounterSnapshot, BackupExportCustodyMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupExportCustodyDenial {
    WrongReadinessFamily {
        actual: S51SecurityScopeReadinessFamily,
        counters: BackupExportCustodyCounterSnapshot,
    },
    WrongKeyScope {
        actual: StoreKeyScope,
        counters: BackupExportCustodyCounterSnapshot,
    },
    WrongTenantScope {
        actual: StoreTenantScope,
        counters: BackupExportCustodyCounterSnapshot,
    },
    WrongAuthenticityRequirement {
        actual: StoreAuthenticityRequirement,
        counters: BackupExportCustodyCounterSnapshot,
    },
    WrongCustodyPosture {
        actual: StoreCustodyPosture,
        counters: BackupExportCustodyCounterSnapshot,
    },
    NonCurrentKeyVersion {
        mode: BackupExportCustodyMode,
        posture: StoreKeyVersionPosture,
        counters: BackupExportCustodyCounterSnapshot,
    },
    ReadmissionNonCurrentKeyVersion {
        posture: StoreKeyVersionPosture,
        counters: BackupExportCustodyCounterSnapshot,
    },
    TrustBoundaryReadmissionDenied {
        source: StoreSecurityScopeAdmissionDenial,
        counters: BackupExportCustodyCounterSnapshot,
    },
    SecurityScopeAdmissionDenied {
        source: StoreSecurityScopeAdmissionDenial,
        counters: BackupExportCustodyCounterSnapshot,
    },
}
