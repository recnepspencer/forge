use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use toml::Value;

use super::evidence_document::{toml_rows, toml_text};

const LANE_IDS: &[&str] = &[
    "L01_COMPILE_TOPOLOGY",
    "L02_FOCUSED_SEMANTIC",
    "L03_IN_PROCESS_INTEGRATION",
    "L04_MANUAL_EXECUTABLE",
    "L05_AUTOMATED_EXECUTABLE_WORLD",
];
const LANE_NAMES: &[&str] = &[
    "compile-and-topology",
    "focused-semantic",
    "in-process-integration",
    "manual-executable",
    "automated-executable-world",
];
const PROOF_IDS: &[&str] = &[
    "P01_MILESTONE_3102_TOPOLOGY",
    "P02_HEADLESS_STATIC_PAINT",
    "P03_EGUI_STATIC_PAINT",
    "P04_PUBLIC_NATIVE_SHELL",
    "P05_REAL_WATCHER_LIFECYCLE",
    "P06_MEASURED_HUMAN_LAUNCH",
    "P07_AUTOMATED_EXECUTABLE_WORLD",
];
const PROOF_LANES: &[(&str, &str)] = &[
    ("P01_MILESTONE_3102_TOPOLOGY", "compile-and-topology"),
    ("P02_HEADLESS_STATIC_PAINT", "in-process-integration"),
    ("P03_EGUI_STATIC_PAINT", "in-process-integration"),
    ("P04_PUBLIC_NATIVE_SHELL", "in-process-integration"),
    ("P05_REAL_WATCHER_LIFECYCLE", "in-process-integration"),
    ("P06_MEASURED_HUMAN_LAUNCH", "manual-executable"),
    (
        "P07_AUTOMATED_EXECUTABLE_WORLD",
        "automated-executable-world",
    ),
];
const ENTRY_EDGE_IDS: &[&str] = &[
    "E01_CARGO_TARGET_TO_MAIN",
    "E02_MAIN_TO_NATIVE_EVENT_LOOP",
    "E03_CREATION_TO_NATIVE_FRAME",
    "E04_SOURCE_ROOT_TO_INITIAL_SNAPSHOT",
    "E05_SNAPSHOT_TO_PREPARED_APPLICATION",
    "E06_NATIVE_FRAME_TO_PUBLIC_SHELL",
    "E07_WATCHER_TO_WORKER",
    "E08_SHELL_TO_FIRST_FRAME",
    "E09_SETTLED_EDIT_TO_REPLACEMENT",
    "E10_NATIVE_FRAME_DROP_TO_WATCHER_SHUTDOWN",
    "E11_NATIVE_FRAME_DROP_TO_HOST_SHUTDOWN",
];
const IN_PROCESS_TESTS: &[&str] = &[
    "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_static_paint.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/platform_pulse.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/platform_pulse_lifecycle.rs",
];

pub(super) fn audit(repository_root: &Path, document: &Value) -> Result<(), String> {
    audit_lanes(document)?;
    audit_existing_proofs(repository_root, document)?;
    audit_entry_edges(document)?;
    audit_in_process_test_names(repository_root)?;
    audit_historical_correction(repository_root)
}

