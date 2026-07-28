use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;
use serde_json::Value;

const CLOSING_PATH: &str = "_docs/worth-ui/milestone-3.10.1-phase-7-closing-evidence.json";
const OPENING_PATH: &str = "_docs/worth-ui/milestone-3.10.1-opening-baseline.json";
const PHASE8_CLOSEOUT_SOURCES: [&str; 2] = [
    "crates/worth-ui-certification/src/topology/milestone_3101_inventory/phase8_closeout.rs",
    "crates/worth-ui-certification/src/topology/milestone_3101_inventory/phase8_closeout_tests.rs",
];
const SUCCESSOR_MILESTONE_SOURCE_PREFIXES: [&str; 12] = [
    "crates/worth-ui-certification/src/scenario/application_authority_closure/platform_pulse_application.rs",
    "crates/worth-ui-certification/src/scenario/application_authority_closure/visual_identity_application.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/platform_pulse.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/visual_identity.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/visual_inspection.rs",
    "crates/worth-ui-certification/src/topology/inspection_topology_audit/",
    "crates/worth-ui-certification/src/topology/milestone_3102_pulse_seed/",
    "crates/worth-ui-certification/src/topology/milestone_3103_executable_world/",
    "crates/worth-ui-certification/src/topology/milestone_3103_product_contract/",
    "crates/worth-ui-certification/src/topology/milestone_3103_external_world/",
    "crates/worth-ui-certification/src/topology/milestone_3103_watched_replacement/",
    "crates/worth-ui-certification/src/topology/milestone_3103_cost_closure/",
];

const REQUIRED_OPERATION_CATEGORIES: &[&str] = &[
    "initial_file_acquisition_and_dsl_lowering",
    "rust_authored_canonicalization",
    "valid_local_replacement",
    "invalid_syntax_denial",
    "runtime_capability_denial",
    "unchanged_steady_frame",
    "changed_mounted_frame",
    "inspection_materialization",
    "verification_lanes",
];

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    repository_root: &Path,
) -> Result<(), String> {
    let opening = load_json(&repository_root.join(OPENING_PATH))?;
    let closing = load_json(&repository_root.join(CLOSING_PATH))?;
    validate_closing_evidence(inventory, repository_root, &opening, &closing)
}

fn validate_closing_evidence(
    inventory: &WorkspaceSourceInventory,
    repository_root: &Path,
    opening: &Value,
    closing: &Value,
) -> Result<(), String> {
    validate_header(opening, closing)?;
    validate_capture_posture(closing)?;
    validate_operation_costs(repository_root, closing)?;
    validate_measurements(opening, closing)?;
    validate_paired_measurements(closing)?;
    validate_compile_budget(closing)?;
    validate_summary(closing)?;
    validate_inventory_budget(inventory, closing)
}

fn validate_paired_measurements(closing: &Value) -> Result<(), String> {
    let paired = closing
        .get("holistic_qa_paired_measurement")
        .ok_or_else(|| "Phase 7 closing evidence lacks paired QA measurements".to_owned())?;
    if text(paired, "methodology")?.trim().is_empty() {
        return Err("paired QA measurement methodology is empty".to_owned());
    }
    validate_paired_sources(paired)?;
    let rows = validate_paired_operation_rows(paired)?;
    validate_paired_compile_posture(&rows)?;
    validate_focused_milestone_measurement(paired)
}

fn validate_paired_sources(paired: &Value) -> Result<(), String> {
    let opening = paired
        .get("opening_source")
        .ok_or_else(|| "paired QA measurement lacks opening source".to_owned())?;
    let current = paired
        .get("closing_source")
        .ok_or_else(|| "paired QA measurement lacks closing source".to_owned())?;
    if text(opening, "commit")? != text(current, "base_commit")? {
        return Err("paired QA sources do not share the opening commit".to_owned());
    }
    let targets = paired
        .get("targets")
        .ok_or_else(|| "paired QA measurement lacks target posture".to_owned())?;
    if targets
        .get("shared_artifacts")
        .and_then(Value::as_bool)
        .unwrap_or(true)
        || text(targets, "opening")? == text(targets, "closing")?
    {
        return Err("paired QA measurements must use distinct isolated targets".to_owned());
    }
    Ok(())
}

