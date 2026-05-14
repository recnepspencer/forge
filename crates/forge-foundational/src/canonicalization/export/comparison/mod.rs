mod basis_entries;
mod manifest;
mod outcome;

pub use outcome::{
    CanonicalExportComparisonOutcome, CanonicalExportManifestMismatch,
    CanonicalExportManifestMismatchKind,
};

use self::basis_entries::first_bundle_mismatch;
use self::manifest::first_manifest_mismatch;
use super::readmission::CanonicalExportReadyArtifact;

pub fn compare_canonical_exports(
    left: &CanonicalExportReadyArtifact,
    right: &CanonicalExportReadyArtifact,
) -> CanonicalExportComparisonOutcome {
    if let Some(manifest_mismatch) = first_manifest_mismatch(left.payload(), right.payload()) {
        return CanonicalExportComparisonOutcome::ManifestMismatch(manifest_mismatch);
    }

    match first_bundle_mismatch(left.payload().bundle(), right.payload().bundle()) {
        Some(mismatch) => CanonicalExportComparisonOutcome::Mismatched(mismatch),
        None => CanonicalExportComparisonOutcome::Equivalent,
    }
}
