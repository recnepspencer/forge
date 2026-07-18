use worth_ui::facade::{
    TaskPresentationCancellationPosture, TaskPresentationDescriptor,
    TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationId,
    TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility,
    TaskPresentationRuntimeAuthorityPosture, WorthUi,
};

fn main() {
    let app = WorthUi::app()
        .register_task_presentation(
            TaskPresentationDescriptor::new(
                TaskPresentationId::new("workspace.task.facade").unwrap(),
                TaskPresentationFamily::progress(),
            )
            .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
            .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
            .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
            .with_projection_eligibility(TaskPresentationProjectionEligibility::progress_indicator())
            .with_runtime_authority_posture(
                TaskPresentationRuntimeAuthorityPosture::presentation_only(),
            ),
        )
        .freeze().expect("application preparation should succeed");

    assert_eq!(app.capabilities().task_presentations().len(), 1);
}
