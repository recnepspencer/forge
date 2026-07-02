#[path = "s4_5_checkpoint_publication_oracle_support.rs"]
#[allow(dead_code)]
mod checkpoint_oracle_support;
#[path = "s4_closeout/fixture.rs"]
#[allow(dead_code)]
mod closeout_fixture;

use checkpoint_oracle_support::{
    actor_step_index, checkpoint_evidence, checkpoint_origin, checkpoint_shortcut_trace,
    checkpoint_trace, copied_checkpoint_evidence, lower_checkpoint_crash_replay_plan,
    lower_checkpoint_plan, lower_checkpoint_shortcut_plan, lower_recovery_plan, recovery_trace,
    same_origin_copied_checkpoint_evidence, schedule,
};
use forge_store_physical_certification::{
    reject_same_run_self_comparison_evidence_attempt, reject_terminal_json_evidence_attempt,
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_terminal_projection_denial,
    ObservationDenial, PhysicalScenarioActorRole, PhysicalSimulationObserver,
    RecoveryOutcomeObservation, S5CheckpointPublicationCrashLaneOutput,
    S5CheckpointPublicationLaneBinding, S5CheckpointPublicationRecoveryOutcomeLaneOutput,
    S5CheckpointPublicationScheduledLaneOutput, S5CheckpointPublicationShortcutDenialLaneOutput,
    S5CheckpointPublicationShortcutRejectionOutput, ShortcutRejectionObservationKind,
};

#[test]
fn scheduled_checkpoint_publication_lane_emits_trace_observation() {
    let plan = lower_checkpoint_plan();
    let trace = checkpoint_trace(&plan);
    let observation = trace.checkpoint_interlock().unwrap();

    assert!(observation.no_mixed_root());
    assert!(observation.old_reader_retained_old_root());
    assert!(observation.post_publication_reader_observed_new_epoch());
    assert!(observation.page_lsn_frontier_bound_to_cutover());
    assert_eq!(observation.readmission_checks(), 1);
    assert_eq!(observation.publication_swaps(), 1);
}

#[test]
fn checkpoint_publication_lane_rejects_unrelated_schedule() {
    let checkpoint_plan = lower_checkpoint_plan();
    let recovery_plan = lower_recovery_plan();
    let recovery_schedule = schedule(&recovery_plan);

    let denial = S5CheckpointPublicationLaneBinding::from_plan_and_schedule(
        &checkpoint_plan,
        &recovery_schedule,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationLaneScheduleMismatch
    );
}

