use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

const OPENING_MEMBERS: u64 = 11;
const HISTORICAL_MAXIMUM_MEMBERS: u64 = 12;
const PHASE_ONE_FOUNDATION_MEMBERS: u64 = 15;
const OPENING_TARGETS: u64 = 20;
const MAXIMUM_TARGETS: u64 = 21;
const MAXIMUM_TARGETS_WITH_OBSERVATION_LIBRARY: u64 = 22;
const MAXIMUM_TARGETS_WITH_EXECUTABLE_WORLD: u64 = 23;
const PHASE_ONE_FOUNDATION_TARGETS: u64 = 27;
const INTEGRATION_TARGETS: u64 = 9;
const INTEGRATION_TARGETS_WITH_EXECUTABLE_WORLD: u64 = 10;
const PHASE_ONE_FOUNDATION_INTEGRATION_TARGETS: u64 = 11;
const COMPILE_SESSIONS: u64 = 2;

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    baseline: &serde_json::Value,
) -> Result<(), String> {
    audit_frozen_budget(baseline)?;
    let (member_count, target_count, integration_target_count) =
        current_workspace_topology(inventory)?;
    if !(OPENING_MEMBERS..=HISTORICAL_MAXIMUM_MEMBERS).contains(&member_count)
        && member_count != PHASE_ONE_FOUNDATION_MEMBERS
    {
        return Err(format!(
            "Worth UI workspace member count should remain within {OPENING_MEMBERS}..={HISTORICAL_MAXIMUM_MEMBERS} or equal the exact Phase 1 foundation count {PHASE_ONE_FOUNDATION_MEMBERS}; found {member_count}"
        ));
    }
    if member_count == PHASE_ONE_FOUNDATION_MEMBERS {
        audit_phase_one_foundation_members(inventory)?;
    }
    let expected_integration_targets = if target_count == PHASE_ONE_FOUNDATION_TARGETS {
        let pulse = parse_manifest(inventory, Path::new("apps/platform-pulse/Cargo.toml"))?;
        audit_successor_executable_world(&pulse)?;
        PHASE_ONE_FOUNDATION_INTEGRATION_TARGETS
    } else if target_count == MAXIMUM_TARGETS_WITH_OBSERVATION_LIBRARY {
        let pulse = parse_manifest(inventory, Path::new("apps/platform-pulse/Cargo.toml"))?;
        audit_successor_observation_library(&pulse)?;
        INTEGRATION_TARGETS
    } else if target_count == MAXIMUM_TARGETS_WITH_EXECUTABLE_WORLD {
        let pulse = parse_manifest(inventory, Path::new("apps/platform-pulse/Cargo.toml"))?;
        audit_successor_executable_world(&pulse)?;
        INTEGRATION_TARGETS_WITH_EXECUTABLE_WORLD
    } else if !(OPENING_TARGETS..=MAXIMUM_TARGETS).contains(&target_count) {
        return Err(format!(
            "Worth UI Cargo target count should remain within {OPENING_TARGETS}..={MAXIMUM_TARGETS}, add the named observation library as target {MAXIMUM_TARGETS_WITH_OBSERVATION_LIBRARY}, or add its exact executable-world test as target {MAXIMUM_TARGETS_WITH_EXECUTABLE_WORLD}; found {target_count}"
        ));
    } else {
        INTEGRATION_TARGETS
    };
    if integration_target_count != expected_integration_targets {
        return Err(format!(
            "integration target count should be {expected_integration_targets} for the admitted successor posture; found {integration_target_count}"
        ));
    }
    Ok(())
}

