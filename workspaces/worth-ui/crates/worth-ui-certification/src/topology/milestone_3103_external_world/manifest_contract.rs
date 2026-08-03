use std::collections::BTreeSet;

pub(super) fn audit(workspace: &str, package: &str) -> Result<(), String> {
    let workspace = workspace
        .parse::<toml::Value>()
        .map_err(|error| format!("workspace manifest should parse: {error}"))?;
    let package = package
        .parse::<toml::Value>()
        .map_err(|error| format!("pulse manifest should parse: {error}"))?;
    audit_test_target(&package)?;
    audit_windows_dev_dependencies(&package)?;
    audit_workspace_native_contracts(&workspace)
}

fn audit_test_target(manifest: &toml::Value) -> Result<(), String> {
    let features = manifest["features"]
        .as_table()
        .ok_or_else(|| "pulse executable-world feature should exist".to_owned())?;
    if features.len() != 1
        || features
            .get("executable-world")
            .and_then(toml::Value::as_array)
            .is_none_or(|members| !members.is_empty())
    {
        return Err("pulse should expose only the empty executable-world feature".to_owned());
    }
    let tests = manifest["test"]
        .as_array()
        .ok_or_else(|| "pulse executable-world test target should exist".to_owned())?;
    if tests.len() != 1
        || tests[0]["name"].as_str() != Some("executable_world")
        || tests[0]["path"].as_str() != Some("tests/executable_world.rs")
        || tests[0]["required-features"]
            .as_array()
            .is_none_or(|required| {
                required.len() != 1 || required[0].as_str() != Some("executable-world")
            })
    {
        return Err("pulse executable-world target identity or feature gate drifted".to_owned());
    }
    Ok(())
}

fn audit_windows_dev_dependencies(manifest: &toml::Value) -> Result<(), String> {
    let dependencies = manifest
        .get("target")
        .and_then(toml::Value::as_table)
        .and_then(|targets| targets.get("cfg(windows)"))
        .and_then(toml::Value::as_table)
        .and_then(|windows| windows.get("dev-dependencies"))
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "Windows courtroom dev dependencies should exist".to_owned())?;
    let expected = ["uiautomation", "winsafe", "xcap"]
        .into_iter()
        .collect::<BTreeSet<_>>();
    let observed = dependencies
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!(
            "Windows courtroom dev dependencies drifted: {observed:?} != {expected:?}"
        ));
    }
    for (name, dependency) in dependencies {
        let table = dependency
            .as_table()
            .ok_or_else(|| format!("`{name}` should inherit from the workspace"))?;
        let expected_keys = if name == "xcap" {
            ["features", "workspace"]
                .into_iter()
                .collect::<BTreeSet<_>>()
        } else {
            ["workspace"].into_iter().collect::<BTreeSet<_>>()
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
        let expected_features = if name == "xcap" {
            ["wgc"].into_iter().collect::<BTreeSet<_>>()
        } else {
            BTreeSet::new()
        };
        if table.get("workspace").and_then(toml::Value::as_bool) != Some(true)
            || observed_keys != expected_keys
            || observed_features != expected_features
        {
            return Err(format!(
                "`{name}` application dependency contract drifted: keys={observed_keys:?}, features={observed_features:?}"
            ));
        }
    }
    Ok(())
}

fn audit_workspace_native_contracts(workspace: &toml::Value) -> Result<(), String> {
    let dependencies = workspace
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "workspace dependencies should exist".to_owned())?;
    audit_dependency(
        dependencies,
        "uiautomation",
        "=0.25.0",
        Some(false),
        &["control", "input"],
    )?;
    audit_dependency(
        dependencies,
        "winsafe",
        "=0.0.28",
        None,
        &["dwm", "kernel", "user"],
    )?;
    audit_dependency(dependencies, "xcap", "=0.9.7", Some(false), &[])
}

fn audit_dependency(
    dependencies: &toml::map::Map<String, toml::Value>,
    name: &str,
    version: &str,
    default_features: Option<bool>,
    features: &[&str],
) -> Result<(), String> {
    let contract = dependencies
        .get(name)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("workspace dependency `{name}` should be a contract table"))?;
    if contract.get("version").and_then(toml::Value::as_str) != Some(version)
        || contract
            .get("default-features")
            .and_then(toml::Value::as_bool)
            != default_features
    {
        return Err(format!("workspace dependency `{name}` contract drifted"));
    }
    let observed = contract
        .get("features")
        .and_then(toml::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(toml::Value::as_str)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let expected = features.iter().copied().collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!(
            "workspace dependency `{name}` features drifted: {observed:?} != {expected:?}"
        ));
    }
    Ok(())
}
