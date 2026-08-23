use super::super::checkpoint_crash::evidence::copy_directory;
use super::super::comparison;
use super::super::history;
use super::checkpoint_mutations::swap_completed_record_lists;
use super::harness::ProcessWorld;
use super::harness::{run_observer, run_recovery_with_profile};

#[test]
fn production_and_observer_reject_rehashed_evidence_mutations() {
    let world = ProcessWorld::start_durable_unacknowledged(0);
    let runtime = world.recover("mutation");
    let observer = world.observe("mutation");
    let history = world.parent_history_after_recovery(&world.writer.root);
    super::harness::compare_runtime_and_observer(&runtime, &observer, &history);

    for (field, encoded) in comparison::mutate_observer_evidence_fields(&observer.encoded) {
        let mutated_observer = worth_store_offline_verifier::RecoveryObserverReport::decode(
            &encoded,
        )
        .unwrap_or_else(|error| panic!("mutated observer field {field} must decode: {error:?}"));
        assert_eq!(
            comparison::compare_runtime_and_observer(&runtime.report, &mutated_observer, &history),
            Err(comparison::RecoveryObserverDisagreement::ParentHistoryMismatch),
            "observer field mutation {field} was not detected"
        );
    }

    let mutated_observer = worth_store_offline_verifier::RecoveryObserverReport::decode(
        &comparison::mutate_artifact_identity_digest(&observer.encoded),
    )
    .expect("mutated rich observer report retains a valid descriptive digest");
    assert_eq!(
        comparison::compare_runtime_and_observer(&runtime.report, &mutated_observer, &history),
        Err(comparison::RecoveryObserverDisagreement::ParentHistoryMismatch)
    );

    let mutated_runtime = worth_store_recovery_runtime::RecoveryReportEnvelope::decode(
        &comparison::mutate_runtime_root_generation(&runtime.encoded),
    )
    .expect("mutated runtime report retains a valid descriptive digest");
    assert_eq!(
        comparison::compare_effectful_runtime_and_observer(
            &mutated_runtime,
            &observer.report,
            &history,
        ),
        Err(comparison::RecoveryObserverDisagreement::RootGenerationMismatch)
    );

    let mutated_runtime = worth_store_recovery_runtime::RecoveryReportEnvelope::decode(
        &comparison::mutate_runtime_peak_recovery_bytes(&runtime.encoded, 0),
    )
    .expect("mutated runtime memory report retains a valid descriptive digest");
    assert_eq!(
        comparison::compare_runtime_and_observer(&mutated_runtime, &observer.report, &history),
        Err(comparison::RecoveryObserverDisagreement::RuntimeCounterMismatch)
    );
    let mutated_runtime = worth_store_recovery_runtime::RecoveryReportEnvelope::decode(
        &comparison::mutate_runtime_recovery_effects(&runtime.encoded, 0),
    )
    .expect("mutated runtime effects report retains a valid descriptive digest");
    assert_eq!(
        comparison::compare_effectful_runtime_and_observer(
            &mutated_runtime,
            &observer.report,
            &history,
        ),
        Err(comparison::RecoveryObserverDisagreement::RuntimeCounterMismatch)
    );
    let mutated_runtime = worth_store_recovery_runtime::RecoveryReportEnvelope::decode(
        &comparison::mutate_runtime_peak_recovery_bytes(
            &runtime.encoded,
            super::harness::C8_RECOVERY_MEMORY_BUDGET_BYTES + 1,
        ),
    )
    .expect("one-over-budget runtime report retains a valid descriptive digest");
    assert_eq!(
        comparison::compare_runtime_and_observer(&mutated_runtime, &observer.report, &history),
        Err(comparison::RecoveryObserverDisagreement::RuntimeCounterMismatch)
    );
    world.finish_within_budget("production/mutation process proof");
}

#[test]
fn fresh_processes_are_checked_against_reclaimed_checkpoint_tuple_bindings() {
    let world = ProcessWorld::start_after_reclamation(0);
    let baseline_history = world.writer.history.clone();
    history::require_completed_bindings_reclaimed(&world.writer.root, &world.writer.expected)
        .expect("completed checkpoint bindings must be absent from retained WAL");
    let mutated_root = world
        .parent_path()
        .join("reclaimed-checkpoint-tuple-mutant");
    copy_directory(&world.writer.root, &mutated_root);
    swap_completed_record_lists(&mutated_root);

    let observer_report = world
        .parent_path()
        .join("reclaimed-checkpoint-tuple-observer.bin");
    let (_, observer_output) =
        run_observer(&mutated_root, observer_report.clone(), world.parent_path());
    assert!(
        observer_output.status.success(),
        "fresh observer must still expose the rehashed physical mutation for the parent oracle: stderr={}",
        String::from_utf8_lossy(&observer_output.stderr)
    );
    assert!(observer_report.exists());

    let recovery_report = world
        .parent_path()
        .join("reclaimed-checkpoint-tuple-recovery.bin");
    let (_, recovery_output) = run_recovery_with_profile(
        &mutated_root,
        &recovery_report,
        world.parent_path(),
        "c8-phase2-admission-v1",
    );
    assert!(
        recovery_output.status.success(),
        "fresh recovery must complete before the independent parent oracle rejects the tuple swap: stderr={}",
        String::from_utf8_lossy(&recovery_output.stderr)
    );
    let observer = worth_store_offline_verifier::RecoveryObserverReport::decode(
        &std::fs::read(&observer_report).expect("read reclaimed checkpoint observer report"),
    )
    .expect("decode reclaimed checkpoint observer report");
    let runtime = worth_store_recovery_runtime::RecoveryReportEnvelope::decode(
        &std::fs::read(&recovery_report).expect("read reclaimed checkpoint recovery report"),
    )
    .expect("decode reclaimed checkpoint recovery report");
    assert_eq!(
        comparison::compare_runtime_and_observer(&runtime, &observer, &baseline_history),
        Err(comparison::RecoveryObserverDisagreement::ParentHistoryMismatch),
        "fresh process reports must expose the rehashed checkpoint tuple disagreement"
    );

    assert!(
        history::classify_persisted_fates(&world.writer.expected, &mutated_root).is_err(),
        "the independent parent oracle must reject a rehashed checkpoint tuple swap after WAL reclamation"
    );
    std::fs::remove_dir_all(&mutated_root).expect("remove checkpoint tuple mutation root");
    world.finish_within_budget("production/reclaimed-checkpoint-tuple mutation proof");
}
