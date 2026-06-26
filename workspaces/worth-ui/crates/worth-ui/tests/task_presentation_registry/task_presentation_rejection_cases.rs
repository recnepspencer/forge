use worth_ui::facade::{
    CapabilityDiagnosticCode, TaskPresentationCancellationPosture, TaskPresentationDescriptor,
    TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationLifecyclePosture,
    TaskPresentationProjectionEligibility, TaskPresentationRuntimeAuthorityPosture, WorthUi,
};

use super::task_presentation_assertions::{
    assert_diagnostic_codes, assert_registered_task_presentation_ids,
};
use super::task_presentation_fixtures::{progress_task_presentation, task_presentation_id};

#[test]
fn duplicate_task_presentation_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_task_presentation(progress_task_presentation("workspace.task.duplicate"))
        .register_task_presentation(progress_task_presentation("workspace.task.duplicate"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().task_presentations().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn task_presentation_without_lifecycle_posture_rejected() {
    let report =
        WorthUi::app()
            .register_task_presentation(
                TaskPresentationDescriptor::new(
                    task_presentation_id("workspace.task.no_lifecycle"),
                    TaskPresentationFamily::progress(),
                )
                .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
                .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
                .with_projection_eligibility(
                    TaskPresentationProjectionEligibility::progress_indicator(),
                )
                .with_runtime_authority_posture(
                    TaskPresentationRuntimeAuthorityPosture::presentation_only(),
                ),
            )
            .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingTaskPresentationLifecyclePosture],
    );
}

#[test]
fn task_presentation_without_cancellation_posture_rejected() {
    let report =
        WorthUi::app()
            .register_task_presentation(
                TaskPresentationDescriptor::new(
                    task_presentation_id("workspace.task.no_cancellation"),
                    TaskPresentationFamily::progress(),
                )
                .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
                .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
                .with_projection_eligibility(
                    TaskPresentationProjectionEligibility::progress_indicator(),
                )
                .with_runtime_authority_posture(
                    TaskPresentationRuntimeAuthorityPosture::presentation_only(),
                ),
            )
            .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingTaskPresentationCancellationPosture],
    );
}

#[test]
fn task_presentation_without_failure_posture_rejected() {
    let report =
        WorthUi::app()
            .register_task_presentation(
                TaskPresentationDescriptor::new(
                    task_presentation_id("workspace.task.no_failure"),
                    TaskPresentationFamily::progress(),
                )
                .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
                .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
                .with_projection_eligibility(
                    TaskPresentationProjectionEligibility::progress_indicator(),
                )
                .with_runtime_authority_posture(
                    TaskPresentationRuntimeAuthorityPosture::presentation_only(),
                ),
            )
            .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingTaskPresentationFailurePosture],
    );
}

#[test]
fn task_presentation_without_projection_eligibility_rejected() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.no_projection"),
                TaskPresentationFamily::progress(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingTaskPresentationProjectionEligibility],
    );
}

#[test]
fn task_presentation_without_runtime_authority_posture_rejected() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.no_authority"),
                TaskPresentationFamily::progress(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(
                TaskPresentationProjectionEligibility::progress_indicator(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingTaskPresentationRuntimeAuthorityPosture],
    );
}

#[test]
fn unknown_task_presentation_family_rejected() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.unknown"),
                TaskPresentationFamily::unknown_for_diagnostics("upload"),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnknownTaskPresentationFamily],
    );
}

#[test]
fn task_presentation_cannot_claim_task_runtime_authority() {
    let report =
        WorthUi::app()
            .register_task_presentation(
                TaskPresentationDescriptor::new(
                    task_presentation_id("workspace.task.runtime_claim"),
                    TaskPresentationFamily::progress(),
                )
                .with_lifecycle_posture(
                    TaskPresentationLifecyclePosture::presentation_owns_lifecycle_for_diagnostics(),
                )
                .with_cancellation_posture(
                    TaskPresentationCancellationPosture::presentation_cancels_task_for_diagnostics(
                    ),
                )
                .with_failure_posture(
                    TaskPresentationFailurePosture::presentation_retries_task_for_diagnostics(),
                )
                .with_projection_eligibility(
                    TaskPresentationProjectionEligibility::progress_indicator(),
                )
                .with_runtime_authority_posture(
                    TaskPresentationRuntimeAuthorityPosture::owns_task_runtime_for_diagnostics(),
                ),
            )
            .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().task_presentations().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::TaskPresentationClaimsTaskRuntimeAuthority],
    );
}

#[test]
fn task_presentation_family_posture_mismatch_rejected() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.mismatch"),
                TaskPresentationFamily::cancellable(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::TaskPresentationFamilyPostureMismatch],
    );
}

#[test]
fn rejected_task_presentation_does_not_poison_valid_task_presentation() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.bad"),
                TaskPresentationFamily::progress(),
            )
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .register_task_presentation(progress_task_presentation("workspace.task.good"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_task_presentation_ids(
        report.accepted_snapshot().task_presentations(),
        &["workspace.task.good"],
    );
}
