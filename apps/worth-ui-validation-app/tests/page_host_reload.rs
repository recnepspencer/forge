use worth_ui::facade::{
    WorthUiPageHostRebindDenial, WorthUiPageHostRebindStatus, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId,
};
use worth_ui_validation_app::reload::{
    ValidationReloadInput, ValidationReloadRequest, ValidationReloadStatus, ValidationReloadTick,
    ValidationRuntimeReloadTickOutcome, ValidationSourcePackage,
};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

#[test]
fn page_host_launch_projects_runtime_owned_content_slots() {
    let workbench = runtime_workbench();
    let plan = workbench.page_host_plan();
    let receipt = plan.execute_frame();

    assert_eq!(receipt.page_name(), "HeaderProofPage");
    assert_eq!(receipt.slots().len(), 1);
    assert_eq!(receipt.slots()[0].slot_name(), "button_proof");
    assert_eq!(
        receipt.slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert!(plan
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::layout_topology("HeaderProofPage")));
    assert!(plan.dependencies().contains_exact(
        &WorthUiRuntimeFactId::authored_mount_component_selection(
            "worth.surface.preview.primitive.proof",
        ),
    ));
}

#[test]
fn activated_source_reload_rebinds_page_host_from_runtime_authoring_snapshot() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.page_host_plan().frame_digest();
    let reload = workbench.prepare_reload(reload_request(&alternate_interaction_payload_source()));
    let evidence = workbench
        .activate_reload(reload)
        .expect("source reload activates before page-host rebind");

    assert_eq!(evidence.status(), ValidationReloadStatus::Activated);
    assert!(evidence.changed_facts().facts().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::PrimitiveInteraction
            && fact.identity() == "worth.surface.preview.primitive.proof"
    }));
    assert!(evidence.changed_facts().facts().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::AuthoredSurfaceProps
            && fact.identity() == "worth.surface.preview.primitive.proof"
    }));
    let (next_plan, receipt) = workbench
        .runtime()
        .rebind_page_host_after_reload(
            workbench.page_host_plan(),
            workbench.validation_page_host_request(),
            &evidence,
        )
        .expect("activated source evidence should rebind page host");

    assert_eq!(
        receipt.status(),
        WorthUiPageHostRebindStatus::ReboundAfterActivation
    );
    assert_eq!(receipt.previous_frame_digest(), before_digest);
    assert_ne!(receipt.rebound_frame_digest(), before_digest);
    assert_eq!(receipt.projection_rebuild_count(), 1);
    assert_eq!(
        receipt
            .projection_rebind_batch()
            .counters()
            .dependency_intersection_count(),
        1
    );
    assert_eq!(
        next_plan.execute_frame().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
}

#[test]
fn ready_but_unactivated_source_reload_cannot_rebind_page_host() {
    let workbench = runtime_workbench();
    let reload = workbench.prepare_reload(reload_request(&alternate_interaction_payload_source()));

    assert_eq!(
        reload.evidence().status(),
        ValidationReloadStatus::ReadyForFrameBoundary
    );
    let denial = workbench
        .runtime()
        .rebind_page_host_after_reload(
            workbench.page_host_plan(),
            workbench.validation_page_host_request(),
            reload.evidence(),
        )
        .expect_err("page host cannot rebind before runtime activation");

    assert_eq!(denial, WorthUiPageHostRebindDenial::ReloadNotActivated);
}

#[test]
fn workbench_reload_tick_updates_visible_page_host_projection() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.page_host_plan().frame_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::SourcePackage(ValidationSourcePackage::new(
            VALIDATION_SAMPLE_MODULE_PATH,
            alternate_interaction_payload_source(),
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded { evidence, .. } = outcome else {
        panic!("source package tick should produce source reload evidence");
    };
    assert_eq!(evidence.status(), ValidationReloadStatus::Activated);
    assert_ne!(workbench.page_host_plan().frame_digest(), before_digest);
    assert_eq!(
        workbench.page_host_plan().execute_frame().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
}

fn runtime_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
            ValidationSourcePackage::sample(),
        ))
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}

fn reload_request(source: &str) -> ValidationReloadRequest {
    ValidationReloadRequest::from_source_module(VALIDATION_SAMPLE_MODULE_PATH, source)
}

fn alternate_interaction_payload_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.page_host\"",
    )
}
