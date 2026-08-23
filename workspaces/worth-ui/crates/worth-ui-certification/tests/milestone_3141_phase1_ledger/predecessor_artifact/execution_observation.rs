use serde_json::Value;

use super::{require_hex, require_str, require_string};
use crate::milestone_3141_phase1_ledger::{runner_artifact_authentication, source_digest};

const REFERENCE_SCHEMA: &str = "worth-ui-ledger-execution-reference-v1";
const OBSERVATION_SCHEMA: &str = "worth-ui-ledger-execution-observation-v1";
const BINDING_SCHEMA: &str = "worth-ui-ledger-execution-binding-v3";
const PORTFOLIO_SCHEMA: &str = "worth-ui-ledger-portfolio-execution-v2";

#[derive(Clone, Debug)]
pub(super) struct Observation {
    pub(super) observation_sha256: String,
    pub(super) execution_binding_key: String,
    pub(super) role: String,
    pub(super) requirement: String,
    pub(super) command: Value,
    pub(super) bindings: Value,
    pub(super) duration_ms: u64,
    pub(super) portfolio_identity: String,
}

pub(super) fn validate_rows(rows: &[Value]) -> Result<Vec<Observation>, String> {
    let mut observations = Vec::new();
    for row in rows {
        let requirement = require_string(row, "requirement")?;
        let references = row["execution_receipts"]
            .as_array()
            .filter(|references| !references.is_empty())
            .ok_or_else(|| "predecessor row omits execution references".to_owned())?;
        for reference in references {
            observations.push(validate(reference, requirement)?);
        }
    }
    Ok(observations)
}

pub(super) fn validate(reference: &Value, requirement: &str) -> Result<Observation, String> {
    let observation = require_string(reference, "observation_sha256")?;
    require_hex(reference, "observation_sha256", 64)?;
    let envelope = read_envelope(observation)?;
    validate_envelope(reference, requirement, &envelope)
}

fn validate_envelope(
    reference: &Value,
    requirement: &str,
    envelope: &Value,
) -> Result<Observation, String> {
    require_str(reference, "schema", REFERENCE_SCHEMA)?;
    let observation_sha256 = require_string(reference, "observation_sha256")?.to_owned();
    let execution_binding_key = require_string(reference, "execution_binding_key")?.to_owned();
    require_hex(reference, "observation_sha256", 64)?;
    require_hex(reference, "execution_binding_key", 64)?;
    let role = require_string(reference, "role")?.to_owned();
    let record = envelope
        .get("record")
        .ok_or_else(|| "execution observation omits its record".to_owned())?;
    let tag = require_string(&envelope, "runner_authentication")?;
    runner_artifact_authentication::validate_tagged(record, tag)?;
    let expected_observation = runner_artifact_authentication::canonical_digest(
        &serde_json::json!({"record": record, "runner_authentication": tag}),
    );
    require_str(&envelope, "observation_sha256", &expected_observation)?;
    require_str(record, "schema", OBSERVATION_SCHEMA)?;
    let binding = &record["execution_binding"];
    require_str(binding, "schema", BINDING_SCHEMA)?;
    let expected_binding = runner_artifact_authentication::canonical_digest(binding);
    require_str(record, "execution_binding_key", &expected_binding)?;
    require_str(reference, "execution_binding_key", &expected_binding)?;
    require_str(
        reference,
        "command_sha256",
        &runner_artifact_authentication::canonical_digest(&binding["command"]),
    )?;
    let duration_ms = reference["duration_ms"]
        .as_u64()
        .ok_or_else(|| "execution reference omits duration".to_owned())?;
    if record["duration_ms"].as_u64() != Some(duration_ms)
        || record["returncode"].as_u64() != Some(0)
        || !matches!(
            reference["acquisition"].as_str(),
            Some("executed" | "reused")
        )
    {
        return Err("execution reference differs from its observation".to_owned());
    }
    let command = binding["command"].clone();
    let bindings = binding["artifact_bindings"].clone();
    if !command.is_array() || !bindings.is_object() {
        return Err("execution observation has no exact causal binding".to_owned());
    }
    let portfolio_identity = runner_artifact_authentication::canonical_digest(&serde_json::json!({
        "schema": PORTFOLIO_SCHEMA,
        "role": role,
        "exact_command": command,
        "normalized_causal_artifact_bindings": bindings,
    }));
    Ok(Observation {
        observation_sha256,
        execution_binding_key,
        role,
        requirement: requirement.to_owned(),
        command,
        bindings,
        duration_ms,
        portfolio_identity,
    })
}

fn read_envelope(observation: &str) -> Result<Value, String> {
    let identity = format!(
        "_docs/worth-ui/milestone-3.14.1-evidence/execution-observations/{}/{}.json",
        &observation[..2],
        observation,
    );
    serde_json::from_str(
        &std::fs::read_to_string(source_digest::repository_file(&identity)?)
            .map_err(|error| format!("cannot read execution observation: {error}"))?,
    )
    .map_err(|error| format!("invalid execution observation: {error}"))
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::{validate_envelope, BINDING_SCHEMA, OBSERVATION_SCHEMA, REFERENCE_SCHEMA};
    use crate::milestone_3141_phase1_ledger::runner_artifact_authentication;

    #[test]
    fn one_binding_preserves_distinct_physical_observations() {
        let (first, first_reference) = fixture(628);
        let (second, second_reference) = fixture(641);
        assert_eq!(
            first_reference["execution_binding_key"],
            second_reference["execution_binding_key"]
        );
        assert_ne!(
            first_reference["observation_sha256"],
            second_reference["observation_sha256"]
        );
        validate_envelope(&first_reference, "P1-FIXTURE-01", &first).unwrap();
        validate_envelope(&second_reference, "P1-FIXTURE-01", &second).unwrap();
    }

    #[test]
    fn reference_duration_cannot_relabel_an_observation() {
        let (envelope, mut reference) = fixture(628);
        reference["duration_ms"] = Value::from(641);
        assert_eq!(
            validate_envelope(&reference, "P1-FIXTURE-01", &envelope).unwrap_err(),
            "execution reference differs from its observation"
        );
    }

    fn fixture(duration_ms: u64) -> (Value, Value) {
        let binding = json!({
            "schema": BINDING_SCHEMA,
            "command": ["cargo", "test", "exact"],
            "source_revision": "a".repeat(40),
            "source_state_digest": "b".repeat(64),
            "artifact_bindings": {},
        });
        let binding_key = runner_artifact_authentication::canonical_digest(&binding);
        let record = json!({
            "schema": OBSERVATION_SCHEMA,
            "execution_binding": binding,
            "execution_binding_key": binding_key,
            "returncode": 0,
            "stdout": "passed",
            "stderr": "",
            "duration_ms": duration_ms,
        });
        let tag = runner_artifact_authentication::sign(&record).unwrap();
        let observation = runner_artifact_authentication::canonical_digest(
            &json!({"record": record, "runner_authentication": tag}),
        );
        let envelope = json!({
            "observation_sha256": observation,
            "record": record,
            "runner_authentication": tag,
        });
        let reference = json!({
            "schema": REFERENCE_SCHEMA,
            "role": "main-test",
            "execution_binding_key": binding_key,
            "observation_sha256": observation,
            "command_sha256": runner_artifact_authentication::canonical_digest(
                &json!(["cargo", "test", "exact"]),
            ),
            "duration_ms": duration_ms,
            "acquisition": "executed",
        });
        (envelope, reference)
    }
}
