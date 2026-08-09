use std::collections::{BTreeMap, BTreeSet};

use super::{repository_document, workspace_source_inventory};

type Manifests = BTreeMap<String, String>;

#[path = "milestone_3141_phase1_topology/authority_residue.rs"]
mod authority_residue;
#[path = "milestone_3141_phase1_topology/pulse_text.rs"]
mod pulse_text;
#[path = "milestone_3141_phase1_topology/repository_manifests.rs"]
mod repository_manifests;
#[path = "milestone_3141_phase1_topology/resolved_graphs.rs"]
mod resolved_graphs;

#[test]
fn phase_one_host_platform_topology_verdict_covers_every_workspace_manifest() {
    let manifests = workspace_manifests();
    validate_topology(&manifests).expect("the real repository topology must be lawful");
    assert!(manifests.contains_key("Cargo.toml"));
    assert!(manifests.contains_key("repository/Cargo.toml"));
    assert!(
        manifests.len() >= 15,
        "every member and fixture manifest is audited"
    );
    assert_resolved_qualified_versions();
}

#[test]
fn phase_one_removes_product_host_replacement_and_forged_native_host_lanes() {
    let inventory = workspace_source_inventory();
    let consumption = inventory
        .source("crates/worth-ui-runtime/src/mounting/presentation/consumption_view.rs")
        .expect("mounted consumption view");
    assert!(!consumption.text().contains("fn projection("));
    assert!(!consumption.text().contains("UiMountedProjectionView"));

    let builder = inventory
        .source("crates/worth-ui-runtime/src/facade/entry/app_builder.rs")
        .expect("application builder");
    for forbidden in [
        "fn with_host<",
        "fn bind_framework_host<",
        "UiApplicationBuilderDefaultHost",
    ] {
        assert!(
            !builder.text().contains(forbidden),
            "builder retains {forbidden}"
        );
    }
    assert!(!builder.text().contains("bind_native_platform_host"));
    assert!(!builder.text().contains("UiNativePlatformBindingGrant"));
    assert!(builder.text().contains("UiApplicationHostUnbound"));

    let native = inventory
        .source("crates/worth-ui-host-native/src/prepared_host.rs")
        .expect("prepared native host");
    assert!(!native.text().contains("derive(Clone"));
    assert!(!native.text().contains("derive(Default"));
    assert!(!native.text().contains("from_platform_binding"));

    for path in [
        "crates/worth-ui-runtime/src/facade/prepared_application_authority/generation_identity.rs",
        "crates/worth-ui-runtime/src/facade/prepared_application_authority/lowering_authority.rs",
    ] {
        let prepared = inventory.source(path).expect("host-neutral prepared owner");
        assert!(
            !prepared.text().contains("host_session_plan"),
            "{path} retains host selection"
        );
    }
    let platform_binding = inventory
        .source("crates/worth-ui-native-platform/src/native_platform_binding.rs")
        .expect("private native binding owner");
    assert!(platform_binding
        .text()
        .contains("pub(crate) struct UiNativePlatformBindingGrant"));
    assert!(!platform_binding.text().contains("#[derive(Clone"));
    let platform_facade = inventory
        .source("crates/worth-ui-native-platform/src/lib.rs")
        .expect("native platform facade");
    assert!(!platform_facade
        .text()
        .contains("pub use native_platform_binding"));
}

#[test]
fn phase_one_protocol_and_observation_revisions_are_exclusive() {
    let protocol = repository_document(
        "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/protocol.rs",
    );
    assert!(protocol.contains("const COMPATIBLE_FLOOR: u16 = 4;"));
    assert!(protocol.contains("const CURRENT: u16 = 4;"));
    assert!(protocol.contains("const CURRENT_OBSERVATION_SCHEMA: u16 = 6;"));
}

