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
//! let _custody = BackupExportCustodyReadiness::from_admitted_readiness(readiness, None, todo!());
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
mod backup_export_custody_scheduler_demand;
#[cfg(any(test, feature = "certification-test-authority"))]
#[cfg_attr(feature = "certification-test-authority", allow(dead_code))]
mod backup_export_custody_test_support;
#[cfg(test)]
mod backup_export_custody_tests;
mod backup_import_readmission;
mod backup_import_source_custody;
mod capsule_chunk_availability;
mod import_placement_plan;
pub mod layout_access;
mod recovery_posture;
mod repair_blast_radius_counters;
mod repair_blast_radius_declaration;
mod repair_blast_radius_denial;
mod repair_blast_radius_handoff;
mod repair_blast_radius_plan;
mod repair_blast_radius_readiness;
mod repair_blast_radius_scheduler_demand;
#[cfg(test)]
mod repair_blast_radius_test_support;
#[cfg(test)]
mod repair_blast_radius_tests;
mod repair_quarantine_readiness;
#[cfg(test)]
mod repair_quarantine_readiness_tests;
mod replication_prep_scheduler_demand;
mod s10_later_io_readiness;
mod s8_runtime_receipt;

pub use backup_export_custody_admission::BackupExportCustodyAdmission;
pub use backup_export_custody_counters::BackupExportCustodyCounterSnapshot;
pub use backup_export_custody_declaration::BackupExportCustodyDeclaration;
pub use backup_export_custody_denial::BackupExportCustodyDenial;
pub use backup_export_custody_emission::{
    BackupExportCapsuleEmission, BackupExportTerminalProjectionPreparation,
};
pub use backup_export_custody_handoff::{
    S10BackupExportCustodyHandoff, S10BackupExportCustodyPermission,
};
pub use backup_export_custody_readiness::BackupExportCustodyReadiness;
pub use backup_export_custody_scheduler_demand::backup_prep_background_pressure_shape;
pub use backup_import_readmission::BackupImportCustodyReadmission;
pub use backup_import_source_custody::{
    admit_backup_import_source_custody_scope, BackupImportSourceCustodyDenial,
    BackupImportSourceCustodyScope,
};
pub use capsule_chunk_availability::{
    classify_capsule_chunk_availability, CapsuleChunkAvailabilityPosture,
};
pub use forge_store_operations_vocabulary::BackupExportCustodyMode;
pub use import_placement_plan::{
    ImportPlacementDisposition, ImportPlacementPlan, ImportPlacementSource,
};
pub use layout_access::backup_family::BackupLayoutEvidenceReport;
pub use layout_access::capsule_operation_family::CapsuleOperationLayoutReport;
pub use layout_access::export_family::ExportLayoutEvidenceReport;
pub use layout_access::import_family::ImportLayoutEvidenceReport;
pub use layout_access::operations_layout_closeout::OperationsLayoutCloseout;
pub use layout_access::restore_family::RestoreLayoutEvidenceReport;
pub use recovery_posture::OperationalRecoveryPosture;
pub use repair_blast_radius_counters::RepairBlastRadiusCounterSnapshot;
pub use repair_blast_radius_declaration::{RepairBlastRadiusDeclaration, RepairPhysicalRegion};
pub use repair_blast_radius_denial::RepairBlastRadiusDenial;
pub use repair_blast_radius_handoff::{
    S10RepairBlastRadiusHandoff, S10RepairBlastRadiusPermission,
};
pub use repair_blast_radius_plan::{RepairBlastRadiusPlan, RepairReadPlan};
pub use repair_blast_radius_readiness::RepairBlastRadiusReadiness;
pub use repair_blast_radius_scheduler_demand::repair_background_pressure_shape;
pub use repair_quarantine_readiness::RepairQuarantineScopePreservation;
pub use replication_prep_scheduler_demand::replication_prep_background_pressure_shape;
pub use s10_later_io_readiness::{
    admit_s10_backup_export_io_readiness_seed, admit_s10_compaction_io_readiness_seed,
    admit_s10_repair_scan_io_readiness_seed, S10BackupExportIoReadinessSeed,
    S10CompactionIoReadinessSeed, S10RepairScanIoReadinessSeed,
};
#[cfg(feature = "certification-test-authority")]
pub use s8_runtime_receipt::s8_security_custody_export_runtime_receipt_for_certification_test;
pub use s8_runtime_receipt::S8SecurityCustodyExportRuntimeReceipt;
