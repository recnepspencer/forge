use worth_store_test_support::{
    deterministic_s4_fresh_runtime_driver, FaultSchedulerDriver, FreshRuntimeRecoveryDriver,
    StorageBoundaryInterposerDriver,
};

use super::{
    RecoveryPhysicsCertificationMatrix, RecoveryPhysicsCounterExpectation,
    RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane, RecoveryPhysicsCrashMatrix,
    RecoveryPhysicsCrashMatrixDenial, RecoveryPhysicsMutationFailureEvidence,
    RecoveryPhysicsMutationValidationDenial, RecoveryPhysicsMutationValidationMatrix,
    RecoveryPhysicsObserverKind, RecoveryPhysicsOracleKind,
    RecoveryPhysicsRoadmap2HarnessCertification, RecoveryPhysicsScenarioDefinition,
    RecoveryPhysicsScenarioDrivers, RecoveryPhysicsScenarioPlan, RecoveryPhysicsScenarioPlanDenial,
    RecoveryPhysicsShortcutAttempt,
};

#[test]
fn s4_recovery_crash_matrix_contains_required_lanes() {
    let matrix = RecoveryPhysicsCrashMatrix::roadmap_2_s4()
        .seed(44)
        .backend_profile("ci-certification")
        .lower()
        .expect("required S.4 lanes lower through scenario plans");

    assert_eq!(
        matrix.plans().len(),
        RecoveryPhysicsCrashLane::REQUIRED_S4_LANES.len()
    );
    for lane in RecoveryPhysicsCrashLane::REQUIRED_S4_LANES {
        let plan = matrix.plan_for_lane(lane).expect("required lane present");
        assert_eq!(plan.seed(), 44);
        assert_eq!(plan.backend_profile(), "ci-certification");
    }
}

#[test]
fn s4_recovery_crash_matrix_denies_live_runtime_reuse_before_execution() {
    let denial = RecoveryPhysicsCrashMatrix::roadmap_2_s4()
        .recovery_driver(FreshRuntimeRecoveryDriver::same_process_live_state_reuse())
        .lower()
        .expect_err("same process live-state reuse is not a valid crash harness driver");

    assert!(matches!(
        denial,
        RecoveryPhysicsCrashMatrixDenial::Plan(RecoveryPhysicsScenarioPlanDenial::LiveRuntimeReuse)
    ));
}

#[test]
fn s4_recovery_plan_denies_mismatched_boundary_events_before_certification() {
    let lane = RecoveryPhysicsCrashLane::WalAppend;
    let storage = StorageBoundaryInterposerDriver::production_like("ci-certification");
    let drivers = RecoveryPhysicsScenarioDrivers::new(
        FaultSchedulerDriver::deterministic(7),
        storage.clone(),
        deterministic_s4_fresh_runtime_driver(),
    );
    let mut builder = RecoveryPhysicsScenarioDefinition::builder(lane)
        .seed(7)
        .backend_profile("ci-certification")
        .boundary_event(storage.lower_boundary_event("wrong.storage.seam", 1))
        .drivers(drivers);

    for observer in RecoveryPhysicsObserverKind::REQUIRED {
        builder = builder.observer(observer);
    }
    for oracle in RecoveryPhysicsOracleKind::REQUIRED_SCENARIO_ORACLES {
        builder = builder.oracle(oracle);
    }
    for counter in RecoveryPhysicsCounterKind::REQUIRED_SCENARIO_COUNTERS {
        builder = builder.counter_expectation(RecoveryPhysicsCounterExpectation::exact(counter, 1));
    }

    let definition = builder.define().expect("definition has required fields");
    assert!(matches!(
        RecoveryPhysicsScenarioPlan::lower(definition),
        Err(RecoveryPhysicsScenarioPlanDenial::BoundaryEventSeamMismatch)
    ));
}

#[test]
fn s4_recovery_certification_transcripts_name_harness_evidence() {
    let roadmap2_certification = RecoveryPhysicsRoadmap2HarnessCertification::certify_s4_ci()
        .expect("Roadmap 2 S.4 harness certifies");
    let certification = roadmap2_certification.certification_matrix();

    assert_eq!(
        certification.shortcut_rejections().len(),
        RecoveryPhysicsShortcutAttempt::required_s4_denials().len()
    );
    for row in certification.rows() {
        let transcript = row.transcript();
        assert!(!transcript.driver_name().is_empty());
        assert_eq!(transcript.observer_names().len(), 7);
        assert_eq!(transcript.oracle_judgments().len(), 4);
        assert_eq!(transcript.seed(), 0x5346_000A);
        assert_eq!(
            transcript.evidence_bundle().lane(),
            transcript.lane(),
            "evidence bundle must be lane-bound"
        );
        assert_eq!(
            transcript.boundary_event().seam(),
            transcript.lane().crash_seam()
        );
        assert_eq!(
            transcript.boundary_event().backend_profile(),
            transcript.backend_profile()
        );
        assert_eq!(transcript.boundary_event().fault_ordinal(), 1);
        assert!(transcript
            .counter_expectations()
            .iter()
            .any(
                |counter| counter.kind() == RecoveryPhysicsCounterKind::Transcripts
                    && counter.expected() == 1
            ));
    }
}

