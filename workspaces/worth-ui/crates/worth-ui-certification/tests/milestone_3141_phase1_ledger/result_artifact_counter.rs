use serde_json::Value;

pub(super) fn validate(requirement: &str, artifact: &Value, expected: u64) -> Result<(), String> {
    if requirement.starts_with("P1-") {
        return validate_phase_one(requirement, artifact, expected);
    }
    let observation = &artifact["boundary_observation"];
    let observed = match requirement {
        "P2-APPLICATION-01" => value(observation, &["peak", "application_drivers"]),
        "P2-CLOSE-01" => observation["terminal_census"]
            .as_object()
            .and_then(|census| {
                census
                    .values()
                    .map(Value::as_u64)
                    .try_fold(0_u64, |total, value| value.map(|value| total + value))
            }),
        "P2-EVENT-LOOP-01" => observation["graphics"]["event_loop_thread_matches_launch"]
            .as_bool()
            .map(u64::from),
        "P2-GRAPHICS-01" => value(observation, &["peak", "devices"]),
        "P2-PIXELS-01" => observation["client_control_points"]
            .as_array()
            .map(|points| points.len() as u64),
        "P2-PORTS-01" => native_port_crossings(observation),
        "P2-PRESENT-01" => value(observation, &["counters", "presents"]),
        "P2-READINESS-01" => value(observation, &["counters", "readiness_signals"]),
        "P2-WINDOW-01" => value(observation, &["peak", "windows"]),
        "P2-WORLD-01" => observation["terminal_zero"].as_bool().map(u64::from),
        _ => None,
    };
    (observed == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("{requirement} structural counter is not causally observed"))
}

fn validate_phase_one(requirement: &str, artifact: &Value, expected: u64) -> Result<(), String> {
    if requirement == "P1-CONSUMERS-01" {
        let observed = artifact["executed_test_count"].as_u64().unwrap_or_default()
            + artifact["hostile_control"]["executed_test_count"]
                .as_u64()
                .unwrap_or_default();
        return (observed == expected)
            .then_some(())
            .ok_or_else(|| "P1-CONSUMERS-01 consumer count is not execution-observed".to_owned());
    }
    let prefix = "WORTH_UI_LEDGER_COUNTERS=";
    let stdout = artifact["test_stdout"]
        .as_str()
        .ok_or_else(|| "Phase 1 artifact omits test stdout".to_owned())?;
    let payloads = stdout
        .lines()
        .filter_map(|line| line.strip_prefix(prefix))
        .collect::<Vec<_>>();
    if payloads.len() != 1 {
        return Err("Phase 1 test must emit one structural counter observation".to_owned());
    }
    let values: Value = serde_json::from_str(payloads[0])
        .map_err(|error| format!("invalid Phase 1 counter observation: {error}"))?;
    (values.get(requirement).and_then(Value::as_u64) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("{requirement} structural counter is not test-observed"))
}

fn value(root: &Value, path: &[&str]) -> Option<u64> {
    let mut current = root;
    for field in path {
        current = current.get(*field)?;
    }
    current.as_u64()
}

fn native_port_crossings(observation: &Value) -> Option<u64> {
    observation["counters"]["port_crossings"].as_u64()
}
