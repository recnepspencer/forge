use super::*;

pub(super) fn assert_restore_checkpoint_authority(manifest: &FinancialLocalityExpectationManifest) {
    let checkpoints = manifest.action_checkpoints();
    assert_eq!(checkpoints.len(), 6);
    let captured = &checkpoints[1];
    assert!(matches!(
        captured.kind,
        ExpectedActionCheckpointKind::CheckpointCaptured
    ));
    assert!(!captured.canonical_causes.is_empty());
    assert_eq!(captured.persisted_causes, captured.canonical_causes);
    assert!(!captured.current_source_bases.is_empty());
    assert_eq!(
        source_fingerprints(&captured.current_source_bases),
        source_fingerprints(&captured.persisted_source_bases)
    );
    assert!(captured
        .current_source_bases
        .iter()
        .all(|basis| basis.runtime_epoch == 1));
    assert!(captured
        .canonical_work
        .keys()
        .all(|work| work.readiness_epoch == 1));
    assert_destroyed_runtime_retains_checkpoint_authority(&checkpoints[2], captured);
    assert_readmitted_authority(&checkpoints[3], captured);
    assert!(matches!(
        checkpoints[4].kind,
        ExpectedActionCheckpointKind::ReadyWorkReconstructed
    ));
    assert_eq!(checkpoints[4].canonical_causes, captured.canonical_causes);
    assert!(checkpoints[4]
        .canonical_work
        .keys()
        .all(|work| work.readiness_epoch == 2));
    assert_eq!(checkpoints[5].canonical_work, checkpoints[4].canonical_work);
    assert_eq!(
        manifest.source_bases(),
        &checkpoints[4].current_source_bases
    );
    assert_reconstructed_work_has_current_authority(&checkpoints[4]);
}

fn assert_destroyed_runtime_retains_checkpoint_authority(
    destroyed: &ExpectedActionCheckpoint,
    captured: &ExpectedActionCheckpoint,
) {
    assert!(matches!(
        destroyed.kind,
        ExpectedActionCheckpointKind::DerivedStateDestroyed
    ));
    assert!(destroyed.canonical_causes.is_empty());
    assert_eq!(destroyed.persisted_causes, captured.persisted_causes);
    assert!(destroyed.canonical_work.is_empty());
    assert!(destroyed.current_source_bases.is_empty());
    assert_eq!(
        source_fingerprints(&destroyed.persisted_source_bases),
        source_fingerprints(&captured.persisted_source_bases)
    );
}

fn assert_readmitted_authority(
    readmitted: &ExpectedActionCheckpoint,
    captured: &ExpectedActionCheckpoint,
) {
    assert!(matches!(
        readmitted.kind,
        ExpectedActionCheckpointKind::CausesReadmitted
    ));
    assert_eq!(readmitted.canonical_causes, captured.canonical_causes);
    assert_eq!(readmitted.persisted_causes, captured.persisted_causes);
    assert!(readmitted.canonical_work.is_empty());
    assert_eq!(
        source_fingerprints(&readmitted.current_source_bases),
        source_fingerprints(&captured.persisted_source_bases)
    );
    assert!(readmitted
        .current_source_bases
        .iter()
        .all(|basis| basis.runtime_epoch == 2));
}

fn assert_reconstructed_work_has_current_authority(checkpoint: &ExpectedActionCheckpoint) {
    for (work, origins) in &checkpoint.canonical_work {
        for origin in origins {
            match origin {
                ExpectedSealedOriginBinding::SourceRecompute {
                    admission_generation,
                } => assert!(checkpoint.current_source_bases.iter().any(|basis| {
                    basis.source == work.target
                        && basis.dependency_revision == work.dependency_revision
                        && basis.admission_generation == *admission_generation
                })),
                ExpectedSealedOriginBinding::DependencyCommit {
                    producer_commit_ordinals,
                    ..
                } => assert!(producer_commit_ordinals.iter().all(|ordinal| {
                    checkpoint.canonical_causes.iter().any(|cause| {
                        cause.consumer == work.target
                            && cause.dependency_revision == work.dependency_revision
                            && cause.output_commit_ordinal == *ordinal
                    })
                })),
                ExpectedSealedOriginBinding::StructuralRecompute { .. } => {
                    panic!("restore reconstruction requires current source or cause authority")
                }
            }
        }
    }
}

fn source_fingerprints(
    bases: &BTreeSet<ExpectedDirectSourceBasis>,
) -> BTreeSet<(
    ExpectedGraphBinding,
    LocalitySemanticOutputId,
    FinancialAspect,
    Option<LocalityScope>,
    u64,
    u64,
)> {
    bases
        .iter()
        .map(|basis| {
            (
                basis.graph,
                basis.source,
                basis.aspect,
                basis.scope,
                basis.admission_generation,
                basis.dependency_revision,
            )
        })
        .collect()
}
