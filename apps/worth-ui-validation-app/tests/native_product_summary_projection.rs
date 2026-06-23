use worth_ui::facade::{
    AppearanceTokenId, DensityTokenId, WorthUiCapabilityReloadStatus, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId,
};
use worth_ui_validation_app::pages::product_summary::{
    ValidationProductSummaryDenialStatus, ValidationProductSummaryEvidenceKind,
    ValidationProductSummaryEvidenceStatus, ValidationProductSummaryProjection,
    ValidationProductSummaryRenderPlan,
};
use worth_ui_validation_app::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationDensitySource, ValidationReloadInput, ValidationReloadRequest,
    ValidationReloadStatus, ValidationReloadTick, ValidationRuntimeReloadTickOutcome,
    ValidationSourcePackage,
};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

#[test]
fn launch_product_summary_projects_runtime_and_page_host_receipts() {
    let workbench = runtime_workbench();
    let plan = product_summary_plan(&workbench, None);

    assert_eq!(plan.page_name(), "HeaderProofPage");
    assert_eq!(plan.slots().len(), 1);
    assert_eq!(plan.slots()[0].slot_name(), "button_proof");
    assert_eq!(
        plan.slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        plan.page_host_frame_digest(),
        workbench.page_host_plan().frame_digest()
    );
    assert_eq!(
        plan.active_artifact_digest(),
        workbench.runtime().inspect_active().artifact_digest()
    );
    assert_eq!(
        plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::LaunchReceipt
    );
}

#[test]
fn activated_source_reload_updates_product_summary_slot_from_page_host_receipt() {
    let mut workbench = runtime_workbench();
    let outcome = workbench.apply_reload_tick(source_tick(alternate_interaction_payload_source()));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded { evidence, .. } = &outcome else {
        panic!("source package tick should produce source reload evidence");
    };
    assert_eq!(evidence.status(), ValidationReloadStatus::Activated);
    let query_bindings_compared = evidence.query_bindings_compared();
    let query_rebind_entries = evidence.query_rebind_entries();
    let changed_fact_count = evidence.changed_facts().len();

    let mut evidence_log = worth_ui_validation_app::reload::ValidationReloadEvidenceLog::default();
    evidence_log.record_runtime_reload_tick_outcome(outcome);
    let plan = product_summary_plan(&workbench, evidence_log.latest());

    assert_eq!(
        plan.slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::RuntimeReload
    );
    assert_eq!(
        plan.evidence().status(),
        &ValidationProductSummaryEvidenceStatus::RuntimeReload(ValidationReloadStatus::Activated)
    );
    assert_eq!(plan.evidence().changed_fact_count(), changed_fact_count);
    assert!(plan.evidence().changed_facts().iter().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::PrimitiveInteraction
            && fact.identity() == "worth.surface.preview.primitive.proof"
    }));
    assert!(plan.evidence().changed_facts().iter().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::AuthoredSurfaceProps
            && fact.identity() == "worth.surface.preview.primitive.proof"
    }));
    assert_eq!(
        plan.evidence().query_bindings_compared(),
        query_bindings_compared
    );
    assert_eq!(plan.evidence().query_rebind_entries(), query_rebind_entries);
}

#[test]
fn denied_source_reload_preserves_product_summary_slot_and_projects_denial_evidence() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.page_host_plan().frame_digest();
    let outcome = workbench.apply_reload_tick(source_tick("app Broken { workspace Missing"));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded { evidence, .. } = &outcome else {
        panic!("denied source package still records source reload evidence");
    };
    assert!(matches!(
        evidence.status(),
        ValidationReloadStatus::Denied(_)
    ));

    let mut evidence_log = worth_ui_validation_app::reload::ValidationReloadEvidenceLog::default();
    evidence_log.record_runtime_reload_tick_outcome(outcome);
    let plan = product_summary_plan(&workbench, evidence_log.latest());

    assert_eq!(workbench.page_host_plan().frame_digest(), before_digest);
    assert_eq!(
        plan.slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::RuntimeReload
    );
    assert!(matches!(
        plan.evidence().status(),
        ValidationProductSummaryEvidenceStatus::RuntimeReload(ValidationReloadStatus::Denied(_))
    ));
    assert_eq!(plan.evidence().changed_fact_count(), 0);
}

#[test]
fn command_reload_projects_family_specific_evidence_into_product_summary() {
    let mut workbench = runtime_workbench();
    let outcome = workbench.apply_reload_tick(command_tick(
        "\
validation.command.file.new = Create File
validation.command.help.docs = Worth UI Docs",
    ));
    let mut evidence_log = worth_ui_validation_app::reload::ValidationReloadEvidenceLog::default();

    evidence_log.record_runtime_reload_tick_outcome(outcome);
    let plan = product_summary_plan(&workbench, evidence_log.latest());

    assert_eq!(
        plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::CommandReload
    );
    assert_eq!(
        plan.evidence().status(),
        &ValidationProductSummaryEvidenceStatus::CapabilityReload(
            WorthUiCapabilityReloadStatus::Activated
        )
    );
    assert_eq!(plan.evidence().touched_count(), Some(2));
    assert_eq!(plan.evidence().changed_fact_count(), 2);
    assert!(plan
        .evidence()
        .changed_facts()
        .iter()
        .all(|fact| fact.family() == WorthUiRuntimeFactFamily::Command));
}

