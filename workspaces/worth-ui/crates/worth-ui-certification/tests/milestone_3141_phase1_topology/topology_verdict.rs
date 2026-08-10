use std::collections::{BTreeMap, BTreeSet};

use super::{
    repository_document, repository_manifests, topology_edges, workspace_source_inventory,
};

pub(super) type Manifests = BTreeMap<String, String>;

const WORKSPACE_MANIFESTS: &[&str] = &[
    "Cargo.toml",
    "apps/platform-pulse/Cargo.toml",
    "crates/worth-ui/Cargo.toml",
    "crates/worth-ui-certification/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/compile_contracts/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/host_contract_only_adapter/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/runtime_effect_adapter/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/topology_negative/admission_facade_bypass_consumer/crates/fake-admission-consumer-direct-alias/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/topology_negative/admission_facade_bypass_consumer/crates/fake-admission-consumer-extern-alias/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/topology_negative/admission_facade_bypass_consumer/crates/fake-admission-consumer/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/topology_negative/host_egui_forbidden_runtime_import/crates/worth-ui-host-egui/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/topology_negative/inspection_facade_bypass_consumer/crates/fake-inspection-consumer/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/topology_negative/obligation_facade_bypass_consumer/crates/fake-obligation-consumer/Cargo.toml",
    "crates/worth-ui-components/Cargo.toml",
    "crates/worth-ui-dsl/Cargo.toml",
    "crates/worth-ui-host-contract/Cargo.toml",
    "crates/worth-ui-host-egui/Cargo.toml",
    "crates/worth-ui-host-headless/Cargo.toml",
    "crates/worth-ui-host-native/Cargo.toml",
    "crates/worth-ui-inspection/Cargo.toml",
    "crates/worth-ui-native-platform/Cargo.toml",
    "crates/worth-ui-query-binding/Cargo.toml",
    "crates/worth-ui-runtime/Cargo.toml",
    "crates/worth-ui-test-support/Cargo.toml",
    "crates/worth-ui-theme/Cargo.toml",
];

pub(super) fn validate_topology(manifests: &Manifests) -> Result<(), String> {
    validate_manifest_inventory(manifests)?;
    validate_every_manifest_edge_set(manifests)?;
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
            set(["worth-ui-runtime"]),
        ),
    ];
    for (path, allowed) in exact {
        validate_manifest(manifests, path, &allowed)?;
    }
    validate_runtime(manifests)
}

fn validate_every_manifest_edge_set(manifests: &Manifests) -> Result<(), String> {
    for path in WORKSPACE_MANIFESTS {
        let manifest = manifests
            .get(*path)
            .ok_or_else(|| format!("missing {path}"))?;
        let observed = worth_ui_dependencies(manifest)?;
        let expected = topology_edges::expected(path)
            .ok_or_else(|| format!("unclassified Worth UI manifest edge set: {path}"))?;
        if observed != expected {
            return Err(format!(
                "{path} Worth UI dependency set drifted: observed={observed:?}, expected={expected:?}"
            ));
        }
    }
    Ok(())
}

fn validate_manifest_inventory(manifests: &Manifests) -> Result<(), String> {
    let observed = manifests
        .keys()
        .filter(|path| !path.starts_with("repository/"))
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected = WORKSPACE_MANIFESTS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!(
            "unclassified Worth UI manifest inventory: {observed:?}"
        ));
    }
    for (path, manifest) in manifests {
        if path.starts_with("repository/") && !worth_ui_dependencies(manifest)?.is_empty() {
            return Err(format!(
                "external repository manifest declares Worth UI edges: {path}"
            ));
        }
    }
    Ok(())
}

pub(super) const fn workspace_manifest_count() -> usize {
    WORKSPACE_MANIFESTS.len()
}

