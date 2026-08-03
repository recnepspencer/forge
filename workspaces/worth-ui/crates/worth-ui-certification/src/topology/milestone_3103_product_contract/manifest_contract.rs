use std::collections::BTreeSet;

const PACKAGE: &str = "worth-ui-platform-pulse";
const MEMBER: &str = "apps/platform-pulse";

pub(super) fn audit(workspace: &str, package: &str) -> Result<(), String> {
    let workspace = workspace
        .parse::<toml::Value>()
        .map_err(|error| format!("workspace manifest should parse: {error}"))?;
    let package = package
        .parse::<toml::Value>()
        .map_err(|error| format!("pulse manifest should parse: {error}"))?;
    audit_workspace_member(&workspace)?;
    audit_package_shape(&package)?;
    audit_dependencies(&package)?;
    audit_phase3_successor_test_surface(&package)
}

fn audit_workspace_member(workspace: &toml::Value) -> Result<(), String> {
    let members = workspace["workspace"]["members"]
        .as_array()
        .ok_or_else(|| "workspace.members should be an array".to_owned())?;
    let matches = members
        .iter()
        .filter(|member| member.as_str() == Some(MEMBER))
        .count();
    if matches != 1 {
        return Err(format!(
            "workspace should contain `{MEMBER}` exactly once; found {matches}"
        ));
    }
    Ok(())
}

fn audit_package_shape(manifest: &toml::Value) -> Result<(), String> {
    if manifest["package"]["name"].as_str() != Some(PACKAGE)
        || manifest["package"]["autotests"].as_bool() != Some(false)
    {
        return Err("pulse package identity or autotest posture drifted".to_owned());
    }
    let library = manifest["lib"]
        .as_table()
        .ok_or_else(|| "pulse package should declare one library".to_owned())?;
    if library.get("name").and_then(toml::Value::as_str) != Some("worth_ui_platform_pulse")
        || library.get("path").and_then(toml::Value::as_str) != Some("src/lib.rs")
    {
        return Err("pulse observation library target drifted".to_owned());
    }
    let binaries = manifest["bin"]
        .as_array()
        .ok_or_else(|| "pulse package should declare one binary".to_owned())?;
    if binaries.len() != 1
        || binaries[0]["name"].as_str() != Some(PACKAGE)
        || binaries[0]["path"].as_str() != Some("src/main.rs")
    {
        return Err("pulse should own exactly one canonical binary target".to_owned());
    }
    for forbidden in ["example", "bench", "dev-dependencies"] {
        if manifest.get(forbidden).is_some() {
            return Err(format!(
                "pulse manifest cannot declare top-level `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn audit_phase3_successor_test_surface(manifest: &toml::Value) -> Result<(), String> {
    let features = manifest["features"]
        .as_table()
        .ok_or_else(|| "pulse should declare the Phase 3 executable-world feature".to_owned())?;
    if features.len() != 1
        || features
            .get("executable-world")
            .and_then(toml::Value::as_array)
            .is_none_or(|members| !members.is_empty())
    {
        return Err("pulse should declare only an empty `executable-world` feature".to_owned());
    }
    let tests = manifest["test"]
        .as_array()
        .ok_or_else(|| "pulse should declare the Phase 3 executable-world test".to_owned())?;
    if tests.len() != 1
        || tests[0]["name"].as_str() != Some("executable_world")
        || tests[0]["path"].as_str() != Some("tests/executable_world.rs")
        || tests[0]["required-features"]
            .as_array()
            .is_none_or(|features| {
                features.len() != 1 || features[0].as_str() != Some("executable-world")
            })
    {
        return Err("pulse should own exactly one feature-gated executable-world test".to_owned());
    }
    let windows = manifest
        .get("target")
        .and_then(toml::Value::as_table)
        .and_then(|targets| targets.get("cfg(windows)"))
        .and_then(toml::Value::as_table)
        .and_then(|windows| windows.get("dev-dependencies"))
        .and_then(toml::Value::as_table)
        .ok_or_else(|| {
            "pulse should declare Windows-only native courtroom dependencies".to_owned()
        })?;
    let expected = ["uiautomation", "winsafe", "xcap"]
        .into_iter()
        .collect::<BTreeSet<_>>();
    let observed = windows.keys().map(String::as_str).collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!(
            "pulse Windows dev-dependency surface drifted: {observed:?} != {expected:?}"
        ));
    }
    for (name, dependency) in windows {
        let table = dependency.as_table().ok_or_else(|| {
            format!("pulse Windows dev dependency `{name}` should use workspace inheritance")
        })?;
        let features = if name == "xcap" { &["wgc"][..] } else { &[] };
        audit_workspace_dependency(name, table, features, "Windows dev dependency")?;
    }
    Ok(())
}

fn audit_dependencies(manifest: &toml::Value) -> Result<(), String> {
    let dependencies = manifest["dependencies"]
        .as_table()
        .ok_or_else(|| "pulse dependencies should be a table".to_owned())?;
    let expected = [
        "eframe",
        "notify",
        "serde",
        "serde_json",
        "worth-query-decl",
        "worth-query-host",
        "worth-ui",
        "worth-ui-host-egui",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let observed = dependencies
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!(
            "pulse dependency surface drifted: {observed:?} != {expected:?}"
        ));
    }
    for (name, dependency) in dependencies {
        let table = dependency
            .as_table()
            .ok_or_else(|| format!("pulse dependency `{name}` should use workspace inheritance"))?;
        let features = if name == "eframe" { &["wgpu"][..] } else { &[] };
        audit_workspace_dependency(name, table, features, "dependency")?;
    }
    Ok(())
}

fn audit_workspace_dependency(
    name: &str,
    table: &toml::map::Map<String, toml::Value>,
    expected_features: &[&str],
    owner: &str,
) -> Result<(), String> {
    let expected_keys = if expected_features.is_empty() {
        ["workspace"].into_iter().collect::<BTreeSet<_>>()
    } else {
        ["features", "workspace"]
            .into_iter()
            .collect::<BTreeSet<_>>()
    };
    let observed_keys = table.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let observed_features = table
        .get("features")
        .and_then(toml::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(toml::Value::as_str)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let expected_features = expected_features.iter().copied().collect::<BTreeSet<_>>();
    if table.get("workspace").and_then(toml::Value::as_bool) != Some(true)
        || observed_keys != expected_keys
        || observed_features != expected_features
    {
        return Err(format!(
            "pulse {owner} `{name}` contract drifted: keys={observed_keys:?}, features={observed_features:?}"
        ));
    }
    Ok(())
}
