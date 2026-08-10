use serde_json::Value;

use super::command_binding::CommandBinding;
use super::result_artifact_binding::{
    cargo_command, ignored_list_command, read_artifact, require_array, require_duration_within,
    require_i64, require_str, require_u64, validate_ledger_fields,
};
use super::source_digest;

pub(super) use super::result_artifact_binding::current_revision;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SourceValidationPosture {
    HistoricalArtifactOnly,
    CurrentSource,
}

pub(super) struct LedgerResult<'a> {
    pub(super) matched_test_count: &'a str,
    pub(super) command_result: &'a str,
    pub(super) artifact: &'a str,
    pub(super) source_revision: &'a str,
    pub(super) source_digest: &'a str,
    pub(super) source_state_digest: &'a str,
    pub(super) run_nonce: &'a str,
    pub(super) source_identity: &'a str,
    pub(super) result_artifact_digest: &'a str,
    pub(super) claim_digest: &'a str,
    pub(super) structural_counter: &'a str,
    pub(super) construction_cost: &'a str,
    pub(super) execution_cost: &'a str,
    pub(super) source_validation: SourceValidationPosture,
}

pub(super) fn validate(ledger: LedgerResult<'_>, command: &CommandBinding) -> Result<(), String> {
    validate_ledger_fields(&ledger, command)?;
    let artifact = read_artifact(ledger.artifact)?;
    validate_artifact_contract(&artifact, &ledger, command)?;
    validate_main_execution(&artifact, command)?;
    validate_artifact_sources(&artifact, &ledger, command)?;
    validate_artifact_proofs(&artifact, &ledger, command)?;
    if source_digest::file_digest(ledger.artifact)? != ledger.result_artifact_digest {
        return Err("result artifact digest is stale".to_owned());
    }
    Ok(())
}

fn validate_artifact_contract(
    artifact: &Value,
    ledger: &LedgerResult<'_>,
    command: &CommandBinding,
) -> Result<(), String> {
    require_u64(
        artifact,
        "schema_version",
        if command.shared_main { 6 } else { 5 },
    )?;
    require_str(artifact, "package", &command.package)?;
    require_str(artifact, "target_kind", &command.target_kind)?;
    require_str(artifact, "target_name", &command.target_name)?;
    require_array(artifact, "features", &command.features)?;
    require_str(artifact, "test_name", &command.test_name)?;
    require_str(artifact, "requirement", &command.requirement)?;
    require_str(artifact, "claim_digest", ledger.claim_digest)?;
    require_str(artifact, "structural_counter", ledger.structural_counter)?;
    require_str(artifact, "construction_cost", ledger.construction_cost)?;
    require_str(artifact, "execution_cost", ledger.execution_cost)
}

fn validate_main_execution(artifact: &Value, command: &CommandBinding) -> Result<(), String> {
    require_u64(artifact, "matched_test_count", 1)?;
    let expected_ignored =
        super::execution_contract::expected_declared_ignored(&command.requirement);
    require_u64(
        artifact,
        "declared_ignored_test_count",
        u64::from(expected_ignored),
    )?;
    require_json(
        artifact,
        "expected_declared_ignored",
        &Value::Bool(expected_ignored),
    )?;
    require_u64(
        artifact,
        "executed_test_count",
        if command.shared_main { 0 } else { 1 },
    )?;
    require_u64(
        artifact,
        "passed_test_count",
        if command.shared_main { 0 } else { 1 },
    )?;
    require_u64(artifact, "ignored_test_count", 0)?;
    require_str(artifact, "exit_posture", "passed")?;
    require_i64(artifact, "list_exit_code", 0)?;
    if command.shared_main {
        if !artifact.get("test_exit_code").is_some_and(Value::is_null) {
            return Err("shared row claims a marginal main-test exit".to_owned());
        }
    } else {
        require_i64(artifact, "test_exit_code", 0)?;
    }
    let budget = super::execution_contract::main_budget_ms(&command.requirement);
    require_u64(artifact, "test_budget_ms", budget)?;
    require_duration_within(artifact, "list_duration_ms", 300_000)?;
    require_duration_within(artifact, "ignored_list_duration_ms", 300_000)?;
    if command.shared_main {
        require_u64(artifact, "test_duration_ms", 0)
    } else {
        require_duration_within(artifact, "test_duration_ms", budget)
    }
}

fn validate_artifact_sources(
    artifact: &Value,
    ledger: &LedgerResult<'_>,
    command: &CommandBinding,
) -> Result<(), String> {
    require_str(artifact, "source_revision", ledger.source_revision)?;
    require_str(artifact, "source_digest", ledger.source_digest)?;
    require_str(artifact, "source_state_digest", ledger.source_state_digest)?;
    require_str(artifact, "run_nonce", ledger.run_nonce)?;
    require_array(artifact, "source_identity", &command.sources)?;
    require_array(artifact, "list_command", &cargo_command(command, true))?;
    require_array(
        artifact,
        "ignored_list_command",
        &ignored_list_command(command),
    )?;
    require_array(artifact, "test_command", &cargo_command(command, false))
}

fn validate_artifact_proofs(
    artifact: &Value,
    ledger: &LedgerResult<'_>,
    command: &CommandBinding,
) -> Result<(), String> {
    if command.shared_main {
        super::shared_world_artifact::validate(
            artifact,
            command,
            ledger.source_revision,
            ledger.source_state_digest,
        )?;
    }
    super::result_artifact_control::validate(artifact, command.control.as_ref())?;
    super::compile_case_binding::validate(&command.requirement, &command.sources)?;
    super::result_artifact_counter::validate(
        &command.requirement,
        artifact,
        super::execution_contract::counter_amount(&command.requirement)
            .expect("every requirement has one counter"),
    )?;
    super::result_artifact_cost::validate(&command.requirement, artifact)?;
    if command.requirement.starts_with("P2-") {
        validate_native_boundary_observation(artifact)?;
    }
    Ok(())
}

#[path = "native_boundary_observation.rs"]
mod native_boundary_observation;

fn validate_native_boundary_observation(artifact: &Value) -> Result<(), String> {
    native_boundary_observation::validate(artifact)
}

fn require_json(value: &Value, field: &str, expected: &Value) -> Result<(), String> {
    (value.get(field) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

#[cfg(test)]
#[path = "result_artifact_mutation.rs"]
mod mutation_tests;
