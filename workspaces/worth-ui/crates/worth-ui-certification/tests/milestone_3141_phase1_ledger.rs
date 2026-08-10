use std::collections::BTreeMap;

use super::{repository_document, workspace_source_inventory};

#[path = "milestone_3141_phase1_ledger/claim_contract.rs"]
mod claim_contract;
#[path = "milestone_3141_phase1_ledger/claim_digest.rs"]
mod claim_digest;
#[path = "milestone_3141_phase1_ledger/command_binding.rs"]
mod command_binding;
#[path = "milestone_3141_phase1_ledger/compile_case_binding.rs"]
mod compile_case_binding;
#[path = "milestone_3141_phase1_ledger/execution_contract.rs"]
mod execution_contract;
#[cfg(test)]
#[path = "milestone_3141_phase1_ledger/mutation_tests.rs"]
mod mutation_tests;
#[path = "milestone_3141_phase1_ledger/requirement_contract.rs"]
mod requirement_contract;
#[path = "milestone_3141_phase1_ledger/result_artifact.rs"]
mod result_artifact;
#[path = "milestone_3141_phase1_ledger/result_artifact_binding.rs"]
mod result_artifact_binding;
#[path = "milestone_3141_phase1_ledger/result_artifact_control.rs"]
mod result_artifact_control;
#[path = "milestone_3141_phase1_ledger/result_artifact_cost.rs"]
mod result_artifact_cost;
#[path = "milestone_3141_phase1_ledger/result_artifact_counter.rs"]
mod result_artifact_counter;
#[path = "milestone_3141_phase1_ledger/result_artifact_environment.rs"]
mod result_artifact_environment;
#[path = "milestone_3141_phase1_ledger/row_evidence.rs"]
mod row_evidence;
#[path = "milestone_3141_phase1_ledger/schema.rs"]
mod schema;
#[path = "milestone_3141_phase1_ledger/shared_world_artifact.rs"]
mod shared_world_artifact;
#[path = "milestone_3141_phase1_ledger/source_digest.rs"]
pub(crate) mod source_digest;
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

fn validate_phase_closure(rows: &BTreeMap<String, Row>, through_phase: u8) -> Result<(), String> {
    let open = rows.values().find(|row| {
        row["phase"]
            .parse::<u8>()
            .is_ok_and(|phase| phase <= through_phase)
            && (row["result"] != "PROVED" || row["final_source"] != "true")
    });
    match open {
        Some(row) => Err(format!(
            "{} remains open for Phase {through_phase} closure",
            row["requirement"]
        )),
        None => Ok(()),
    }
}

#[test]
#[ignore = "milestone closure gate: run only after every Phase 1 row has final evidence"]
fn phase_one_closure_requires_every_phase_one_row() {
    let rows = parse(&repository_document(LEDGER)).expect("the milestone ledger should parse");
    validate_phase_closure(&rows, 1)
        .expect("every Phase 1 requirement must be final-source proved");
}

#[test]
#[ignore = "closure prerequisite: execute through the governed ledger runner"]
fn phase_one_closure_prerequisites_are_final_source() {
    let rows = parse(&repository_document(LEDGER)).expect("the milestone ledger should parse");
    for (requirement, row) in &rows {
        if requirement.starts_with("P1-") && requirement != "P1-CLOSE-01" {
            assert_eq!(row["result"], "PROVED", "{requirement} remains open");
            assert_eq!(
                row["final_source"], "true",
                "{requirement} is not final-source evidence"
            );
        }
    }
    let phase_one_rows = rows
        .keys()
        .filter(|requirement| requirement.starts_with("P1-"))
        .count();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P1-CLOSE-01\":{phase_one_rows}}}");
}

#[test]
#[ignore = "milestone closure gate: run only after every Phase 2 row has final evidence"]
fn phase_two_closure_requires_every_phase_one_and_two_row() {
    let rows = parse(&repository_document(LEDGER)).expect("the milestone ledger should parse");
    validate_phase_closure(&rows, 2)
        .expect("every Phase 1 and Phase 2 requirement must be final-source proved");
}

fn validate_row(row: &Row) -> Result<(), String> {
    validate_requirement_contract(row)?;
    match row["result"].as_str() {
        "OPEN" if row["final_source"] == "false" => return Ok(()),
        "PROVED" if row["final_source"] == "true" => {}
        _ => return Err("invalid result/final-source posture".to_owned()),
    }
    validate_proved_presence(row)?;
    row_evidence::validate_world(row)?;
    row_evidence::validate_execution(row)?;
    row_evidence::validate_observations(row)?;
    row_evidence::validate_sources(row)
}

