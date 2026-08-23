use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{execution_observation, require_hex, require_str, require_string};
use crate::milestone_3141_phase1_ledger::runner_artifact_authentication;

const SCHEMA: &str = "worth-ui-ledger-portfolio-execution-v2";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Group {
    role: String,
    command: Value,
    bindings: Value,
    observations: BTreeMap<String, (String, u64)>,
    requirements: BTreeSet<String>,
}

pub(super) fn totals(artifact: &Value) -> Result<(u64, u64), String> {
    let rows = artifact["rows"]
        .as_array()
        .ok_or_else(|| "predecessor artifact omits rows".to_owned())?;
    let observations = execution_observation::validate_rows(rows)?;
    let expected = expected_groups(&observations)?;
    let entries = artifact["execution_identities"]
        .as_array()
        .ok_or_else(|| "predecessor artifact omits execution identities".to_owned())?;
    let mut represented = BTreeSet::new();
    let mut main = 0;
    let mut controls = 0;
    for entry in entries {
        let identity = require_string(entry, "portfolio_execution_identity")?;
        require_hex(entry, "portfolio_execution_identity", 64)?;
        let observed = observed_group(entry)?;
        let expected_group = expected
            .get(identity)
            .ok_or_else(|| "predecessor execution identity is unknown or duplicated".to_owned())?;
        validate_logical_identity(entry, identity, &observed)?;
        if &observed != expected_group || !represented.insert(identity.to_owned()) {
            return Err("predecessor execution identity differs from row evidence".to_owned());
        }
        match observed.role.as_str() {
            "main-test" => main += 1,
            "control-test" => controls += 1,
            _ => {}
        }
    }
    if represented.len() != expected.len() {
        return Err("predecessor execution identity inventory is incomplete".to_owned());
    }
    validate_counts(artifact, &observations, expected.len())?;
    Ok((main, controls))
}

fn expected_groups(
    observations: &[execution_observation::Observation],
) -> Result<BTreeMap<String, Group>, String> {
    let mut groups = BTreeMap::new();
    for observation in observations {
        let identity = observation.portfolio_identity.clone();
        let group = groups.entry(identity).or_insert_with(|| Group {
            role: observation.role.clone(),
            command: observation.command.clone(),
            bindings: observation.bindings.clone(),
            observations: BTreeMap::new(),
            requirements: BTreeSet::new(),
        });
        if group.role != observation.role
            || group.command != observation.command
            || group.bindings != observation.bindings
        {
            return Err("one logical execution identity has conflicting evidence".to_owned());
        }
        let physical = (
            observation.execution_binding_key.clone(),
            observation.duration_ms,
        );
        if let Some(previous) = group
            .observations
            .insert(observation.observation_sha256.clone(), physical.clone())
        {
            if previous != physical {
                return Err("one physical observation has conflicting evidence".to_owned());
            }
        }
        group.requirements.insert(observation.requirement.clone());
    }
    Ok(groups)
}

fn observed_group(entry: &Value) -> Result<Group, String> {
    let role = require_string(entry, "role")?.to_owned();
    let command = entry["exact_command"].clone();
    let bindings = entry["normalized_causal_artifact_bindings"].clone();
    if !command.is_array() || !bindings.is_object() {
        return Err("predecessor execution identity omits its causal binding".to_owned());
    }
    let inventory = entry["observations"]
        .as_array()
        .filter(|inventory| !inventory.is_empty())
        .ok_or_else(|| "predecessor execution identity omits observations".to_owned())?;
    let mut observations = BTreeMap::new();
    for item in inventory {
        let identity = require_string(item, "observation_sha256")?.to_owned();
        let binding = require_string(item, "execution_binding_key")?.to_owned();
        require_hex(item, "observation_sha256", 64)?;
        require_hex(item, "execution_binding_key", 64)?;
        let duration = item["duration_ms"]
            .as_u64()
            .ok_or_else(|| "predecessor execution observation omits duration".to_owned())?;
        if observations.insert(identity, (binding, duration)).is_some() {
            return Err("predecessor execution observation is duplicated".to_owned());
        }
    }
    Ok(Group {
        role,
        command,
        bindings,
        observations,
        requirements: string_set(entry, "requirements")?,
    })
}

fn validate_logical_identity(entry: &Value, identity: &str, group: &Group) -> Result<(), String> {
    let expected = runner_artifact_authentication::canonical_digest(&serde_json::json!({
        "schema": SCHEMA,
        "role": group.role,
        "exact_command": group.command,
        "normalized_causal_artifact_bindings": group.bindings,
    }));
    require_str(entry, "portfolio_execution_identity", &expected)?;
    if identity != expected {
        return Err("predecessor logical execution identity is substituted".to_owned());
    }
    Ok(())
}

fn validate_counts(
    artifact: &Value,
    observations: &[execution_observation::Observation],
    logical_count: usize,
) -> Result<(), String> {
    let source_bound = observations
        .iter()
        .map(|item| item.execution_binding_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let physical = observations
        .iter()
        .map(|item| item.observation_sha256.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    for (field, expected) in [
        ("logical_execution_count", logical_count),
        ("source_bound_execution_count", source_bound),
        ("physical_observation_count", physical),
        ("execution_reference_count", observations.len()),
    ] {
        if artifact[field].as_u64() != Some(expected as u64) {
            return Err(format!("predecessor artifact has wrong {field}"));
        }
    }
    Ok(())
}

fn string_set(value: &Value, field: &str) -> Result<BTreeSet<String>, String> {
    value[field]
        .as_array()
        .ok_or_else(|| format!("predecessor execution identity omits {field}"))?
        .iter()
        .map(Value::as_str)
        .collect::<Option<BTreeSet<_>>>()
        .map(|values| values.into_iter().map(ToOwned::to_owned).collect())
        .ok_or_else(|| format!("predecessor execution identity has invalid {field}"))
}
