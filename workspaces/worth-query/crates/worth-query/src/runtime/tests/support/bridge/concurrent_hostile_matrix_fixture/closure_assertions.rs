use crate::application::{
    WorthQueryConcurrentHostileMatrixArtifact, WorthQueryConcurrentHostileMatrixPosture,
    WorthQueryConcurrentHostileMatrixSabotage, WorthQueryConcurrentHostileMatrixSabotageKind,
};

pub(in crate::runtime::tests) fn phase_sixteen_sabotage_proofs(
    artifact: &WorthQueryConcurrentHostileMatrixArtifact,
) -> Vec<WorthQueryConcurrentHostileMatrixSabotage> {
    [
        WorthQueryConcurrentHostileMatrixSabotageKind::CommittedReadHotPathLock,
        WorthQueryConcurrentHostileMatrixSabotageKind::SharedReadMintRowClone,
        WorthQueryConcurrentHostileMatrixSabotageKind::ReaderDerivedEvaluation,
        WorthQueryConcurrentHostileMatrixSabotageKind::OrphanedSnapshotGeneration,
        WorthQueryConcurrentHostileMatrixSabotageKind::UnretiredReadPin,
        WorthQueryConcurrentHostileMatrixSabotageKind::JournalGap,
        WorthQueryConcurrentHostileMatrixSabotageKind::ReplayResidue,
        WorthQueryConcurrentHostileMatrixSabotageKind::DeliveryResidue,
    ]
    .into_iter()
    .map(|kind| WorthQueryConcurrentHostileMatrixSabotage::perturb(kind, artifact))
    .collect()
}

pub(in crate::runtime::tests) fn assert_phase_sixteen_closed(
    artifact: &WorthQueryConcurrentHostileMatrixArtifact,
) {
    assert_eq!(
        artifact.posture(),
        WorthQueryConcurrentHostileMatrixPosture::Closed
    );
    assert!(artifact.topology().satisfies_phase_sixteen_minimums());
    assert_eq!(artifact.counters().exact_zero_residue_count(), 0);
    assert!(
        artifact
            .counters()
            .published_artifact_registry_lease_count()
            > 0
    );
}
