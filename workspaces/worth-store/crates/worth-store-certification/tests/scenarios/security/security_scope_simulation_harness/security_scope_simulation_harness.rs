#[path = "../../../support/security/security_scope_simulation_harness/support.rs"]
mod support;

use support::{
    assert_lower_store_counter_crosscheck, assert_physical_binding_matches_replay,
    assert_security_scope_harness_evidence, assert_security_scope_typed_counters,
    expected_counters_for_mutation, physical_replay_for_scenario,
    physical_replay_for_scenario_with_replay_binding, replay_scenario, ExpectedTypedCounters,
};
use worth_store_physical_certification::{
    SecurityScopeHarnessOutcomeKind, SecurityScopeHarnessScenario, SecurityScopeHarnessSchedule,
    SecurityScopePhysicalReplayDenial, SecurityScopeReplayMutationKind,
};
use worth_store_security::{StoreKeyScope, StoreTenantScope};
use worth_store_test_support::{
    execute_security_scope_harness_replay_with_physical_replay,
    execute_security_scope_harness_scenario, security_scope_drift_scenario,
    security_scope_metadata_preservation_scenarios, security_scope_missing_authenticity_scenario,
    security_scope_replayed_custody_scenario, security_scope_stale_key_scenario,
    security_scope_wrong_tenant_scenario,
};

#[test]
fn security_scope_harness_preserves_metadata_across_all_schedules() {
    for scenario in security_scope_metadata_preservation_scenarios() {
        let execution = execute_security_scope_harness_scenario(scenario);
        let evidence = execution.evidence();

        assert_security_scope_harness_evidence(
            evidence,
            SecurityScopeHarnessOutcomeKind::Admitted,
            1,
            0,
        );
        assert_security_scope_typed_counters(evidence, ExpectedTypedCounters::admitted());
        assert_eq!(
            evidence
                .scenario()
                .schedule()
                .physical_schedule()
                .production_boundary_yieldpoint(),
            evidence.scenario().schedule().yieldpoint_name()
        );
        let readiness = execution
            .accepted_security_scope()
            .expect("metadata preservation must produce Store readiness");
        assert_eq!(
            readiness.witnesses().key_scope().key_scope(),
            StoreKeyScope::PageEnvelope
        );
        assert_eq!(
            readiness.witnesses().tenant_scope().tenant_scope(),
            StoreTenantScope::TenantPhysicalBoundary
        );
        let physical_replay = physical_replay_for_scenario(scenario);
        assert_eq!(physical_replay.scenario(), scenario);
        assert!(physical_replay
            .replay_bundle()
            .schedule()
            .replay_identity_matches_plan(physical_replay.replay_bundle().plan()));
        assert_physical_binding_matches_replay(&physical_replay);
        assert_lower_store_counter_crosscheck(evidence, ExpectedTypedCounters::admitted());
    }
}

#[test]
fn security_scope_harness_models_adversarial_scope_failures_before_decode() {
    let cases = [
        (
            security_scope_drift_scenario(),
            SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift,
            ExpectedTypedCounters::physical_scope_drift(),
        ),
        (
            security_scope_stale_key_scenario(),
            SecurityScopeHarnessOutcomeKind::StaleKeyPosture,
            ExpectedTypedCounters::stale_key_posture(),
        ),
        (
            security_scope_wrong_tenant_scenario(),
            SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope,
            ExpectedTypedCounters::wrong_tenant_scope(),
        ),
        (
            security_scope_missing_authenticity_scenario(),
            SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement,
            ExpectedTypedCounters::missing_authenticity_requirement(),
        ),
        (
            security_scope_replayed_custody_scenario(),
            SecurityScopeHarnessOutcomeKind::DeniedReplayedCustodyPosture,
            ExpectedTypedCounters::replayed_custody_posture(),
        ),
    ];

    for (scenario, expected_outcome, expected_counters) in cases {
        let evidence = execute_security_scope_harness_scenario(scenario).evidence();
        assert_security_scope_harness_evidence(evidence, expected_outcome, 0, 1);
        assert_security_scope_typed_counters(evidence, expected_counters);
        assert_lower_store_counter_crosscheck(evidence, expected_counters);
    }
}

