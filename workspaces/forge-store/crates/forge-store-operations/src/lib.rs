#![forbid(unsafe_code)]
//!
//! Backup/export custody readiness is not raw security metadata:
//!
//! ```compile_fail
//! use forge_store_operations::BackupExportCapsuleEmission;
//! use forge_store_security::StoreRawSecurityScopeDeclaration;
//!
//! let raw: StoreRawSecurityScopeDeclaration = todo!();
//! let _emission: BackupExportCapsuleEmission = raw;
//! ```
//!
//! Imported capsule observation is not readmitted readiness:
//!
//! ```compile_fail
//! use forge_store_offline_verifier::OfflineCustodyCapsuleObservation;
//! use forge_store_operations::BackupExportCustodyReadiness;
//!
//! let observed: OfflineCustodyCapsuleObservation = todo!();
//! let _readiness: BackupExportCustodyReadiness = observed;
//! ```
//!
//! Generic S.5.1 readiness cannot construct backup/export custody readiness:
//!
//! ```compile_fail
//! use forge_store_operations::BackupExportCustodyReadiness;
//! use forge_store_readiness::S51AdmittedSecurityScopeReadiness;
//!
//! let readiness: S51AdmittedSecurityScopeReadiness = todo!();
//! let _custody = BackupExportCustodyReadiness::from_s5_1_readiness(readiness);
//! ```
//!
//! S.5.1 security-scope readiness still cannot bypass custody preparation:
//!
//! ```compile_fail
//! use forge_store_operations::BackupExportTerminalProjectionPreparation;
//! use forge_store_readiness::S51AdmittedSecurityScopeReadiness;
//!
//! let readiness: S51AdmittedSecurityScopeReadiness = todo!();
//! let _projection: BackupExportTerminalProjectionPreparation = readiness;
//! ```
//!
//! Counters are evidence, not authority:
//!
//! ```compile_fail
//! use forge_store_operations::{
//!     BackupExportCapsuleEmission, BackupExportCustodyCounterSnapshot,
//! };
//!
//! let counters: BackupExportCustodyCounterSnapshot = todo!();
//! let _emission: BackupExportCapsuleEmission = counters;
//! ```
//!
//! Repair blast-radius readiness is not offline verifier evidence:
//!
//! ```compile_fail
//! use forge_store_offline_verifier::OfflineRepairBlastRadiusObservation;
//! use forge_store_operations::RepairBlastRadiusReadiness;
//!
//! let observed: OfflineRepairBlastRadiusObservation = todo!();
//! let _readiness: RepairBlastRadiusReadiness = observed;
//! ```
//!
//! Operator identity cannot stand in for repair-read physical readiness:
//!
//! ```compile_fail
//! use forge_store_operations::RepairBlastRadiusReadiness;
//! use forge_store_security::StoreOperatorIdentityClaim;
//!
//! let operator = StoreOperatorIdentityClaim::raw("operator-123");
//! let _readiness: RepairBlastRadiusReadiness = operator;
//! ```
//!
//! Repair readiness cannot stand in for operator authorization:
//!
//! ```compile_fail
//! use forge_store_operations::{RepairBlastRadiusReadiness, RepairOperatorAuthorization};
//!
//! let readiness: RepairBlastRadiusReadiness = todo!();
//! let _authorization: RepairOperatorAuthorization = readiness;
//! ```

mod backup_export_custody_admission;
mod backup_export_custody_counters;
mod backup_export_custody_declaration;
mod backup_export_custody_denial;
mod backup_export_custody_emission;
mod backup_export_custody_handoff;
mod backup_export_custody_readiness;
#[cfg(test)]
mod backup_export_custody_test_support;
#[cfg(test)]
mod backup_export_custody_tests;
mod backup_import_readmission;
mod repair_blast_radius_counters;
mod repair_blast_radius_declaration;
mod repair_blast_radius_denial;
mod repair_blast_radius_handoff;
mod repair_blast_radius_plan;
mod repair_blast_radius_readiness;
#[cfg(test)]
mod repair_blast_radius_test_support;
#[cfg(test)]
mod repair_blast_radius_tests;
mod repair_quarantine_readiness;
#[cfg(test)]
mod repair_quarantine_readiness_tests;

pub use backup_export_custody_admission::BackupExportCustodyAdmission;
pub use backup_export_custody_counters::BackupExportCustodyCounterSnapshot;
pub use backup_export_custody_declaration::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode,
};
pub use backup_export_custody_denial::BackupExportCustodyDenial;
pub use backup_export_custody_emission::{
    BackupExportCapsuleEmission, BackupExportTerminalProjectionPreparation,
};
pub use backup_export_custody_handoff::{
    S10BackupExportCustodyHandoff, S10BackupExportCustodyPermission,
};
pub use backup_export_custody_readiness::BackupExportCustodyReadiness;
pub use backup_import_readmission::BackupImportCustodyReadmission;
pub use repair_blast_radius_counters::RepairBlastRadiusCounterSnapshot;
pub use repair_blast_radius_declaration::{RepairBlastRadiusDeclaration, RepairPhysicalRegion};
pub use repair_blast_radius_denial::RepairBlastRadiusDenial;
pub use repair_blast_radius_handoff::{
    S10RepairBlastRadiusHandoff, S10RepairBlastRadiusPermission,
};
pub use repair_blast_radius_plan::{RepairBlastRadiusPlan, RepairReadPlan};
pub use repair_blast_radius_readiness::RepairBlastRadiusReadiness;
pub use repair_quarantine_readiness::RepairQuarantineScopePreservation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalRecoveryPosture {
    TrustedTruth,
    DegradedDerived,
    Quarantined,
    Unrecoverable,
}
