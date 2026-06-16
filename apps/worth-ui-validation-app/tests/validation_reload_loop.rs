use std::fs;
use std::path::PathBuf;

use worth_ui::facade::{WorthUiCapabilityReloadStatus, WorthUiHeaderFrameRebindStatus};
use worth_ui_validation_app::reload::{
    ValidationReloadEvidenceEntry, ValidationReloadEvidenceLog, ValidationReloadInput,
    ValidationReloadInputDenial, ValidationReloadLoop, ValidationReloadLoopConfig,
    ValidationReloadStatus, ValidationReloadTick, ValidationRuntimeReloadTickOutcome,
};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::ValidationWorkbenchLaunch;

#[test]
fn reload_loop_ignores_unchanged_source_and_theme_inputs() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();

    let first_tick = reload_loop.poll_inputs();
    let second_tick = reload_loop.poll_inputs();

    assert!(matches!(first_tick, ValidationReloadTick::Unchanged(_)));
    assert!(matches!(second_tick, ValidationReloadTick::Unchanged(_)));
}

#[test]
fn reload_loop_reports_source_and_theme_changes_as_one_typed_tick() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fixture.write_source(&meaningfully_changed_source());
    fixture.write_theme(
        "# Worth UI validation header theme.\nvalidation.theme.header.panel = #102030\n",
    );

    let tick = reload_loop.poll_inputs();

    let ValidationReloadTick::Changed(ValidationReloadInput::SourcePackageAndHeaderTheme {
        source,
        theme,
    }) = tick
    else {
        panic!("source and theme edits should be represented as one combined reload input");
    };
    assert_eq!(source.module_path(), VALIDATION_SAMPLE_MODULE_PATH);
    assert_ne!(source.source_text(), VALIDATION_SAMPLE_SOURCE);
    assert!(theme.source_text().contains("#102030"));
}

#[test]
fn unreadable_source_input_is_reported_as_typed_denial() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fixture.delete_source();

    let tick = reload_loop.poll_inputs();

    let ValidationReloadTick::Unreadable(denial) = tick else {
        panic!("missing source must be reported as a typed unreadable input denial");
    };
    assert_eq!(denial.path(), &fixture.source_path);
    assert!(!denial.reason().is_empty());
}

#[test]
fn unreadable_tick_reaches_runtime_outcome_without_mutating_header() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let denial = ValidationReloadInputDenial::unreadable(
        "missing-header.wui",
        &std::io::Error::new(std::io::ErrorKind::NotFound, "missing source"),
    );

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Unreadable(denial.clone()));

    assert_eq!(
        outcome,
        ValidationRuntimeReloadTickOutcome::InputUnreadable(denial)
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_digest);
}

#[test]
fn invalid_source_change_preserves_active_runtime_and_header_frame() {
    let mut workbench = runtime_workbench();
    let before_active = workbench.runtime().inspect_active();
    let before_header = workbench.header_frame_plan().frame_digest();
    let source = worth_ui_validation_app::reload::ValidationSourcePackage::new(
        VALIDATION_SAMPLE_MODULE_PATH,
        "app Broken { workspace Missing",
    );

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::SourcePackage(source),
    ));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded {
        evidence,
        header_receipt,
    } = outcome
    else {
        panic!("invalid source still returns runtime reload evidence");
    };
    assert!(matches!(
        evidence.status(),
        worth_ui_validation_app::reload::ValidationReloadStatus::Denied(_)
    ));
    assert_eq!(workbench.runtime().inspect_active(), before_active);
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        header_receipt
            .expect("denied reload should preserve header through a receipt")
            .status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
}