fn audit_lanes(document: &Value) -> Result<(), String> {
    require_exact_ids(document, "evidence_lane", LANE_IDS)?;
    let names = toml_rows(document, "evidence_lane")?
        .iter()
        .map(|row| toml_text(row, "name"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = LANE_NAMES.iter().copied().collect::<BTreeSet<_>>();
    if names != expected {
        return Err(format!(
            "Phase 1 evidence lane names should be {expected:?}; found {names:?}"
        ));
    }
    audit_required_fields(
        document,
        "evidence_lane",
        &["name", "entry_boundary", "claim_ceiling"],
    )
}

fn audit_existing_proofs(repository_root: &Path, document: &Value) -> Result<(), String> {
    require_exact_ids(document, "existing_proof", PROOF_IDS)?;
    audit_required_fields(
        document,
        "existing_proof",
        &["path", "lane", "world", "boundary", "oracle", "claim"],
    )?;
    let rows = toml_rows(document, "existing_proof")?;
    for (id, expected_lane) in PROOF_LANES {
        let proof = rows
            .iter()
            .find(|row| row.get("id").and_then(Value::as_str) == Some(*id))
            .ok_or_else(|| format!("existing proof `{id}` is missing"))?;
        let actual_lane = toml_text(proof, "lane")?;
        if actual_lane != *expected_lane {
            return Err(format!(
                "existing proof `{id}` should remain `{expected_lane}`; found `{actual_lane}`"
            ));
        }
    }
    for proof in rows.iter().take(6) {
        let path = toml_text(proof, "path")?;
        if !repository_root.join(path).is_file() {
            return Err(format!("existing proof path `{path}` should exist"));
        }
    }
    let executable = rows
        .iter()
        .find(|row| row.get("id").and_then(Value::as_str) == Some("P07_AUTOMATED_EXECUTABLE_WORLD"))
        .ok_or_else(|| "automated executable-world proof row is missing".to_owned())?;
    if toml_text(executable, "claim")? != "ABSENT_UNTIL_PHASE_3"
        || toml_text(executable, "world")? != "not yet implemented"
        || toml_text(executable, "boundary")? != "not yet implemented"
        || toml_text(executable, "oracle")? != "not yet implemented"
    {
        return Err(
            "Phase 1 must retain its historical automated executable-world absence record"
                .to_owned(),
        );
    }
    let successor = repository_root.join(toml_text(executable, "path")?);
    if successor.exists() && !successor.is_file() {
        return Err("the Phase 3 executable-world successor must be a test entry file".to_owned());
    }
    Ok(())
}

fn audit_entry_edges(document: &Value) -> Result<(), String> {
    require_exact_ids(document, "product_entry_edge", ENTRY_EDGE_IDS)?;
    audit_required_fields(
        document,
        "product_entry_edge",
        &[
            "producer",
            "consumer",
            "cardinality",
            "lifetime",
            "authority_owner",
            "failure_owner",
            "cost_class",
            "forbidden_shortcut",
        ],
    )
}

fn audit_in_process_test_names(repository_root: &Path) -> Result<(), String> {
    for relative in IN_PROCESS_TESTS {
        let source = read(repository_root, relative)?;
        for forbidden in ["end_to_end", "executable_world", "product_entry"] {
            if source.contains(forbidden) {
                return Err(format!(
                    "in-process proof `{relative}` must not claim `{forbidden}`"
                ));
            }
        }
    }
    let headless = read(repository_root, IN_PROCESS_TESTS[0])?;
    if !headless.contains("in_process_real_filesystem_pulse") {
        return Err("the headless pulse proof must name its in-process boundary".to_owned());
    }
    let adapter = read(repository_root, IN_PROCESS_TESTS[1])?;
    for required in [
        "in_process_checked_in_pulse",
        "in_process_public_native_shell",
    ] {
        if !adapter.contains(required) {
            return Err(format!(
                "the egui pulse proof must name its in-process boundary with `{required}`"
            ));
        }
    }
    let lifecycle = read(repository_root, IN_PROCESS_TESTS[2])?;
    if !lifecycle.contains("in_process_real_watcher") {
        return Err("the watched pulse proof must name its in-process boundary".to_owned());
    }
    Ok(())
}

fn audit_historical_correction(repository_root: &Path) -> Result<(), String> {
    let spec = normalize(&read(
        repository_root,
        "_docs/worth-ui/milestone-3.10.2.md",
    )?);
    let lifecycle = normalize(&read(
        repository_root,
        "workspaces/worth-ui/docs/application-lifecycle.md",
    )?);
    for required in [
        "integration in-process",
        "not automated executable-world certification",
        "Milestone 3.10.3",
    ] {
        if !spec.contains(required) && !lifecycle.contains(required) {
            return Err(format!(
                "historical evidence correction should retain `{required}`"
            ));
        }
    }
    Ok(())
}

fn require_exact_ids(document: &Value, family: &str, expected: &[&str]) -> Result<(), String> {
    let actual = toml_rows(document, family)?
        .iter()
        .map(|row| toml_text(row, "id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "Phase 1 `{family}` ids should be {expected:?}; found {actual:?}"
        ))
    }
}

fn audit_required_fields(document: &Value, family: &str, fields: &[&str]) -> Result<(), String> {
    for row in toml_rows(document, family)? {
        let id = toml_text(row, "id")?;
        for field in fields {
            toml_text(row, field).map_err(|error| format!("{id}: {error}"))?;
        }
    }
    Ok(())
}

fn read(repository_root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(repository_root.join(relative))
        .map_err(|error| format!("`{relative}` should be readable: {error}"))
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
