use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::evidence_document::toml_text;

const PULSE_MEMBER: &str = "apps/platform-pulse";
const PULSE_PACKAGE: &str = "worth-ui-platform-pulse";
const UNCONDITIONAL_DEPENDENCIES: &[&str] = &[
    "eframe",
    "notify",
    "serde",
    "serde_json",
    "worth-query-decl",
    "worth-query-host",
    "worth-ui",
    "worth-ui-host-egui",
    "worth-ui-native-platform",
];
const WINDOWS_DEV_DEPENDENCIES: &[&str] = &["uiautomation", "win32job", "winsafe", "xcap"];
const ALLOWED_DEPENDENCIES: &[&str] = &[
    "eframe",
    "notify",
    "serde",
    "serde_json",
    "uiautomation",
    "win32job",
    "winsafe",
    "worth-query-decl",
    "worth-query-host",
    "worth-ui",
    "worth-ui-host-egui",
    "worth-ui-native-platform",
    "xcap",
];
pub(super) const EFRAME_VERSION: &str = "=0.35.0";
const WORKSPACE_EFRAME_FEATURES: &[&str] = &["default_fonts"];
pub(super) const PULSE_EFRAME_FEATURES: &[&str] = &["default_fonts", "wgpu_no_default_features"];

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    repository_root: &Path,
    contract: &toml::Value,
) -> Result<(), String> {
    audit_composition_owner(contract)?;
    let config_text =
        fs::read_to_string(repository_root.join("tools/boundary-check/config/road1.toml"))
            .map_err(|error| format!("boundary config should be readable: {error}"))?;
    let config = config_text
        .parse::<toml::Value>()
        .map_err(|error| format!("boundary config should parse: {error}"))?;
    audit_boundary_config(&config)?;
    audit_workspace_membership(inventory)
}

fn audit_composition_owner(contract: &toml::Value) -> Result<(), String> {
    let root = contract
        .get("composition_root")
        .ok_or_else(|| "Phase 1 contract should name composition ownership".to_owned())?;
    if toml_text(root, "owner")? != PULSE_PACKAGE {
        return Err("only the permanent pulse package may own application composition".to_owned());
    }
    Ok(())
}

pub(super) fn audit_boundary_config(config: &toml::Value) -> Result<(), String> {
    let rows = config
        .get("source_dependency_allowlists")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "boundary config should contain source dependency allowlists".to_owned())?;
    let matches = rows
        .iter()
        .filter(|row| {
            row.get("sources")
                .and_then(toml::Value::as_array)
                .is_some_and(|sources| {
                    sources
                        .iter()
                        .any(|source| source.as_str() == Some(PULSE_PACKAGE))
                })
        })
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(format!(
            "boundary config should contain exactly one pulse source allowlist; found {}",
            matches.len()
        ));
    }
    let row = matches[0];
    if toml_text(row, "workspace_manifest")? != "workspaces/worth-ui/Cargo.toml" {
        return Err("pulse allowlist should govern the Worth UI workspace".to_owned());
    }
    let actual = string_set(row, "allowed_targets")?;
    let expected = ALLOWED_DEPENDENCIES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "pulse dependencies should be exactly {expected:?}; found {actual:?}"
        ));
    }
    audit_native_shell_dependency_contract(row)?;
    Ok(())
}

fn audit_native_shell_dependency_contract(row: &toml::Value) -> Result<(), String> {
    let contracts = row
        .get("dependency_contracts")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "pulse allowlist should contain dependency contracts".to_owned())?;
    let native_shell_contracts = contracts
        .iter()
        .filter(|contract| contract.get("target").and_then(toml::Value::as_str) == Some("eframe"))
        .collect::<Vec<_>>();
    if native_shell_contracts.len() != 1 {
        return Err(format!(
            "pulse allowlist should contain one eframe native-shell contract; found {}",
            native_shell_contracts.len()
        ));
    }
    let contract = native_shell_contracts[0];
    if toml_text(contract, "target")? != "eframe"
        || toml_text(contract, "version_requirement")? != EFRAME_VERSION
        || contract
            .get("uses_default_features")
            .and_then(toml::Value::as_bool)
            != Some(false)
        || string_set(contract, "features")?
            != PULSE_EFRAME_FEATURES
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
    {
        return Err("pulse native-shell dependency contract drifted".to_owned());
    }
    Ok(())
}

