use worth_ui::facade::declaration::{
    TaskPresentationCancellationPosture, TaskPresentationDescriptor,
    TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationId,
    TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility,
    TaskPresentationRuntimeAuthorityPosture,
};

pub(crate) fn task_presentation_id(raw_text: &str) -> TaskPresentationId {
    TaskPresentationId::new(raw_text).expect("valid task presentation id")
}

pub(crate) fn progress_task_presentation(id: &str) -> TaskPresentationDescriptor {
    TaskPresentationDescriptor::new(
        task_presentation_id(id),
        TaskPresentationFamily::progress(),
    )
    .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
    .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
    .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
    .with_projection_eligibility(TaskPresentationProjectionEligibility::progress_indicator())
    .with_runtime_authority_posture(TaskPresentationRuntimeAuthorityPosture::presentation_only())
}

pub(crate) fn cancellable_task_presentation(id: &str) -> TaskPresentationDescriptor {
    TaskPresentationDescriptor::new(
        task_presentation_id(id),
        TaskPresentationFamily::cancellable(),
    )
    .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
    .with_cancellation_posture(TaskPresentationCancellationPosture::runtime_cancellable())
    .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
    .with_projection_eligibility(TaskPresentationProjectionEligibility::status_summary())
    .with_runtime_authority_posture(
        TaskPresentationRuntimeAuthorityPosture::runtime_state_reference(),
    )
}

pub(crate) fn retryable_task_presentation(id: &str) -> TaskPresentationDescriptor {
    TaskPresentationDescriptor::new(
        task_presentation_id(id),
        TaskPresentationFamily::retryable(),
    )
    .with_lifecycle_posture(TaskPresentationLifecyclePosture::application_owned())
    .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
    .with_failure_posture(TaskPresentationFailurePosture::retry_offered_by_application())
    .with_projection_eligibility(TaskPresentationProjectionEligibility::failure_summary())
    .with_runtime_authority_posture(
        TaskPresentationRuntimeAuthorityPosture::application_state_reference(),
    )
}
