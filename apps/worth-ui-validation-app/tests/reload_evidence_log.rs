use worth_ui::facade::{
    AppearanceTokenId, DensityTokenId, ThemeTokenId, WorthUiCapabilityReloadStatus,
    WorthUiHeaderFrameRebindStatus, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
};
use worth_ui_validation_app::reload::{
    ValidationCommandProjectionSource, ValidationCommandSource, ValidationReloadEvidenceEntry,
    ValidationReloadEvidenceLog, ValidationReloadInput, ValidationReloadInputDenial,
    ValidationReloadStatus, ValidationReloadTick, ValidationRuntimeReloadTickOutcome,
};

mod validation_reload_loop_support;

use validation_reload_loop_support::{
    meaningfully_changed_source, runtime_workbench, SAMPLE_MODULE_PATH,
};

#[test]
fn combined_appearance_and_density_outcome_keeps_both_family_entries_visible() {
    let mut workbench = runtime_workbench();
    let mut evidence_log = ValidationReloadEvidenceLog::default();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearanceAndDensity {
            appearance:
                worth_ui_validation_app::reload::ValidationAppearanceSource::from_observed_file(
                    "apps/worth-ui-validation-app/theme/header.appearance",
                    "validation.appearance.header.menu_min_width = 260px",
                ),
            density: worth_ui_validation_app::reload::ValidationDensitySource::from_observed_file(
                "apps/worth-ui-validation-app/theme/header.density",
                "validation.density.header.control_spacing = 12px",
            ),
        },
    ));

    evidence_log.record_runtime_reload_tick_outcome(outcome);

    assert_eq!(evidence_log.entries().len(), 2);
    assert!(matches!(
        evidence_log.entries()[0],
        ValidationReloadEvidenceEntry::AppearanceReload { .. }
    ));
    assert!(matches!(
        evidence_log.entries()[1],
        ValidationReloadEvidenceEntry::DensityReload { .. }
    ));
}

#[test]
fn combined_source_and_theme_outcome_keeps_both_evidence_entries_visible() {
    let mut workbench = runtime_workbench();
    let source = worth_ui_validation_app::reload::ValidationSourcePackage::new(
        SAMPLE_MODULE_PATH,
        meaningfully_changed_source(),
    );
    let theme = worth_ui_validation_app::reload::ValidationThemeSource::new(
        "validation.theme.header.panel = #102030",
    );
    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::SourcePackageAndHeaderTheme { source, theme },
    ));
    let mut evidence_log = ValidationReloadEvidenceLog::default();

    evidence_log.record_runtime_reload_tick_outcome(outcome);

    assert_eq!(evidence_log.entries().len(), 2);
    let [runtime_entry, theme_entry] = evidence_log.entries() else {
        panic!("combined reload should record source and theme runtime evidence");
    };
    let ValidationReloadEvidenceEntry::RuntimeReload {
        status,
        header_rebind,
        ..
    } = runtime_entry
    else {
        panic!("source evidence should be recorded before theme denial");
    };
    assert_eq!(*status, ValidationReloadStatus::Activated);
    let header_rebind = header_rebind
        .as_ref()
        .expect("runtime evidence should retain full header rebind proof");
    assert_eq!(
        header_rebind.status(),
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
    );
    let ValidationReloadEvidenceEntry::ThemeReload {
        status,
        header_rebind,
        touched_theme_token_count,
        changed_fact_count,
        changed_facts,
        ..
    } = theme_entry
    else {
        panic!("theme capability evidence should remain visible after source activation evidence");
    };
    assert_eq!(*status, WorthUiCapabilityReloadStatus::Activated);
    let header_rebind = header_rebind
        .as_ref()
        .expect("theme evidence should retain full header rebind proof");
    assert_eq!(
        header_rebind.status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert_eq!(*touched_theme_token_count, 1);
    assert_eq!(*changed_fact_count, 1);
    assert_eq!(
        changed_facts,
        &vec![WorthUiRuntimeFactId::theme_token(
            &ThemeTokenId::new("validation.theme.header.panel").unwrap(),
        )]
    );
}

