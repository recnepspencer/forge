use std::collections::BTreeSet;

use serde_json::Value;
use sha2::{Digest, Sha256};

use super::{
    execution_contract, future_requirement_contract, predecessor_current_mapping,
    requirement_contract, result_artifact_binding, source_digest,
};

const EXPECTED_MAPPING_DIGEST: &str =
    "e8e8507f746b51bd8019af3382420716d6e7a8266d8107fe689defce1063d136";
const EXPECTED_PHASE_THREE_MAPPING_DIGEST: &str =
    "b7d3f182c3bcd19baee831dc1097ec01c7cd1ef704554b61c5edf7cd47145c91";
const EXPECTED_PHASE_FOUR_MAPPING_DIGEST: &str =
    "f894d2b284d5d3c4efcfa6dd98fff9ff3ec7a8884995f2478db87172f7ede1b6";

#[cfg(test)]
#[path = "predecessor_artifact_tests.rs"]
mod tests;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct PredecessorObservation {
    requirement_count: u64,
}

impl PredecessorObservation {
    pub(super) const fn requirement_count(&self) -> u64 {
        self.requirement_count
    }
}

pub(super) fn validate(identity: &str) -> Result<PredecessorObservation, String> {
    let artifact = result_artifact_binding::read_artifact(identity)?;
    let revision = result_artifact_binding::current_revision()?;
    let source_state = source_digest::calculate_source_state(&revision)?;
    validate_value(&artifact, &revision, &source_state)
}

fn validate_value(
    artifact: &Value,
    revision: &str,
    source_state: &str,
) -> Result<PredecessorObservation, String> {
    validate_value_with_mapping(artifact, revision, source_state, EXPECTED_MAPPING_DIGEST)
}

fn validate_value_with_mapping(
    artifact: &Value,
    revision: &str,
    source_state: &str,
    expected_mapping: &str,
) -> Result<PredecessorObservation, String> {
    require_str(artifact, "schema", "worth-ui-phase-predecessor-handoff-v1")?;
    let through_phase = artifact["through_phase"]
        .as_u64()
        .filter(|phase| matches!(phase, 2 | 3 | 4))
        .ok_or_else(|| "predecessor artifact has wrong through_phase".to_owned())?;
    let requirement_count = match through_phase {
        2 => 30,
        3 => 47,
        4 => 68,
        _ => return Err("predecessor artifact has wrong through_phase".to_owned()),
    };
    require_str(artifact, "source_revision", revision)?;
    require_str(artifact, "source_state_digest", source_state)?;
    require_u64(artifact, "verified_requirement_count", requirement_count)?;
    require_u64(artifact, "closure_test_executions", 2)?;
    require_u64(artifact, "compile_sessions", 2)?;
    validate_derived_totals(artifact)?;
    require_hex(artifact, "run_nonce", 32)?;
    require_str(
        artifact,
        "mapping_digest",
        &calculate_mapping_digest(&artifact["rows"]),
    )?;
    let phase_mapping = match through_phase {
        2 => expected_mapping,
        3 => EXPECTED_PHASE_THREE_MAPPING_DIGEST,
        4 => EXPECTED_PHASE_FOUR_MAPPING_DIGEST,
        _ => unreachable!("through_phase was validated above"),
    };
    require_str(artifact, "mapping_digest", phase_mapping)?;
    validate_rows(artifact, revision, source_state, requirement_count as usize)?;
    Ok(PredecessorObservation { requirement_count })
}

fn validate_derived_totals(artifact: &Value) -> Result<(), String> {
    let rows = artifact["rows"]
        .as_array()
        .ok_or_else(|| "predecessor artifact omits rows".to_owned())?;
    let main = unique_execution_total(rows, "main-test")?;
    let controls = unique_execution_total(rows, "control-test")?;
    let product = unique_cost_total(rows, "construction_cost", "product-processes");
    let worlds = unique_cost_total(rows, "construction_cost", "courtroom-worlds");
    let presentations = unique_cost_total(rows, "execution_cost", "presentations");
    for (field, expected) in [
        ("main_test_executions", main),
        ("hostile_control_executions", controls),
        ("product_processes", product),
        ("courtroom_worlds", worlds),
        ("presentations", presentations),
    ] {
        require_u64(artifact, field, expected)?;
    }
    Ok(())
}

fn unique_execution_total(rows: &[Value], role: &str) -> Result<u64, String> {
    let mut identities = BTreeSet::new();
    for row in rows {
        let receipts = row["execution_receipts"]
            .as_array()
            .ok_or_else(|| "predecessor row omits execution receipts".to_owned())?;
        for receipt in receipts {
            if receipt["role"].as_str() != Some(role) {
                continue;
            }
            let key = receipt["key"]
                .as_str()
                .filter(|key| key.len() == 64 && key.bytes().all(|byte| byte.is_ascii_hexdigit()))
                .ok_or_else(|| "predecessor row has invalid execution identity".to_owned())?;
            identities.insert(key);
        }
    }
    Ok(identities.len() as u64)
}

