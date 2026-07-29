use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

const EVIDENCE_PATH: &str = "_docs/worth-ui/milestone-3.10.2-phase-4-closing-cost-evidence.json";
const LIFECYCLE_DOC: &str = "workspaces/worth-ui/docs/application-lifecycle.md";
const PHASE3_LEDGER: &str = "_docs/worth-ui/milestone-3.10.2-phase-3-proof-ledger.csv";
const PHASE4_LEDGER: &str = "_docs/worth-ui/milestone-3.10.2-phase-4-proof-ledger.csv";
const SPEC: &str = "_docs/worth-ui/milestone-3.10.2.md";
const SUCCESSOR_SPEC: &str = "_docs/worth-ui/milestone-3.10.3.md";
const ROADMAP: &str = "_docs/worth-ui/worth_ui_roadmap.md";

pub(super) fn audit(repository_root: &Path) -> Result<(), String> {
    let evidence = super::evidence_document::load_json(&repository_root.join(EVIDENCE_PATH))?;
    audit_header(&evidence)?;
    audit_budgets(&evidence)?;
    audit_topology(&evidence)?;
    audit_frame_cost(&evidence)?;
    audit_lifecycle(&evidence)?;
    audit_documentation(repository_root)?;
    audit_ledgers(repository_root)?;
    audit_source_witnesses(repository_root)
}

fn audit_header(evidence: &Value) -> Result<(), String> {
    if text(evidence, "schema")? != "worth-ui.milestone-3.10.2.phase-4-closing-cost-evidence.v1"
        || text(evidence, "milestone")? != "3.10.2"
        || integer(evidence, "phase")? != 4
    {
        return Err("3.10.2 closing evidence header drifted".to_owned());
    }
    Ok(())
}

fn audit_budgets(evidence: &Value) -> Result<(), String> {
    let build = object(evidence, "build")?;
    require_within(build, "clean_link_seconds", "clean_link_budget_seconds")?;
    require_within(build, "warm_link_seconds", "warm_link_budget_seconds")?;
    if integer(build, "failed_retries")? != 0
        || integer(build, "pulse_application_local_warnings")? != 0
    {
        return Err("pulse build evidence contains retry or app-local warning debt".to_owned());
    }
    let launch = object(evidence, "launch")?;
    require_within(
        launch,
        "launch_to_first_publication_upper_bound_milliseconds",
        "budget_milliseconds",
    )?;
    if text(launch, "evidence_marker")? != "WORTH_UI_PLATFORM_PULSE_PUBLISHED"
        || !boolean(launch, "native_process_remained_live_after_publication")?
    {
        return Err("native launch evidence is incomplete".to_owned());
    }
    let artifacts = object(evidence, "retained_artifacts")?;
    require_within(
        artifacts,
        "package_identifiable_retained_bytes",
        "retained_budget_bytes",
    )?;
    if !boolean(artifacts, "isolated_target_removed_after_measurement")?
        || integer(artifacts, "isolated_full_dependency_target_bytes")?
            <= integer(artifacts, "package_identifiable_retained_bytes")?
    {
        return Err("retained-artifact evidence conflates package and target cost".to_owned());
    }
    Ok(())
}

fn audit_topology(evidence: &Value) -> Result<(), String> {
    let topology = object(evidence, "topology")?;
    if integer(topology, "workspace_members")? != 12
        || integer(topology, "workspace_cargo_targets")? != 21
        || integer(topology, "integration_test_targets")? != 9
        || integer(topology, "native_executables_added")? != 1
        || integer(topology, "compiler_contract_sessions_added")? != 0
        || integer(topology, "nested_cargo_invocations_added")? != 0
        || boolean(topology, "ordinary_tests_launch_native_windows")?
    {
        return Err("pulse closing topology exceeds the frozen budget".to_owned());
    }
    let dependencies = strings(topology, "pulse_dependencies")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = ["eframe", "worth-ui", "worth-ui-host-egui"]
        .into_iter()
        .collect::<BTreeSet<_>>();
    if dependencies != expected {
        return Err("pulse closing dependency set drifted".to_owned());
    }
    Ok(())
}

