use worth_store::physical_runtime::{
    PhysicalExecutorCommand, PhysicalSignalSettlementOutcome, PhysicalWorkEffectFate,
};
use worth_store_physical_backend::{MediaFaultDirective, MediaOperationRole};

use super::{
    executor::admitted_write,
    fault_fixture::serving_from_open_with_positioned_write_fault,
    fixture::{
        disjoint_mutation_fixture, serving_from_initialization_with_work_profile, work_fixture,
    },
};

#[test]
fn signal_evaluation_is_filesystem_effect_free() {
    let probe = std::env::temp_dir().join(format!(
        "worth-c5-signal-evaluation-effect-{}",
        std::process::id()
    ));
    if probe.exists() {
        std::fs::remove_file(&probe).unwrap();
    }
    let root = tempfile::tempdir().unwrap();
    let (profile, _, request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let before = serving.media_counters();

    let admitted = admitted_write(&serving, request);

    assert!(
        !probe.exists() && serving.media_counters() == before,
        "C5_PREDICATE:signal-evaluation-effect: Signal evaluation produced a filesystem effect"
    );
    drop(admitted);
    serving.close();
}

#[test]
fn generic_signal_completion_cannot_upgrade_proven_no_effect() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, request) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let serving = serving_from_open_with_positioned_write_fault(
        root.path(),
        profile,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::PermissionDenied,
            raw_os_error: None,
        },
    );
    let admitted = admitted_write(&serving, request);
    let command = PhysicalExecutorCommand::exact_write(admitted, b"noeffect".as_slice()).unwrap();
    let before = serving.media_counters();

    let outcome = serving.execute_physical_work(command).unwrap();

    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::ProvenNoEffect
    );
    assert_eq!(
        outcome.signal(),
        PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth,
        "C5_PREDICATE:generic-signal-settlement: generic completion upgraded physical no-effect truth"
    );
    let after = serving.media_counters();
    assert_eq!(
        after.completed_operations_for(MediaOperationRole::PositionedWrite),
        before.completed_operations_for(MediaOperationRole::PositionedWrite)
    );
    assert_eq!(
        after.completed_bytes_for(MediaOperationRole::PositionedWrite),
        before.completed_bytes_for(MediaOperationRole::PositionedWrite)
    );
    assert_eq!(
        after.denied_before_effect_for(MediaOperationRole::PositionedWrite)
            - before.denied_before_effect_for(MediaOperationRole::PositionedWrite),
        1
    );
    serving.close();
}

#[test]
fn scheduler_counters_cannot_settle_cross_bound_backend_receipts() {
    let root = tempfile::tempdir().unwrap();
    let (profile, first_request, second_request) = disjoint_mutation_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let first = admitted_write(&serving, first_request);
    let second = admitted_write(&serving, second_request);
    let before = serving.media_counters();

    let fates = serving
        .certification_cross_settle_physical_writes(
            PhysicalExecutorCommand::exact_write(first, b"first001".as_slice()).unwrap(),
            PhysicalExecutorCommand::exact_write(second, b"second02".as_slice()).unwrap(),
        )
        .unwrap();

    assert_eq!(
        fates,
        [
            PhysicalWorkEffectFate::StaleOrForeignOutcome,
            PhysicalWorkEffectFate::StaleOrForeignOutcome,
        ],
        "C5_PREDICATE:scheduler-counter-settlement: scheduler counters settled foreign backend receipts"
    );
    assert_eq!(
        serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        2
    );
    assert!(serving.close_plan().execute().requires_inspection());
}

#[test]
fn one_canonical_write_requires_one_backend_effect() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let admitted = admitted_write(&serving, request);
    let command = PhysicalExecutorCommand::exact_write(admitted, b"onewrite".as_slice()).unwrap();
    let before = serving.media_counters();

    let outcome = serving.execute_physical_work(command);

    let outcome = outcome.expect("the admitted canonical write must reach settlement");
    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted,
        "C5_PREDICATE:skipped-backend-write: canonical write skipped its backend effect"
    );
    let after = serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        1,
        "C5_PREDICATE:raw-backend-dispatch: a write escaped the canonical executor path"
    );
    assert_eq!(
        after.completed_bytes_for(MediaOperationRole::PositionedWrite)
            - before.completed_bytes_for(MediaOperationRole::PositionedWrite),
        8
    );
    serving.close();
}
