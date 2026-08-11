use std::path::{Path, PathBuf};

use worth_store_physical_format::RecordArtifactFile;
use worth_store_recovery_physics::WalSegmentArtifactIdentity;
use worth_store_recovery_runtime::{
    RecoveredPhysicalRuntimeHandoff, RecoveryCleanupDeferralReason, RecoveryCleanupDisposition,
    RecoveryCleanupDispositionKind, RecoveryCleanupEvidence, RecoveryCleanupPosture,
    RecoveryCleanupTarget,
};

use super::{required_text, EXPECTED_POSTURE};

pub(super) fn assert_selected_wal_is_retained(
    root: &Path,
    handoff: &RecoveredPhysicalRuntimeHandoff,
    evidence: &RecoveryCleanupEvidence,
) {
    for segment in handoff.selected_sources().wal_tail().segments() {
        let identity = segment.identity();
        assert!(evidence.dispositions().iter().any(|disposition| {
            disposition.target() == &RecoveryCleanupTarget::Wal(identity)
                && disposition.kind() == RecoveryCleanupDispositionKind::Retained
        }));
        assert!(root
            .join("families/wal")
            .join(identity.file_name())
            .exists());
    }
}

pub(super) fn assert_selected_base_is_retained(
    root: &Path,
    handoff: &RecoveredPhysicalRuntimeHandoff,
    evidence: &RecoveryCleanupEvidence,
) {
    for artifact in handoff.base_image().source_artifacts() {
        let target = RecoveryCleanupTarget::Record(*artifact);
        let disposition = evidence
            .dispositions()
            .iter()
            .find(|disposition| disposition.target() == &target)
            .expect("every selected base artifact has one cleanup disposition");
        assert!(matches!(
            disposition.kind(),
            RecoveryCleanupDispositionKind::Current | RecoveryCleanupDispositionKind::Retained
        ));
        let path = record_artifact_path(root, *artifact);
        assert!(
            path.exists(),
            "retained source artifact {artifact:?} must exist at {}",
            path.display()
        );
    }
    let current = RecoveryCleanupTarget::Record(RecordArtifactFile::CurrentRootSelector);
    assert!(evidence.dispositions().iter().any(|disposition| {
        disposition.target() == &current
            && disposition.kind() == RecoveryCleanupDispositionKind::Current
    }));
}

pub(super) fn assert_expected_posture(
    posture: &RecoveryCleanupPosture,
    disposition: &RecoveryCleanupDisposition,
    expected_identity: WalSegmentArtifactIdentity,
    expected_deferred: u64,
) {
    let evidence = posture.evidence();
    match required_text(EXPECTED_POSTURE).as_str() {
        "complete" => assert_complete(posture, disposition, expected_identity),
        "byte-limit" => assert_deferred(
            posture,
            disposition,
            RecoveryCleanupDeferralReason::ByteLimit,
        ),
        "unresolved" => assert_deferred(
            posture,
            disposition,
            RecoveryCleanupDeferralReason::UnresolvedOperationFate,
        ),
        "candidate-limit" => {
            assert!(matches!(posture, RecoveryCleanupPosture::Deferred(_)));
            assert_eq!(
                disposition.kind(),
                RecoveryCleanupDispositionKind::SafelyRemoved
            );
            assert_eq!(evidence.counters().actions_planned, 1);
            assert_eq!(evidence.counters().actions_completed, 1);
            assert_eq!(evidence.counters().actions_deferred, expected_deferred);
            assert!(evidence
                .dispositions()
                .iter()
                .any(|disposition| disposition.kind()
                    == RecoveryCleanupDispositionKind::Deferred(
                        RecoveryCleanupDeferralReason::CandidateLimit,
                    )));
        }
        expected => panic!("unsupported expected cleanup posture: {expected}"),
    }
}

fn assert_complete(
    posture: &RecoveryCleanupPosture,
    disposition: &RecoveryCleanupDisposition,
    expected_identity: WalSegmentArtifactIdentity,
) {
    let evidence = posture.evidence();
    assert!(matches!(posture, RecoveryCleanupPosture::Complete(_)));
    assert_eq!(
        disposition.kind(),
        RecoveryCleanupDispositionKind::SafelyRemoved
    );
    assert_eq!(evidence.performed_removals().len(), 1);
    let occurrence = evidence.performed_removals()[0].occurrence();
    assert_eq!(occurrence.plan(), evidence.plan_identity());
    assert_eq!(
        occurrence.artifact().segment(),
        expected_identity.segment().get()
    );
    assert_eq!(
        occurrence.artifact().generation(),
        expected_identity.generation().get()
    );
}

fn assert_deferred(
    posture: &RecoveryCleanupPosture,
    disposition: &RecoveryCleanupDisposition,
    reason: RecoveryCleanupDeferralReason,
) {
    assert!(matches!(posture, RecoveryCleanupPosture::Deferred(_)));
    assert_eq!(
        disposition.kind(),
        RecoveryCleanupDispositionKind::Deferred(reason)
    );
    assert!(posture.evidence().performed_removals().is_empty());
    assert_eq!(posture.evidence().counters().freshness_evaluations, 0);
}

fn record_artifact_path(root: &Path, artifact: RecordArtifactFile) -> PathBuf {
    let records = root.join("families/records");
    let directory = match artifact {
        RecordArtifactFile::BootstrapCatalog
        | RecordArtifactFile::CurrentRootSelector
        | RecordArtifactFile::PreviousRootSelector => records,
        RecordArtifactFile::RootSelectorCandidate { .. }
        | RecordArtifactFile::CatalogCandidate { .. } => root.join("staging/records"),
        RecordArtifactFile::RootManifest { .. } | RecordArtifactFile::RootRoutingBlock { .. } => {
            records.join("roots")
        }
        RecordArtifactFile::Segment { .. } => records.join("segments"),
        RecordArtifactFile::SegmentManifest { .. }
        | RecordArtifactFile::SegmentMembershipBlock { .. } => records.join("segment-manifests"),
        RecordArtifactFile::Extent { .. } => records.join("extents"),
        RecordArtifactFile::ExtentManifest { .. } => records.join("extent-manifests"),
        RecordArtifactFile::FreeSpaceManifest { .. }
        | RecordArtifactFile::FreeSpaceMembershipBlock { .. } => records.join("free-space"),
    };
    directory.join(artifact.file_name())
}