fn validate_requirement_contract(row: &Row) -> Result<(), String> {
    validate_profile_digests(row)?;
    claim_contract::validate_platform_dependencies(&row["requirement"])?;
    let contract = requirement_contract::for_requirement(&row["requirement"])
        .ok_or_else(|| "missing requirement contract".to_owned())?;
    validate_closed_identity(row, contract)?;
    if row["result"] == "PROVED" {
        validate_proved_claim(row, contract)?;
    }
    Ok(())
}

fn validate_closed_identity(
    row: &Row,
    contract: &requirement_contract::RequirementContract,
) -> Result<(), String> {
    let expected_phase = if row["requirement"].starts_with("P1-") {
        "1"
    } else if row["requirement"].starts_with("P2-") {
        "2"
    } else {
        return Err("unknown requirement phase".to_owned());
    };
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
        || row["platform_versions"] != claim_contract::platform_versions(&row["requirement"])
    {
        return Err("closed identity mismatch".to_owned());
    }
    Ok(())
}

fn validate_proved_claim(
    row: &Row,
    contract: &requirement_contract::RequirementContract,
) -> Result<(), String> {
    validate_mutation_control(&row["mutation_control"], contract.mutation_family)?;
    let expected_case = claim_contract::scenario_delta(&row["requirement"])
        .ok_or_else(|| "requirement omits its exact mutation case".to_owned())?;
    let expected_baseline = claim_contract::baseline_digest(&row["requirement"])?;
    let expected_fields = [
        ("scenario_delta", expected_case),
        ("generated_seed", "not-applicable"),
        (
            "construction_cost",
            claim_contract::construction_cost(&row["requirement"]),
        ),
        (
            "execution_cost",
            claim_contract::execution_cost(&row["requirement"]),
        ),
        ("baseline_digest", expected_baseline.as_str()),
    ];
    for (field, expected) in expected_fields {
        if row[field] != expected {
            return Err(format!(
                "proved {field} differs from its immutable requirement contract"
            ));
        }
    }
    validate_proved_counter_and_fault(row, contract)
}

fn validate_proved_counter_and_fault(
    row: &Row,
    contract: &requirement_contract::RequirementContract,
) -> Result<(), String> {
    let counters = named_numeric_fields(&row["structural_counters"])?;
    let expected_amount = execution_contract::counter_amount(&row["requirement"])
        .ok_or_else(|| "requirement omits its exact counter amount".to_owned())?;
    if counters.len() != 1 || counters.get(contract.counter_family) != Some(&expected_amount) {
        return Err("proved evidence omits its exact counter family".to_owned());
    }
    let expected_fault = execution_contract::fault_boundary(&row["requirement"])
        .ok_or_else(|| "requirement omits its exact fault boundary".to_owned())?;
    if row["fault_injection_boundary"] != expected_fault {
        return Err("proved evidence has the wrong fault boundary".to_owned());
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

pub(super) fn validate_cost(value: &str) -> Result<(), String> {
    let fields = named_numeric_fields(value)?;
    (!fields.is_empty())
        .then_some(())
        .ok_or_else(|| "empty cost evidence".to_owned())
}

fn named_numeric_fields(value: &str) -> Result<BTreeMap<&str, u64>, String> {
    let mut fields = BTreeMap::new();
    for field in value.split(';') {
        let Some((name, amount)) = field.split_once('=') else {
            return Err("cost evidence must be named numeric counters".to_owned());
        };
        let amount = amount
            .parse::<u64>()
            .map_err(|_| "invalid cost counter".to_owned())?;
        if name.is_empty() || fields.insert(name, amount).is_some() {
            return Err("invalid cost counter".to_owned());
        }
    }
    Ok(fields)
}

fn validate_mutation_control(value: &str, expected_family: &str) -> Result<(), String> {
    let fields = value
        .split(';')
        .map(|field| {
            field
                .split_once('=')
                .ok_or_else(|| "mutation control must use named fields".to_owned())
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    if fields.get("family") != Some(&expected_family)
        || fields.get("case").is_none_or(|case| case.is_empty())
    {
        return Err("proved evidence has the wrong mutation family".to_owned());
    }
    Ok(())
}