pub(super) fn assert_hiding_mutations_fail_topology_verdict() {
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
            "worth-ui-host-native",
        ),
        (
            "crates/worth-ui-runtime/Cargo.toml",
            "target.'cfg(windows)'.dependencies",
            "worth-ui-components",
        ),
        (
            "crates/worth-ui-theme/Cargo.toml",
            "dependencies",
            "worth-ui-runtime",
        ),
    ] {
        let mut mutated = lawful.clone();
        mutated.get_mut(path).unwrap().push_str(&format!(
            "\n[{section}]\n{dependency} = {{ path = \"../{dependency}\" }}\n"
        ));
        assert!(
            validate_topology(&mutated).is_err(),
            "{path} {section} mutation must be denied"
        );
    }
    let mut aliased = lawful;
    aliased.get_mut("crates/worth-ui-host-native/Cargo.toml").unwrap().push_str(
        "\n[target.'cfg(windows)'.dependencies]\nhidden-runtime = { package = \"worth-ui-runtime\", path = \"../worth-ui-runtime\" }\n",
    );
    assert!(validate_topology(&aliased).is_err());
}

fn validate_manifest(
    manifests: &Manifests,
    path: &str,
    allowed: &BTreeSet<String>,
) -> Result<(), String> {
    let manifest = manifests
        .get(path)
        .ok_or_else(|| format!("missing {path}"))?;
    let dependencies = worth_ui_dependencies(manifest)?;
    if !dependencies.is_subset(allowed) {
        return Err(format!(
            "{path} has forbidden Worth UI dependencies: {dependencies:?}"
        ));
    }
    let ordinary = dependency_packages(manifest, DependencyScope::Ordinary)?
        .into_iter()
        .filter(|package| package.starts_with("worth-ui"))
        .collect::<BTreeSet<_>>();
    let expected = if path.contains("host-headless") || path.contains("host-native") {
        set(["worth-ui-host-contract"])
    } else {
        allowed.clone()
    };
    if ordinary != expected {
        return Err(format!(
            "{path} ordinary dependency set drifted: {ordinary:?}"
        ));
    }
    Ok(())
}

fn validate_runtime(manifests: &Manifests) -> Result<(), String> {
    let runtime = manifests
        .get("crates/worth-ui-runtime/Cargo.toml")
        .ok_or("missing runtime manifest")?;
    let dependencies = worth_ui_dependencies(runtime)?;
    if !dependencies.contains("worth-ui-host-native") {
        return Err("runtime omits the fixed qualified native activation facade".to_owned());
    }
    validate_optional_transition(runtime, "worth-ui-host-headless", "certification-support")?;
    validate_optional_transition(runtime, "worth-ui-host-egui", "legacy-egui-migration")
}

fn validate_optional_transition(
    manifest: &str,
    dependency: &str,
    feature: &str,
) -> Result<(), String> {
    let value = manifest
        .parse::<toml::Value>()
        .map_err(|error| error.to_string())?;
    let declaration = &value["dependencies"][dependency];
    if declaration["optional"].as_bool() != Some(true) {
        return Err(format!(
            "runtime {dependency} transition must remain optional"
        ));
    }
    let feature_members = value["features"][feature]
        .as_array()
        .ok_or_else(|| format!("runtime omits {feature} transition feature"))?;
    let expected = format!("dep:{dependency}");
    if !feature_members
        .iter()
        .any(|member| member.as_str() == Some(&expected))
    {
        return Err(format!(
            "runtime {feature} does not own exact {dependency} edge"
        ));
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
            collect_dependency_entries(value, packages);
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

fn collect_dependency_entries(value: &toml::Value, packages: &mut BTreeSet<String>) {
    for (alias, declaration) in value.as_table().into_iter().flatten() {
        let package = declaration
            .get("package")
            .and_then(toml::Value::as_str)
            .unwrap_or(alias);
        packages.insert(package.to_owned());
    }
}

fn worth_ui_dependencies(manifest: &str) -> Result<BTreeSet<String>, String> {
    Ok(dependency_packages(manifest, DependencyScope::All)?
        .into_iter()
        .filter(|package| package.starts_with("worth-ui"))
        .collect())
}

pub(super) fn workspace_manifests() -> Manifests {
    repository_manifests::all(workspace_source_inventory())
}

pub(super) fn assert_resolved_qualified_versions() {
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