#[test]
fn command_projection_reload_projects_selection_policy_evidence_into_product_summary() {
    let mut workbench = runtime_workbench();
    let outcome = workbench.apply_reload_tick(command_projection_tick(
        "\
validation.header.menu.file = multi",
    ));
    let mut evidence_log = worth_ui_validation_app::reload::ValidationReloadEvidenceLog::default();

    evidence_log.record_runtime_reload_tick_outcome(outcome);
    let plan = product_summary_plan(&workbench, evidence_log.latest());

    assert_eq!(
        plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::CommandProjectionReload
    );
    assert_eq!(
        plan.evidence().status(),
        &ValidationProductSummaryEvidenceStatus::CapabilityReload(
            WorthUiCapabilityReloadStatus::Activated
        )
    );
    assert_eq!(plan.evidence().touched_count(), Some(1));
    assert_eq!(plan.evidence().changed_fact_count(), 2);
    assert!(plan.evidence().changed_facts().iter().all(|fact| matches!(
        fact.family(),
        WorthUiRuntimeFactFamily::CommandProjection | WorthUiRuntimeFactFamily::InteractionPolicy
    )));
}

#[test]
fn appearance_and_density_reload_project_family_specific_evidence_into_product_summary() {
    let mut workbench = runtime_workbench();
    let mut evidence_log = worth_ui_validation_app::reload::ValidationReloadEvidenceLog::default();

    evidence_log.record_runtime_reload_tick_outcome(workbench.apply_reload_tick(
        ValidationReloadTick::Changed(ValidationReloadInput::HeaderAppearance(
            ValidationAppearanceSource::new("validation.appearance.header.menu_min_width = 260px"),
        )),
    ));
    let appearance_plan = product_summary_plan(&workbench, evidence_log.latest());
    assert_eq!(
        appearance_plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::AppearanceReload
    );
    assert_eq!(appearance_plan.evidence().touched_count(), Some(1));
    assert_eq!(appearance_plan.evidence().changed_fact_count(), 1);
    assert_eq!(
        appearance_plan.evidence().changed_facts(),
        &[WorthUiRuntimeFactId::appearance_token(
            &AppearanceTokenId::new("validation.appearance.header.menu_min_width").unwrap(),
        )]
    );

    evidence_log.record_runtime_reload_tick_outcome(workbench.apply_reload_tick(
        ValidationReloadTick::Changed(ValidationReloadInput::HeaderDensity(
            ValidationDensitySource::new("validation.density.header.control_spacing = 12px"),
        )),
    ));
    let density_plan = product_summary_plan(&workbench, evidence_log.latest());
    assert_eq!(
        density_plan.evidence().kind(),
        ValidationProductSummaryEvidenceKind::DensityReload
    );
    assert_eq!(density_plan.evidence().touched_count(), Some(1));
    assert_eq!(density_plan.evidence().changed_fact_count(), 1);
    assert_eq!(
        density_plan.evidence().changed_facts(),
        &[WorthUiRuntimeFactId::density_token(
            &DensityTokenId::new("validation.density.header.control_spacing").unwrap(),
        )]
    );
}

#[test]
fn denied_activation_evidence_remains_typed_in_product_summary() {
    let workbench = runtime_workbench();
    let mut evidence_log = worth_ui_validation_app::reload::ValidationReloadEvidenceLog::default();

    evidence_log.record_source_activation_denial(
        worth_ui_validation_app::reload::ValidationReloadStage::RuntimeInstanceMismatch,
    );
    let plan = product_summary_plan(&workbench, evidence_log.latest());

    assert_eq!(
        plan.evidence().status(),
        &ValidationProductSummaryEvidenceStatus::Denial(
            ValidationProductSummaryDenialStatus::SourceActivationDenied(
                worth_ui_validation_app::reload::ValidationReloadStage::RuntimeInstanceMismatch
            )
        )
    );
}

#[test]
fn render_plan_is_read_only_over_product_summary_projection() {
    let workbench = runtime_workbench();
    let projection = ValidationProductSummaryProjection::from_runtime_receipts(
        workbench.runtime().inspect_active(),
        workbench.page_host_plan(),
        None,
    );
    let plan = ValidationProductSummaryRenderPlan::from_projection(projection);

    assert_eq!(plan.slots()[0].slot_name(), "button_proof");
    assert_eq!(plan.evidence().query_bindings_compared(), 0);
    assert_eq!(
        plan.capability_snapshot_digest(),
        workbench.runtime().inspect_active().snapshot_digest()
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

fn product_summary_plan(
    workbench: &ValidationRuntimeWorkbench,
    latest: Option<&worth_ui_validation_app::reload::ValidationReloadEvidenceEntry>,
) -> ValidationProductSummaryRenderPlan {
    ValidationProductSummaryRenderPlan::from_projection(
        ValidationProductSummaryProjection::from_runtime_receipts(
            workbench.runtime().inspect_active(),
            workbench.page_host_plan(),
            latest,
        ),
    )
}

fn source_tick(source: impl Into<String>) -> ValidationReloadTick {
    ValidationReloadTick::Changed(ValidationReloadInput::SourcePackage(
        ValidationSourcePackage::new(VALIDATION_SAMPLE_MODULE_PATH, source),
    ))
}

fn command_tick(source: impl Into<String>) -> ValidationReloadTick {
    ValidationReloadTick::Changed(ValidationReloadInput::HeaderCommands(
        ValidationCommandSource::new(source),
    ))
}

fn command_projection_tick(source: impl Into<String>) -> ValidationReloadTick {
    ValidationReloadTick::Changed(ValidationReloadInput::HeaderCommandProjections(
        ValidationCommandProjectionSource::new(source),
    ))
}

fn alternate_interaction_payload_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.product_summary\"",
    )
}

#[allow(dead_code)]
fn reload_request(source: &str) -> ValidationReloadRequest {
    ValidationReloadRequest::from_source_module(VALIDATION_SAMPLE_MODULE_PATH, source)
}