#[test]
fn s4_recovery_shortcuts_fail_certification_with_boundary_specific_evidence() {
    let roadmap2_certification = RecoveryPhysicsRoadmap2HarnessCertification::certify_s4_ci()
        .expect("Roadmap 2 S.4 harness certifies");
    let certification = roadmap2_certification.certification_matrix();
    let attempts: Vec<_> = certification
        .shortcut_rejections()
        .iter()
        .map(|rejection| rejection.attempt())
        .collect();

    for required in RecoveryPhysicsShortcutAttempt::required_s4_denials() {
        assert!(attempts.contains(&required));
        assert!(
            RecoveryPhysicsCertificationMatrix::certify_shortcut_attempt(required).is_err(),
            "shortcut attempts must fail at the certification boundary"
        );
        let rejection = certification
            .shortcut_rejections()
            .iter()
            .find(|rejection| rejection.attempt() == required)
            .expect("required shortcut rejection present");
        assert_eq!(rejection.boundary(), required.denial_boundary());
        assert_eq!(rejection.reason(), required.denial_reason());
        assert_eq!(
            rejection.oracle(),
            RecoveryPhysicsOracleKind::RejectSyntheticShortcut
        );
    }
    assert!(certification.shortcut_rejections().iter().all(|rejection| {
        rejection.counter().kind() == RecoveryPhysicsCounterKind::ShortcutDenials
            && rejection.counter().expected() == 1
    }));
}

#[test]
fn s4_recovery_mutation_validation_matrix_fails_required_mutants() {
    let roadmap2_certification = RecoveryPhysicsRoadmap2HarnessCertification::certify_s4_ci()
        .expect("Roadmap 2 S.4 harness certifies");
    let mutations = RecoveryPhysicsMutationValidationMatrix::validate(
        roadmap2_certification.certification_matrix(),
        roadmap2_certification.mutation_evidence().rows(),
    )
    .expect("mutants fail");

    assert!(mutations.all_required_mutants_failed());
    assert!(mutations.rows().iter().any(|row| matches!(
        row.failure_evidence(),
        RecoveryPhysicsMutationFailureEvidence::CompileFailBoundary
    )));
    assert!(mutations.rows().iter().all(|row| {
        row.counter().kind() == RecoveryPhysicsCounterKind::MutationFailures
            && row.counter().expected() == 1
    }));
}

#[test]
fn s4_recovery_mutation_validation_denies_missing_or_wrong_suite_evidence() {
    let roadmap2_certification = RecoveryPhysicsRoadmap2HarnessCertification::certify_s4_ci()
        .expect("Roadmap 2 S.4 harness certifies");
    let certification = roadmap2_certification.certification_matrix();
    let mut evidence = roadmap2_certification.mutation_evidence().rows().to_vec();
    evidence.pop();

    assert!(matches!(
        RecoveryPhysicsMutationValidationMatrix::validate(&certification, &evidence),
        Err(RecoveryPhysicsMutationValidationDenial::MissingEvidence(_))
    ));

    let mut wrong_lane = roadmap2_certification.mutation_evidence().rows().to_vec();
    wrong_lane[0] = wrong_lane[0].with_lane(RecoveryPhysicsCrashLane::RenameDurability);
    assert!(matches!(
        RecoveryPhysicsMutationValidationMatrix::validate(&certification, &wrong_lane),
        Err(RecoveryPhysicsMutationValidationDenial::WrongLane { .. })
    ));

    let mut wrong_evidence = roadmap2_certification.mutation_evidence().rows().to_vec();
    wrong_evidence[0] = wrong_evidence[0]
        .with_failure_evidence(RecoveryPhysicsMutationFailureEvidence::CompileFailBoundary);
    assert!(matches!(
        RecoveryPhysicsMutationValidationMatrix::validate(&certification, &wrong_evidence),
        Err(RecoveryPhysicsMutationValidationDenial::WrongEvidence(_))
    ));

    let mut wrong_counter = roadmap2_certification.mutation_evidence().rows().to_vec();
    wrong_counter[0] = wrong_counter[0].with_counter(RecoveryPhysicsCounterExpectation::exact(
        RecoveryPhysicsCounterKind::MutationFailures,
        0,
    ));
    assert!(matches!(
        RecoveryPhysicsMutationValidationMatrix::validate(&certification, &wrong_counter),
        Err(RecoveryPhysicsMutationValidationDenial::WrongCounter(_))
    ));
}
