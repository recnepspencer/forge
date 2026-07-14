use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LegacyDisposition {
    Owner,
    Removed,
}

struct LegacyCrateExpectation {
    package_name: &'static str,
    member_path: &'static str,
    disposition: LegacyDisposition,
}

const LEGACY_CRATE_EXPECTATIONS: &[LegacyCrateExpectation] = &[
    LegacyCrateExpectation {
        package_name: "worth-ui-components",
        member_path: "crates/worth-ui-components",
        disposition: LegacyDisposition::Owner,
    },
    LegacyCrateExpectation {
        package_name: "worth-ui-theme",
        member_path: "crates/worth-ui-theme",
        disposition: LegacyDisposition::Owner,
    },
    LegacyCrateExpectation {
        package_name: "worth-ui-adapters",
        member_path: "crates/worth-ui-adapters",
        disposition: LegacyDisposition::Removed,
    },
    LegacyCrateExpectation {
        package_name: "worth-ui-state",
        member_path: "crates/worth-ui-state",
        disposition: LegacyDisposition::Removed,
    },
    LegacyCrateExpectation {
        package_name: "worth-ui-types",
        member_path: "crates/worth-ui-types",
        disposition: LegacyDisposition::Removed,
    },
];

fn parse_toml(path: &Path) -> toml::Value {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{} should decode: {error}", path.display()))
        .parse::<toml::Value>()
        .unwrap_or_else(|error| panic!("{} should parse as TOML: {error}", path.display()))
}

fn workspace_manifest(workspace_root: &Path) -> toml::Value {
    parse_toml(&workspace_root.join("Cargo.toml"))
}