fn audit_frame_cost(evidence: &Value) -> Result<(), String> {
    let frame = object(evidence, "initial_native_frame")?;
    let expected = [
        ("presented_surfaces", 1),
        ("adapter_translated_rows", 6),
        ("adapter_translated_bytes", 560),
        ("native_shapes", 1),
        ("resource_cache_hits", 0),
        ("resource_cache_misses", 0),
        ("asynchronous_handoffs", 0),
    ];
    if expected
        .into_iter()
        .any(|(field, value)| integer(frame, field) != Ok(value))
    {
        return Err("initial native-frame cost evidence drifted".to_owned());
    }
    Ok(())
}

fn audit_lifecycle(evidence: &Value) -> Result<(), String> {
    let lifecycle = object(evidence, "real_watched_lifecycle")?;
    let exact = [
        ("settled_snapshots", 4),
        ("replacement_publications", 2),
        ("predecessor_preservations", 1),
        ("typed_dsl_denials", 1),
        ("released_host_surfaces", 1),
        ("unfinished_mounted_shutdown_attempts", 0),
    ];
    if exact
        .into_iter()
        .any(|(field, value)| integer(lifecycle, field) != Ok(value))
        || integer(lifecycle, "minimum_observed_os_notifications")? < 1
        || !boolean(lifecycle, "watcher_shutdown_succeeded")?
        || !boolean(lifecycle, "host_session_release_succeeded")?
    {
        return Err("real watched lifecycle evidence drifted".to_owned());
    }
    Ok(())
}

fn audit_documentation(repository_root: &Path) -> Result<(), String> {
    let document = read(repository_root, LIFECYCLE_DOC)?;
    let normalized = document.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        "cargo run --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-platform-pulse",
        "workspaces/worth-ui/apps/platform-pulse/app/main.wui",
        "theme.platform_pulse.blue",
        "theme.platform_pulse.green",
        "component platform.pulse.component.seed {",
        "FirstFramePublished",
        "RebindPublished",
        "VisualComparison",
        "Close the native window normally",
    ] {
        if !normalized.contains(required) {
            return Err(format!("Platform Pulse documentation lacks `{required}`"));
        }
    }
    let spec = read(repository_root, SPEC)?;
    let successor_spec = read(repository_root, SUCCESSOR_SPEC)?;
    let roadmap = read(repository_root, ROADMAP)?;
    let normalized_spec = spec.split_whitespace().collect::<Vec<_>>().join(" ");
    let normalized_successor = successor_spec
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let normalized_roadmap = roadmap.split_whitespace().collect::<Vec<_>>().join(" ");
    if !normalized_spec.contains("Status: Complete.")
        || !normalized_spec.contains("automated executable-world certification")
        || !normalized_successor
            .contains("Status: Complete. Phases 1 through 5 closed on 2026-07-27.")
        || !normalized_roadmap.contains(
            "Status: Product capability complete. A later evidence audit found that the automated pulse worlds stop below the executable composition root.",
        )
        || !normalized_roadmap.contains(
            "Status: Closed on 2026-07-27. Phases 1 through 5 are closed; this corrective",
        )
        || !normalized_roadmap
            .contains("Milestone 3.10.3 is the permanent executable-world foundation")
    {
        return Err(
            "3.10.2 product closure or 3.10.3 executable-world handoff drifted".to_owned(),
        );
    }
    let milestone_3102 = roadmap
        .find("### Milestone 3.10.2:")
        .ok_or_else(|| "roadmap lacks Milestone 3.10.2".to_owned())?;
    let milestone_3103 = roadmap
        .find("### Milestone 3.10.3:")
        .ok_or_else(|| "roadmap lacks Milestone 3.10.3".to_owned())?;
    let milestone_311 = roadmap
        .find("### Milestone 3.11:")
        .ok_or_else(|| "roadmap lacks Milestone 3.11".to_owned())?;
    if !(milestone_3102 < milestone_3103 && milestone_3103 < milestone_311) {
        return Err("3.10.3 must remain between 3.10.2 and 3.11".to_owned());
    }
    Ok(())
}

