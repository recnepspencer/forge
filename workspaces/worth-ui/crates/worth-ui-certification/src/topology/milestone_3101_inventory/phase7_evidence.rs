use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::topology::WorkspaceSourceInventory;

use super::ledger;

const REQUIRED_CLAIMS: &[&str] = &[
    "real_file_lifecycle",
    "independent_semantic_oracle",
    "hot_frame_source_exclusion",
    "invalid_edit_preservation",
    "runtime_denial_preservation",
    "interrupted_replacement_preservation",
    "adapter_parity",
    "allocation_cost",
    "scope_scaling",
    "facade_only_journey",
    "build_budget",
    "comparable_closing_costs",
];

const REQUIRED_COST_CATEGORIES: &[&str] = &[
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

const EXECUTABLE_FIXTURES: &[&str] = &[
    "crates/worth-ui-certification/tests/fixtures/compile_contracts/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/host_contract_only_adapter/Cargo.toml",
    "crates/worth-ui-certification/tests/fixtures/runtime_effect_adapter/Cargo.toml",
];

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    validate_header(document)?;
    validate_required_cost_categories(document)?;
    validate_claims(inventory, document)?;
    super::phase7_adapter_boundary::audit(inventory)?;
    validate_target_budget(inventory)?;
    validate_executable_fixtures(inventory, document)?;
    validate_compile_budget(inventory, document)
}

fn validate_header(document: &toml::Value) -> Result<(), String> {
    if ledger::text(document, "schema")? != "worth-ui.milestone-3.10.1.phase-7-evidence.v1"
        || ledger::integer(document, "phase")? != 7
    {
        return Err("Phase 7 evidence manifest header is invalid".to_owned());
    }
    if ledger::integer(document, "integration_target_budget")? != 9
        || ledger::integer(document, "compile_contract_cargo_sessions")? != 2
    {
        return Err("Phase 7 build budgets must retain the opening posture".to_owned());
    }
    Ok(())
}

pub(super) fn validate_required_cost_categories(document: &toml::Value) -> Result<(), String> {
    let observed = ledger::strings(document, "cost_categories")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = REQUIRED_COST_CATEGORIES.iter().copied().collect();
    if observed != expected {
        return Err(format!(
            "Phase 7 cost categories differ: observed={observed:?}, expected={expected:?}"
        ));
    }
    Ok(())
}

fn validate_claims(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    let rows = ledger::tables(document, "claim")?;
    let mut observed = BTreeSet::new();
    for row in rows {
        let id = ledger::text(row, "id")?;
        if !observed.insert(id) {
            return Err(format!("duplicate Phase 7 evidence claim `{id}`"));
        }
        validate_claim(inventory, row, id)?;
    }
    let expected = REQUIRED_CLAIMS.iter().copied().collect();
    if observed != expected {
        return Err(format!(
            "Phase 7 evidence claims differ: observed={observed:?}, expected={expected:?}"
        ));
    }
    Ok(())
}

fn validate_claim(
    inventory: &WorkspaceSourceInventory,
    row: &toml::Value,
    id: &str,
) -> Result<(), String> {
    for field in [
        "owner",
        "authority",
        "mechanism",
        "oracle",
        "forbidden_shortcut",
    ] {
        if ledger::text(row, field)?.trim().is_empty() {
            return Err(format!("Phase 7 claim `{id}` has empty `{field}`"));
        }
    }
    if row.get("independent_oracle").and_then(toml::Value::as_bool) != Some(true) {
        return Err(format!(
            "Phase 7 claim `{id}` must name an independent oracle"
        ));
    }
    validate_mechanism(id, ledger::text(row, "mechanism")?)?;
    let sources = ledger::strings(row, "sources")?;
    let witnesses = ledger::strings(row, "witnesses")?;
    if sources.is_empty() || sources.len() != witnesses.len() {
        return Err(format!(
            "Phase 7 claim `{id}` must pair each source with one witness"
        ));
    }
    for (source, witness) in sources.into_iter().zip(witnesses) {
        let file = inventory
            .source(source)
            .ok_or_else(|| format!("Phase 7 claim `{id}` source `{source}` is absent"))?;
        if !file.text().contains(witness) {
            return Err(format!(
                "Phase 7 claim `{id}` witness `{witness}` is absent from `{source}`"
            ));
        }
    }
    Ok(())
}

