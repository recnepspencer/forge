use serde_json::Value;

use super::source_digest;

pub(super) fn validate(requirement: &str, artifact: &Value) -> Result<(), String> {
    let control_tests = artifact["hostile_control"]["executed_test_count"]
        .as_u64()
        .unwrap_or(0);
    if matches!(
        requirement,
        "P3-PREDECESSOR-01" | "P4-PREDECESSOR-01" | "P5-PREDECESSOR-01"
    ) {
        return validate_predecessor(requirement, artifact, control_tests);
    }
    if requirement == "P3-HP02-WORLD-01" {
        return validate_phase_three_world(artifact, control_tests);
    }
    if requirement == "P5-ATLAS-PINNING-01" {
        return validate_gate_d_pin_world(artifact, control_tests);
    }
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
    let compile_sessions = compile_sessions(requirement, artifact)?;
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

fn validate_gate_d_pin_world(artifact: &Value, control_tests: u64) -> Result<(), String> {
    let observation = &artifact["boundary_observation"];
    let transactions = observation["atlas_transactions"]
        .as_u64()
        .ok_or_else(|| "Gate D pin world omits atlas transaction count".to_owned())?;
    let presentations = observation["presentations"]
        .as_u64()
        .ok_or_else(|| "Gate D pin world omits presentation count".to_owned())?;
    let construction = format!(
        "main-tests=1;hostile-controls={control_tests};product-processes=1;compile-sessions=0;courtroom-worlds=1"
    );
    let execution = format!(
        "executed-tests={};presentations={presentations};atlas-transactions={transactions}",
        1 + control_tests
    );
    if control_tests != 1
        || artifact["construction_cost"].as_str() != Some(&construction)
        || artifact["execution_cost"].as_str() != Some(&execution)
    {
        return Err("Gate D pin cost is not derived from the product observation".to_owned());
    }
    Ok(())
}

fn validate_phase_three_world(artifact: &Value, control_tests: u64) -> Result<(), String> {
    let support = &artifact["supporting_world"];
    let support_worlds = metric(support, "worlds")?;
    let support_presentations = metric(support, "presentations")?;
    let stdout = artifact["test_stdout"].as_str().unwrap_or_default();
    let main_worlds = u64::from(stdout.lines().any(|line| line == "WORTH_UI_LEDGER_WORLD=1"));
    let main_presentations = stdout_numeric(stdout, "WORTH_UI_LEDGER_PRESENTATIONS=")?.unwrap_or(0);
    let construction = format!(
        "main-tests=1;hostile-controls={control_tests};product-processes=1;compile-sessions=0;courtroom-worlds={main_worlds};shared-mounted-worlds={support_worlds}"
    );
    let execution = format!(
        "executed-tests={};presentations={main_presentations};shared-presentations={support_presentations}",
        1 + control_tests,
    );
    if control_tests != 1
        || artifact["construction_cost"].as_str() != Some(&construction)
        || artifact["execution_cost"].as_str() != Some(&execution)
    {
        return Err("Phase 3 world cost is not bound to both executed worlds".to_owned());
    }
    Ok(())
}

fn validate_predecessor(
    requirement: &str,
    artifact: &Value,
    control_tests: u64,
) -> Result<(), String> {
    let phase = requirement
        .strip_prefix('P')
        .and_then(|suffix| suffix.chars().next())
        .ok_or_else(|| "predecessor result has invalid requirement identity".to_owned())?;
    let identity = artifact["source_identity"]
        .as_array()
        .and_then(|sources| {
            sources
                .iter()
                .filter_map(Value::as_str)
                .find(|source| source.ends_with(&format!("p{phase}-predecessor-handoff.json")))
        })
        .ok_or_else(|| "predecessor result omits its handoff artifact".to_owned())?;
    let content = std::fs::read_to_string(source_digest::repository_file(identity)?)
        .map_err(|error| format!("cannot read predecessor handoff cost: {error}"))?;
    let predecessor: Value = serde_json::from_str(&content)
        .map_err(|error| format!("invalid predecessor handoff cost: {error}"))?;
    let main = metric(&predecessor, "main_test_executions")?;
    let hostile = metric(&predecessor, "hostile_control_executions")?;
    let closure = metric(&predecessor, "closure_test_executions")?;
    let compile_sessions = metric(&predecessor, "compile_sessions")?;
    let processes = metric(&predecessor, "product_processes")?;
    let worlds = metric(&predecessor, "courtroom_worlds")?;
    let presentations = metric(&predecessor, "presentations")?;
    let construction = format!(
        "main-tests={};hostile-controls={};product-processes={processes};compile-sessions={compile_sessions};courtroom-worlds={worlds}",
        main + 1,
        hostile + control_tests,
    );
    let execution = format!(
        "executed-tests={};presentations={presentations}",
        main + hostile + closure + 1 + control_tests,
    );
    let operational = &artifact["operational_predecessor_cost"];
    let records_operational_cost = !operational.is_null();
    if records_operational_cost
        && (operational["construction_cost"].as_str() != Some(&construction)
            || operational["execution_cost"].as_str() != Some(&execution))
    {
        return Err(
            "operational predecessor cost is not derived from its governed rerun".to_owned(),
        );
    }
    if matches!(requirement, "P4-PREDECESSOR-01" | "P5-PREDECESSOR-01")
        && (!records_operational_cost
            || artifact["construction_cost"].as_str() != Some(&construction)
            || artifact["execution_cost"].as_str() != Some(&execution))
    {
        return Err("current predecessor claim does not own its operational cost".to_owned());
    }
    Ok(())
}

fn metric(value: &Value, field: &str) -> Result<u64, String> {
    value[field]
        .as_u64()
        .ok_or_else(|| format!("predecessor handoff omits {field}"))
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
    if requirement.starts_with("P3-") {
        let (shared_world, shared_presentations) =
            if matches!(requirement, "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01") {
                ("shared-mounted-worlds", 5)
            } else {
                ("shared-native-worlds", 7)
            };
        let construction = format!(
            "main-tests=0;hostile-controls={control_tests};product-processes=0;compile-sessions=0;courtroom-worlds=0;{shared_world}=1"
        );
        let execution = format!(
            "executed-tests={control_tests};presentations=0;shared-presentations={shared_presentations}"
        );
        if control_tests == 1
            && artifact["construction_cost"].as_str() == Some(&construction)
            && artifact["execution_cost"].as_str() == Some(&execution)
        {
            return Ok(());
        }
        return Err("shared Phase 3 cost is not bound to one native world".to_owned());
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

fn compile_sessions(requirement: &str, artifact: &Value) -> Result<u64, String> {
    let public_examples = artifact["execution_receipts"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|receipt| receipt["role"].as_str() == Some("public-example"))
        .count() as u64;
    if public_examples != u64::from(requirement == "P4-FONT-COLLECTION-01") {
        return Err("result artifact has the wrong public-example execution census".to_owned());
    }
    let sources = artifact["source_identity"]
        .as_array()
        .ok_or_else(|| "artifact source identity is absent".to_owned())?;
    let identity = sources
        .iter()
        .filter_map(Value::as_str)
        .find(|source| source.ends_with("compile-contracts.json"));
    let Some(identity) = identity else {
        return Ok(public_examples);
    };
    let content = std::fs::read_to_string(source_digest::repository_file(identity)?)
        .map_err(|error| format!("cannot read compile artifact cost: {error}"))?;
    let compile: Value = serde_json::from_str(&content)
        .map_err(|error| format!("invalid compile artifact cost: {error}"))?;
    compile["cargo_sessions"]
        .as_u64()
        .map(|sessions| sessions + public_examples)
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
