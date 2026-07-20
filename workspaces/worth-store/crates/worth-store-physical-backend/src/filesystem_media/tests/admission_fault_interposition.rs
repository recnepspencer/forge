use super::super::*;

#[test]
fn admission_effects_are_faulted_before_and_after_the_real_boundary() {
    let parent = tempfile::tempdir().unwrap();
    let before_root = parent.path().join("before");
    let before = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::CreateDirectory,
        1,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::PermissionDenied,
            raw_os_error: None,
        },
    )])
    .unwrap();
    assert!(FilesystemMediaOwner::admit_with_schedule(&before_root, before).is_err());
    assert!(!before_root.exists());

    let after_root = parent.path().join("after");
    let after = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::CreateDirectory,
        1,
        MediaFaultDirective::IndeterminateAfterEffect,
    )])
    .unwrap();
    assert!(FilesystemMediaOwner::admit_with_schedule(&after_root, after).is_err());
    assert!(after_root.is_dir());
}

#[test]
fn contention_returns_prior_authority_only_when_this_attempt_changed_nothing() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("effectful-contention");
    let gate = MediaPauseGate::for_certification();
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::CreateDirectory,
        1,
        MediaFaultDirective::PauseAfter(gate.clone()),
    )])
    .unwrap();
    let contender_root = root.clone();
    let contender = std::thread::spawn(move || {
        FilesystemMediaOwner::qualify(
            FilesystemQualificationRequest::production(
                contender_root,
                FilesystemAccessPosture::CoordinatedServiceAccount,
            )
            .with_fault_schedule(schedule),
        )
        .into_raw()
    });
    gate.wait_until_reached();
    let winner = match FilesystemMediaOwner::qualify(FilesystemQualificationRequest::production(
        &root,
        FilesystemAccessPosture::CoordinatedServiceAccount,
    ))
    .into_raw()
    {
        worth_proof::TransitionOutcome::Success(media) => media,
        _ => panic!("unpaused contender must acquire the real lease"),
    };
    gate.release();
    let failure = contender.join().unwrap();
    let worth_proof::TransitionOutcome::Failed(MediaQualificationFailure::OwnerAfterEffect {
        denial,
        counters,
    }) = failure
    else {
        panic!("effectful contention must consume admission authority");
    };
    assert_eq!(
        denial,
        FilesystemMediaOwnerAdmissionDenial::Ownership(MutationOwnershipDenial::Contended)
    );
    assert_eq!(
        counters.completed_operations_for(MediaOperationRole::CreateDirectory),
        1
    );
    assert!(counters.is_conserved());
    winner.close();
}

#[test]
fn lease_observation_barrier_failure_releases_the_real_os_lock() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PublishMutationLeaseObservation,
        1,
        MediaFaultDirective::FailBarrier {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    )])
    .unwrap();
    assert!(FilesystemMediaOwner::admit_with_schedule(&root, schedule).is_err());
    assert!(!std::fs::read(root.join("namespace/mutation.lock"))
        .unwrap()
        .is_empty());

    let successor =
        FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
            .expect("failed lease publication must not retain OS authority");
    successor.close();
}
