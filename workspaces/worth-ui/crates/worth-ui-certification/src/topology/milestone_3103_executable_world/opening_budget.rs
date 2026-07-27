use std::collections::BTreeSet;

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

use crate::topology::WorkspaceSourceInventory;

use super::evidence_document::{json_integer, json_number, json_object, json_text};

const OPENING_MEMBERS: i64 = 12;
const OPENING_TARGETS: i64 = 21;
const OPENING_INTEGRATION_TARGETS: i64 = 9;

pub(super) fn audit(
    _inventory: &WorkspaceSourceInventory,
    contract: &TomlValue,
    baseline: &JsonValue,
) -> Result<(), String> {
    audit_header(baseline)?;
    audit_environment(baseline)?;
    audit_source_fingerprints(baseline)?;
    audit_opening_topology(baseline)?;
    audit_inherited_measurements(baseline)?;
    audit_closing_budgets(baseline)?;
    audit_contract_cost_budgets(contract)?;
    audit_historical_phase_order(contract)
}

fn audit_header(baseline: &JsonValue) -> Result<(), String> {
    if json_text(baseline, "schema")? != "worth-ui.milestone-3.10.3.phase-1-opening-baseline.v1"
        || json_text(baseline, "milestone")? != "3.10.3"
        || json_integer(baseline, "phase")? != 1
        || json_text(baseline, "captured_from_commit")?
            != "ca50b0b5ed1ce53abf84c0117a3ab47b9db68149"
    {
        return Err("Milestone 3.10.3 Phase 1 opening baseline header drifted".to_owned());
    }
    json_text(baseline, "source_posture")?;
    json_text(baseline, "amendment_rule")?;
    Ok(())
}

fn audit_environment(baseline: &JsonValue) -> Result<(), String> {
    let environment = json_object(baseline, "environment")?;
    for field in [
        "operating_system",
        "operating_system_version",
        "architecture",
        "processor",
        "rustc",
        "cargo",
        "host",
        "renderer",
        "window_server_posture",
    ] {
        json_text(environment, field)?;
    }
    if json_integer(environment, "physical_cores")? <= 0
        || json_integer(environment, "logical_processors")?
            < json_integer(environment, "physical_cores")?
    {
        return Err("opening environment CPU posture is invalid".to_owned());
    }
    Ok(())
}

fn audit_source_fingerprints(baseline: &JsonValue) -> Result<(), String> {
    let fingerprints = json_object(baseline, "source_fingerprints_sha256")?
        .as_object()
        .ok_or_else(|| "source fingerprints should be an object".to_owned())?;
    let expected = [
        "workspaces/worth-ui/Cargo.toml",
        "workspaces/worth-ui/Cargo.lock",
        "workspaces/worth-ui/apps/platform-pulse/Cargo.toml",
        "workspaces/worth-ui/apps/platform-pulse/app/main.wui",
        "workspaces/worth-ui/apps/platform-pulse/src/main.rs",
        "workspaces/worth-ui/apps/platform-pulse/src/application.rs",
        "workspaces/worth-ui/apps/platform-pulse/src/native_frame.rs",
        "workspaces/worth-ui/apps/platform-pulse/src/source_watch.rs",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let actual = fingerprints
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!(
            "opening source fingerprints should cover {expected:?}; found {actual:?}"
        ));
    }
    if fingerprints
        .values()
        .any(|value| value.as_str().is_none_or(|hash| hash.len() != 64))
    {
        return Err("opening source fingerprints must be SHA-256 hex strings".to_owned());
    }
    Ok(())
}

fn audit_opening_topology(baseline: &JsonValue) -> Result<(), String> {
    let topology = json_object(baseline, "topology")?;
    for (field, expected) in [
        ("workspace_members", OPENING_MEMBERS),
        ("workspace_cargo_targets", OPENING_TARGETS),
        ("integration_test_targets", OPENING_INTEGRATION_TARGETS),
        ("pulse_binary_targets", 1),
        ("pulse_library_targets", 0),
        ("pulse_integration_test_targets", 0),
        ("pulse_executable_world_features", 0),
        ("nested_cargo_invocations", 0),
        ("automatic_retry_budget", 0),
    ] {
        if json_integer(topology, field)? != expected {
            return Err(format!(
                "opening topology `{field}` should remain {expected}"
            ));
        }
    }
    Ok(())
}