#[test]
fn source_change_activates_then_rebinds_header_through_workbench() {
    let mut workbench = runtime_workbench();
    let before_active = workbench.runtime().inspect_active();
    let source = worth_ui_validation_app::reload::ValidationSourcePackage::new(
        VALIDATION_SAMPLE_MODULE_PATH,
        meaningfully_changed_source(),
    );

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::SourcePackage(source),
    ));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded {
        evidence,
        header_receipt,
    } = outcome
    else {
        panic!("meaningful source change should activate through workbench");
    };
    assert_eq!(
        evidence.status(),
        worth_ui_validation_app::reload::ValidationReloadStatus::Activated
    );
    assert_ne!(workbench.runtime().inspect_active(), before_active);
    let receipt = header_receipt.expect("activated reload should rebind header frame");
    assert_eq!(
        receipt.status(),
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
    );
    assert_eq!(receipt.source_parse_count(), 0);
    assert_eq!(receipt.registry_lookup_count(), 0);
    assert_eq!(receipt.artifact_tree_scan_count(), 0);
}

#[test]
fn combined_source_and_theme_outcome_keeps_both_evidence_entries_visible() {
    let mut workbench = runtime_workbench();
    let source = worth_ui_validation_app::reload::ValidationSourcePackage::new(
        VALIDATION_SAMPLE_MODULE_PATH,
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
        header_rebind_status,
        ..
    } = runtime_entry
    else {
        panic!("source evidence should be recorded before theme denial");
    };
    assert_eq!(*status, ValidationReloadStatus::Activated);
    assert_eq!(
        *header_rebind_status,
        Some(WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation)
    );
    let ValidationReloadEvidenceEntry::ThemeReload {
        status,
        header_rebind_status,
        touched_theme_token_count,
        ..
    } = theme_entry
    else {
        panic!("theme capability evidence should remain visible after source activation evidence");
    };
    assert_eq!(*status, WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(
        *header_rebind_status,
        Some(WorthUiHeaderFrameRebindStatus::ReboundAfterActivation)
    );
    assert_eq!(
        *touched_theme_token_count, 1,
        "single-token edit should report the exact runtime theme delta size"
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
        &PathBuf::from("missing-8.wui")
    );
    let Some(ValidationReloadEvidenceEntry::InputUnreadable(denial)) = evidence_log.latest() else {
        panic!("latest evidence should preserve the most recent unreadable input denial");
    };
    assert_eq!(denial.path(), &PathBuf::from("missing-39.wui"));
}

#[test]
fn packaged_validation_source_matches_embedded_launch_source() {
    let packaged_source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source/header.wui");

    let packaged_source =
        fs::read_to_string(packaged_source_path).expect("packaged source should be readable");

    assert_eq!(packaged_source, VALIDATION_SAMPLE_SOURCE);
}

fn runtime_workbench() -> worth_ui_validation_app::ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}

fn meaningfully_changed_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "proof -> validation.surface.header.proof",
        "proof -> validation.surface.header.proof.alt",
    )
}

struct ReloadLoopFixture {
    source_path: PathBuf,
    theme_path: PathBuf,
}

impl ReloadLoopFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-ui-validation-reload-loop-{}",
            unique_suffix()
        ));
        fs::create_dir_all(&root).expect("fixture root should be created");
        let source_path = root.join("header.wui");
        let theme_path = root.join("header.theme");
        fs::write(&source_path, VALIDATION_SAMPLE_SOURCE)
            .expect("source fixture should be written");
        fs::write(&theme_path, "# Worth UI validation header theme.\n")
            .expect("theme fixture should be written");
        Self {
            source_path,
            theme_path,
        }
    }

    fn start_loop(&self) -> ValidationReloadLoop {
        ValidationReloadLoop::start(
            ValidationReloadLoopConfig::new(&self.theme_path).with_source_path(&self.source_path),
        )
        .expect("reload loop should start from readable fixture files")
    }

    fn write_source(&self, source_text: &str) {
        fs::write(&self.source_path, source_text).expect("source fixture should be writable");
    }

    fn write_theme(&self, theme_text: &str) {
        fs::write(&self.theme_path, theme_text).expect("theme fixture should be writable");
    }

    fn delete_source(&self) {
        fs::remove_file(&self.source_path).expect("source fixture should be removable");
    }
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos()
}
