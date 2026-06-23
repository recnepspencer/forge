use worth_ui::facade::{
    AppearanceTokenId, DensityTokenId, WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus,
    WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus, WorthUiRebindPhaseLane,
    WorthUiRebindPhaseSelectionStatus, WorthUiRuntimeFactId,
};
use worth_ui_validation_app::header::applied_header_style_receipt;
use worth_ui_validation_app::reload::{
    ValidationAppearanceSource, ValidationDensitySource, ValidationReloadInput,
    ValidationReloadTick, ValidationRuntimeReloadTickOutcome, ValidationSourcePackage,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

#[test]
fn appearance_reload_updates_runtime_and_applied_header_style_receipt() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();
    let before_applied = current_applied_style(&workbench);

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearance(ValidationAppearanceSource::new(
            "validation.appearance.header.menu_min_width = 260px",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::AppearanceReloaded {
        evidence,
        phase_execution,
    } = outcome
    else {
        panic!("appearance reload should return typed capability evidence");
    };
    let phase_execution = phase_execution.expect("appearance reload should emit phase execution");
    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.touched_appearance_count(), 1);
    assert_eq!(evidence.changed_appearance_count(), 1);
    assert_eq!(evidence.canonicalization_count(), 1);
    assert_eq!(evidence.registry_lookup_count(), 1);
    assert_eq!(
        evidence.family_rebuild_breadth(),
        workbench
            .app()
            .capabilities()
            .appearance_tokens()
            .entries()
            .len()
    );
    assert!(evidence
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::appearance_token(
            &AppearanceTokenId::new("validation.appearance.header.menu_min_width").unwrap(),
        )));
    assert_ne!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_ne!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert_eq!(
        phase_execution.page_host_rebind().status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
    assert_phase_row(
        &phase_execution,
        WorthUiRebindPhaseLane::HeaderFrame,
        WorthUiRebindPhaseSelectionStatus::RebuildScheduled,
    );
    assert_phase_row(
        &phase_execution,
        WorthUiRebindPhaseLane::PageHost,
        WorthUiRebindPhaseSelectionStatus::PreservedWithoutIntersection,
    );

    let after_applied = current_applied_style(&workbench);
    assert_eq!(before_applied.menu_min_width_points(), 220.0);
    assert_eq!(after_applied.menu_min_width_points(), 260.0);
    assert_eq!(
        after_applied.control_spacing_points(),
        before_applied.control_spacing_points(),
        "appearance reload should not smear density authority"
    );
}

#[test]
fn density_reload_updates_runtime_and_applied_header_style_receipt() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();
    let before_applied = current_applied_style(&workbench);

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderDensity(ValidationDensitySource::new(
            "validation.density.header.control_spacing = 12px",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::DensityReloaded {
        evidence,
        phase_execution,
    } = outcome
    else {
        panic!("density reload should return typed capability evidence");
    };
    let phase_execution = phase_execution.expect("density reload should emit phase execution");
    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.touched_density_count(), 1);
    assert_eq!(evidence.changed_density_count(), 1);
    assert_eq!(evidence.canonicalization_count(), 1);
    assert_eq!(evidence.registry_lookup_count(), 1);
    assert_eq!(
        evidence.family_rebuild_breadth(),
        workbench
            .app()
            .capabilities()
            .density_tokens()
            .entries()
            .len()
    );
    assert!(evidence
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::density_token(
            &DensityTokenId::new("validation.density.header.control_spacing").unwrap(),
        )));
    assert_ne!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_ne!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert_eq!(
        phase_execution.page_host_rebind().status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
    assert_phase_row(
        &phase_execution,
        WorthUiRebindPhaseLane::HeaderFrame,
        WorthUiRebindPhaseSelectionStatus::RebuildScheduled,
    );
    assert_phase_row(
        &phase_execution,
        WorthUiRebindPhaseLane::PageHost,
        WorthUiRebindPhaseSelectionStatus::PreservedWithoutIntersection,
    );

    let after_applied = current_applied_style(&workbench);
    assert_eq!(before_applied.control_spacing_points(), 8.0);
    assert_eq!(after_applied.control_spacing_points(), 12.0);
    assert_eq!(
        after_applied.menu_min_width_points(),
        before_applied.menu_min_width_points(),
        "density reload should not seize appearance authority"
    );
}

#[test]
fn appearance_reload_rejects_wrong_value_kind_without_runtime_or_header_mutation() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearance(ValidationAppearanceSource::new(
            "validation.appearance.header.font_size = #102030",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::AppearanceReloaded {
        evidence,
        phase_execution,
        ..
    } = outcome
    else {
        panic!("denied appearance reload should still report capability evidence");
    };
    let phase_execution =
        phase_execution.expect("denied appearance reload should still emit phase execution");
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::AppearanceSourceParse)
    );
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
}