#[test]
fn security_scope_harness_replay_rejects_changed_scope_on_same_schedule() {
    let schedules = [
        SecurityScopeHarnessSchedule::StableReadPlanAdmission,
        SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
        SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
        SecurityScopeHarnessSchedule::RepairReadAdmission,
    ];
    let mutations = [
        (
            SecurityScopeReplayMutationKind::ChangedTenantScope,
            SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope,
        ),
        (
            SecurityScopeReplayMutationKind::ChangedKeyVersionPosture,
            SecurityScopeHarnessOutcomeKind::StaleKeyPosture,
        ),
        (
            SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement,
            SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement,
        ),
    ];

    for schedule in schedules {
        for (mutation, expected_outcome) in mutations {
            let execution = execute_security_scope_harness_replay_with_physical_replay(
                schedule,
                mutation,
                physical_replay_for_scenario(SecurityScopeHarnessScenario::metadata_preserved(
                    schedule,
                )),
                physical_replay_for_scenario(replay_scenario(schedule, mutation)),
            )
            .expect("physical replay evidence must bind to the same schedule");
            let transcript = execution.transcript();
            assert_eq!(transcript.schedule(), schedule);
            assert_eq!(transcript.mutation(), mutation);
            assert!(transcript.replays_same_physical_schedule());
            assert!(transcript.replay_rejected_before_logical_decode());
            assert_eq!(
                transcript
                    .baseline_physical_replay()
                    .replay_bundle()
                    .schedule()
                    .identity(),
                transcript
                    .replay_physical_replay()
                    .replay_bundle()
                    .schedule()
                    .identity()
            );
            assert_eq!(
                transcript
                    .baseline_physical_replay()
                    .replay_bundle()
                    .replay_basis_identity(),
                transcript
                    .replay_physical_replay()
                    .replay_bundle()
                    .replay_basis_identity()
            );
            assert_security_scope_harness_evidence(
                transcript.baseline_evidence(),
                SecurityScopeHarnessOutcomeKind::Admitted,
                1,
                0,
            );
            assert_security_scope_typed_counters(
                transcript.baseline_evidence(),
                ExpectedTypedCounters::admitted(),
            );
            assert_lower_store_counter_crosscheck(
                transcript.baseline_evidence(),
                ExpectedTypedCounters::admitted(),
            );
            assert_security_scope_harness_evidence(
                transcript.replay_evidence(),
                expected_outcome,
                0,
                1,
            );
            assert_security_scope_typed_counters(
                transcript.replay_evidence(),
                expected_counters_for_mutation(mutation),
            );
            assert_lower_store_counter_crosscheck(
                transcript.replay_evidence(),
                expected_counters_for_mutation(mutation),
            );
            assert_eq!(transcript.counters().baseline_admissions(), 1);
            assert_eq!(transcript.counters().replay_attempts(), 1);
            assert_eq!(
                transcript.counters().replay_denials_before_logical_decode(),
                1
            );
        }
    }
}

#[test]
fn security_scope_harness_replay_denies_changed_physical_schedule_before_identity_reuse() {
    let mutation = SecurityScopeReplayMutationKind::ChangedTenantScope;
    let denial = execute_security_scope_harness_replay_with_physical_replay(
        SecurityScopeHarnessSchedule::StableReadPlanAdmission,
        mutation,
        physical_replay_for_scenario(SecurityScopeHarnessScenario::metadata_preserved(
            SecurityScopeHarnessSchedule::StableReadPlanAdmission,
        )),
        physical_replay_for_scenario(replay_scenario(
            SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
            mutation,
        )),
    )
    .expect_err("replay transcript must reject a different S5 physical schedule identity");

    assert_eq!(
        denial,
        SecurityScopePhysicalReplayDenial::ReplayScenarioMismatch
    );
}

#[test]
fn security_scope_harness_replay_denies_baseline_scenario_substitution() {
    let schedule = SecurityScopeHarnessSchedule::StableReadPlanAdmission;
    let mutation = SecurityScopeReplayMutationKind::ChangedTenantScope;
    let denial = execute_security_scope_harness_replay_with_physical_replay(
        schedule,
        mutation,
        physical_replay_for_scenario(SecurityScopeHarnessScenario::wrong_tenant_scope(schedule)),
        physical_replay_for_scenario(replay_scenario(schedule, mutation)),
    )
    .expect_err("baseline replay evidence must carry the baseline metadata scenario");

    assert_eq!(
        denial,
        SecurityScopePhysicalReplayDenial::BaselineScenarioMismatch
    );
}

#[test]
fn security_scope_harness_replay_denies_replay_scenario_substitution() {
    let schedule = SecurityScopeHarnessSchedule::StableReadPlanAdmission;
    let mutation = SecurityScopeReplayMutationKind::ChangedTenantScope;
    let denial = execute_security_scope_harness_replay_with_physical_replay(
        schedule,
        mutation,
        physical_replay_for_scenario(SecurityScopeHarnessScenario::metadata_preserved(schedule)),
        physical_replay_for_scenario(SecurityScopeHarnessScenario::stale_key_posture(schedule)),
    )
    .expect_err("replay evidence must carry the requested replay mutation scenario");

    assert_eq!(
        denial,
        SecurityScopePhysicalReplayDenial::ReplayScenarioMismatch
    );
}

#[test]
fn security_scope_physical_replay_denies_wrong_physical_isolation_family_binding() {
    let scenario = SecurityScopeHarnessScenario::metadata_preserved(
        SecurityScopeHarnessSchedule::StableReadPlanAdmission,
    );
    let scenario_binding = scenario.schedule().physical_replay_binding();
    let wrong_replay_binding =
        SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode.physical_replay_binding();
    let denial = physical_replay_for_scenario_with_replay_binding(
        scenario,
        scenario_binding,
        wrong_replay_binding,
    )
    .expect_err("S5.1 scenario must not bind to a different S5 physical family");

    assert_eq!(
        denial,
        SecurityScopePhysicalReplayDenial::PhysicalReplayFamilyMismatch
    );
}