fn validate_paired_operation_rows(paired: &Value) -> Result<BTreeMap<&str, &Value>, String> {
    let rows = named_rows(array(paired, "measurements")?)?;
    let commands = BTreeMap::from([
        (
            "cold_workspace_check",
            "cargo check --quiet --manifest-path <repository>/workspaces/worth-ui/Cargo.toml --workspace --all-targets --all-features",
        ),
        (
            "warm_platform_check",
            "python scripts/ci/run_worth_ui_test_lane.py platform-check",
        ),
        (
            "warm_fast",
            "python scripts/ci/run_worth_ui_test_lane.py fast",
        ),
        (
            "warm_compile_contracts",
            "python scripts/ci/run_worth_ui_test_lane.py compile-contracts",
        ),
    ]);
    if rows.keys().copied().collect::<BTreeSet<_>>()
        != commands.keys().copied().collect::<BTreeSet<_>>()
    {
        return Err("paired QA measurement names differ".to_owned());
    }
    for (name, command) in commands {
        let row = rows[name];
        if text(row, "command")? != command
            || integer(row, "opening_exit_code")? != 0
            || integer(row, "closing_exit_code")? != 0
            || positive_number(row, "opening_seconds")?.is_nan()
            || positive_number(row, "closing_seconds")?.is_nan()
            || text(row, "adjudication")?.trim().is_empty()
        {
            return Err(format!("paired QA measurement `{name}` is not comparable"));
        }
    }
    Ok(rows)
}

fn validate_paired_compile_posture(rows: &BTreeMap<&str, &Value>) -> Result<(), String> {
    let compile = rows["warm_compile_contracts"];
    if integer(compile, "opening_cases")? != 24
        || integer(compile, "closing_cases")? != 35
        || integer(compile, "opening_cargo_sessions")? != 2
        || integer(compile, "closing_cargo_sessions")? != 2
    {
        return Err("paired QA compile-contract posture changed".to_owned());
    }
    Ok(())
}

fn validate_focused_milestone_measurement(paired: &Value) -> Result<(), String> {
    let focused = paired
        .get("focused_milestone")
        .ok_or_else(|| "paired QA measurement lacks focused milestone evidence".to_owned())?;
    if integer(focused, "repeated_warm_exit_code")? != 0
        || positive_number(focused, "repeated_warm_seconds")?.is_nan()
        || integer(focused, "unit_cases")? != 53
        || integer(focused, "integration_cases")? != 1
        || text(focused, "adjudication")?.trim().is_empty()
    {
        return Err("paired QA focused milestone evidence is incomplete".to_owned());
    }
    Ok(())
}

fn validate_header(opening: &Value, closing: &Value) -> Result<(), String> {
    if text(closing, "schema")? != "worth-ui.milestone-3.10.1.phase-7-closing-evidence.v1"
        || text(closing, "milestone")? != "3.10.1"
        || integer(closing, "phase")? != 7
        || text(closing, "comparison_label")? != "closing"
        || text(closing, "opening_baseline")? != OPENING_PATH
    {
        return Err("Phase 7 closing evidence header is invalid".to_owned());
    }
    if closing.get("machine") != opening.get("machine") {
        return Err("Phase 7 closing evidence must use the opening machine".to_owned());
    }
    if text(closing, "comparison_methodology")? != text(opening, "comparison_methodology")? {
        return Err("Phase 7 closing methodology differs from opening".to_owned());
    }
    Ok(())
}

fn validate_capture_posture(closing: &Value) -> Result<(), String> {
    let posture = closing
        .get("capture_posture")
        .ok_or_else(|| "closing evidence should contain capture_posture".to_owned())?;
    if text(posture, "target")? != "shared workspaces/worth-ui/target"
        || integer(posture, "failed_retries")? != 0
        || integer(posture, "integration_targets")? != 9
        || integer(posture, "successful_setup_warmups_excluded")? < 0
    {
        return Err("Phase 7 closing capture posture changed".to_owned());
    }
    Ok(())
}

fn validate_operation_costs(repository_root: &Path, closing: &Value) -> Result<(), String> {
    let rows = array(closing, "operation_costs")?;
    let mut observed = BTreeSet::new();
    for row in rows {
        let category = text(row, "category")?;
        if !observed.insert(category) {
            return Err(format!("duplicate Phase 7 operation category `{category}`"));
        }
        validate_successful_evidence_row(row, category)?;
        let source = text(row, "source")?;
        let witness = text(row, "witness")?;
        let source_text = fs::read_to_string(repository_root.join(source))
            .map_err(|error| format!("operation source `{source}` should be readable: {error}"))?;
        if !source_text.contains(witness) {
            return Err(format!(
                "operation category `{category}` witness `{witness}` is absent from `{source}`"
            ));
        }
    }
    let expected = REQUIRED_OPERATION_CATEGORIES.iter().copied().collect();
    if observed != expected {
        return Err(format!(
            "Phase 7 operation categories differ: observed={observed:?}, expected={expected:?}"
        ));
    }
    Ok(())
}

fn validate_successful_evidence_row(row: &Value, label: &str) -> Result<(), String> {
    for field in [
        "command",
        "structural_observation",
        "independent_oracle",
        "adjudication",
    ] {
        if text(row, field)?.trim().is_empty() {
            return Err(format!("Phase 7 evidence `{label}` has empty `{field}`"));
        }
    }
    if integer(row, "exit_code")? != 0 || positive_number(row, "duration_seconds")?.is_nan() {
        return Err(format!("Phase 7 evidence `{label}` did not pass"));
    }
    Ok(())
}

