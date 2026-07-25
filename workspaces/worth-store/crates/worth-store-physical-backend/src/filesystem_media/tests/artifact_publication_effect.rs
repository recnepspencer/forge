use super::super::*;
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::StorePhysicalBoundaryWitness;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeIdentity, StoreTenantScope,
};

fn qualified(root: &std::path::Path, schedule: MediaFaultSchedule) -> QualifiedFilesystemMedia {
    let request = FilesystemQualificationRequest::certification(
        root,
        FilesystemAccessPosture::CoordinatedServiceAccount,
    )
    .with_fault_schedule(schedule);
    match FilesystemMediaOwner::qualify(request).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("qualification failed"),
    }
}

fn artifacts(
    media: &QualifiedFilesystemMedia,
) -> (ArtifactTreeDirectory, ArtifactTreeFile, ArtifactTreeFile) {
    let records = ArtifactTreeDirectory::families().child("records").unwrap();
    let tree = media.artifact_tree();
    tree.create_directory(&records).unwrap();
    let current = records.file("bootstrap.catalog").unwrap();
    let candidate = records.file("bootstrap.candidate").unwrap();
    tree.write_new(&current, b"old").unwrap();
    tree.write_new(&candidate, b"new").unwrap();
    (records, current, candidate)
}

fn security_scope() -> StoreSecurityScopeIdentity {
    let authority = StorePhysicalAuthorityWitness::for_aspect_native_boundary(
        ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    )
    .unwrap();
    StoreSecurityScopeIdentity::from_physical_security_scope(
        StorePhysicalBoundaryWitness::from_physical_authority(authority).unwrap(),
        StoreKeyScope::StoreManagedRoot,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn binding(
    media: &QualifiedFilesystemMedia,
    profile: crate::BackendTargetProfile,
) -> crate::BackendQueueExecutionPlanBinding {
    let scope = security_scope();
    let replay = crate::BackendQueueExecutionReplayBinding::from_store_queue_replay(
        1,
        1,
        1,
        scope,
        scope.tenant_scope(),
        scope.key_scope(),
        scope.authenticity_requirement(),
        1,
        0,
        0,
        crate::BackendQueueExecutionBudgetBinding::new(1, 1, 0, 0, 1, 1, 0, 1, 1, 0),
    );
    crate::BackendQueueExecutionPlanBinding::from_store_replay_binding(
        replay,
        None,
        profile,
        media.execution_capability().evidence_class(),
        0,
    )
}

#[test]
fn invalid_queue_binding_denies_before_file_synchronization() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(&parent.path().join("denied"), MediaFaultSchedule::default());
    let (_records, current, _candidate) = artifacts(&media);
    let before = media
        .counters()
        .attempts_for(MediaOperationRole::SynchronizeFileState);

    assert!(matches!(
        media.artifact_tree().synchronize_scheduled_file(
            &current,
            binding(&media, crate::BackendTargetProfile::SimulatedStrictDurable),
            crate::BackendQueueExecutionAdaptation::None,
        ),
        ScheduledArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(_)
    ));
    assert_eq!(
        media
            .counters()
            .attempts_for(MediaOperationRole::SynchronizeFileState),
        before,
        "backend effect ran before queue admission"
    );
    media.close();
}

#[test]
fn synchronization_and_replacement_return_exact_effect_receipts() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(
        &parent.path().join("completed"),
        MediaFaultSchedule::default(),
    );
    let (records, current, candidate) = artifacts(&media);
    let tree = media.artifact_tree();

    let file_sync = match tree.synchronize_file_effect(&candidate) {
        ArtifactTreePublicationEffectOutcome::Completed(completed) => completed,
        outcome => panic!("file synchronization failed: {outcome:?}"),
    };
    assert!(matches!(
        file_sync.effect(),
        ArtifactTreePublicationEffect::FileSynchronization(artifact)
            if artifact == &candidate
    ));
    let replacement = match tree.replace_effect(&candidate, &current) {
        ArtifactTreePublicationEffectOutcome::Completed(completed) => completed,
        outcome => panic!("replacement failed: {outcome:?}"),
    };
    assert!(matches!(
        replacement.effect(),
        ArtifactTreePublicationEffect::Replacement { source, destination }
            if source == &candidate && destination == &current
    ));
    let directory_sync = match tree.synchronize_directory_effect(&records) {
        ArtifactTreePublicationEffectOutcome::Completed(completed) => completed,
        outcome => panic!("directory synchronization failed: {outcome:?}"),
    };
    assert!(matches!(
        directory_sync.effect(),
        ArtifactTreePublicationEffect::DirectorySynchronization(directory)
            if directory == &records
    ));
    assert_ne!(file_sync.operation(), replacement.operation());
    assert_ne!(replacement.operation(), directory_sync.operation());
    media.close();
}

#[test]
fn post_effect_synchronization_fault_retains_indeterminate_identity() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline = qualified(
        &baseline_parent.path().join("baseline"),
        MediaFaultSchedule::default(),
    );
    let (_records, _current, candidate) = artifacts(&baseline);
    let ordinal = baseline
        .counters()
        .attempts_for(MediaOperationRole::SynchronizeFileState)
        + 1;
    assert!(!candidate.file_name.is_empty());
    baseline.close();

    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::SynchronizeFileState,
        ordinal,
        MediaFaultDirective::IndeterminateAfterEffect,
    )])
    .unwrap();
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(&parent.path().join("faulted"), schedule);
    let (_records, _current, candidate) = artifacts(&media);
    let failure = match media.artifact_tree().synchronize_file_effect(&candidate) {
        ArtifactTreePublicationEffectOutcome::Indeterminate(failure) => failure,
        outcome => panic!("post-effect fault was not indeterminate: {outcome:?}"),
    };
    assert_eq!(
        failure.failure().kind(),
        ArtifactTreeFailureKind::IndeterminateEffect
    );
    assert!(matches!(
        failure.effect(),
        ArtifactTreePublicationEffect::FileSynchronization(artifact)
            if artifact == &candidate
    ));
    media.close();
}
