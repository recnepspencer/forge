#![forbid(unsafe_code)]
//!
//! Raw callers cannot promote declarations into confirmed offline truth:
//!
//! ```compile_fail
//! use worth_store_offline_verifier::{
//!     OfflineFileTruthEvidence, OfflineSecurityEvidencePosture,
//! };
//!
//! let _forged = OfflineFileTruthEvidence::new("authority.manifest")
//!     .with_authenticity(OfflineSecurityEvidencePosture::Confirmed);
//! ```
//!
//! A producer-side materialization receipt cannot skip independent manifest reopen:
//!
//! ```compile_fail
//! use worth_store_offline_verifier::verify_disaster_recovery_bundle;
//! use worth_store_replication::MaterializedDisasterRecoveryBundle;
//!
//! let producer_receipt: MaterializedDisasterRecoveryBundle = todo!();
//! let policy = todo!();
//! let _ = verify_disaster_recovery_bundle(producer_receipt, 4096, policy);
//! ```

mod backup_verification;
mod blob_corruption_observation;
#[cfg(test)]
mod blob_corruption_observation_tests;
mod boundary;
mod custody_capsule_observation;
#[cfg(test)]
mod custody_capsule_observation_tests;
mod disaster_recovery_verification;
mod export_bundle_observation;
mod forensic_acquisition;
mod handoff;
mod inspection;
mod media_acquisition;
mod repair_blast_radius_observation;
#[cfg(test)]
mod repair_blast_radius_observation_tests;
mod replica_target_verification;
mod scan;
mod staged_recovery_verification;
mod truth_composition;

pub use backup_verification::{
    verify_backup_cut_sources, verify_backup_cut_sources_with_cancellation,
    verify_materialized_backup, verify_materialized_backup_with_cancellation,
    BackupArtifactSemanticDefectKind, BackupCutSourceVerificationDenial,
    BackupCutSourceVerificationReport, BackupStructuralVerificationDenial,
    BackupVerificationAllocationPhase, BackupVerificationBudget, BackupVerificationDefect,
    BackupVerificationReadAccounting, BackupVerificationReport, StructurallyVerifiedBackupBundle,
};
pub use blob_corruption_observation::{
    classify_offline_damage_case, OfflineBlobCorruptionClassification,
    OfflineBlobCorruptionEvidenceKind, OfflineBlobCorruptionObservation,
    OfflineBlobCorruptionObservationDenial, OfflineBlobDamageCaseHint,
};
pub use boundary::OfflineVerifierBoundarySeam;
pub use custody_capsule_observation::{
    OfflineCustodyCapsuleObservation, OfflineCustodyCapsuleObservationDenial,
};
pub use disaster_recovery_verification::{
    open_disaster_recovery_bundle, verify_disaster_recovery_bundle,
    BootstrapSourceCutResolutionDenial, DisasterRecoveryClosureDenial,
    DisasterRecoveryIndependentOpenDenial, DisasterRecoveryVerificationCounters,
    DisasterRecoveryVerificationDenial, DisasterRecoveryVerificationPolicy,
    DisasterRecoveryVerificationPolicyDenial, IndependentlyOpenedDisasterRecoveryBundle,
    IndependentlyVerifiedDisasterRecoveryBundle,
};
pub use export_bundle_observation::{
    inspect_offline_export_bundle, OfflineExportBundleObservation,
    OfflineExportBundleObservationDenial, OfflineExportChunkDeclaration,
    OfflineExportDigestEvidence,
};
pub use forensic_acquisition::{
    ForensicAcquisitionCounters, ForensicAcquisitionDenial, ForensicAcquisitionIntent,
    ForensicAcquisitionPlan, ForensicAcquisitionProgress, ForensicAcquisitionSession,
    ForensicBundle, ForensicBundleRange, ForensicCustodyRecord, ForensicEvidencePosture,
    ForensicRangePosture,
};
pub use handoff::{
    map_offline_damage_hint_to_handoff, reject_offline_classification_as_blob_authority,
    reject_offline_observation_as_blob_authority, OfflineBlobAuthorityRejection,
};
pub use inspection::{
    OfflineInspectionBudget, OfflineInspectionCancellation, OfflineInspectionCheckpoint,
    OfflineInspectionCheckpointCodecDenial, OfflineInspectionCounters, OfflineInspectionDenial,
    OfflineInspectionProgress, OfflineInspectionScope, OfflineInspectionSession,
    OfflineMediaAcquisitionBudget, OfflineStoreInspection, OfflineStructuralIdentification,
    OfflineWalkedFile, RestartingOfflineScanDenial, RestartingOfflineScanReceipt,
    StructurallyWalkedMedia,
};
pub use media_acquisition::{
    OfflineMediaAcquisitionDenial, OfflineMediaAcquisitionDimension, UntrustedOfflineMediaSet,
};
pub use repair_blast_radius_observation::{
    OfflineRepairBlastRadiusObservation, OfflineRepairBlastRadiusObservationDenial,
    OfflineRepairEvidenceKind,
};
pub use replica_target_verification::{
    verify_replica_bootstrap_target, verify_replica_promotion_target,
    IndependentlyVerifiedReplicaTarget, ReplicaTargetVerificationBudget,
    ReplicaTargetVerificationDenial,
};
pub use scan::{
    offline_repair_scan_background_pressure_shape,
    offline_verification_pressure_background_pressure_shape,
};
pub use staged_recovery_verification::{
    post_verify_closed_staged_recovery, ClosedStagedRecoveryVerificationRequest,
    PostVerifiedStagedRecovery, StagedRecoveryAuthorityPosture, StagedRecoveryExpectedFrontier,
    StagedRecoveryOwnerVerificationSet, StagedRecoveryPostVerificationDenial,
    StagedRecoveryRegionPosture,
};
pub use truth_composition::{
    compose_operational_truth, CanonicalPhysicalCoverageProof, EvidenceBoundTruthRegion,
    OfflineAuthorityClass, OfflineFileTruthEvidence, OfflineRecoveryAvailability,
    OfflineSecurityEvidencePosture, OfflineTruthEvidenceAdmissionDenial,
    OfflineTruthEvidenceReferences, OfflineTruthEvidenceSet, OperationalTruthCompositionBudget,
    OperationalTruthCompositionDenial, OperationalTruthRegion, OperationalTruthReport,
};
