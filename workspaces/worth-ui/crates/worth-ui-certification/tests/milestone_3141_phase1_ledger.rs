use std::collections::BTreeMap;

use super::{repository_document, workspace_source_inventory};

#[path = "milestone_3141_phase1_ledger/claim_digest.rs"]
mod claim_digest;
#[path = "milestone_3141_phase1_ledger/command_binding.rs"]
mod command_binding;
#[cfg(test)]
#[path = "milestone_3141_phase1_ledger/mutation_tests.rs"]
mod mutation_tests;
#[path = "milestone_3141_phase1_ledger/requirement_contract.rs"]
mod requirement_contract;
#[path = "milestone_3141_phase1_ledger/result_artifact.rs"]
mod result_artifact;
#[path = "milestone_3141_phase1_ledger/schema.rs"]
mod schema;
#[path = "milestone_3141_phase1_ledger/source_digest.rs"]
mod source_digest;
#[path = "milestone_3141_phase1_ledger/source_symbol.rs"]
mod source_symbol;

use schema::{EXPECTED_REQUIREMENTS, HEADER};

const LEDGER: &str = "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv";

type Row = BTreeMap<String, String>;

fn parse(ledger: &str) -> Result<BTreeMap<String, Row>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(ledger.as_bytes());
    let header = reader.headers().map_err(|error| error.to_string())?;
    if header.iter().collect::<Vec<_>>() != HEADER {
        return Err("ledger schema drift".to_owned());
    }
    let mut rows = BTreeMap::new();
    let mut observed_order = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|error| error.to_string())?;
        if record.len() != HEADER.len() {
            return Err("wrong column count".to_owned());
        }
        let row = HEADER
            .iter()
            .zip(record.iter())
            .map(|(name, value)| ((*name).to_owned(), value.to_owned()))
            .collect::<Row>();
        validate_row(&row)?;
        let requirement = row["requirement"].clone();
        observed_order.push(requirement.clone());
        if rows.insert(requirement.clone(), row).is_some() {
            return Err(format!("duplicate {requirement}"));
        }
    }
    if observed_order != EXPECTED_REQUIREMENTS {
        return Err("requirements must be exact and sorted".to_owned());
    }
    validate_phase_progression(&rows)?;
    validate_proved_run_uniqueness(&rows)?;
    Ok(rows)
}

fn validate_proved_run_uniqueness(rows: &BTreeMap<String, Row>) -> Result<(), String> {
    for field in ["run_nonce", "retained_result_artifact"] {
        let mut observed = std::collections::BTreeSet::new();
        for value in rows
            .values()
            .filter(|row| row["result"] == "PROVED")
            .map(|row| row[field].as_str())
        {
            if !observed.insert(value) {
                return Err(format!("proved rows reuse {field}"));
            }
        }
    }
    Ok(())
}

fn validate_phase_progression(rows: &BTreeMap<String, Row>) -> Result<(), String> {
    let phase_one_open = rows
        .values()
        .any(|row| row["phase"] == "1" && row["result"] != "PROVED");
    let phase_two_proved = rows
        .values()
        .any(|row| row["phase"] == "2" && row["result"] == "PROVED");
    (!phase_one_open || !phase_two_proved)
        .then_some(())
        .ok_or_else(|| "Phase 2 proof cannot precede Phase 1 closure".to_owned())
}

fn validate_row(row: &Row) -> Result<(), String> {
    validate_requirement_contract(row)?;
    match row["result"].as_str() {
        "OPEN" if row["final_source"] == "false" => return Ok(()),
        "PROVED" if row["final_source"] == "true" => {}
        _ => return Err("invalid result/final-source posture".to_owned()),
    }
    validate_proved_presence(row)?;
    validate_world_evidence(row)?;
    validate_execution_evidence(row)?;
    validate_observations(row)?;
    validate_source_identity(row)
}

fn validate_requirement_contract(row: &Row) -> Result<(), String> {
    validate_profile_digests(row)?;
    let expected_phase = if row["requirement"].starts_with("P1-") {
        "1"
    } else if row["requirement"].starts_with("P2-") {
        "2"
    } else {
        return Err("unknown requirement phase".to_owned());
    };
    let contract = requirement_contract::for_requirement(&row["requirement"])
        .ok_or_else(|| "missing requirement contract".to_owned())?;
    if row["phase"] != expected_phase
        || row["owner"] != contract.owner
        || row["production_boundary"] != contract.boundary
        || row["world_identity"] != contract.world
        || row["world_version"] != "1"
        || row["proof_kind"] != contract.proof_kind
        || row["evidence_schema"] != requirement_contract::EVIDENCE_SCHEMA
        || row["authority_provenance"] != contract.authority
        || row["font_profile_identity"] != "worth-ui-body-default-v1"
        || row["font_profile_digest"] != requirement_contract::FONT_PROFILE_DIGEST
        || row["native_profile_identity"] != "worth-ui-windows-dx12-v1"
        || row["native_profile_digest"] != requirement_contract::NATIVE_PROFILE_DIGEST
        || !row["platform_versions"].contains("protocol=4")
    {
        return Err("closed identity mismatch".to_owned());
    }
    if row["result"] == "PROVED"
        && (!row["mutation_control"].starts_with(contract.mutation_family)
            || !row["structural_counters"].contains(contract.counter_family))
    {
        return Err("proved evidence does not satisfy its requirement contract".to_owned());
    }
    Ok(())
}

