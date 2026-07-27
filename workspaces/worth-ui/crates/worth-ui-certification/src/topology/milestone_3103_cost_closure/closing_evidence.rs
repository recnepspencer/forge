use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::Value;

const BASE_COMMIT: &str = "ca50b0b5ed1ce53abf84c0117a3ab47b9db68149";
const HUMAN_DOC: &str = "workspaces/worth-ui/docs/application-lifecycle.md";
const ROADMAP: &str = "_docs/worth-ui/worth_ui_roadmap.md";

pub(super) fn load(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("`{}` should be readable: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("`{}` should parse as JSON: {error}", path.display()))
}

pub(super) fn audit(repository_root: &Path, evidence: &Value) -> Result<(), String> {
    audit_header(evidence)?;
    audit_source_revision(evidence)?;
    audit_environment(evidence)?;
    audit_lanes(evidence)?;
    audit_build_cost(evidence)?;
    audit_journey_cost(evidence)?;
    audit_artifacts(evidence)?;
    audit_topology(evidence)?;
    audit_platforms(evidence)?;
    audit_handoffs(evidence)?;
    audit_documentation(repository_root)
}

fn audit_header(evidence: &Value) -> Result<(), String> {
    if text(evidence, "schema")? != "worth-ui.milestone-3.10.3.phase-5-closing-evidence.v1"
        || text(evidence, "milestone")? != "3.10.3"
        || integer(evidence, "phase")? != 5
        || text(evidence, "status")? != "closed"
    {
        return Err("Milestone 3.10.3 Phase 5 evidence header drifted".to_owned());
    }
    Ok(())
}

fn audit_source_revision(evidence: &Value) -> Result<(), String> {
    let source = child(evidence, "source_revision")?;
    let digest = text(source, "aggregate_sha256")?;
    if text(source, "base_commit")? != BASE_COMMIT
        || integer(source, "present_source_files")? < 1
        || integer(source, "absent_tracked_source_entries")? < 0
        || integer(source, "derived_cargo_output_files")? != 0
        || digest.len() != 64
        || !digest.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("closing source revision is not exact and source-only".to_owned());
    }
    Ok(())
}

fn audit_environment(evidence: &Value) -> Result<(), String> {
    let environment = child(evidence, "environment")?;
    if text(environment, "operating_system")? != "Microsoft Windows 11 Home"
        || text(environment, "architecture")? != "x86_64"
        || text(environment, "rustc")? != "rustc 1.94.0 (4a4ef493e 2026-03-02)"
        || text(environment, "cargo")? != "cargo 1.94.0 (85eff7c80 2026-01-15)"
        || text(environment, "renderer")? != "eframe 0.31.1 glow"
        || text(environment, "window_server_posture")? != "interactive Windows desktop"
    {
        return Err("closing environment posture drifted".to_owned());
    }
    Ok(())
}

fn audit_lanes(evidence: &Value) -> Result<(), String> {
    let lanes = named(array(evidence, "proof_lanes")?, "name")?;
    for (name, tests, boundary) in [
        ("observation_protocol", 3, "library protocol semantics"),
        (
            "in_process_integration",
            4,
            "in-process production integration",
        ),
        (
            "windows_executable_world",
            6,
            "real executable product world",
        ),
    ] {
        let lane = lanes
            .get(name)
            .ok_or_else(|| format!("closing proof lane `{name}` is missing"))?;
        if integer(lane, "passed_tests")? != tests
            || integer(lane, "failed_tests")? != 0
            || text(lane, "claim_boundary")? != boundary
        {
            return Err(format!("closing proof lane `{name}` blurred its claim"));
        }
    }
    if lanes.len() != 3 {
        return Err("closing evidence should contain exactly three pulse lanes".to_owned());
    }
    Ok(())
}