#[test]
fn phase_one_product_preparation_is_effect_free_and_host_neutral() {
    let inventory = workspace_source_inventory();
    let application = inventory
        .source("crates/worth-ui-native-platform/src/application.rs")
        .expect("native application preparation owner");
    for required in [
        "pub enum UiNativeApplicationPreparationOutcome",
        "Prepared(UiPreparedNativeApplication)",
        "Denied(UiNativeApplicationPreparationDenial)",
        "WorthUiHostNeutralApp",
    ] {
        assert!(
            application.text().contains(required),
            "product preparation omits {required}"
        );
    }
    for forbidden in [
        "register_filesystem_source",
        "register_query_owner",
        "register_intent_owner",
        "register_inspection_owner",
        "register_readiness_owner",
        "UiNativePreparationActivation",
        "Condvar",
        "JoinHandle",
    ] {
        assert!(
            !application.text().contains(forbidden),
            "phase-one product preparation owns runtime effect {forbidden}"
        );
    }
    assert!(
        inventory
            .source("crates/worth-ui-native-platform/src/preparation_worker.rs")
            .is_none(),
        "retired generic preparation worker still exists"
    );
    let facade = inventory
        .source("crates/worth-ui-native-platform/src/lib.rs")
        .expect("native platform facade");
    for forbidden in [
        "UiNativeStoppedPreparationResource",
        "UiNativeStoppedReadinessOwner",
        "refusing_cleanup",
    ] {
        assert!(
            !facade.text().contains(forbidden),
            "facade exposes synthetic lifecycle lane {forbidden}"
        );
    }
}

#[test]
fn independent_oracle_has_no_disputed_production_imports() {
    let oracle = workspace_source_inventory()
        .source("crates/worth-ui-certification/tests/application_contracts/host_platform/oracle.rs")
        .expect("independent host-platform oracle");
    for forbidden in [
        "worth_ui_runtime",
        "worth_ui_host_contract",
        "worth_ui_host_headless",
        "work_producer",
        "order_integrity",
    ] {
        assert!(
            !oracle.text().contains(forbidden),
            "oracle imports {forbidden}"
        );
    }
    let controls = workspace_source_inventory()
        .absolute_path("crates/worth-ui-certification/tests/application_contracts/host_platform/control_points.toml");
    let manifest = std::fs::read_to_string(controls).expect("control manifest");
    assert!(manifest.contains("world_version = 1"));
    assert!(manifest.contains("maximum_rectangles = 2048"));
    assert_eq!(manifest.matches("[[filled_rect]]").count(), 2);
}

#[test]
fn platform_and_presentation_issuers_have_exact_source_homes() {
    let inventory = workspace_source_inventory();
    let retired_native_issuer =
        ["UiNativePlatformBinding", "Issuer::for_prepared_platform"].concat();
    assert_exact_symbol_homes(inventory, &retired_native_issuer, &[]);
    let retired_presentation_issuer =
        ["UiMountedPresentationRuntimeAuthority", "::for_runtime"].concat();
    assert_exact_symbol_homes(
        inventory,
        &retired_presentation_issuer,
        &["crates/worth-ui-host-contract/tests/ui/presentation_work_issuance_requires_runtime_authority.rs"],
    );
}

fn assert_exact_symbol_homes(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
    symbol: &str,
    expected: &[&str],
) {
    let actual = inventory
        .rust_files_under("crates")
        .filter(|source| source.text().contains(symbol))
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual,
        expected.iter().map(|path| (*path).to_owned()).collect()
    );
}

#[test]
fn hiding_mutations_fail_the_same_repository_topology_verdict() {
    let lawful = workspace_manifests();
    validate_topology(&lawful).unwrap();
    for (path, section, dependency) in [
        (
            "crates/worth-ui-host-headless/Cargo.toml",
            "dependencies",
            "worth-ui-runtime",
        ),
        (
            "crates/worth-ui-host-native/Cargo.toml",
            "dev-dependencies",
            "worth-ui-runtime",
        ),
        (
            "crates/worth-ui-native-platform/Cargo.toml",
            "build-dependencies",
            "worth-ui-runtime",
        ),
        (
            "crates/worth-ui-runtime/Cargo.toml",
            "target.'cfg(windows)'.dependencies",
            "worth-ui-host-native",
        ),
    ] {
        let mut mutated = lawful.clone();
        let manifest = mutated.get_mut(path).expect("governed manifest");
        manifest.push_str(&format!(
            "\n[{section}]\n{dependency} = {{ path = \"../{dependency}\" }}\n"
        ));
        assert!(
            validate_topology(&mutated).is_err(),
            "{path} {section} mutation must be denied"
        );
    }
    let mut aliased = lawful;
    let native = aliased
        .get_mut("crates/worth-ui-host-native/Cargo.toml")
        .unwrap();
    native.push_str(
        "\n[target.'cfg(windows)'.dependencies]\nhidden-runtime = { package = \"worth-ui-runtime\", path = \"../worth-ui-runtime\" }\n",
    );
    assert!(validate_topology(&aliased).is_err());
}

