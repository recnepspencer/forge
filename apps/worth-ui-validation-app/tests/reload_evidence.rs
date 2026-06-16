use worth_ui_validation_app::reload::ValidationReloadStage;
use worth_ui_validation_app::reload::{ValidationReloadRequest, ValidationReloadStatus};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::ValidationWorkbenchLaunch;

#[test]
fn reload_equivalent_source_is_noop_and_does_not_mutate_active_runtime() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI");
    let before = launch.runtime().inspect_active();
    let reload = launch.runtime().prepare_validation_reload(
        launch.app().capabilities(),
        reload_request(VALIDATION_SAMPLE_SOURCE),
    );
    let evidence = reload.evidence();

    assert_eq!(evidence.status(), ValidationReloadStatus::EquivalentNoOp);
    assert!(!reload.is_ready());
    assert_eq!(
        evidence.active_artifact_digest_before(),
        evidence.active_artifact_digest_after()
    );
    assert_eq!(
        evidence.active_plan_digest_before(),
        evidence.active_plan_digest_after()
    );
    assert_eq!(before, launch.runtime().inspect_active());
    assert_eq!(evidence.raw_events_observed(), 1);
    assert_eq!(evidence.source_revisions_emitted(), 1);
    assert_eq!(evidence.candidate_submissions_emitted(), 1);
    assert_eq!(evidence.frame_path_work(), 0);
    assert_eq!(evidence.active_runtime_mutations_before_activation(), 0);
}

#[test]
fn reload_invalid_source_preserves_last_valid_runtime() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI");
    let before_active = launch.runtime().inspect_active();
    let before_last_valid = launch.runtime().last_valid();
    let reload = launch.runtime().prepare_validation_reload(
        launch.app().capabilities(),
        reload_request("app Broken { workspace MissingWorkspace"),
    );
    let evidence = reload.evidence();

    assert!(matches!(
        evidence.status(),
        ValidationReloadStatus::Denied(_)
    ));
    assert!(!reload.is_ready());
    assert_eq!(before_active, launch.runtime().inspect_active());
    assert_eq!(before_last_valid, launch.runtime().last_valid());
    assert_eq!(
        evidence.active_artifact_digest_before(),
        evidence.active_artifact_digest_after()
    );
    assert_eq!(evidence.frame_path_work(), 0);
    assert_eq!(evidence.active_runtime_mutations_before_activation(), 0);
}

#[test]
fn reload_valid_source_reaches_activation_only_at_frame_boundary() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI");
    let mut workbench = launch.into_runtime_workbench();
    let before = workbench.runtime().inspect_active();
    let reload = workbench.prepare_reload(reload_request(&meaningfully_changed_source()));
    let prepared_evidence = reload.evidence().clone();

    assert_eq!(
        prepared_evidence.status(),
        ValidationReloadStatus::ReadyForFrameBoundary,
        "reload denied with detail: {:?}",
        prepared_evidence.denial_detail()
    );
    assert!(reload.is_ready());
    assert_eq!(before, workbench.runtime().inspect_active());
    assert_eq!(
        prepared_evidence.active_artifact_digest_before(),
        prepared_evidence.active_artifact_digest_after()
    );
    assert_ne!(
        prepared_evidence.candidate_artifact_digest(),
        Some(prepared_evidence.active_artifact_digest_before())
    );
    assert!(prepared_evidence.candidate_plan_digest().is_some());
    assert_eq!(prepared_evidence.frame_path_work(), 0);
    assert_eq!(
        prepared_evidence.active_runtime_mutations_before_activation(),
        0
    );

    let activated = workbench
        .activate_reload(reload)
        .expect("ready reload activates at safe frame boundary");
    assert_eq!(activated.status(), ValidationReloadStatus::Activated);
    assert_ne!(before, workbench.runtime().inspect_active());
    assert_ne!(
        activated.active_plan_digest_before(),
        activated.active_plan_digest_after()
    );
}

#[test]
fn reload_evidence_records_query_and_state_planning_surfaces() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI");
    let reload = launch.runtime().prepare_validation_reload(
        launch.app().capabilities(),
        reload_request(&meaningfully_changed_source()),
    );
    let evidence = reload.evidence();

    assert_eq!(
        evidence.status(),
        ValidationReloadStatus::ReadyForFrameBoundary,
        "reload denied with detail: {:?}",
        evidence.denial_detail()
    );
    assert!(evidence.query_binding_planning_ran());
    assert!(evidence.durable_state_planning_ran());
    assert_eq!(evidence.frame_path_work(), 0);
    assert_eq!(evidence.active_runtime_mutations_before_activation(), 0);
}

#[test]
fn prepared_reload_cannot_activate_against_an_unrelated_runtime() {
    let source = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("source validation app launches through Worth UI")
        .into_runtime_workbench();
    let mut target = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("target validation app launches through Worth UI")
        .into_runtime_workbench();
    let target_before = target.runtime().inspect_active();
    let reload = source.prepare_reload(reload_request(&meaningfully_changed_source()));

    assert_eq!(
        reload.evidence().status(),
        ValidationReloadStatus::ReadyForFrameBoundary,
        "source reload must be ready before proving wrong-runtime activation rejection"
    );
    let denial = target
        .activate_reload(reload)
        .expect_err("prepared reload must not activate against a different runtime");

    assert_eq!(denial, ValidationReloadStage::RuntimeInstanceMismatch);
    assert_eq!(target_before, target.runtime().inspect_active());
}

fn reload_request(source: &str) -> ValidationReloadRequest {
    ValidationReloadRequest::from_source_module(VALIDATION_SAMPLE_MODULE_PATH, source)
}

fn meaningfully_changed_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "proof -> validation.surface.header.proof",
        "proof -> validation.surface.header.proof.alt",
    )
}