fn audit_ledgers(repository_root: &Path) -> Result<(), String> {
    for (path, expected_rows) in [(PHASE3_LEDGER, 12), (PHASE4_LEDGER, 12)] {
        let ledger = read(repository_root, path)?;
        let rows = ledger.lines().skip(1).collect::<Vec<_>>();
        if rows.len() != expected_rows
            || rows.iter().any(|row| !row.ends_with("\"PROVED\""))
            || ledger.contains("\"OPEN\"")
        {
            return Err(format!("closing ledger `{path}` is not fully proved"));
        }
    }
    Ok(())
}

fn audit_source_witnesses(repository_root: &Path) -> Result<(), String> {
    let courtroom = read(
        repository_root,
        "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/platform_pulse.rs",
    )?;
    let scheduler = read(
        repository_root,
        "workspaces/worth-ui/crates/worth-ui-runtime/src/runtime/allocation_frame_dispatch/framework_turn/scheduler.rs",
    )?;
    let native = read(
        repository_root,
        "workspaces/worth-ui/apps/platform-pulse/src/native_frame.rs",
    )?;
    let native_rebind = read(
        repository_root,
        "workspaces/worth-ui/apps/platform-pulse/src/native_frame/rebind.rs",
    )?;
    let protocol = read(
        repository_root,
        "workspaces/worth-ui/apps/platform-pulse/src/observation_contract/envelope.rs",
    )?;
    for required in [
        "adapter_cost.translated_rows(), 8",
        "std::mem::size_of::<UiMountedNodeProjectionView>()",
        "std::mem::size_of::<UiMountedFilledRectMechanic>()",
        "adapter_cost.native_resource_cache_hits(), 0",
    ] {
        if !courtroom.contains(required) {
            return Err(format!("pulse exact-cost courtroom lacks `{required}`"));
        }
    }
    if !scheduler.contains("24 * 1024")
        || !scheduler.contains("512 * 1024")
        || !protocol.contains("\"worth-ui.platform-pulse.lifecycle-observation\"")
        || !protocol.contains("\"WORTH_UI_PLATFORM_PULSE_EVENT \"")
        || !native.contains("self.publisher.first_frame(&source, &publication)")
        || !native.contains("self.publisher.replacement(")
        || !native.contains(".application_publication()")
        || !native.contains(".mounted_publication()")
        || !native.contains(".compare_after_rebind(")
        || !native_rebind.contains(".begin_source_rebind(")
        || native.contains("WORTH_UI_PLATFORM_PULSE_PUBLISHED")
        || native.contains("WORTH_UI_PLATFORM_PULSE_REPLACED")
        || native.contains("pulse-checkpoint")
    {
        return Err("pulse stack or publication source witness drifted".to_owned());
    }
    Ok(())
}

fn read(repository_root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(repository_root.join(relative))
        .map_err(|error| format!("`{relative}` should be readable: {error}"))
}

fn object<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value
        .get(key)
        .filter(|row| row.is_object())
        .ok_or_else(|| format!("closing evidence should contain object `{key}`"))
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("closing evidence should contain string `{key}`"))
}

fn integer(value: &Value, key: &str) -> Result<i64, String> {
    value
        .get(key)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("closing evidence should contain integer `{key}`"))
}

fn number(value: &Value, key: &str) -> Result<f64, String> {
    value
        .get(key)
        .and_then(Value::as_f64)
        .filter(|number| number.is_finite() && *number >= 0.0)
        .ok_or_else(|| format!("closing evidence should contain number `{key}`"))
}

fn boolean(value: &Value, key: &str) -> Result<bool, String> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("closing evidence should contain boolean `{key}`"))
}

fn strings<'a>(value: &'a Value, key: &str) -> Result<Vec<&'a str>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("closing evidence should contain array `{key}`"))?
        .iter()
        .map(|row| {
            row.as_str()
                .ok_or_else(|| format!("closing evidence `{key}` entries should be strings"))
        })
        .collect()
}

fn require_within(value: &Value, actual: &str, budget: &str) -> Result<(), String> {
    if number(value, actual)? > number(value, budget)? {
        return Err(format!("closing measurement `{actual}` exceeds `{budget}`"));
    }
    Ok(())
}