fn unique_cost_total(rows: &[Value], field: &str, name: &str) -> u64 {
    rows.iter()
        .filter(|row| row.get("shared_main_artifact").is_none())
        .filter_map(|row| row[field].as_str())
        .flat_map(|cost| cost.split(';'))
        .filter_map(|entry| entry.strip_prefix(&format!("{name}=")))
        .filter_map(|amount| amount.parse::<u64>().ok())
        .sum()
}

fn validate_rows(
    artifact: &Value,
    revision: &str,
    source_state: &str,
    count: usize,
) -> Result<(), String> {
    predecessor_current_mapping::validate_contract()?;
    let rows = artifact["rows"]
        .as_array()
        .filter(|rows| rows.len() == count)
        .ok_or_else(|| "predecessor artifact has the wrong row count".to_owned())?;
    let expected = super::predecessor_inventory::predecessor_requirements(count);
    let mut requirements = BTreeSet::new();
    let mut nonces = BTreeSet::new();
    let mut artifacts = BTreeSet::new();
    for row in rows {
        require_str(row, "exit_posture", "passed")?;
        require_str(row, "source_revision", revision)?;
        require_str(row, "source_state_digest", source_state)?;
        require_hex(row, "run_nonce", 32)?;
        require_hex(row, "artifact_sha256", 64)?;
        validate_execution(row)?;
        requirements.insert(require_string(row, "requirement")?);
        nonces.insert(require_string(row, "run_nonce")?);
        artifacts.insert(require_string(row, "artifact_sha256")?);
    }
    if requirements != expected || nonces.len() != count || artifacts.len() != count {
        let missing = expected
            .difference(&requirements)
            .copied()
            .collect::<Vec<_>>();
        let extra = requirements
            .difference(&expected)
            .cloned()
            .collect::<Vec<_>>();
        return Err(format!(
            "predecessor rows are incomplete, duplicated, or substituted: missing={missing:?}; extra={extra:?}; nonces={}; artifacts={}; expected={count}",
            nonces.len(),
            artifacts.len(),
        ));
    }
    Ok(())
}

fn validate_execution(row: &Value) -> Result<(), String> {
    let requirement = require_string(row, "requirement")?;
    let expected = execution_contract::current_predecessor_main_for(requirement)
        .ok_or_else(|| "predecessor row has no exact execution contract".to_owned())?;
    require_str(row, "package", expected.package)?;
    require_str(row, "target_kind", expected.target_kind)?;
    require_str(row, "target_name", expected.target_name)?;
    require_str(row, "test_name", expected.test_name)?;
    require_string_array(row, "features", expected.features)?;
    require_u64(row, "matched_test_count", 1)?;
    let marginal_main = u64::from(!execution_contract::is_shared_main(requirement));
    require_u64(row, "executed_test_count", marginal_main)?;
    require_u64(row, "passed_test_count", marginal_main)?;
    require_u64(row, "ignored_test_count", 0)?;
    let expected_ignored = execution_contract::expected_declared_ignored(requirement);
    require_u64(
        row,
        "declared_ignored_test_count",
        u64::from(expected_ignored),
    )?;
    if row["expected_declared_ignored"].as_bool() != Some(expected_ignored) {
        return Err("predecessor row changed declared-ignore posture".to_owned());
    }
    require_execution_role(row, "main-test")?;
    if execution_contract::control_for(requirement).is_some() {
        require_execution_role(row, "control-test")?;
    }
    validate_counter(row, requirement)?;
    validate_execution_sources(row)?;
    if requirement.starts_with("P1-") || requirement.starts_with("P2-") {
        predecessor_current_mapping::validate(row)?;
    }
    validate_control(row, requirement)
}

fn require_execution_role(row: &Value, role: &str) -> Result<(), String> {
    let receipts = row["execution_receipts"]
        .as_array()
        .ok_or_else(|| "predecessor row omits execution receipts".to_owned())?;
    receipts
        .iter()
        .find(|receipt| receipt["role"].as_str() == Some(role))
        .ok_or_else(|| format!("predecessor row omits {role} receipt"))
        .and_then(|receipt| require_hex(receipt, "key", 64))
}

fn validate_counter(row: &Value, requirement: &str) -> Result<(), String> {
    let contract = requirement_contract::for_requirement(requirement)
        .or_else(|| future_requirement_contract::for_requirement(requirement))
        .ok_or_else(|| format!("predecessor row {requirement} lacks requirement contract"))?;
    let amount = execution_contract::current_predecessor_counter_amount(requirement)
        .ok_or_else(|| "predecessor row lacks counter amount".to_owned())?;
    require_str(
        row,
        "structural_counter",
        &format!("{}={amount}", contract.counter_family),
    )
}

