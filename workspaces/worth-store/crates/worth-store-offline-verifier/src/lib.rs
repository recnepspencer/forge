#![forbid(unsafe_code)]

mod blob_corruption_observation;
#[cfg(test)]
mod blob_corruption_observation_tests;
mod boundary;
mod custody_capsule_observation;
#[cfg(test)]
mod custody_capsule_observation_tests;
mod export_bundle_observation;
mod handoff;
mod repair_blast_radius_observation;
#[cfg(test)]
mod repair_blast_radius_observation_tests;
mod scan;

pub use blob_corruption_observation::{
    classify_offline_damage_case, OfflineBlobCorruptionClassification,
    OfflineBlobCorruptionEvidenceKind, OfflineBlobCorruptionObservation,
    OfflineBlobCorruptionObservationDenial, OfflineBlobDamageCaseHint,
};
pub use boundary::OfflineVerifierBoundarySeam;
pub use custody_capsule_observation::{
    OfflineCustodyCapsuleObservation, OfflineCustodyCapsuleObservationDenial,
};
pub use export_bundle_observation::{
    inspect_offline_export_bundle, OfflineExportBundleObservation,
    OfflineExportBundleObservationDenial, OfflineExportChunkDeclaration,
    OfflineExportDigestEvidence,
};
pub use handoff::{
    map_offline_damage_hint_to_handoff, reject_offline_classification_as_blob_authority,
    reject_offline_observation_as_blob_authority, OfflineBlobAuthorityRejection,
};
pub use repair_blast_radius_observation::{
    OfflineRepairBlastRadiusObservation, OfflineRepairBlastRadiusObservationDenial,
    OfflineRepairEvidenceKind,
};
pub use scan::{
    offline_repair_scan_background_pressure_shape,
    offline_verification_pressure_background_pressure_shape,
};
