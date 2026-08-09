use serde_json::Value;

use super::command_binding::CommandBinding;
use super::result_artifact_binding::{
    cargo_command, ignored_list_command, read_artifact, require_array, require_duration_within,
    require_i64, require_str, require_u64, validate_ledger_fields,
};
use super::source_digest;

pub(super) use super::result_artifact_binding::current_revision;

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
}

pub(super) fn validate(ledger: LedgerResult<'_>, command: &CommandBinding) -> Result<(), String> {
    validate_ledger_fields(&ledger, command)?;
    let artifact = read_artifact(ledger.artifact)?;
    require_u64(&artifact, "schema_version", 5)?;
    require_str(&artifact, "package", &command.package)?;
    require_str(&artifact, "target_kind", &command.target_kind)?;
    require_str(&artifact, "target_name", &command.target_name)?;
    require_array(&artifact, "features", &command.features)?;
    require_str(&artifact, "test_name", &command.test_name)?;
    require_str(&artifact, "requirement", &command.requirement)?;
    require_str(&artifact, "claim_digest", ledger.claim_digest)?;
    require_str(&artifact, "structural_counter", ledger.structural_counter)?;
    require_str(&artifact, "construction_cost", ledger.construction_cost)?;
    require_str(&artifact, "execution_cost", ledger.execution_cost)?;
    require_u64(&artifact, "matched_test_count", 1)?;
    let expected_ignored =
        super::execution_contract::expected_declared_ignored(&command.requirement);
    require_u64(
        &artifact,
        "declared_ignored_test_count",
        u64::from(expected_ignored),
    )?;
    require_json(
        &artifact,
        "expected_declared_ignored",
        &Value::Bool(expected_ignored),
    )?;
    require_u64(&artifact, "executed_test_count", 1)?;
    require_u64(&artifact, "passed_test_count", 1)?;
    require_u64(&artifact, "ignored_test_count", 0)?;
    require_str(&artifact, "exit_posture", "passed")?;
    require_i64(&artifact, "list_exit_code", 0)?;
    require_i64(&artifact, "test_exit_code", 0)?;
    let budget = super::execution_contract::main_budget_ms(&command.requirement);
    require_u64(&artifact, "test_budget_ms", budget)?;
    require_duration_within(&artifact, "list_duration_ms", 300_000)?;
    require_duration_within(&artifact, "ignored_list_duration_ms", 300_000)?;
    require_duration_within(&artifact, "test_duration_ms", budget)?;
    require_str(&artifact, "source_revision", ledger.source_revision)?;
    require_str(&artifact, "source_digest", ledger.source_digest)?;
    require_str(&artifact, "source_state_digest", ledger.source_state_digest)?;
    require_str(&artifact, "run_nonce", ledger.run_nonce)?;
    require_array(&artifact, "source_identity", &command.sources)?;
    require_array(&artifact, "list_command", &cargo_command(command, true))?;
    require_array(
        &artifact,
        "ignored_list_command",
        &ignored_list_command(command),
    )?;
    require_array(&artifact, "test_command", &cargo_command(command, false))?;
    super::result_artifact_control::validate(&artifact, command.control.as_ref())?;
    super::compile_case_binding::validate(&command.requirement, &command.sources)?;
    super::result_artifact_counter::validate(
        &command.requirement,
        &artifact,
        super::execution_contract::counter_amount(&command.requirement)
            .expect("every requirement has one counter"),
    )?;
    super::result_artifact_cost::validate(&command.requirement, &artifact)?;
    if command.requirement.starts_with("P2-") {
        validate_native_boundary_observation(&artifact)?;
    }
    if source_digest::file_digest(ledger.artifact)? != ledger.result_artifact_digest {
        return Err("result artifact digest is stale".to_owned());
    }
    Ok(())
}

fn validate_native_boundary_observation(artifact: &Value) -> Result<(), String> {
    let observation = artifact
        .get("boundary_observation")
        .ok_or_else(|| "Phase 2 artifact omits its boundary observation".to_owned())?;
    require_str(
        observation,
        "schema",
        "worth-ui-native-boundary-observation-v1",
    )?;
    require_json(
        observation,
        "presented_source",
        &serde_json::json!([47, 129, 247, 255]),
    )?;
    require_json(
        observation,
        "retained_center",
        &serde_json::json!([47, 129, 247, 255]),
    )?;
    require_json(
        observation,
        "retained_baseline",
        &serde_json::json!([0, 0, 0, 0]),
    )?;
    require_json(observation, "quiescent_capture_equal", &Value::Bool(true))?;
    require_json(observation, "normal_os_close_requests", &Value::from(1))?;
    require_u64(observation, "product_processes", 1)?;
    require_json(observation, "terminal_zero", &Value::Bool(true))?;
    super::result_artifact_environment::validate(observation)?;
    validate_dpi_basis(observation)?;
    validate_client_control_points(observation)?;
    validate_client_baseline_point(observation)?;
    validate_native_identity_attribution(observation)?;
    validate_native_counters(observation)?;
    validate_native_graphics(observation)?;
    validate_native_resources(observation)
}

fn validate_client_baseline_point(observation: &Value) -> Result<(), String> {
    let baseline = observation
        .get("client_baseline_point")
        .ok_or_else(|| "native boundary omits client baseline point".to_owned())?;
    require_u64(baseline, "x", 0)?;
    require_u64(baseline, "y", 0)?;
    require_json(baseline, "rgba", &serde_json::json!([255, 255, 255, 255]))
}