fn validate_execution_sources(row: &Value) -> Result<(), String> {
    require_hex(row, "source_digest", 64)?;
    let executed_sources = row["source_identity"]
        .as_array()
        .filter(|sources| !sources.is_empty())
        .ok_or_else(|| "predecessor row omits executed sources".to_owned())?;
    if executed_sources
        .iter()
        .any(|source| source.as_str().is_none())
    {
        return Err("predecessor executed source identity is not text".to_owned());
    }
    let mapped_sources = row["mapping_source_identity"]
        .as_array()
        .ok_or_else(|| "predecessor row omits mapped sources".to_owned())?;
    if executed_sources.len() != mapped_sources.len() {
        return Err("predecessor source inventories have different lengths".to_owned());
    }
    let rebindings = row["source_rebindings"]
        .as_array()
        .ok_or_else(|| "predecessor row omits source rebindings".to_owned())?;
    let mut rebound = 0;
    for (executed, mapped) in executed_sources.iter().zip(mapped_sources) {
        let executed = executed.as_str().unwrap();
        let mapped = mapped
            .as_str()
            .ok_or_else(|| "predecessor mapped source identity is not text".to_owned())?;
        if executed == mapped {
            continue;
        }
        let record = rebindings
            .get(rebound)
            .ok_or_else(|| "predecessor row omits one source rebinding".to_owned())?;
        if !is_rebindable(mapped)
            || !executed.starts_with("workspaces/worth-ui/target/worth-ui-3141-verify-")
            || record["canonical"].as_str() != Some(mapped)
            || record["executed"].as_str() != Some(executed)
        {
            return Err("predecessor row has an unlawful source rebinding".to_owned());
        }
        require_hex(record, "sha256", 64)?;
        rebound += 1;
    }
    (rebound == rebindings.len())
        .then_some(())
        .ok_or_else(|| "predecessor row has an extra source rebinding".to_owned())
}

fn is_rebindable(identity: &str) -> bool {
    matches!(
        identity,
        "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
            | "_docs/worth-ui/milestone-3.14.1-evidence/p1-worlds-01.json"
            | "_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json"
            | "_docs/worth-ui/milestone-3.14.1-evidence/p3-delta-source-01.json"
            | "_docs/worth-ui/milestone-3.14.1-evidence/p3-hp02-world-01.json"
            | "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json"
            | "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
    )
}

fn calculate_mapping_digest(rows: &Value) -> String {
    let mut ordered = rows.as_array().cloned().unwrap_or_default();
    ordered.sort_by(|left, right| {
        left["requirement"]
            .as_str()
            .cmp(&right["requirement"].as_str())
    });
    let mut digest = Sha256::new();
    for row in ordered {
        for field in ["requirement", "production_entry", "independent_oracle"] {
            digest.update(row[field].as_str().unwrap_or_default().as_bytes());
            digest.update([0]);
        }
        for source in row["mapping_source_identity"]
            .as_array()
            .into_iter()
            .flatten()
        {
            digest.update(source.as_str().unwrap_or_default().as_bytes());
            digest.update([0]);
        }
        digest.update([0xff]);
    }
    format!("{:x}", digest.finalize())
}

fn validate_control(row: &Value, requirement: &str) -> Result<(), String> {
    let expected = execution_contract::control_for(requirement);
    let Some(expected) = expected else {
        return row["hostile_control"]
            .is_null()
            .then_some(())
            .ok_or_else(|| "predecessor row has an unexpected control".to_owned());
    };
    let control = &row["hostile_control"];
    require_str(control, "package", expected.package)?;
    require_str(control, "target_kind", expected.target_kind)?;
    require_str(control, "target_name", expected.target_name)?;
    require_str(control, "test_name", expected.test_name)?;
    require_string_array(control, "features", expected.features)?;
    require_u64(control, "matched_test_count", 1)?;
    require_u64(control, "executed_test_count", 1)?;
    require_u64(control, "passed_test_count", 1)?;
    require_u64(control, "ignored_test_count", 0)?;
    require_str(control, "exit_posture", "passed")
}

fn require_string_array(value: &Value, field: &str, expected: &[&str]) -> Result<(), String> {
    let observed = value[field]
        .as_array()
        .and_then(|values| values.iter().map(Value::as_str).collect::<Option<Vec<_>>>())
        .ok_or_else(|| format!("predecessor artifact omits {field}"))?;
    (observed == expected)
        .then_some(())
        .ok_or_else(|| format!("predecessor artifact has wrong {field}"))
}

fn require_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .ok_or_else(|| format!("predecessor artifact omits {field}"))
}

fn require_str(value: &Value, field: &str, expected: &str) -> Result<(), String> {
    (require_string(value, field)? == expected)
        .then_some(())
        .ok_or_else(|| format!("predecessor artifact has wrong {field}"))
}

fn require_u64(value: &Value, field: &str, expected: u64) -> Result<(), String> {
    (value[field].as_u64() == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("predecessor artifact has wrong {field}"))
}

fn require_hex(value: &Value, field: &str, length: usize) -> Result<(), String> {
    let observed = require_string(value, field)?;
    (observed.len() == length
        && observed
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
    .then_some(())
    .ok_or_else(|| format!("predecessor artifact has invalid {field}"))
}
