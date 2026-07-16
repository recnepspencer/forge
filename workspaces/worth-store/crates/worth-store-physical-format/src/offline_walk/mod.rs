mod bounded_decode;
mod structural_observation;

pub use bounded_decode::{
    verify_bounded_extent_artifact, verify_bounded_extent_artifact_from_reader,
    verify_bounded_page_artifact, verify_bounded_page_artifact_from_reader,
    verify_bounded_root_manifest_artifact, verify_bounded_root_manifest_artifact_from_reader,
    BoundedPhysicalArtifactDenial, BoundedPhysicalArtifactObservation,
    VerifiedRootManifestArtifact,
};
pub use structural_observation::{
    classify_offline_artifact_family, observe_bounded_physical_bytes,
    OfflinePhysicalArtifactFamily, OfflineStructuralObservation,
};
