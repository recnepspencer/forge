use serde_json::Value;

use super::source_digest;

pub(super) fn validate(requirement: &str, artifact: &Value) -> Result<(), String> {
    let control_tests = artifact["hostile_control"]["executed_test_count"]
        .as_u64()
        .unwrap_or(0);
    let p2 = requirement.starts_with("P2-");
    if artifact.get("shared_main_artifact").is_some() {
        return validate_shared(requirement, artifact, control_tests);
    }
    let product_processes = if p2 {
        artifact["boundary_observation"]["product_processes"]
            .as_u64()
            .ok_or_else(|| "native world omits its product process census".to_owned())?
    } else {
        0
    };
    let compile_sessions = compile_sessions(artifact)?;
    let stdout = artifact["test_stdout"].as_str().unwrap_or_default();
    let worlds = u64::from(p2 || stdout.lines().any(|line| line == "WORTH_UI_LEDGER_WORLD=1"));
    let presentations = if p2 {
        artifact["boundary_observation"]["counters"]["presents"]
            .as_u64()
            .ok_or_else(|| "native world omits presentation count".to_owned())?
    } else {
        stdout_numeric(stdout, "WORTH_UI_LEDGER_PRESENTATIONS=")?.unwrap_or(0)
    };
    let construction = format!(
        "main-tests=1;hostile-controls={control_tests};product-processes={product_processes};compile-sessions={compile_sessions};courtroom-worlds={worlds}"
    );
    let execution = format!(
        "executed-tests={};presentations={presentations}",
        1 + control_tests
    );
    if artifact["construction_cost"].as_str() != Some(&construction)
        || artifact["execution_cost"].as_str() != Some(&execution)
    {
        return Err("result artifact cost is not derived from execution observations".to_owned());
    }
    Ok(())
}

fn validate_shared(requirement: &str, artifact: &Value, control_tests: u64) -> Result<(), String> {
    if requirement == "P1-HEADLESS-COST-01" {
        let construction = "main-tests=0;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=0;shared-mounted-worlds=1";
        let execution = "executed-tests=0;presentations=0;shared-presentations=7";
        if control_tests == 0
            && artifact["construction_cost"].as_str() == Some(construction)
            && artifact["execution_cost"].as_str() == Some(execution)
        {
            return Ok(());
        }
        return Err("shared Phase 1 cost is not bound to one mounted world".to_owned());
    }
    let construction = format!(
        "main-tests=0;hostile-controls={control_tests};product-processes=0;compile-sessions=0;courtroom-worlds=0;shared-native-worlds=1"
    );
    let execution =
        format!("executed-tests={control_tests};presentations=0;shared-presentations=1");
    if artifact["boundary_observation"]["product_processes"].as_u64() != Some(1)
        || artifact["boundary_observation"]["counters"]["presents"].as_u64() != Some(1)
        || artifact["construction_cost"].as_str() != Some(&construction)
        || artifact["execution_cost"].as_str() != Some(&execution)
    {
        return Err("shared Phase 2 cost is not bound to one native world".to_owned());
    }
    Ok(())
}

fn compile_sessions(artifact: &Value) -> Result<u64, String> {
    let sources = artifact["source_identity"]
        .as_array()
        .ok_or_else(|| "artifact source identity is absent".to_owned())?;
    let identity = sources
        .iter()
        .filter_map(Value::as_str)
        .find(|source| source.ends_with("compile-contracts.json"));
    let Some(identity) = identity else {
        return Ok(0);
    };
    let content = std::fs::read_to_string(source_digest::repository_file(identity)?)
        .map_err(|error| format!("cannot read compile artifact cost: {error}"))?;
    let compile: Value = serde_json::from_str(&content)
        .map_err(|error| format!("invalid compile artifact cost: {error}"))?;
    compile["cargo_sessions"]
        .as_u64()
        .ok_or_else(|| "compile artifact omits Cargo session count".to_owned())
}

fn stdout_numeric(stdout: &str, prefix: &str) -> Result<Option<u64>, String> {
    let values = stdout
        .lines()
        .filter_map(|line| line.strip_prefix(prefix))
        .collect::<Vec<_>>();
    match values.as_slice() {
        [] => Ok(None),
        [value] => value
            .parse()
            .map(Some)
            .map_err(|_| "invalid numeric execution-cost observation".to_owned()),
        _ => Err("duplicate execution-cost observations".to_owned()),
    }
}