pub(super) fn audit_successor_observation_library(manifest: &toml::Value) -> Result<(), String> {
    let library = manifest
        .get("lib")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "target 22 must be the named pulse observation library".to_owned())?;
    let binaries = manifest
        .get("bin")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "pulse should retain one canonical binary".to_owned())?;
    if manifest["package"]["name"].as_str() != Some("worth-ui-platform-pulse")
        || manifest["package"]["autotests"].as_bool() != Some(false)
        || library.get("name").and_then(toml::Value::as_str) != Some("worth_ui_platform_pulse")
        || library.get("path").and_then(toml::Value::as_str) != Some("src/lib.rs")
        || binaries.len() != 1
        || binaries[0]["name"].as_str() != Some("worth-ui-platform-pulse")
        || binaries[0]["path"].as_str() != Some("src/main.rs")
        || ["features", "test", "example", "bench"]
            .iter()
            .any(|key| manifest.get(*key).is_some())
    {
        return Err(
            "target 22 must be exactly the observation-only library beside the canonical pulse binary"
                .to_owned(),
        );
    }
    Ok(())
}

pub(super) fn audit_successor_executable_world(manifest: &toml::Value) -> Result<(), String> {
    let library = manifest
        .get("lib")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "target 23 requires the named pulse observation library".to_owned())?;
    let binaries = manifest
        .get("bin")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "target 23 requires the canonical pulse binary".to_owned())?;
    let tests = manifest
        .get("test")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "target 23 must be the named executable-world test".to_owned())?;
    let features = manifest
        .get("features")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "target 23 requires the executable-world feature".to_owned())?;
    if manifest["package"]["name"].as_str() != Some("worth-ui-platform-pulse")
        || manifest["package"]["autotests"].as_bool() != Some(false)
        || library.get("name").and_then(toml::Value::as_str) != Some("worth_ui_platform_pulse")
        || library.get("path").and_then(toml::Value::as_str) != Some("src/lib.rs")
        || binaries.len() != 1
        || binaries[0]["name"].as_str() != Some("worth-ui-platform-pulse")
        || binaries[0]["path"].as_str() != Some("src/main.rs")
        || tests.len() != 1
        || tests[0]["name"].as_str() != Some("executable_world")
        || tests[0]["path"].as_str() != Some("tests/executable_world.rs")
        || tests[0]["required-features"]
            .as_array()
            .is_none_or(|required| {
                required.len() != 1 || required[0].as_str() != Some("executable-world")
            })
        || features.len() != 1
        || features
            .get("executable-world")
            .and_then(toml::Value::as_array)
            .is_none_or(|feature| !feature.is_empty())
        || ["example", "bench"]
            .iter()
            .any(|key| manifest.get(*key).is_some())
    {
        return Err("target 23 must be exactly the pulse executable-world test".to_owned());
    }
    Ok(())
}

fn audit_frozen_budget(baseline: &serde_json::Value) -> Result<(), String> {
    let topology = baseline
        .get("topology")
        .ok_or_else(|| "opening baseline should contain topology".to_owned())?;
    for (field, expected) in [
        ("opening_workspace_members", OPENING_MEMBERS),
        ("maximum_workspace_members", HISTORICAL_MAXIMUM_MEMBERS),
        ("allowed_workspace_member_increase", 1),
        ("opening_workspace_cargo_targets", OPENING_TARGETS),
        ("maximum_workspace_cargo_targets", MAXIMUM_TARGETS),
        ("allowed_cargo_target_increase", 1),
        ("opening_integration_test_targets", INTEGRATION_TARGETS),
        ("maximum_integration_test_targets", INTEGRATION_TARGETS),
        ("compile_contract_cargo_sessions", COMPILE_SESSIONS),
        ("trybuild_sessions", 0),
        ("flake_retry_budget", 0),
    ] {
        let actual = topology.get(field).and_then(serde_json::Value::as_u64);
        if actual != Some(expected) {
            return Err(format!(
                "opening topology `{field}` should be {expected}; found {actual:?}"
            ));
        }
    }
    let pulse = baseline
        .get("pulse_executable_opening")
        .ok_or_else(|| "opening baseline should name the unborn pulse executable".to_owned())?;
    if pulse
        .get("package_exists")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
        || pulse
            .get("clean_link_seconds")
            .is_none_or(|value| !value.is_null())
        || pulse
            .get("warm_link_seconds")
            .is_none_or(|value| !value.is_null())
        || pulse
            .get("launch_to_first_presented_frame_milliseconds")
            .is_none_or(|value| !value.is_null())
    {
        return Err(
            "the unborn pulse must have explicit null opening link and launch measurements"
                .to_owned(),
        );
    }
    Ok(())
}

