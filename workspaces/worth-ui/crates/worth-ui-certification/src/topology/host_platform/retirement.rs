use std::path::Path;

use super::destination;

const RETIRED_SOURCE_SURFACES: [&str; 13] = [
    "worth-ui-host-egui",
    "worth_ui_host_egui",
    "worth-ui-theme",
    "worth_ui_theme",
    "worth-ui-components",
    "worth_ui_components",
    "WorthUiHostKind::Egui",
    "WorthUiHostContract::egui",
    "WorthUiLegacyEguiApplicationTransition",
    "UiHostMigrationGrant",
    "legacy-egui-migration",
    "eframe::",
    "egui::",
];

const RETIRED_PATH_SURFACES: [&str; 6] = [
    "egui",
    "eframe",
    "worth-ui-theme",
    "worth_ui_theme",
    "worth-ui-components",
    "worth_ui_components",
];

pub(super) fn audit_path(relative: &Path, failures: &mut Vec<String>) {
    if destination::is_exact_negative_fixture(relative)
        || destination::is_historical_record(relative)
        || destination::is_detector_source(relative)
    {
        return;
    }
    let normalized = destination::normalize(relative);
    let lowercase = normalized.to_ascii_lowercase();
    for retired in RETIRED_PATH_SURFACES {
        if lowercase.contains(retired) {
            failures.push(format!(
                "{normalized} retains retired path identity `{retired}`"
            ));
        }
    }
}

pub(super) fn audit_source(relative: &Path, text: &str, failures: &mut Vec<String>) {
    if destination::is_exact_negative_fixture(relative)
        || destination::is_historical_record(relative)
        || destination::is_detector_source(relative)
    {
        return;
    }
    for retired in RETIRED_SOURCE_SURFACES {
        if text.contains(retired) {
            failures.push(format!(
                "{} retains retired source surface `{retired}`",
                destination::normalize(relative)
            ));
        }
    }
    if destination::is_current_native_vision(relative) {
        let lowercase = text.to_ascii_lowercase();
        for retired in ["egui", "eframe"] {
            if lowercase.contains(retired) {
                failures.push(format!(
                    "{} grants current vision space to retired `{retired}` authority",
                    destination::normalize(relative)
                ));
            }
        }
    }
}

pub(super) fn audit_compile_twins(fixture_manifest: &str, cases: &str, failures: &mut Vec<String>) {
    for required in [
        destination::POSITIVE_COMPILE_TWIN,
        destination::NEGATIVE_COMPILE_TWIN,
    ] {
        let file_name = required
            .rsplit('/')
            .next()
            .expect("compile twin has a file name");
        if !fixture_manifest.contains(file_name) {
            failures.push(format!("compile fixture omits `{file_name}`"));
        }
        if !cases.contains(required) {
            failures.push(format!("compile case inventory omits `{required}`"));
        }
    }
    if !cases.contains(&format!("pass,{}", destination::POSITIVE_COMPILE_TWIN)) {
        failures.push("headless compile twin is not a positive case".to_owned());
    }
    if !cases.contains(&format!("fail,{}", destination::NEGATIVE_COMPILE_TWIN)) {
        failures.push("retired-surface compile twin is not a negative case".to_owned());
    }
}

pub(super) fn mutation_controls() -> Vec<String> {
    let mut failures = Vec::new();
    for path in [
        Path::new(
            "workspaces/worth-ui/crates/worth-ui-certification/tests/fixtures/topology_negative/unregistered_retired_host.rs",
        ),
        Path::new("unregistered/tests/ui/host/retired_egui_surface_is_absent.rs"),
    ] {
        audit_path(path, &mut failures);
        audit_source(
            path,
            "fn main() { let _ = egui::Context::default(); }",
            &mut failures,
        );
    }
    failures
}
