use worth_ui::facade::{
    declaration::{TaskPresentationCancellationPosture, TaskPresentationDescriptor, TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationId, TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility, TaskPresentationRuntimeAuthorityPosture},
};

fn main() {
    let _descriptor = TaskPresentationDescriptor {
        id: TaskPresentationId::new("workspace.task.raw").unwrap(),
        family: TaskPresentationFamily::progress(),
        lifecycle_posture: Some(TaskPresentationLifecyclePosture::runtime_owned()),
        cancellation_posture: Some(TaskPresentationCancellationPosture::not_cancellable()),
        failure_posture: Some(TaskPresentationFailurePosture::runtime_reported()),
        projection_eligibility: Some(TaskPresentationProjectionEligibility::progress_indicator()),
        runtime_authority_posture: Some(
            TaskPresentationRuntimeAuthorityPosture::presentation_only(),
        ),
    };
}