#[test]
fn density_reload_rejects_wrong_value_kind_without_runtime_or_header_mutation() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderDensity(ValidationDensitySource::new(
            "validation.density.header.control_spacing = compact",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::DensityReloaded {
        evidence,
        phase_execution,
        ..
    } = outcome
    else {
        panic!("denied density reload should still report capability evidence");
    };
    let phase_execution =
        phase_execution.expect("denied density reload should still emit phase execution");
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::DensitySourceParse)
    );
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
}

#[test]
fn equivalent_appearance_length_forms_are_proven_as_noop() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearance(ValidationAppearanceSource::new(
            "validation.appearance.header.menu_min_width = 220.0px",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::AppearanceReloaded {
        evidence,
        phase_execution,
        ..
    } = outcome
    else {
        panic!("equivalent appearance reload should still return typed evidence");
    };
    let phase_execution =
        phase_execution.expect("equivalent appearance reload should emit phase execution");
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::EquivalentNoOp
    );
    assert_eq!(evidence.touched_appearance_count(), 1);
    assert_eq!(evidence.changed_appearance_count(), 0);
    assert_eq!(evidence.canonicalization_count(), 1);
    assert!(evidence.changed_facts().is_empty());
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload
    );
    assert_eq!(
        phase_execution.header_rebind().projection_rebuild_count(),
        0,
        "equivalent appearance edits must not hide projection rebuild work"
    );
}

#[test]
fn equivalent_density_padding_forms_are_proven_as_noop() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderDensity(ValidationDensitySource::new(
            "validation.density.header.container_padding = 4px 8px 4px 8px",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::DensityReloaded {
        evidence,
        phase_execution,
        ..
    } = outcome
    else {
        panic!("equivalent density reload should still return typed evidence");
    };
    let phase_execution =
        phase_execution.expect("equivalent density reload should emit phase execution");
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::EquivalentNoOp
    );
    assert_eq!(evidence.touched_density_count(), 1);
    assert_eq!(evidence.changed_density_count(), 0);
    assert_eq!(evidence.canonicalization_count(), 1);
    assert!(evidence.changed_facts().is_empty());
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload
    );
    assert_eq!(
        phase_execution.header_rebind().projection_rebuild_count(),
        0,
        "equivalent density edits must not hide projection rebuild work"
    );
}

#[test]
fn conflicting_appearance_edits_are_parse_denied_before_snapshot_mutation() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearance(ValidationAppearanceSource::new(
            "\
validation.appearance.header.menu_min_width = 220px
validation.appearance.header.menu_min_width = 260px",
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::AppearanceReloaded { evidence, .. } = outcome else {
        panic!("conflicting appearance edits should still surface typed evidence");
    };
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::AppearanceSourceParse)
    );
    assert!(evidence
        .denial_detail()
        .expect("conflicting edits should preserve detail")
        .contains("conflicting appearance token edits"));
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
}

#[test]
fn stale_prepared_appearance_reload_cannot_overwrite_newer_density_truth() {
    let mut workbench = runtime_workbench();
    let stale = workbench.prepare_appearance_capability_reload(&ValidationAppearanceSource::new(
        "validation.appearance.header.menu_min_width = 260px",
    ));
    assert_eq!(
        stale.evidence().status(),
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
    );

    let newer = workbench.prepare_density_capability_reload(&ValidationDensitySource::new(
        "validation.density.header.control_spacing = 12px",
    ));
    let newer_evidence = workbench
        .activate_capability_reload(newer)
        .expect("newer density reload should activate first");
    assert_eq!(
        newer_evidence.status(),
        WorthUiCapabilityReloadStatus::Activated
    );

    assert_eq!(
        workbench
            .activate_capability_reload(stale)
            .expect_err("stale appearance reload must not overwrite newer density truth"),
        WorthUiCapabilityReloadStage::ActiveSnapshotDrift
    );
}

fn current_applied_style(
    workbench: &ValidationRuntimeWorkbench,
) -> worth_ui_validation_app::header::ValidationHeaderAppliedStyleReceipt {
    applied_header_style_receipt(
        workbench.header_frame_plan().theme_plan().execute_frame(),
        workbench
            .header_frame_plan()
            .appearance_plan()
            .execute_frame(),
    )
}

fn runtime_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
            ValidationSourcePackage::sample(),
        ))
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}

fn assert_phase_row(
    phase_execution: &worth_ui::facade::WorthUiRebindPhaseExecutionReceipt,
    lane: WorthUiRebindPhaseLane,
    status: WorthUiRebindPhaseSelectionStatus,
) {
    assert!(
        phase_execution
            .rows()
            .iter()
            .any(|row| row.lane() == lane && row.status() == status),
        "expected phase row {:?} -> {:?}, got {:?}",
        lane,
        status,
        phase_execution.rows()
    );
}