fn validate_measurements(opening: &Value, closing: &Value) -> Result<(), String> {
    let opening_rows = named_rows(array(opening, "measurements")?)?;
    let closing_rows = named_rows(array(closing, "measurements")?)?;
    let required = opening
        .get("closing_rules")
        .ok_or_else(|| "opening baseline should contain closing_rules".to_owned())
        .and_then(|rules| strings(rules, "required_measurement_names"))?
        .into_iter()
        .collect::<BTreeSet<_>>();
    if closing_rows.keys().copied().collect::<BTreeSet<_>>() != required {
        return Err("Phase 7 closing measurement names differ from opening".to_owned());
    }
    for name in required {
        let opening_row = opening_rows
            .get(name)
            .ok_or_else(|| format!("opening measurement `{name}` is absent"))?;
        let closing_row = closing_rows
            .get(name)
            .ok_or_else(|| format!("closing measurement `{name}` is absent"))?;
        if text(closing_row, "command")? != text(opening_row, "command")?
            || text(closing_row, "target_posture")? != text(opening_row, "target_posture")?
            || integer(closing_row, "exit_code")? != 0
            || positive_number(closing_row, "seconds")?.is_nan()
            || positive_number(closing_row, "opening_seconds")?
                != positive_number(opening_row, "seconds")?
            || text(closing_row, "adjudication")?.trim().is_empty()
        {
            return Err(format!(
                "Phase 7 closing measurement `{name}` is not comparable"
            ));
        }
    }
    Ok(())
}

fn validate_compile_budget(closing: &Value) -> Result<(), String> {
    let budget = closing
        .get("compile_contracts")
        .ok_or_else(|| "closing evidence should contain compile_contracts".to_owned())?;
    if integer(budget, "fail_targets")? != 23
        || integer(budget, "pass_targets")? != 12
        || integer(budget, "cargo_sessions")? != 2
        || integer(budget, "exit_code")? != 0
    {
        return Err("Phase 7 compile-contract budget changed".to_owned());
    }
    Ok(())
}

fn validate_summary(closing: &Value) -> Result<(), String> {
    let summary = strings(closing, "comparison_summary")?;
    if summary.is_empty() || summary.iter().any(|row| row.trim().is_empty()) {
        return Err("Phase 7 closing comparison summary is empty".to_owned());
    }
    Ok(())
}

fn validate_inventory_budget(
    inventory: &WorkspaceSourceInventory,
    closing: &Value,
) -> Result<(), String> {
    let budget = closing
        .get("structural_inventory")
        .ok_or_else(|| "closing evidence should contain structural_inventory".to_owned())?;
    let certification_files = inventory
        .rust_files_under("crates/worth-ui-certification/src")
        .filter(|source| {
            let path = source.relative_path().to_string_lossy().replace('\\', "/");
            belongs_to_phase7_inventory(&path)
        })
        .count();
    if integer(budget, "certification_audience_source_files")? as usize != certification_files
        || integer(budget, "total_integration_targets")? != 9
    {
        return Err("Phase 7 closing structural inventory changed".to_owned());
    }
    Ok(())
}

fn belongs_to_phase7_inventory(path: &str) -> bool {
    !PHASE8_CLOSEOUT_SOURCES.contains(&path)
        && !SUCCESSOR_MILESTONE_SOURCE_PREFIXES
            .iter()
            .any(|prefix| path.starts_with(prefix))
}

fn named_rows(rows: &[Value]) -> Result<BTreeMap<&str, &Value>, String> {
    let mut named = BTreeMap::new();
    for row in rows {
        let name = text(row, "name")?;
        if named.insert(name, row).is_some() {
            return Err(format!("duplicate measurement `{name}`"));
        }
    }
    Ok(named)
}

fn load_json(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("{} should be readable: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("{} should be valid JSON: {error}", path.display()))
}

fn array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("closing evidence should contain array `{key}`"))
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

fn positive_number(value: &Value, key: &str) -> Result<f64, String> {
    value
        .get(key)
        .and_then(Value::as_f64)
        .filter(|number| number.is_finite() && *number > 0.0)
        .ok_or_else(|| format!("closing evidence should contain positive number `{key}`"))
}

fn strings<'a>(value: &'a Value, key: &str) -> Result<Vec<&'a str>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("closing evidence should contain array `{key}`"))?
        .iter()
        .map(|row| {
            row.as_str()
                .ok_or_else(|| format!("`{key}` entries should be strings"))
        })
        .collect()
}

#[cfg(test)]
#[path = "phase7_closing_evidence_tests.rs"]
mod tests;