fn audit_build_cost(evidence: &Value) -> Result<(), String> {
    let build = child(evidence, "build")?;
    within(build, "clean_seconds", "clean_budget_seconds")?;
    within(build, "warm_relink_seconds", "warm_relink_budget_seconds")?;
    within(
        build,
        "package_identifiable_bytes",
        "package_identifiable_budget_bytes",
    )?;
    if integer(build, "failed_retries")? != 0
        || !boolean(build, "isolated_target_removed")?
        || number(build, "whole_isolated_target_bytes")? <= 0.0
        || integer(build, "package_identifiable_files")? < 1
    {
        return Err("closing build evidence has retry, residue, or empty cost".to_owned());
    }
    Ok(())
}

fn audit_journey_cost(evidence: &Value) -> Result<(), String> {
    let journey = child(evidence, "ordinary_journey")?;
    within(
        journey,
        "first_publication_milliseconds",
        "first_publication_budget_milliseconds",
    )?;
    within(
        journey,
        "journey_milliseconds",
        "journey_budget_milliseconds",
    )?;
    within(journey, "lifecycle_events", "lifecycle_event_budget")?;
    within(journey, "lifecycle_bytes", "lifecycle_byte_budget")?;
    for (field, expected) in [
        ("source_actions", 3),
        ("native_captures", 4),
        ("process_launches", 1),
        ("native_windows", 1),
        ("scenario_retries", 0),
        ("normal_close_requests", 1),
    ] {
        if integer(journey, field)? != expected {
            return Err(format!("ordinary journey `{field}` should be {expected}"));
        }
    }
    if integer(journey, "window_lookups")? < 1
        || !boolean(journey, "successful_exit")?
        || !boolean(journey, "installation_removed")?
    {
        return Err("ordinary journey is missing native readiness or cleanup".to_owned());
    }
    Ok(())
}

fn audit_artifacts(evidence: &Value) -> Result<(), String> {
    let artifacts = child(evidence, "failure_artifacts")?;
    within(
        artifacts,
        "largest_diagnosed_artifact_bytes",
        "artifact_budget_bytes",
    )?;
    if !boolean(artifacts, "diagnosed_artifacts_removed")?
        || integer(artifacts, "passing_temp_roots")? != 0
        || integer(artifacts, "passing_child_processes")? != 0
        || integer(artifacts, "passing_native_windows")? != 0
    {
        return Err("passing closeout retains executable-world residue".to_owned());
    }
    Ok(())
}

fn audit_topology(evidence: &Value) -> Result<(), String> {
    let topology = child(evidence, "topology")?;
    for (field, expected) in [
        ("workspace_members", 12),
        ("workspace_cargo_targets", 23),
        ("integration_test_targets", 10),
        ("pulse_library_targets", 1),
        ("pulse_binary_targets", 1),
        ("pulse_executable_world_targets", 1),
        ("nested_cargo_invocations", 0),
    ] {
        if integer(topology, field)? != expected {
            return Err(format!("closing topology `{field}` should be {expected}"));
        }
    }
    Ok(())
}

fn audit_platforms(evidence: &Value) -> Result<(), String> {
    let platforms = named(array(evidence, "native_platforms")?, "platform")?;
    for (platform, posture) in [
        ("windows", "certified_executable"),
        ("linux_x11", "compile_only"),
        ("linux_wayland", "compile_only"),
        ("macos", "compile_only"),
    ] {
        let row = platforms
            .get(platform)
            .ok_or_else(|| format!("native platform `{platform}` is absent"))?;
        if text(row, "posture")? != posture {
            return Err(format!(
                "native platform `{platform}` overclaims certification"
            ));
        }
    }
    Ok(())
}