#[test]
fn reload_evidence_log_reports_family_specific_capability_delta_widths() {
    let mut workbench = runtime_workbench();
    let mut evidence_log = ValidationReloadEvidenceLog::default();

    let command_outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderCommands(ValidationCommandSource::new(
            "\
validation.command.file.new = Create File
validation.command.help.docs = Worth UI Docs",
        )),
    ));
    evidence_log.record_runtime_reload_tick_outcome(command_outcome);
    let Some(ValidationReloadEvidenceEntry::CommandReload {
        touched_command_count,
        changed_fact_count,
        changed_facts,
        ..
    }) = evidence_log.latest()
    else {
        panic!("command reload should be recorded as command-family evidence");
    };
    assert_eq!(*touched_command_count, 2);
    assert_eq!(*changed_fact_count, 2);
    assert!(changed_facts
        .iter()
        .all(|fact| matches!(fact.family(), WorthUiRuntimeFactFamily::Command)));

    let projection_outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderCommandProjections(ValidationCommandProjectionSource::new(
            "validation.header.menu.file = multi",
        )),
    ));
    evidence_log.record_runtime_reload_tick_outcome(projection_outcome);
    let Some(ValidationReloadEvidenceEntry::CommandProjectionReload {
        touched_projection_count,
        changed_fact_count,
        changed_facts,
        ..
    }) = evidence_log.latest()
    else {
        panic!("command projection reload should be recorded as projection-family evidence");
    };
    assert_eq!(*touched_projection_count, 1);
    assert_eq!(*changed_fact_count, 2);
    assert!(changed_facts.iter().all(|fact| matches!(
        fact.family(),
        WorthUiRuntimeFactFamily::CommandProjection | WorthUiRuntimeFactFamily::InteractionPolicy
    )));

    let appearance_outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearance(
            worth_ui_validation_app::reload::ValidationAppearanceSource::new(
                "validation.appearance.header.menu_min_width = 260px",
            ),
        ),
    ));
    evidence_log.record_runtime_reload_tick_outcome(appearance_outcome);
    let Some(ValidationReloadEvidenceEntry::AppearanceReload {
        touched_appearance_count,
        changed_fact_count,
        changed_appearance_count,
        canonicalization_count,
        descriptor_lookup_count,
        family_rebuild_breadth,
        changed_facts,
        ..
    }) = evidence_log.latest()
    else {
        panic!("appearance reload should be recorded as appearance-family evidence");
    };
    assert_eq!(*touched_appearance_count, 1);
    assert_eq!(*changed_appearance_count, 1);
    assert_eq!(*canonicalization_count, 1);
    assert_eq!(*descriptor_lookup_count, 1);
    assert_eq!(*family_rebuild_breadth, 4);
    assert_eq!(*changed_fact_count, 1);
    assert_eq!(
        changed_facts,
        &vec![WorthUiRuntimeFactId::appearance_token(
            &AppearanceTokenId::new("validation.appearance.header.menu_min_width").unwrap(),
        )]
    );

    let density_outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderDensity(
            worth_ui_validation_app::reload::ValidationDensitySource::new(
                "validation.density.header.control_spacing = 12px",
            ),
        ),
    ));
    evidence_log.record_runtime_reload_tick_outcome(density_outcome);
    let Some(ValidationReloadEvidenceEntry::DensityReload {
        touched_density_count,
        changed_fact_count,
        changed_density_count,
        canonicalization_count,
        descriptor_lookup_count,
        family_rebuild_breadth,
        changed_facts,
        ..
    }) = evidence_log.latest()
    else {
        panic!("density reload should be recorded as density-family evidence");
    };
    assert_eq!(*touched_density_count, 1);
    assert_eq!(*changed_density_count, 1);
    assert_eq!(*canonicalization_count, 1);
    assert_eq!(*descriptor_lookup_count, 1);
    assert_eq!(*family_rebuild_breadth, 19);
    assert_eq!(*changed_fact_count, 1);
    assert_eq!(
        changed_facts,
        &vec![WorthUiRuntimeFactId::density_token(
            &DensityTokenId::new("validation.density.header.control_spacing").unwrap(),
        )]
    );
}

#[test]
fn unchanged_outcome_does_not_pollute_reload_evidence_log() {
    let mut evidence_log = ValidationReloadEvidenceLog::default();

    evidence_log.record_runtime_reload_tick_outcome(ValidationRuntimeReloadTickOutcome::Unchanged(
        worth_ui_validation_app::reload::ValidationReloadObservation::new(11, 29),
    ));

    assert!(evidence_log.entries().is_empty());
}

#[test]
fn reload_evidence_log_keeps_unreadable_failures_visible_and_bounded() {
    let mut evidence_log = ValidationReloadEvidenceLog::default();
    for index in 0..40 {
        let denial = ValidationReloadInputDenial::unreadable(
            format!("missing-{index}.wui"),
            &std::io::Error::new(std::io::ErrorKind::NotFound, "missing source"),
        );
        evidence_log.record_input_unreadable(denial);
    }

    assert_eq!(evidence_log.entries().len(), 32);
    let Some(ValidationReloadEvidenceEntry::InputUnreadable(first_retained_denial)) =
        evidence_log.entries().first()
    else {
        panic!("bounded evidence log should keep entries as typed unreadable denials");
    };
    assert_eq!(
        first_retained_denial.path(),
        &std::path::PathBuf::from("missing-8.wui")
    );
    let Some(ValidationReloadEvidenceEntry::InputUnreadable(denial)) = evidence_log.latest() else {
        panic!("latest evidence should preserve the most recent unreadable input denial");
    };
    assert_eq!(denial.path(), &std::path::PathBuf::from("missing-39.wui"));
}
