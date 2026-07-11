#[path = "s4_5_checkpoint_publication_oracle/support.rs"]
#[allow(dead_code)]
mod checkpoint_oracle_support;
#[path = "s4_closeout/fixture.rs"]
#[allow(dead_code)]
mod closeout_fixture;

use checkpoint_oracle_support::{
    actor_step_index, checkpoint_evidence, checkpoint_origin, copied_checkpoint_evidence,
    lower_checkpoint_crash_replay_plan, lower_checkpoint_plan, lower_checkpoint_shortcut_plan,
    lower_recovery_plan, recovery_trace, schedule,
};
use forge_store_physical_certification::{
    reject_same_run_self_comparison_evidence_attempt, shortcut_denial_from_evidence_bundle_denial,
    ObservationDenial, PhysicalScenarioActorRole, S5CheckpointPublicationCrashLaneOutput,
    S5CheckpointPublicationLaneBinding, S5CheckpointPublicationRecoveryOutcomeLaneOutput,
    S5CheckpointPublicationScheduledLaneOutput, S5CheckpointPublicationShortcutDenialLaneOutput,
    S5CheckpointPublicationShortcutRejectionOutput,
};

#[test]
fn copied_checkpoint_report_wrong_origin_is_denied_before_copied_report_branch() {
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
            copied_checkpoint_evidence(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationEvidenceOriginMismatch
    );
}

#[test]
fn scheduled_checkpoint_shortcut_lane_rejects_receipt_from_wrong_checkpoint_origin() {
    let plan = lower_checkpoint_shortcut_plan();
    let schedule = schedule(&plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(&plan, &schedule).unwrap();
    let receipt = shortcut_denial_from_evidence_bundle_denial(
        reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
    )
    .unwrap();
    let wrong_origin = copied_checkpoint_evidence().origin().clone();
    let wrong_origin_denial_output =
        S5CheckpointPublicationShortcutDenialLaneOutput::from_denial_receipt(
            &binding,
            &schedule,
            actor_step_index(&schedule, PhysicalScenarioActorRole::ShortcutRejectionProbe),
            &wrong_origin,
            copied_checkpoint_evidence(),
            receipt,
        )
        .unwrap();

    let denial = S5CheckpointPublicationShortcutRejectionOutput::from_scheduled_same_run_denial(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        wrong_origin_denial_output,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ObservationDenial::CheckpointPublicationShortcutLaneScheduleMismatch
    );
}

#[test]
fn checkpoint_crash_lane_rejects_recovery_receipt_bound_to_wrong_checkpoint_origin() {
    let checkpoint_plan = lower_checkpoint_crash_replay_plan();
    let checkpoint_schedule = schedule(&checkpoint_plan);
    let binding = S5CheckpointPublicationLaneBinding::from_plan_and_schedule(
        &checkpoint_plan,
        &checkpoint_schedule,
    )
    .unwrap();
    let recovery_plan = lower_recovery_plan();
    let recovery_schedule = schedule(&recovery_plan);
    let wrong_origin = copied_checkpoint_evidence().origin().clone();
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
            &wrong_origin,
            copied_checkpoint_evidence(),
        )
        .unwrap();

    let denial = S5CheckpointPublicationCrashLaneOutput::from_schedule_step_recovery(
        &binding,
        &checkpoint_schedule,
        binding.checkpoint_actor_step_index(),
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
