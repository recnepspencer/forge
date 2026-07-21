use worth_ui::facade::{
    app::WorthUi,
    diagnostics::CapabilityDiagnosticCode,
    registry::{
        TaskPresentationCancellationPosture, TaskPresentationDescriptor,
        TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationLifecyclePosture,
        TaskPresentationProjectionEligibility, TaskPresentationRuntimeAuthorityPosture,
    },
};

use super::task_presentation_assertions::assert_diagnostic_codes;
use super::task_presentation_fixtures::task_presentation_id;

#[test]
fn status_projected_task_presentation_requires_status_projection_eligibility() {
    let report =
        WorthUi::app()
            .register_task_presentation(
                TaskPresentationDescriptor::new(
                    task_presentation_id("workspace.task.status_mismatch"),
                    TaskPresentationFamily::status_projected(),
                )
                .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
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
        &[CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch],
    );
}

#[test]
fn terminal_task_presentation_families_require_terminal_projection_eligibility() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.completed_mismatch"),
                TaskPresentationFamily::completed(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.failed_mismatch"),
                TaskPresentationFamily::failed(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::completion_badge())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch,
            CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch,
        ],
    );
}

#[test]
fn active_task_presentation_families_require_matching_projection_eligibility() {
    let report = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.progress_mismatch"),
                TaskPresentationFamily::progress(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.blocking_mismatch"),
                TaskPresentationFamily::blocking(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::progress_indicator())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                task_presentation_id("workspace.task.retry_projection_mismatch"),
                TaskPresentationFamily::retryable(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::retry_offered_by_runtime())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::runtime_state_reference(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch,
            CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch,
            CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch,
        ],
    );
}