fn validate_native_identity_attribution(observation: &Value) -> Result<(), String> {
    let runtime = observation
        .get("runtime_attribution")
        .ok_or_else(|| "native boundary omits runtime attribution".to_owned())?;
    for name in [
        "surface",
        "binding",
        "mounted_instance",
        "frame",
        "presentation_attempt",
        "node_receipt",
    ] {
        let expected = runtime
            .get(name)
            .and_then(Value::as_u64)
            .filter(|identity| *identity > 0)
            .ok_or_else(|| format!("runtime attribution has invalid {name}"))?;
        require_u64(observation, name, expected)?;
    }
    Ok(())
}

fn validate_dpi_basis(observation: &Value) -> Result<(), String> {
    require_json(
        observation,
        "logical_bounds_milli",
        &serde_json::json!([16_000, 12_000, 128_000, 72_000]),
    )?;
    let scale = observation
        .get("scale_factor_milli")
        .and_then(Value::as_u64)
        .filter(|scale| *scale > 0)
        .ok_or_else(|| "native boundary DPI scale is invalid".to_owned())?;
    let size = client_physical_size(observation)?;
    let expected = [160 * scale / 1_000, 96 * scale / 1_000];
    if size == expected {
        Ok(())
    } else {
        Err("native boundary physical extent is not bound to its DPI basis".to_owned())
    }
}

fn validate_client_control_points(observation: &Value) -> Result<(), String> {
    let points = observation
        .get("client_control_points")
        .and_then(Value::as_array)
        .filter(|points| points.len() == 3)
        .ok_or_else(|| "native boundary must retain three client control points".to_owned())?;
    let expected = serde_json::json!([47, 129, 247, 255]);
    let [width, height] = client_physical_size(observation)?;
    let expected_coordinates = [
        [width / 4, height / 4],
        [width / 2, height / 2],
        [width * 3 / 4, height * 3 / 4],
    ];
    for (point, [x, y]) in points.iter().zip(expected_coordinates) {
        if point.get("rgba") != Some(&expected)
            || point.get("x").and_then(Value::as_u64) != Some(x)
            || point.get("y").and_then(Value::as_u64) != Some(y)
        {
            return Err("native client control point coordinate or pixel drifted".to_owned());
        }
    }
    Ok(())
}

fn validate_native_counters(observation: &Value) -> Result<(), String> {
    let counters = observation
        .get("counters")
        .ok_or_else(|| "native boundary omits counters".to_owned())?;
    for name in [
        "surface_acquisitions",
        "queue_submissions",
        "presents",
        "readiness_signals",
        "redraw_turns",
        "idle_wait_turns",
    ] {
        require_u64(counters, name, 1)?;
    }
    require_u64(counters, "render_passes", 2)?;
    require_u64(counters, "port_crossings", 4)?;
    if counters
        .get("coalesced_wakes")
        .and_then(Value::as_u64)
        .is_some_and(|count| count <= 4)
    {
        Ok(())
    } else {
        Err("native coalesced_wakes exceeds the bounded startup schedule".to_owned())
    }
}

fn validate_native_graphics(observation: &Value) -> Result<(), String> {
    let graphics = observation
        .get("graphics")
        .ok_or_else(|| "native boundary omits graphics qualification".to_owned())?;
    for (field, expected) in [
        ("backend", "Dx12"),
        ("surface_format", "Bgra8UnormSrgb"),
        ("present_mode", "Fifo"),
        ("alpha_mode", "PreMultiplied"),
        ("retained_format", "Rgba8UnormSrgb"),
    ] {
        require_str(graphics, field, expected)?;
    }
    if !matches!(
        graphics.get("device_type").and_then(Value::as_str),
        Some("DiscreteGpu" | "IntegratedGpu" | "VirtualGpu")
    ) {
        return Err("native adapter has a forbidden device class".to_owned());
    }
    require_json(
        graphics,
        "event_loop_thread_matches_launch",
        &Value::Bool(true),
    )?;
    if graphics
        .get("max_texture_dimension_2d")
        .and_then(Value::as_u64)
        .is_some_and(|value| value >= 16_384)
    {
        Ok(())
    } else {
        Err("native adapter does not meet the qualified extent".to_owned())
    }
}

fn validate_native_resources(observation: &Value) -> Result<(), String> {
    let peak = observation
        .get("peak")
        .ok_or_else(|| "native boundary omits peak resource census".to_owned())?;
    let classes = worth_ui_host_native::UiNativeResourceCensus::field_names().collect::<Vec<_>>();
    require_exact_resource_schema(peak, &classes)?;
    for class in &classes {
        require_u64(peak, class, 1)?;
    }
    let terminal = observation
        .get("terminal_census")
        .ok_or_else(|| "native boundary omits terminal resource census".to_owned())?;
    require_exact_resource_schema(terminal, &classes)?;
    for class in &classes {
        require_u64(terminal, class, 0)?;
    }
    let [width, height] = client_physical_size(observation)?;
    if width > 0 && height > 0 {
        Ok(())
    } else {
        Err("native boundary client extent is invalid".to_owned())
    }
}

fn require_exact_resource_schema(value: &Value, classes: &[&str]) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "native resource census is not an object".to_owned())?;
    if object.len() == classes.len() && classes.iter().all(|class| object.contains_key(*class)) {
        Ok(())
    } else {
        Err("native resource census schema drifted".to_owned())
    }
}

fn client_physical_size(observation: &Value) -> Result<[u64; 2], String> {
    observation
        .get("client_physical_size")
        .and_then(Value::as_array)
        .filter(|size| size.len() == 2)
        .and_then(|size| Some([size[0].as_u64()?, size[1].as_u64()?]))
        .ok_or_else(|| "native boundary client extent is invalid".to_owned())
}

fn require_json(value: &Value, field: &str, expected: &Value) -> Result<(), String> {
    (value.get(field) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

#[cfg(test)]
#[path = "result_artifact_mutation.rs"]
mod mutation_tests;