fn audit_workspace_membership(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let manifest = inventory
        .text("Cargo.toml")
        .parse::<toml::Value>()
        .map_err(|error| format!("Worth UI workspace manifest should parse: {error}"))?;
    let members = manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "Worth UI workspace should declare members".to_owned())?;
    let pulse_count = members
        .iter()
        .filter(|member| member.as_str() == Some(PULSE_MEMBER))
        .count();
    if pulse_count > 1 {
        return Err("Worth UI workspace may contain only one permanent pulse member".to_owned());
    }
    if pulse_count == 0 {
        return Ok(());
    }
    audit_workspace_native_shell(&manifest)?;
    audit_born_pulse_manifest(inventory)
}

fn audit_workspace_native_shell(manifest: &toml::Value) -> Result<(), String> {
    let eframe = manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(|dependencies| dependencies.get("eframe"))
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "born pulse requires a workspace-owned eframe dependency".to_owned())?;
    if eframe.get("version").and_then(toml::Value::as_str) != Some(EFRAME_VERSION)
        || eframe
            .get("default-features")
            .and_then(toml::Value::as_bool)
            != Some(false)
        || eframe
            .get("features")
            .and_then(toml::Value::as_array)
            .ok_or_else(|| "workspace eframe dependency should freeze features".to_owned())?
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<BTreeSet<_>>()
            != WORKSPACE_EFRAME_FEATURES
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
    {
        return Err("workspace eframe dependency should match the frozen native shell".to_owned());
    }
    Ok(())
}

fn audit_born_pulse_manifest(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let path = Path::new(PULSE_MEMBER).join("Cargo.toml");
    let manifest = inventory
        .source(&path)
        .ok_or_else(|| "born pulse member should contain Cargo.toml".to_owned())?
        .text()
        .parse::<toml::Value>()
        .map_err(|error| format!("pulse manifest should parse: {error}"))?;
    audit_pulse_manifest(&manifest)
}

pub(super) fn audit_pulse_manifest(manifest: &toml::Value) -> Result<(), String> {
    let package = manifest
        .get("package")
        .ok_or_else(|| "pulse manifest should contain [package]".to_owned())?;
    let package_name = package.get("name").and_then(toml::Value::as_str);
    if package_name != Some(PULSE_PACKAGE) {
        return Err(format!(
            "pulse member package should be `{PULSE_PACKAGE}`; found {package_name:?}"
        ));
    }
    if package.get("autotests").and_then(toml::Value::as_bool) != Some(false) {
        return Err("pulse manifest should set `autotests = false`".to_owned());
    }
    let actual: BTreeSet<&str> = manifest
        .get("dependencies")
        .and_then(toml::Value::as_table)
        .map(|dependencies| dependencies.keys().map(String::as_str).collect())
        .unwrap_or_default();
    let expected = UNCONDITIONAL_DEPENDENCIES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "born pulse manifest dependencies should be exactly {expected:?}; found {actual:?}"
        ));
    }
    let eframe = manifest
        .get("dependencies")
        .and_then(|dependencies| dependencies.get("eframe"))
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "pulse eframe dependency should be a workspace table".to_owned())?;
    if eframe.get("workspace").and_then(toml::Value::as_bool) != Some(true) {
        return Err("pulse should inherit the frozen workspace eframe dependency".to_owned());
    }
    if table_string_set(eframe, "features")?
        != ["wgpu_no_default_features"]
            .into_iter()
            .collect::<BTreeSet<_>>()
    {
        return Err("pulse should select only the no-default WGPU lane".to_owned());
    }
    audit_windows_dev_dependencies(manifest)?;
    Ok(())
}

fn audit_windows_dev_dependencies(manifest: &toml::Value) -> Result<(), String> {
    let actual = manifest
        .get("target")
        .and_then(|target| target.get("cfg(windows)"))
        .and_then(|windows| windows.get("dev-dependencies"))
        .and_then(toml::Value::as_table)
        .map(|dependencies| {
            dependencies
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
        })
        .ok_or_else(|| "pulse manifest should contain Windows-only dev dependencies".to_owned())?;
    let expected = WINDOWS_DEV_DEPENDENCIES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "pulse Windows dev dependencies should be exactly {expected:?}; found {actual:?}"
        ));
    }
    Ok(())
}

fn string_set<'a>(row: &'a toml::Value, field: &str) -> Result<BTreeSet<&'a str>, String> {
    row.get(field)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("boundary row `{field}` should be an array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| format!("boundary row `{field}` values should be text"))
        })
        .collect()
}

fn table_string_set<'a>(
    table: &'a toml::map::Map<String, toml::Value>,
    field: &str,
) -> Result<BTreeSet<&'a str>, String> {
    table
        .get(field)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("{field} should be an array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| format!("{field} should contain strings"))
        })
        .collect()
}
