use worth_ui::facade::{
    app::WorthUi,
    registry::{
        TaskPresentationCancellationPosture, TaskPresentationDescriptor,
        TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationLifecyclePosture,
        TaskPresentationProjectionEligibility, TaskPresentationRuntimeAuthorityPosture,
    },
};

use super::task_presentation_assertions::assert_registered_task_presentation_ids;
use super::task_presentation_fixtures::{
    cancellable_task_presentation, progress_task_presentation, retryable_task_presentation,
    task_presentation_id,
};

#[test]
fn equivalent_task_presentations_produce_equivalent_projection_eligibility() {
    let left = WorthUi::app()
        .register_task_presentation(progress_task_presentation("workspace.task.progress"))
        .freeze()
        .expect("application preparation should succeed");
    let right = WorthUi::app()
        .register_task_presentation(progress_task_presentation("workspace.task.progress"))
        .freeze()
        .expect("application preparation should succeed");

    let left_entry = &left.capabilities().task_presentations().entries()[0];
    let right_entry = &right.capabilities().task_presentations().entries()[0];

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left_entry.descriptor().projection_eligibility(),
        right_entry.descriptor().projection_eligibility()
    );
    assert_eq!(
        left_entry.key().projection_basis(),
        right_entry.key().projection_basis()
    );
}

#[test]
fn accepted_task_presentations_are_canonically_ordered_and_inspectable() {
    let app = WorthUi::app()
        .register_task_presentation(retryable_task_presentation("workspace.task.retry"))
        .register_task_presentation(progress_task_presentation("workspace.task.progress"))
        .freeze()
        .expect("application preparation should succeed");

    assert_registered_task_presentation_ids(
        app.capabilities().task_presentations(),
        &["workspace.task.progress", "workspace.task.retry"],
    );
    assert!(app
        .capabilities()
        .task_presentations()
        .get(&task_presentation_id("workspace.task.retry"))
        .is_some());
}

#[test]
fn all_builtin_task_presentation_families_are_admitted() {
    let app = WorthUi::app()
        .register_task_presentation(progress_task_presentation("workspace.task.progress"))
        .register_task_presentation(cancellable_task_presentation("workspace.task.cancellable"))
        .register_task_presentation(retryable_task_presentation("workspace.task.retryable"))
        .register_task_presentation(presentation_with_family(
            "workspace.task.blocking",
            TaskPresentationFamily::blocking(),
            TaskPresentationProjectionEligibility::blocking_indicator(),
        ))
        .register_task_presentation(presentation_with_family(
            "workspace.task.background",
            TaskPresentationFamily::background(),
            TaskPresentationProjectionEligibility::status_summary(),
        ))
        .register_task_presentation(presentation_with_family(
            "workspace.task.completed",
            TaskPresentationFamily::completed(),
            TaskPresentationProjectionEligibility::completion_badge(),
        ))
        .register_task_presentation(presentation_with_family(
            "workspace.task.failed",
            TaskPresentationFamily::failed(),
            TaskPresentationProjectionEligibility::failure_summary(),
        ))
        .register_task_presentation(presentation_with_family(
            "workspace.task.status_projected",
            TaskPresentationFamily::status_projected(),
            TaskPresentationProjectionEligibility::status_summary(),
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().task_presentations().len(), 8);
}

#[test]
fn task_presentation_projection_eligibility_change_changes_snapshot_digest() {
    let status_summary_app = WorthUi::app()
        .register_task_presentation(presentation_with_family(
            "workspace.task.status",
            TaskPresentationFamily::background(),
            TaskPresentationProjectionEligibility::status_summary(),
        ))
        .freeze()
        .expect("application preparation should succeed");
    let hidden_app = WorthUi::app()
        .register_task_presentation(presentation_with_family(
            "workspace.task.status",
            TaskPresentationFamily::background(),
            TaskPresentationProjectionEligibility::hidden_from_projection(),
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        status_summary_app.capabilities().digest(),
        hidden_app.capabilities().digest()
    );
}

fn presentation_with_family(
    id: &str,
    family: TaskPresentationFamily,
    eligibility: TaskPresentationProjectionEligibility,
) -> TaskPresentationDescriptor {
    TaskPresentationDescriptor::new(task_presentation_id(id), family)
        .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
        .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
        .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
        .with_projection_eligibility(eligibility)
        .with_runtime_authority_posture(TaskPresentationRuntimeAuthorityPosture::presentation_only())
}

#[test]
fn task_presentation_family_metadata_change_changes_snapshot_digest() {
    let progress_app = WorthUi::app()
        .register_task_presentation(progress_task_presentation("workspace.task.family"))
        .freeze()
        .expect("application preparation should succeed");
    let cancellable_app = WorthUi::app()
        .register_task_presentation(cancellable_task_presentation("workspace.task.family"))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        progress_app.capabilities().digest(),
        cancellable_app.capabilities().digest()
    );
}
