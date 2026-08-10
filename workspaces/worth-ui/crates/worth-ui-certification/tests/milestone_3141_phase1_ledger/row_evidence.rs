use super::{
    claim_digest, command_binding, requirement_contract, result_artifact, source_symbol,
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
    let command = command_binding::validate(command_binding::CommandClaim {
        command: &row["exact_command"],
        requirement: &row["requirement"],
        production_entry: &row["production_entry"],
        oracle_entry: &row["independent_oracle"],
        source_identity: &row["source_identity"],
    })?;
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
        },
        &command,
    )?;
    validate_named_entry(&row["production_entry"])?;
    validate_named_entry(&row["independent_oracle"])?;
    Ok(())
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
    let contract = requirement_contract::for_requirement(&row["requirement"])
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