fn audit_phase_one_foundation_members(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let workspace = parse_manifest(inventory, Path::new("Cargo.toml"))?;
    let members = workspace["workspace"]["members"]
        .as_array()
        .ok_or_else(|| "Worth UI workspace should declare members".to_owned())?
        .iter()
        .filter_map(toml::Value::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let required = [
        "crates/worth-ui-host-headless",
        "crates/worth-ui-host-native",
        "crates/worth-ui-native-platform",
    ];
    if required.iter().any(|member| !members.contains(member)) {
        return Err(format!(
            "the Phase 1 member exception requires exactly the named host foundations; members={members:?}"
        ));
    }
    Ok(())
}

fn current_workspace_topology(
    inventory: &WorkspaceSourceInventory,
) -> Result<(u64, u64, u64), String> {
    let workspace = parse_manifest(inventory, Path::new("Cargo.toml"))?;
    let members = workspace
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .ok_or_else(|| "Worth UI workspace should declare members".to_owned())?;
    let mut targets = 0_u64;
    let mut integration_targets = 0_u64;
    for member in members {
        let member = member
            .as_str()
            .ok_or_else(|| "workspace member paths should be text".to_owned())?;
        let manifest_path = Path::new(member).join("Cargo.toml");
        let manifest = parse_manifest(inventory, &manifest_path)?;
        targets += package_default_target_count(inventory, Path::new(member), &manifest);
        let explicit_tests = manifest
            .get("test")
            .and_then(toml::Value::as_array)
            .map_or(0_u64, |tests| tests.len() as u64);
        let test_entries = inventory
            .direct_entries_under(Path::new(member).join("tests"))
            .map(Path::to_path_buf)
            .collect::<Vec<_>>();
        let implicit_tests = implicit_test_target_count(&manifest, &test_entries);
        targets += explicit_tests + implicit_tests;
        integration_targets += explicit_tests + implicit_tests;
    }
    Ok((members.len() as u64, targets, integration_targets))
}

pub(super) fn implicit_test_target_count(
    manifest: &toml::Value,
    direct_test_entries: &[std::path::PathBuf],
) -> u64 {
    if manifest
        .get("package")
        .and_then(|package| package.get("autotests"))
        .and_then(toml::Value::as_bool)
        == Some(false)
    {
        return 0;
    }
    direct_test_entries
        .iter()
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .count() as u64
}

fn package_default_target_count(
    inventory: &WorkspaceSourceInventory,
    member: &Path,
    manifest: &toml::Value,
) -> u64 {
    let has_library =
        manifest.get("lib").is_some() || inventory.contains(member.join("src/lib.rs"));
    let explicit_bins = manifest
        .get("bin")
        .and_then(toml::Value::as_array)
        .map_or(0_u64, |bins| bins.len() as u64);
    let implicit_bin = u64::from(
        explicit_bins == 0
            && manifest.get("lib").is_none()
            && inventory.contains(member.join("src/main.rs")),
    );
    u64::from(has_library) + explicit_bins + implicit_bin
}

fn parse_manifest(
    inventory: &WorkspaceSourceInventory,
    path: &Path,
) -> Result<toml::Value, String> {
    inventory
        .source(path)
        .ok_or_else(|| format!("{} should be captured", path.display()))?
        .text()
        .parse::<toml::Value>()
        .map_err(|error| format!("{} should parse: {error}", path.display()))
}