#[test]
fn copied_checkpoint_report_is_denied_after_schedule_binding() {
    let plan = lower_checkpoint_plan();
    let schedule = schedule(&plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(&plan, &schedule).unwrap();

    let denial =
        S5CheckpointPublicationScheduledLaneOutput::reject_copied_checkpoint_report_attempt(
            &binding,
            &schedule,
            binding.checkpoint_actor_step_index(),
            &checkpoint_origin(),
            same_origin_copied_checkpoint_evidence(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CopiedCheckpointReportObservationDenied
    );
}

#[test]
fn same_run_checkpoint_self_comparison_is_denied_after_schedule_binding() {
    let plan = lower_checkpoint_shortcut_plan();
    let trace = checkpoint_shortcut_trace(&plan);

    assert!(trace.shortcut_rejections().iter().any(|observation| {
        observation.kind() == ShortcutRejectionObservationKind::SameRunSelfComparisonDenied
    }));
}

#[test]
fn scheduled_checkpoint_shortcut_lane_rejects_wrong_checkpoint_origin() {
    let plan = lower_checkpoint_shortcut_plan();
    let schedule = schedule(&plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(&plan, &schedule).unwrap();
    let denial_output = same_run_shortcut_denial_output(&binding, &schedule);

    let denial = S5CheckpointPublicationShortcutRejectionOutput::from_scheduled_same_run_denial(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        copied_checkpoint_evidence(),
        denial_output,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationEvidenceOriginMismatch
    );
}

#[test]
fn scheduled_checkpoint_shortcut_lane_rejects_missing_shortcut_actor_step() {
    let plan = lower_checkpoint_plan();
    let schedule = schedule(&plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(&plan, &schedule).unwrap();
    let receipt = shortcut_denial_from_evidence_bundle_denial(
        reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
    )
    .unwrap();

    let denial = S5CheckpointPublicationShortcutDenialLaneOutput::from_denial_receipt(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        receipt,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationShortcutLaneScheduleMismatch
    );
}

#[test]
fn checkpoint_crash_lane_rejects_recovery_driver_without_fresh_runtime_replay() {
    let checkpoint_plan = lower_checkpoint_crash_replay_plan();
    let checkpoint_schedule = schedule(&checkpoint_plan);
    let binding = S5CheckpointPublicationLaneBinding::from_plan_and_schedule(
        &checkpoint_plan,
        &checkpoint_schedule,
    )
    .unwrap();
    let recovery_plan = lower_recovery_plan();
    let recovery_trace = recovery_trace(&recovery_plan);

    let denial =
        S5CheckpointPublicationRecoveryOutcomeLaneOutput::from_fresh_runtime_recovery_trace(
            &binding,
            &checkpoint_schedule,
            &recovery_plan,
            &checkpoint_schedule,
            actor_step_index(
                &checkpoint_schedule,
                PhysicalScenarioActorRole::RecoveryDriver,
            ),
            &recovery_trace,
            &checkpoint_origin(),
            checkpoint_evidence(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationCrashRecoveryTraceMismatch
    );
}

#[test]
fn checkpoint_crash_lane_rejects_mixed_recovery_outcome_receipt() {
    let checkpoint_plan = lower_checkpoint_crash_replay_plan();
    let checkpoint_schedule = schedule(&checkpoint_plan);
    let binding = S5CheckpointPublicationLaneBinding::from_plan_and_schedule(
        &checkpoint_plan,
        &checkpoint_schedule,
    )
    .unwrap();
    let recovery_plan = lower_recovery_plan();
    let recovery_schedule = schedule(&recovery_plan);
    let mixed_recovery_trace = PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(&recovery_plan)
        .unwrap()
        .with_runtime_trace(recovery_trace(&recovery_plan).runtime_trace().clone())
        .with_recovery_outcome_observation(RecoveryOutcomeObservation::mixed_root())
        .complete()
        .unwrap();

    let denial =
        S5CheckpointPublicationRecoveryOutcomeLaneOutput::from_fresh_runtime_recovery_trace(
            &binding,
            &checkpoint_schedule,
            &recovery_plan,
            &recovery_schedule,
            actor_step_index(
                &recovery_schedule,
                PhysicalScenarioActorRole::RecoveryDriver,
            ),
            &mixed_recovery_trace,
            &checkpoint_origin(),
            checkpoint_evidence(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationCrashOutcomeMixedRoot
    );
}

#[test]
fn checkpoint_crash_lane_rejects_recovery_receipt_from_unrelated_checkpoint_schedule() {
    let checkpoint_plan = lower_checkpoint_crash_replay_plan();
    let checkpoint_schedule = schedule(&checkpoint_plan);
    let binding = S5CheckpointPublicationLaneBinding::from_plan_and_schedule(
        &checkpoint_plan,
        &checkpoint_schedule,
    )
    .unwrap();
    let recovery_plan = lower_recovery_plan();
    let recovery_schedule = schedule(&recovery_plan);
    let recovery_output =
        S5CheckpointPublicationRecoveryOutcomeLaneOutput::from_fresh_runtime_recovery_trace(
            &binding,
            &checkpoint_schedule,
            &recovery_plan,
            &recovery_schedule,
            actor_step_index(
                &recovery_schedule,
                PhysicalScenarioActorRole::RecoveryDriver,
            ),
            &recovery_trace(&recovery_plan),
            &checkpoint_origin(),
            checkpoint_evidence(),
        )
        .unwrap();
    let unrelated_plan = lower_checkpoint_shortcut_plan();
    let unrelated_schedule = schedule(&unrelated_plan);
    let unrelated_binding = S5CheckpointPublicationLaneBinding::from_plan_and_schedule(
        &unrelated_plan,
        &unrelated_schedule,
    )
    .unwrap();

    let denial = S5CheckpointPublicationCrashLaneOutput::from_schedule_step_recovery(
        &unrelated_binding,
        &unrelated_schedule,
        unrelated_binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        &recovery_output,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationCrashRecoveryTraceMismatch
    );
}

#[test]
fn scheduled_checkpoint_shortcut_lane_rejects_wrong_shortcut_boundary() {
    let plan = lower_checkpoint_shortcut_plan();
    let schedule = schedule(&plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(&plan, &schedule).unwrap();
    let terminal_receipt = shortcut_denial_from_terminal_projection_denial(
        reject_terminal_json_evidence_attempt().unwrap_err(),
    );

    let denial = S5CheckpointPublicationShortcutDenialLaneOutput::from_denial_receipt(
        &binding,
        &schedule,
        actor_step_index(&schedule, PhysicalScenarioActorRole::ShortcutRejectionProbe),
        &checkpoint_origin(),
        checkpoint_evidence(),
        terminal_receipt,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationShortcutBoundaryMismatch
    );
}

fn same_run_shortcut_denial_output(
    binding: &S5CheckpointPublicationLaneBinding,
    schedule: &forge_store_physical_certification::PhysicalInterleavingSchedule,
) -> S5CheckpointPublicationShortcutDenialLaneOutput {
    let receipt = shortcut_denial_from_evidence_bundle_denial(
        reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
    )
    .unwrap();
    S5CheckpointPublicationShortcutDenialLaneOutput::from_denial_receipt(
        binding,
        schedule,
        actor_step_index(schedule, PhysicalScenarioActorRole::ShortcutRejectionProbe),
        &checkpoint_origin(),
        checkpoint_evidence(),
        receipt,
    )
    .unwrap()
}
