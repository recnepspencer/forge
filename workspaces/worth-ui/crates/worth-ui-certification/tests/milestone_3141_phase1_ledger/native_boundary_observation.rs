use serde_json::Value;

use crate::milestone_3141_phase1_ledger::result_artifact_binding::{require_str, require_u64};

pub(super) fn validate(artifact: &Value) -> Result<(), String> {
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
    require_json(
        observation,
        "quiescent_control_points_equal",
        &Value::Bool(true),
    )?;
    require_json(observation, "normal_os_close_requests", &Value::from(1))?;
    require_u64(observation, "product_processes", 1)?;
    require_json(observation, "terminal_zero", &Value::Bool(true))?;
    super::super::result_artifact_environment::validate(observation)?;
    validate_dpi_basis(observation)?;
    validate_client_control_points(observation)?;
    validate_native_identity_attribution(observation)?;
    validate_native_counters(observation)?;
    validate_native_graphics(observation)?;
    validate_native_resources(observation)
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
    require_u64(
        runtime,
        "authored_provenance_digest",
        expected_native_seed_authored_provenance_digest(),
    )
    .map_err(|_| "runtime attribution has wrong authored provenance digest".to_owned())?;
    require_u64(
        runtime,
        "authored_semantic_identity_digest",
        expected_native_seed_authored_semantic_identity_digest(),
    )
    .map_err(|_| "runtime attribution has wrong authored semantic identity digest".to_owned())?;
    Ok(())
}

fn expected_native_seed_authored_provenance_digest() -> u64 {
    independent_text_digest("app/native_seed.wui") ^ 1_u64.rotate_left(13)
}

fn expected_native_seed_authored_semantic_identity_digest() -> u64 {
    independent_text_digest("component:platform.pulse.native_seed.rectangle")
}

fn independent_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325_u64, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
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
    let schema = resource_schema(peak, &classes)?;
    for class in &classes {
        if schema == ResourceSchema::Current || PHASE_TWO_RESOURCE_CLASSES.contains(class) {
            let expected = if *class == "retained_targets" {
                2
            } else if PHASE_TWO_RESOURCE_CLASSES.contains(class) {
                1
            } else if PHASE_FIVE_STANDING_RESOURCE_CLASSES.contains(class) {
                1
            } else {
                0
            };
            require_u64(peak, class, expected)?;
        }
    }
    let terminal = observation
        .get("terminal_census")
        .ok_or_else(|| "native boundary omits terminal resource census".to_owned())?;
    if resource_schema(terminal, &classes)? != schema {
        return Err("native resource census schemas disagree".to_owned());
    }
    for class in &classes {
        if schema == ResourceSchema::Current || PHASE_TWO_RESOURCE_CLASSES.contains(class) {
            require_u64(terminal, class, 0)?;
        }
    }
    let [width, height] = client_physical_size(observation)?;
    if width > 0 && height > 0 {
        Ok(())
    } else {
        Err("native boundary client extent is invalid".to_owned())
    }
}

const PHASE_TWO_RESOURCE_CLASSES: &[&str] = &[
    "windows",
    "surfaces",
    "adapters",
    "devices",
    "queues",
    "retained_targets",
    "registrations",
    "readback_buffers",
    "pending_submissions",
    "event_wake_registrations",
    "application_drivers",
];

const PHASE_FIVE_STANDING_RESOURCE_CLASSES: &[&str] =
    &["physical_signal_runtimes", "physical_signal_workers"];

#[derive(Clone, Copy, Eq, PartialEq)]
enum ResourceSchema {
    PhaseTwo,
    Current,
}

fn resource_schema(value: &Value, classes: &[&str]) -> Result<ResourceSchema, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "native resource census is not an object".to_owned())?;
    if object.len() == classes.len() && classes.iter().all(|class| object.contains_key(*class)) {
        Ok(ResourceSchema::Current)
    } else if object.len() == PHASE_TWO_RESOURCE_CLASSES.len()
        && PHASE_TWO_RESOURCE_CLASSES
            .iter()
            .all(|class| object.contains_key(*class))
    {
        Ok(ResourceSchema::PhaseTwo)
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