fn validate_topology(manifests: &Manifests) -> Result<(), String> {
    let exact = [
        (
            "crates/worth-ui-host-headless/Cargo.toml",
            set(["worth-ui-host-contract", "worth-ui-test-support"]),
        ),
        (
            "crates/worth-ui-host-native/Cargo.toml",
            set(["worth-ui-host-contract"]),
        ),
        (
            "crates/worth-ui-native-platform/Cargo.toml",
            set(["worth-ui", "worth-ui-host-contract", "worth-ui-host-native"]),
        ),
    ];
    for (path, allowed) in exact {
        let manifest = manifests
            .get(path)
            .ok_or_else(|| format!("missing {path}"))?;
        let dependencies = worth_ui_dependencies(manifest)?;
        if !dependencies.is_subset(&allowed) {
            return Err(format!(
                "{path} has forbidden Worth UI dependencies: {dependencies:?}"
            ));
        }
        let ordinary = dependency_packages(manifest, DependencyScope::Ordinary)?
            .into_iter()
            .filter(|package| package.starts_with("worth-ui"))
            .collect::<BTreeSet<_>>();
        if path.contains("host-headless") || path.contains("host-native") {
            if ordinary != set(["worth-ui-host-contract"]) {
                return Err(format!(
                    "{path} ordinary dependency set drifted: {ordinary:?}"
                ));
            }
        } else if ordinary != allowed {
            return Err(format!("{path} dependency set drifted: {ordinary:?}"));
        }
    }
    let runtime = manifests
        .get("crates/worth-ui-runtime/Cargo.toml")
        .ok_or("missing runtime manifest")?;
    let runtime_dependencies = worth_ui_dependencies(runtime)?;
    for forbidden in [
        "worth-ui-host-headless",
        "worth-ui-host-native",
        "worth-ui-host-egui",
    ] {
        if runtime_dependencies.contains(forbidden) {
            return Err(format!("runtime imports {forbidden}"));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum DependencyScope {
    Ordinary,
    All,
}

fn dependency_packages(manifest: &str, scope: DependencyScope) -> Result<BTreeSet<String>, String> {
    let value = manifest
        .parse::<toml::Value>()
        .map_err(|error| error.to_string())?;
    let mut packages = BTreeSet::new();
    collect_dependency_tables(&value, scope, &mut packages);
    Ok(packages)
}

fn collect_dependency_tables(
    value: &toml::Value,
    scope: DependencyScope,
    packages: &mut BTreeSet<String>,
) {
    let Some(table) = value.as_table() else {
        return;
    };
    for (key, value) in table {
        let dependency_table = key == "dependencies"
            || matches!(scope, DependencyScope::All)
                && matches!(key.as_str(), "dev-dependencies" | "build-dependencies");
        if dependency_table {
            if let Some(entries) = value.as_table() {
                for (alias, declaration) in entries {
                    let package = declaration
                        .get("package")
                        .and_then(toml::Value::as_str)
                        .unwrap_or(alias);
                    packages.insert(package.to_owned());
                }
            }
        } else if key == "target" {
            for target in value
                .as_table()
                .into_iter()
                .flat_map(|targets| targets.values())
            {
                collect_dependency_tables(target, scope, packages);
            }
        }
    }
}

fn worth_ui_dependencies(manifest: &str) -> Result<BTreeSet<String>, String> {
    Ok(dependency_packages(manifest, DependencyScope::All)?
        .into_iter()
        .filter(|package| package.starts_with("worth-ui"))
        .collect())
}

fn workspace_manifests() -> Manifests {
    repository_manifests::all(workspace_source_inventory())
}

fn assert_resolved_qualified_versions() {
    let lock = repository_document("workspaces/worth-ui/Cargo.lock");
    let parsed = lock.parse::<toml::Value>().expect("workspace lockfile");
    let resolved = parsed
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?,
                package.get("version")?.as_str()?,
            ))
        })
        .collect::<BTreeSet<_>>();
    for expected in [
        ("winit", "0.30.13"),
        ("wgpu", "29.0.4"),
        ("rustybuzz", "0.20.1"),
        ("swash", "0.2.10"),
    ] {
        assert!(
            resolved.contains(&expected),
            "missing resolved pin {expected:?}"
        );
    }
}

fn set<const N: usize>(values: [&str; N]) -> BTreeSet<String> {
    values.into_iter().map(str::to_owned).collect()
}
