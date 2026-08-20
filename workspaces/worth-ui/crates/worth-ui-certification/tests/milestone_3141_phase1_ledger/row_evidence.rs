use super::{
    claim_digest, command_binding, requirement_contract_for, result_artifact, source_symbol,
    validate_cost, workspace_source_inventory, Row,
};

pub(super) fn validate_world(row: &Row) -> Result<(), String> {
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

pub(super) fn validate_execution(row: &Row) -> Result<(), String> {
    if row["retained_failure_artifact"] != row["retained_result_artifact"] {
        return Err("failure posture is not retained by the governed result artifact".to_owned());
    }
    let current_source = row_is_current_source(row)?;
    let command = command_binding::validate(command_binding::CommandClaim {
        command: &row["exact_command"],
        requirement: &row["requirement"],
        production_entry: &row["production_entry"],
        oracle_entry: &row["independent_oracle"],
        source_identity: &row["source_identity"],
        current_source,
    })
    .map_err(|error| format!("{}: {error}", row["requirement"]))?;
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
            structural_counter: &row["structural_counters"],
            construction_cost: &row["construction_cost"],
            execution_cost: &row["execution_cost"],
            source_validation: if current_source {
                result_artifact::SourceValidationPosture::CurrentSource
            } else {
                source_validation_posture(&row["phase"])
            },
        },
        &command,
    )
    .map_err(|error| format!("{}: {error}", row["requirement"]))?;
    validate_named_entry_for_row(row, &row["production_entry"])?;
    validate_named_entry_for_row(row, &row["independent_oracle"])?;
    Ok(())
}

fn validate_named_entry_for_row(row: &Row, value: &str) -> Result<(), String> {
    if !row_is_current_source(row)?
        && matches!(
            source_validation_posture(&row["phase"]),
            result_artifact::SourceValidationPosture::HistoricalArtifactOnly
        )
    {
        if matches!(row["phase"].as_str(), "3" | "4") {
            return validate_unreconstructible_historical_entry(row, value);
        }
        return validate_historical_named_entry(value, &row["source_revision"]);
    }
    validate_named_entry(value)
}

fn validate_unreconstructible_historical_entry(row: &Row, value: &str) -> Result<(), String> {
    let Some((source, symbol)) = value.rsplit_once("::") else {
        return Err("evidence entry lacks a named symbol".to_owned());
    };
    if symbol.is_empty()
        || !source.ends_with(".rs")
        || !row["source_identity"]
            .split(';')
            .any(|identity| identity == source)
    {
        return Err("historical evidence entry is not source-bound".to_owned());
    }
    Ok(())
}

fn validate_historical_named_entry(value: &str, revision: &str) -> Result<(), String> {
    let Some((source, symbol)) = value.rsplit_once("::") else {
        return Err("evidence entry lacks a named symbol".to_owned());
    };
    if symbol.is_empty() || !source.ends_with(".rs") {
        return Err("invalid evidence entry".to_owned());
    }
    let source_text = super::source_digest::file_at_revision(revision, source)?;
    let source_text = std::str::from_utf8(&source_text)
        .map_err(|error| format!("historical Rust source is not UTF-8: {error}"))?;
    source_symbol::validate_text(source_text, symbol, source)
}

pub(super) fn source_validation_posture(phase: &str) -> result_artifact::SourceValidationPosture {
    if matches!(phase, "1" | "2" | "3" | "4") {
        result_artifact::SourceValidationPosture::HistoricalArtifactOnly
    } else {
        result_artifact::SourceValidationPosture::CurrentSource
    }
}

pub(super) fn validate_observations(row: &Row) -> Result<(), String> {
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
    let contract = requirement_contract_for(&row["requirement"])
        .ok_or_else(|| "missing requirement contract".to_owned())?;
    if contract.requires_presented_source()
        && !row["presented_source_readback"].starts_with("observed:")
    {
        return Err("requirement needs a presented-source observation".to_owned());
    }
    if contract.requires_client_area() && !row["client_area_observation"].starts_with("observed:") {
        return Err("requirement needs a client-area observation".to_owned());
    }
    Ok(())
}

pub(super) fn validate_sources(row: &Row) -> Result<(), String> {
    let current_source = row_is_current_source(row)?;
    for source in row["source_identity"].split(';') {
        if !current_source {
            if historical_or_retained_source_exists(source, &row["source_revision"])
                || retained_artifact_names_source(row, source)
            {
                continue;
            }
            return Err(format!("missing historical source {source}"));
        }
        if verifier_owned_source_exists(source) {
            continue;
        }
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

fn retained_artifact_names_source(row: &Row, source: &str) -> bool {
    let Ok(path) = super::source_digest::repository_file(&row["retained_result_artifact"]) else {
        return false;
    };
    let Ok(bytes) = std::fs::read(path) else {
        return false;
    };
    let Ok(artifact) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return false;
    };
    artifact["source_digest"].as_str() == Some(row["source_digest"].as_str())
        && artifact["source_identity"]
            .as_array()
            .is_some_and(|sources| sources.iter().any(|value| value.as_str() == Some(source)))
}

pub(super) fn row_is_current_source(row: &Row) -> Result<bool, String> {
    let path = super::source_digest::repository_file(&row["retained_result_artifact"])?;
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let artifact: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    Ok(artifact["mapping_source_identity"].is_array()
        && artifact["source_rebindings"].is_array()
        && row["source_digest"] == super::source_digest::calculate(&row["source_identity"])?)
}

fn verifier_owned_source_exists(source: &str) -> bool {
    source
        .replace('\\', "/")
        .starts_with("workspaces/worth-ui/target/worth-ui-3141-verify-")
        && super::source_digest::repository_file(source).is_ok()
}

fn historical_or_retained_source_exists(source: &str, revision: &str) -> bool {
    super::source_digest::repository_file(source).is_ok()
        || super::source_digest::file_at_revision(revision, source).is_ok()
}

pub(super) fn validate_named_entry(value: &str) -> Result<(), String> {
    let Some((source, symbol)) = value.rsplit_once("::") else {
        return Err("evidence entry lacks a named symbol".to_owned());
    };
    if symbol.is_empty() || !source.ends_with(".rs") {
        return Err("invalid evidence entry".to_owned());
    }
    let source_path = resolve_source(source).ok_or_else(|| format!("missing source {source}"))?;
    source_symbol::validate(&source_path, symbol)
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