fn validate_profile_digests(row: &Row) -> Result<(), String> {
    let font =
        "workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-body-default-v1.toml";
    let native =
        "workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-windows-dx12-v1.toml";
    if row["font_profile_digest"] != source_digest::file_digest(font)?
        || row["native_profile_digest"] != source_digest::file_digest(native)?
    {
        return Err("ledger profile digest does not match canonical bytes".to_owned());
    }
    Ok(())
}

fn validate_proved_presence(row: &Row) -> Result<(), String> {
    for name in HEADER {
        if row[name].trim().is_empty() {
            return Err(format!("proved row has blank {name}"));
        }
    }
    if row.values().any(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "evidence" | "pending" | "todo" | "tbd" | "unknown"
        )
    }) {
        return Err("proved row contains placeholder evidence".to_owned());
    }
    Ok(())
}

fn validate_world_evidence(row: &Row) -> Result<(), String> {
    if row["world_version"]
        .parse::<u16>()
        .ok()
        .filter(|value| *value > 0)
        .is_none()
    {
        return Err("invalid world version".to_owned());
    }
    if row["baseline_digest"].len() != 64
        || !row["baseline_digest"]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("invalid baseline digest".to_owned());
    }
    if !matches!(
        row["teardown_result"].as_str(),
        "terminal" | "not-applicable"
    ) {
        return Err("invalid teardown result".to_owned());
    }
    validate_cost(&row["construction_cost"])?;
    validate_cost(&row["execution_cost"])?;
    validate_cost(&row["structural_counters"])?;
    if !row["authority_provenance"].contains("::") {
        return Err("authority provenance is not a named owner path".to_owned());
    }
    Ok(())
}

fn validate_execution_evidence(row: &Row) -> Result<(), String> {
    let command = command_binding::validate(
        &row["exact_command"],
        &row["requirement"],
        &row["production_entry"],
        &row["independent_oracle"],
        &row["source_identity"],
    )?;
    result_artifact::validate(
        result_artifact::LedgerResult {
            matched_test_count: &row["matched_test_count"],
            command_result: &row["command_result"],
            artifact: &row["retained_result_artifact"],
            source_revision: &row["source_revision"],
            source_digest: &row["source_digest"],
            source_state_digest: &row["source_state_digest"],
            run_nonce: &row["run_nonce"],
            source_identity: &row["source_identity"],
            result_artifact_digest: &row["result_artifact_digest"],
            claim_digest: &claim_digest::calculate(row),
        },
        &command,
    )?;
    validate_named_entry(&row["production_entry"])?;
    validate_named_entry(&row["independent_oracle"])?;
    Ok(())
}

fn validate_observations(row: &Row) -> Result<(), String> {
    if !matches!(
        row["fault_injection_boundary"].as_str(),
        "before-effects" | "after-effects-may-have-begun" | "not-applicable"
    ) {
        return Err("invalid fault injection boundary".to_owned());
    }
    for observation in ["presented_source_readback", "client_area_observation"] {
        if !matches!(row[observation].as_str(), "not-applicable")
            && !row[observation].starts_with("observed:")
        {
            return Err(format!("invalid {observation}"));
        }
    }
    Ok(())
}

fn validate_source_identity(row: &Row) -> Result<(), String> {
    for source in row["source_identity"].split(';') {
        if source.starts_with("workspaces/worth-ui/") {
            let relative = source.trim_start_matches("workspaces/worth-ui/");
            if !workspace_source_inventory().contains(relative) {
                return Err(format!("missing source {source}"));
            }
        } else {
            let repository_root = workspace_source_inventory()
                .root()
                .parent()
                .and_then(std::path::Path::parent)
                .expect("repository root");
            if !repository_root.join(source).exists() {
                return Err(format!("missing source {source}"));
            }
        }
    }
    Ok(())
}

fn validate_named_entry(value: &str) -> Result<(), String> {
    let Some((source, symbol)) = value.rsplit_once("::") else {
        return Err("evidence entry lacks a named symbol".to_owned());
    };
    if symbol.is_empty() || !source.ends_with(".rs") {
        return Err("invalid evidence entry".to_owned());
    }
    let source_path = resolve_source(source).ok_or_else(|| format!("missing source {source}"))?;
    source_symbol::validate(&source_path, symbol)
}

fn validate_cost(value: &str) -> Result<(), String> {
    let mut count = 0;
    for field in value.split(';') {
        let Some((name, amount)) = field.split_once('=') else {
            return Err("cost evidence must be named numeric counters".to_owned());
        };
        if name.is_empty() || amount.parse::<u64>().is_err() {
            return Err("invalid cost counter".to_owned());
        }
        count += 1;
    }
    (count > 0)
        .then_some(())
        .ok_or_else(|| "empty cost evidence".to_owned())
}

fn resolve_source(source: &str) -> Option<std::path::PathBuf> {
    if source.starts_with("workspaces/worth-ui/") {
        let relative = source.trim_start_matches("workspaces/worth-ui/");
        return workspace_source_inventory()
            .contains(relative)
            .then(|| workspace_source_inventory().root().join(relative));
    }
    let repository_root = workspace_source_inventory()
        .root()
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repository root");
    let path = repository_root.join(source);
    path.exists().then_some(path)
}
