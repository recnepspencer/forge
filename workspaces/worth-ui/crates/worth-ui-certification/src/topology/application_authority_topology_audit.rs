use super::WorkspaceSourceInventory;

const ORDINARY_AUTHORITY_LANES: &[(&str, &str)] = &[
    (
        "crates/worth-ui-runtime/src/facade/lifecycle/freeze.rs",
        "pub(crate) fn prepare_application_authority(",
    ),
    (
        "crates/worth-ui-runtime/src/facade/entry/app.rs",
        "pub fn launch(",
    ),
    (
        "crates/worth-ui-runtime/src/facade/host_session_authority.rs",
        "pub(crate) fn activate(",
    ),
    (
        "crates/worth-ui-runtime/src/facade/entry/application_replacement.rs",
        "pub fn activate_prepared_replacement(",
    ),
];

const FORBIDDEN_ORDINARY_SURFACES: &[&str] = &[
    "pub fn launch_runtime(",
    "pub fn from_canonical_artifact(",
    "pub fn into_candidate(",
    "pub fn freeze_infallibly(",
];

pub fn audit_application_authority_topology(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let mut findings = Vec::new();
    for (path, signature) in ORDINARY_AUTHORITY_LANES {
        let occurrences = inventory.text(path).matches(signature).count();
        if occurrences != 1 {
            findings.push(format!(
                "{path} must own exactly one `{}` lane; found {occurrences}",
                signature.trim()
            ));
        }
    }

    for source in inventory.rust_files_under("crates/worth-ui-runtime/src") {
        if is_support_or_test_source(source.relative_path()) {
            continue;
        }
        for forbidden in FORBIDDEN_ORDINARY_SURFACES {
            if source.text().contains(forbidden) {
                findings.push(format!(
                    "{} exposes removed ordinary authority surface `{forbidden}`",
                    source.relative_path().display()
                ));
            }
        }
    }
    findings.sort();
    findings
}

fn is_support_or_test_source(path: &std::path::Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.contains("/certification_support/")
        || normalized.contains("/tests/")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with("_test_support.rs")
}
