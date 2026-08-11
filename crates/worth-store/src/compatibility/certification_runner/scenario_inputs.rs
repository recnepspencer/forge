use super::super::admission::{
    CompatibilityAdapterCostClass, CompatibilityAdapterDigest, CompatibilityAdapterId,
    CompatibilityRejectionKind, CompatibilityRelation, DeclaredCompatibilityAdapter,
};

use super::super::catalog::CompatibilityRegistrySnapshot;
use super::super::decoding::QuarantinedDecodedArtifact;
use worth_store_contracts::CompatibilityFamilyKind;

use super::super::certification::Milestone12CertificationLaneInput;

use super::super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityManifestDigest, CompatibilityManifestPublicationLedger,
};

use super::super::restore::BackupCompatibilityManifest;

use super::super::rolling::RollingUpgradeWindow;

pub(super) fn recovered_manifest_index(
    snapshot: &CompatibilityRegistrySnapshot,
) -> super::super::admission::CompatibilityManifestIndex {
    let mut ledger = CompatibilityManifestPublicationLedger::new();
    for declaration in snapshot.declarations() {
        ledger.publish_declaration(declaration);
    }
    super::super::admission::CompatibilityManifestIndex::rebuild_from_recovered_manifests(
        snapshot,
        &ledger.recover(),
    )
}

pub(super) fn artifact_for_family(
    family_kind: CompatibilityFamilyKind,
    version: u32,
) -> QuarantinedDecodedArtifact {
    let family_id = family_kind.family_id();
    let window = ArtifactCompatibilityWindow::native(version);
    let digest = CompatibilityManifestDigest::compute(
        &family_id,
        &window,
        family_kind.authority_classification().label(),
    );
    QuarantinedDecodedArtifact::new(
        family_id,
        ArtifactFormatVersion::new(version),
        ArtifactSemanticVersion::new(version),
        digest,
        format!("structural-digest-{}", family_kind.label()),
        format!("diagnostic-context-{}", family_kind.label()),
    )
}

pub(super) fn backup_manifest(
    family_id: ArtifactFamilyId,
    version: u32,
) -> BackupCompatibilityManifest {
    let window = ArtifactCompatibilityWindow::native(version);
    let digest = CompatibilityManifestDigest::compute(&family_id, &window, "backup");
    BackupCompatibilityManifest::new(family_id, window, digest)
}

pub(super) fn rolling_window(family_id: ArtifactFamilyId) -> RollingUpgradeWindow {
    RollingUpgradeWindow::new(
        family_id,
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    )
}

pub(super) fn adapter(cost_class: CompatibilityAdapterCostClass) -> DeclaredCompatibilityAdapter {
    DeclaredCompatibilityAdapter::new(
        CompatibilityAdapterId::new("first-ship-certification-adapter"),
        CompatibilityAdapterDigest::new("first-ship-certification-adapter-digest"),
        cost_class,
    )
}

pub(super) fn lane_input(
    family_id: ArtifactFamilyId,
    source_semantic_version: u32,
    target_semantic_version: u32,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection_kind: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneInput {
    Milestone12CertificationLaneInput::new(
        family_id,
        ArtifactSemanticVersion::new(source_semantic_version),
        ArtifactSemanticVersion::new(target_semantic_version),
        expected_relation,
        expected_rejection_kind,
    )
}