fn workspace_members(manifest: &toml::Value) -> BTreeSet<String> {
    manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn workspace_dependency_keys(manifest: &toml::Value) -> BTreeSet<String> {
    manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .into_iter()
        .flat_map(|table| table.keys())
        .cloned()
        .collect()
}

fn crate_dependency_keys(crate_manifest_path: &Path) -> BTreeSet<String> {
    let manifest = parse_toml(crate_manifest_path);
    ["dependencies", "dev-dependencies", "build-dependencies"]
        .into_iter()
        .filter_map(|section| manifest.get(section))
        .filter_map(toml::Value::as_table)
        .flat_map(|table| table.keys())
        .cloned()
        .collect()
}

fn read_source(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{} should decode: {error}", path.display()))
}

fn removed_legacy_crate_still_ships(
    workspace_root: &Path,
    expectation: &LegacyCrateExpectation,
) -> bool {
    let crate_root = workspace_root.join(expectation.member_path);
    crate_root.join("Cargo.toml").exists() || crate_root.join("src/lib.rs").exists()
}

fn surviving_public_surface_contains_forbidden_symbols(
    crate_root_source: &str,
    forbidden_symbols: &[&str],
) -> Vec<String> {
    forbidden_symbols
        .iter()
        .filter(|symbol| crate_root_source.contains(**symbol))
        .map(|symbol| (*symbol).to_string())
        .collect()
}

pub fn audit_legacy_crate_dispositions(workspace_root: &Path) -> Vec<String> {
    let manifest = workspace_manifest(workspace_root);
    let members = workspace_members(&manifest);
    let workspace_dependencies = workspace_dependency_keys(&manifest);
    let mut violations = Vec::new();

    for expectation in LEGACY_CRATE_EXPECTATIONS {
        let is_member = members.contains(expectation.member_path);
        let is_workspace_dependency = workspace_dependencies.contains(expectation.package_name);

        match expectation.disposition {
            LegacyDisposition::Owner => {
                if !is_member {
                    violations.push(format!(
                        "{} must remain an explicit workspace owner member",
                        expectation.package_name
                    ));
                }
                if !is_workspace_dependency {
                    violations.push(format!(
                        "{} must remain an explicit workspace dependency entry",
                        expectation.package_name
                    ));
                }
            }
            LegacyDisposition::Removed => {
                if is_member {
                    violations.push(format!(
                        "{} must be removed from workspace members",
                        expectation.package_name
                    ));
                }
                if is_workspace_dependency {
                    violations.push(format!(
                        "{} must be removed from workspace dependency entries",
                        expectation.package_name
                    ));
                }
                if removed_legacy_crate_still_ships(workspace_root, expectation) {
                    violations.push(format!(
                        "{} still ships as a path-dependable crate instead of being materially removed",
                        expectation.package_name
                    ));
                }
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_no_parallel_legacy_authority(workspace_root: &Path) -> Vec<String> {
    let mut violations = Vec::new();

    let owner_expectations = [
        (
            "worth-ui-components",
            workspace_root.join("crates/worth-ui-components/Cargo.toml"),
        ),
        (
            "worth-ui-theme",
            workspace_root.join("crates/worth-ui-theme/Cargo.toml"),
        ),
        (
            "worth-ui",
            workspace_root.join("crates/worth-ui/Cargo.toml"),
        ),
    ];

    for (crate_name, manifest_path) in owner_expectations {
        let dependency_keys = crate_dependency_keys(&manifest_path);
        for forbidden_legacy_dep in ["worth-ui-adapters", "worth-ui-state", "worth-ui-types"] {
            if dependency_keys.contains(forbidden_legacy_dep) {
                violations.push(format!(
                    "{crate_name} still depends on removed legacy crate `{forbidden_legacy_dep}`"
                ));
            }
        }
    }

    let component_root_source =
        read_source(&workspace_root.join("crates/worth-ui-components/src/lib.rs"));
    let component_forbidden_symbols = surviving_public_surface_contains_forbidden_symbols(
        &component_root_source,
        &[
            "UiFeature",
            "UiFeatureId",
            "UiFeatureKind",
            "UiPlane",
            "KernelTelemetry",
            "ChatMessage",
            "MessageRole",
            "MessageContent",
            "FeatureStatus",
            "AppState",
            "ModelVm",
            "ChatVm",
        ],
    );
    if !component_forbidden_symbols.is_empty() {
        violations.push(format!(
            "worth-ui-components crate root reintroduces legacy authority symbols: {}",
            component_forbidden_symbols.join(", ")
        ));
    }

    let theme_root_source = read_source(&workspace_root.join("crates/worth-ui-theme/src/lib.rs"));
    let theme_forbidden_symbols = surviving_public_surface_contains_forbidden_symbols(
        &theme_root_source,
        &[
            "UiFeature",
            "UiFeatureId",
            "UiFeatureKind",
            "UiPlane",
            "KernelTelemetry",
            "ChatMessage",
            "MessageRole",
            "MessageContent",
            "FeatureStatus",
            "AppState",
            "ModelVm",
            "ChatVm",
        ],
    );
    if !theme_forbidden_symbols.is_empty() {
        violations.push(format!(
            "worth-ui-theme crate root reintroduces legacy authority symbols: {}",
            theme_forbidden_symbols.join(", ")
        ));
    }

    let worth_ui_root_source = read_source(&workspace_root.join("crates/worth-ui/src/lib.rs"));
    if worth_ui_root_source.contains("worth_ui_") {
        violations
            .push("worth-ui crate root reintroduces worth-ui compatibility authority".to_string());
    }
    let worth_ui_public_lines: Vec<&str> = worth_ui_root_source
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("pub "))
        .collect();
    let admitted_worth_ui_public_lines = ["pub mod facade;"];
    for line in worth_ui_public_lines {
        if !admitted_worth_ui_public_lines.contains(&line) {
            violations.push(format!(
                "worth-ui crate root widened beyond the admitted facade-only export: {line}"
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_legacy_shim_honesty(workspace_root: &Path) -> Vec<String> {
    let mut violations = Vec::new();

    for retired_root in [
        workspace_root.join("crates/worth-ui-adapters"),
        workspace_root.join("crates/worth-ui-state"),
        workspace_root.join("crates/worth-ui-types"),
    ] {
        if !retired_root.exists() {
            continue;
        }

        let residual_files: Vec<String> = fs::read_dir(&retired_root)
            .unwrap_or_else(|error| {
                panic!("{} should be readable: {error}", retired_root.display())
            })
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .map(|path| path.display().to_string())
            .collect();

        if !residual_files.is_empty() {
            violations.push(format!(
                "{} still contains shim-like root files: {}",
                retired_root.display(),
                residual_files.join(", ")
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_legacy_public_surface_narrowing(workspace_root: &Path) -> Vec<String> {
    let manifest = workspace_manifest(workspace_root);
    let members = workspace_members(&manifest);
    let mut violations = Vec::new();

    for retired_member in [
        "crates/worth-ui-adapters",
        "crates/worth-ui-state",
        "crates/worth-ui-types",
    ] {
        if members.contains(retired_member) {
            violations.push(format!(
                "{retired_member} still ships as a workspace member instead of retiring its public surface"
            ));
        }
    }

    let component_dependencies =
        crate_dependency_keys(&workspace_root.join("crates/worth-ui-components/Cargo.toml"));
    if component_dependencies.contains("worth-ui-types") {
        violations.push(
            "worth-ui-components still widens the old worth-ui-types surface instead of owning its visual API directly"
                .to_string(),
        );
    }

    let theme_dependencies =
        crate_dependency_keys(&workspace_root.join("crates/worth-ui-theme/Cargo.toml"));
    if theme_dependencies.contains("worth-ui-types") {
        violations.push(
            "worth-ui-theme still widens the old worth-ui-types surface instead of owning theme truth directly"
                .to_string(),
        );
    }

    violations.sort();
    violations.dedup();
    violations
}