fn validate_mechanism(id: &str, observed: &str) -> Result<(), String> {
    let expected = match id {
        "real_file_lifecycle" | "invalid_edit_preservation" => "operating-system-watcher",
        "hot_frame_source_exclusion" => "production-counter-plus-generation",
        "runtime_denial_preservation" => "valid-dsl-runtime-denial",
        "adapter_parity" => "independent-headless-and-egui",
        "allocation_cost" => "thread-scoped-allocation-observer",
        "build_budget" | "comparable_closing_costs" => "repository-lane-runner",
        _ => return Ok(()),
    };
    if observed != expected {
        return Err(format!(
            "Phase 7 claim `{id}` mechanism `{observed}` must be `{expected}`"
        ));
    }
    Ok(())
}

fn validate_target_budget(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let product = integration_targets(inventory.text("crates/worth-ui/Cargo.toml"))?;
    let certification =
        integration_targets(inventory.text("crates/worth-ui-certification/Cargo.toml"))?;
    validate_target_counts(product, certification)
}

fn validate_target_counts(product: usize, certification: usize) -> Result<(), String> {
    if (product, certification, product + certification) != (2, 7, 9) {
        return Err(format!(
            "Phase 7 integration target budget changed: product={product}, \
             certification={certification}, total={}",
            product + certification
        ));
    }
    Ok(())
}

fn integration_targets(manifest: &str) -> Result<usize, String> {
    let parsed = manifest
        .parse::<toml::Value>()
        .map_err(|error| format!("Cargo manifest should parse: {error}"))?;
    Ok(parsed
        .get("test")
        .and_then(toml::Value::as_array)
        .map_or(0, Vec::len))
}

fn validate_executable_fixtures(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    let declared = ledger::strings(document, "executable_fixture_manifests")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = EXECUTABLE_FIXTURES.iter().copied().collect();
    if declared != expected {
        return Err("Phase 7 executable fixture manifest budget changed".to_owned());
    }
    let root = Path::new("crates/worth-ui-certification/tests/fixtures");
    let actual = inventory
        .entries_under(root)
        .filter(|path| path.file_name().is_some_and(|name| name == "Cargo.toml"))
        .filter(|path| !path.to_string_lossy().contains("topology_negative"))
        .map(normalize)
        .collect::<BTreeSet<_>>();
    let expected_paths = EXECUTABLE_FIXTURES
        .iter()
        .map(|path| (*path).to_owned())
        .collect();
    if actual != expected_paths {
        return Err(format!(
            "Phase 7 executable fixture workspaces changed: {actual:?}"
        ));
    }
    Ok(())
}

fn validate_compile_budget(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    let root = inventory
        .root()
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| {
            "Worth UI workspace should have a repository parent for compile budgets".to_owned()
        })?;
    let runner = read(root.join("scripts/ci/run_worth_ui_compile_contracts.py"))?;
    if runner.matches("cargo_check(").count() != 3
        || !runner.contains("fail_status, fail_diagnostics = cargo_check(failing)")
        || !runner.contains("pass_status, pass_diagnostics = cargo_check(passing)")
        || !runner.contains("2 Cargo sessions")
    {
        return Err("compile-contract runner must retain exactly two Cargo checks".to_owned());
    }
    let (fail, pass) = compile_case_counts(inventory.root())?;
    let expected_fail = ledger::integer(document, "compile_fail_targets")? as usize;
    let expected_pass = ledger::integer(document, "compile_pass_targets")? as usize;
    if (fail, pass) != (expected_fail, expected_pass) {
        return Err(format!(
            "compile-contract target budget changed: fail={fail}, pass={pass}"
        ));
    }
    Ok(())
}

fn compile_case_counts(root: &Path) -> Result<(usize, usize), String> {
    let inventories = [
        "crates/worth-ui-certification/tests/suites/compile_contract_execution.csv",
        "crates/worth-ui-host-contract/tests/suites/compile_contract_cases.csv",
        "crates/worth-ui/tests/suites/compile_contract_execution.csv",
    ];
    let mut fail = 0;
    let mut pass = 0;
    for inventory in inventories {
        for row in read(root.join(inventory))?.lines().skip(1) {
            if row.starts_with("fail,") {
                fail += 1;
            } else if row.starts_with("pass,") {
                pass += 1;
            } else if !row.trim().is_empty() {
                return Err(format!("invalid compile-contract row in `{inventory}`"));
            }
        }
    }
    Ok((fail, pass))
}

fn read(path: PathBuf) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|error| format!("{} should be readable: {error}", path.display()))
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
#[path = "phase7_evidence_tests.rs"]
mod tests;