fn audit_inherited_measurements(baseline: &JsonValue) -> Result<(), String> {
    let measurements = json_object(baseline, "inherited_product_measurements")?;
    if json_text(measurements, "provenance")?
        != "_docs/worth-ui/milestone-3.10.2-phase-4-closing-cost-evidence.json"
        || json_number(measurements, "clean_pulse_build_link_seconds")? != 142.257
        || json_number(measurements, "warm_pulse_relink_seconds")? != 3.636
        || json_integer(
            measurements,
            "manual_launch_to_first_publication_upper_bound_milliseconds",
        )? != 258
        || json_integer(measurements, "automated_native_window_captures")? != 0
        || json_integer(measurements, "automated_native_close_requests")? != 0
        || json_integer(measurements, "automated_child_process_teardowns")? != 0
        || measurements
            .get("automated_executable_world_journey_seconds")
            .is_none_or(|value| !value.is_null())
    {
        return Err(
            "opening measurements must preserve manual evidence and absent automation".into(),
        );
    }
    json_text(measurements, "inheritance_reason")?;
    Ok(())
}

fn audit_closing_budgets(baseline: &JsonValue) -> Result<(), String> {
    let budgets = json_object(baseline, "closing_budgets")?;
    let exact = [
        ("maximum_workspace_members", 12),
        ("maximum_workspace_cargo_targets", 23),
        ("maximum_integration_test_targets", 10),
        ("maximum_pulse_library_targets", 1),
        ("maximum_pulse_integration_test_targets", 1),
        ("maximum_pulse_binary_targets", 1),
        ("maximum_ordinary_journey_seconds", 20),
        ("maximum_transition_seconds", 5),
        ("minimum_first_frame_liveness_hold_milliseconds", 500),
        ("maximum_additional_retained_bytes", 524_288_000),
        ("maximum_failure_artifact_bytes", 67_108_864),
        ("maximum_lifecycle_events", 256),
        ("maximum_lifecycle_observation_bytes", 1_048_576),
        ("maximum_automatic_retries", 0),
        ("maximum_nested_cargo_invocations", 0),
        ("maximum_ordinary_child_processes", 1),
        ("maximum_ordinary_native_windows", 1),
        ("maximum_unchanged_frame_observation_work", 0),
    ];
    for (field, expected) in exact {
        if json_integer(budgets, field)? != expected {
            return Err(format!("closing budget `{field}` should be {expected}"));
        }
    }
    if json_number(budgets, "maximum_clean_build_link_seconds")? != 240.0
        || json_number(budgets, "maximum_warm_relink_seconds")? != 20.0
    {
        return Err("closing build budgets drifted".to_owned());
    }
    Ok(())
}

fn audit_contract_cost_budgets(contract: &TomlValue) -> Result<(), String> {
    let budgets = contract
        .get("cost_budgets")
        .ok_or_else(|| "Phase 1 inventory should freeze [cost_budgets]".to_owned())?;
    for (field, expected) in [
        ("new_library_targets", 1),
        ("new_integration_targets", 1),
        ("nested_cargo_invocations", 0),
        ("ordinary_child_processes", 1),
        ("ordinary_native_windows", 1),
        ("transition_timeout_seconds", 5),
        ("first_frame_liveness_hold_milliseconds", 500),
        ("ordinary_journey_seconds", 20),
        ("automatic_retries", 0),
        ("maximum_events", 256),
        ("maximum_observation_bytes", 1_048_576),
        ("maximum_failure_artifact_bytes", 67_108_864),
        ("clean_build_link_seconds", 240),
        ("warm_relink_seconds", 20),
        ("maximum_additional_retained_bytes", 524_288_000),
        ("unchanged_frame_observation_work", 0),
    ] {
        if budgets.get(field).and_then(TomlValue::as_integer) != Some(expected) {
            return Err(format!(
                "Phase 1 cost budget `{field}` should be {expected}"
            ));
        }
    }
    Ok(())
}

fn audit_historical_phase_order(contract: &TomlValue) -> Result<(), String> {
    let phase = contract
        .get("phase_order")
        .ok_or_else(|| "Phase 1 inventory should freeze [phase_order]".to_owned())?;
    if phase
        .get("completed_through")
        .and_then(TomlValue::as_integer)
        != Some(1)
    {
        return Err("Phase 1 historical contract must remain completed through phase 1".into());
    }
    Ok(())
}