fn audit_handoffs(evidence: &Value) -> Result<(), String> {
    let rows = named(array(evidence, "successor_handoffs")?, "milestone")?;
    let required = (11..=23)
        .map(|minor| format!("3.{minor}"))
        .collect::<Vec<_>>();
    if rows.len() != required.len() {
        return Err("successor handoff count should cover 3.11 through 3.23".to_owned());
    }
    for milestone in required {
        let row = rows
            .get(milestone.as_str())
            .ok_or_else(|| format!("successor handoff `{milestone}` is missing"))?;
        if text(row, "world_extension")?.trim().is_empty() {
            return Err(format!("successor handoff `{milestone}` has no extension"));
        }
    }
    if text(child(evidence, "mature_world")?, "milestone")? != "3.24"
        || !boolean(
            child(evidence, "mature_world")?,
            "product_entry_already_mature",
        )?
    {
        return Err("Milestone 3.24 is not constrained to mature world infrastructure".to_owned());
    }
    Ok(())
}

fn audit_documentation(repository_root: &Path) -> Result<(), String> {
    let human = fs::read_to_string(repository_root.join(HUMAN_DOC))
        .map_err(|error| format!("human lifecycle doc should be readable: {error}"))?;
    for required in [
        "--source-root $sourceRoot",
        "-p worth-ui-certification --test application_contracts platform_pulse",
        "--features executable-world --test executable_world",
        "WORTH_UI_PLATFORM_PULSE_EVENT ",
        "worth-ui-platform-pulse-failure-<pid>-<ordinal>",
        "compile-only posture",
    ] {
        if !human.contains(required) {
            return Err(format!("human lifecycle doc is missing `{required}`"));
        }
    }
    let roadmap = fs::read_to_string(repository_root.join(ROADMAP))
        .map_err(|error| format!("roadmap should be readable: {error}"))?;
    if !roadmap.contains("Status: Closed on 2026-07-27. Phases 1 through 5 are closed") {
        return Err("roadmap does not close Milestone 3.10.3".to_owned());
    }
    for minor in 11..=23 {
        if !roadmap.contains(&format!("### Milestone 3.{minor}:")) {
            return Err(format!("roadmap is missing Milestone 3.{minor} handoff"));
        }
    }
    if !roadmap.contains("any future Milestone 3.24 begins after executable installation") {
        return Err("roadmap does not constrain 3.24 to mature world infrastructure".to_owned());
    }
    Ok(())
}

fn child<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value
        .get(key)
        .ok_or_else(|| format!("closing evidence should contain `{key}`"))
}

fn array<'a>(value: &'a Value, key: &str) -> Result<&'a [Value], String> {
    child(value, key)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("closing evidence `{key}` should be an array"))
}

fn named<'a>(rows: &'a [Value], key: &str) -> Result<BTreeMap<&'a str, &'a Value>, String> {
    let mut named = BTreeMap::new();
    for row in rows {
        let name = text(row, key)?;
        if named.insert(name, row).is_some() {
            return Err(format!("closing evidence duplicates `{name}`"));
        }
    }
    Ok(named)
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    child(value, key)?
        .as_str()
        .ok_or_else(|| format!("closing evidence `{key}` should be text"))
}

fn integer(value: &Value, key: &str) -> Result<i64, String> {
    child(value, key)?
        .as_i64()
        .ok_or_else(|| format!("closing evidence `{key}` should be an integer"))
}

fn number(value: &Value, key: &str) -> Result<f64, String> {
    child(value, key)?
        .as_f64()
        .ok_or_else(|| format!("closing evidence `{key}` should be numeric"))
}

fn boolean(value: &Value, key: &str) -> Result<bool, String> {
    child(value, key)?
        .as_bool()
        .ok_or_else(|| format!("closing evidence `{key}` should be boolean"))
}

fn within(value: &Value, actual: &str, budget: &str) -> Result<(), String> {
    let actual_value = number(value, actual)?;
    let budget_value = number(value, budget)?;
    if actual_value < 0.0 || actual_value > budget_value {
        Err(format!(
            "closing `{actual}` cost {actual_value} exceeds `{budget}` {budget_value}"
        ))
    } else {
        Ok(())
    }
}
