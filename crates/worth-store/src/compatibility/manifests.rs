mod identity;
mod publication;
mod recovery;
mod versions;

pub use identity::{
    ArtifactFamilyId, AuthoritativeCompatibilityManifest, CompatibilityManifestDigest,
    DerivedCompatibilityManifest,
};
pub use publication::{
    CompatibilityManifestFrontier, CompatibilityManifestPublicationLedger,
    CompatibilityManifestPublicationReceipt, CompatibilityManifestPublicationRecord,
    CompatibilityManifestPublicationUnit,
};
pub use recovery::{
    CompatibilityManifestRecoveryPlan, CompatibilityRecoveredManifestIndex, ManifestDigestMismatch,
    ManifestPublicationGap, ManifestPublicationWitness, ManifestRecoverySummary,
};
pub use versions::{ArtifactCompatibilityWindow, ArtifactFormatVersion, ArtifactSemanticVersion};

#[cfg(test)]
mod tests {
    use super::{ArtifactCompatibilityWindow, ArtifactFamilyId, CompatibilityManifestDigest};

    #[test]
    fn manifest_digest_identity_is_deterministic() {
        let family = ArtifactFamilyId::new("canonical_commit_envelope");
        let window = ArtifactCompatibilityWindow::native(1);
        let left = CompatibilityManifestDigest::compute(&family, &window, "authoritative");
        let right = CompatibilityManifestDigest::compute(&family, &window, "authoritative");
        assert_eq!(left, right);
    }
}
