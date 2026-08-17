use serde_json::{json, Value};

use super::{require_duration_within, validate_native_boundary_observation};
use crate::milestone_3141_phase1_ledger::{
    command_binding::ControlBinding, result_artifact_control,
};

#[test]
fn phase_two_artifact_rejects_missing_renamed_or_nonexecuted_hostile_control() {
    let binding = ControlBinding {
        package: "worth-ui-host-native".to_owned(),
        target_kind: "lib".to_owned(),
        target_name: "lib".to_owned(),
        features: Vec::new(),
        test_name: "native::readiness::tests::hostile".to_owned(),
    };
    let mut artifact = json!({"hostile_control": {
        "package": binding.package,
        "target_kind": binding.target_kind,
        "target_name": binding.target_name,
        "test_name": binding.test_name,
        "features": [],
        "matched_test_count": 1,
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "exit_posture": "passed",
        "list_exit_code": 0,
        "test_exit_code": 0,
        "list_duration_ms": 1,
        "test_duration_ms": 1,
        "test_budget_ms": 10_000,
        "list_command": result_artifact_control::cargo_command(&binding, true),
        "test_command": result_artifact_control::cargo_command(&binding, false),
    }});
    result_artifact_control::validate(&artifact, Some(&binding), 10_000, "P2-READINESS-01")
        .unwrap();
    for (field, value) in [
        ("matched_test_count", json!(0)),
        ("executed_test_count", json!(0)),
        ("passed_test_count", json!(0)),
        ("exit_posture", json!("test-failed")),
        ("test_name", json!("native::readiness::tests::renamed")),
    ] {
        let original = artifact["hostile_control"][field].clone();
        artifact["hostile_control"][field] = value;
        assert!(result_artifact_control::validate(
            &artifact,
            Some(&binding),
            10_000,
            "P2-READINESS-01"
        )
        .is_err());
        artifact["hostile_control"][field] = original;
    }
    artifact["hostile_control"] = Value::Null;
    assert!(result_artifact_control::validate(
        &artifact,
        Some(&binding),
        10_000,
        "P2-READINESS-01"
    )
    .is_err());
}

#[test]
fn retained_artifact_rejects_execution_duration_over_its_owned_budget() {
    let lawful = json!({"test_duration_ms": 29_999});
    require_duration_within(&lawful, "test_duration_ms", 30_000).unwrap();
    let over = json!({"test_duration_ms": 30_001});
    assert!(require_duration_within(&over, "test_duration_ms", 30_000).is_err());
}

#[test]
fn phase_two_boundary_observation_rejects_each_causal_mutation() {
    let lawful = artifact();
    validate_native_boundary_observation(&lawful).expect("lawful boundary observation");
    assert_opaque_identity_twin(&lawful);
    assert_pixel_and_environment_mutants(&lawful);
    assert_attribution_and_geometry_mutants(&lawful);
    assert_counter_and_graphics_mutants(&lawful);
    assert_control_point_mutants(&lawful);
    assert_terminal_census_mutants();
}

fn assert_opaque_identity_twin(lawful: &Value) {
    let mut relational_twin = lawful.clone();
    for (index, identity) in [
        "frame",
        "surface",
        "binding",
        "mounted_instance",
        "node_receipt",
        "presentation_attempt",
    ]
    .into_iter()
    .enumerate()
    {
        let value = json!(100_u64 + index as u64);
        relational_twin["boundary_observation"][identity] = value.clone();
        relational_twin["boundary_observation"]["runtime_attribution"][identity] = value;
    }
    validate_native_boundary_observation(&relational_twin)
        .expect("causally equal opaque identities must not be frozen literals");
}

fn assert_pixel_and_environment_mutants(lawful: &Value) {
    assert_mutations(
        lawful,
        &[
            (
                &["presented_source"],
                json!([0, 0, 0, 0]),
                "presented_source",
            ),
            (&["retained_center"], json!([0, 0, 0, 0]), "retained_center"),
            (
                &["retained_baseline"],
                json!([47, 129, 247, 255]),
                "retained_baseline",
            ),
            (
                &["quiescent_control_points_equal"],
                json!(false),
                "quiescent_control_points_equal",
            ),
            (
                &["normal_os_close_requests"],
                json!(0),
                "normal_os_close_requests",
            ),
            (&["terminal_zero"], json!(false), "terminal_zero"),
            (&["os_version"], json!("Linux"), "OS version"),
            (
                &["os_version"],
                json!("Microsoft Windows [Version 10.0.19045.4652]"),
                "OS version",
            ),
            (&["architecture"], json!("aarch64"), "OS version"),
            (&["scale_factor_milli"], json!(0), "DPI scale"),
        ],
    );
}

fn assert_attribution_and_geometry_mutants(lawful: &Value) {
    assert_mutations(
        lawful,
        &[
            (&["frame"], json!(9), "frame"),
            (&["surface"], json!(2), "surface"),
            (&["binding"], json!(4), "binding"),
            (&["mounted_instance"], json!(8), "mounted_instance"),
            (&["node_receipt"], json!(1), "node_receipt"),
            (&["presentation_attempt"], json!(12), "presentation_attempt"),
            (
                &["runtime_attribution", "authored_provenance_digest"],
                json!(1),
                "authored provenance",
            ),
            (
                &["runtime_attribution", "authored_semantic_identity_digest"],
                json!(1),
                "authored semantic identity",
            ),
            (
                &["logical_bounds_milli"],
                json!([0, 0, 80_000, 48_000]),
                "logical_bounds_milli",
            ),
            (&["client_physical_size"], json!([0, 0]), "extent"),
        ],
    );
}

fn assert_counter_and_graphics_mutants(lawful: &Value) {
    assert_mutations(
        lawful,
        &[
            (&["counters", "presents"], json!(2), "presents"),
            (&["counters", "port_crossings"], json!(3), "port_crossings"),
            (
                &["counters", "coalesced_wakes"],
                json!(5),
                "coalesced_wakes",
            ),
            (&["graphics", "backend"], json!("Vulkan"), "backend"),
            (&["graphics", "device_type"], json!("Cpu"), "device class"),
            (
                &["graphics", "event_loop_thread_matches_launch"],
                json!(false),
                "event_loop_thread_matches_launch",
            ),
            (
                &["graphics", "max_texture_dimension_2d"],
                json!(8_192),
                "extent",
            ),
            (&["peak", "queues"], json!(0), "queues"),
            (
                &["peak", "physical_signal_runtimes"],
                json!(0),
                "physical_signal_runtimes",
            ),
        ],
    );
}

fn assert_mutations(lawful: &Value, mutations: &[(&[&str], Value, &str)]) {
    for (path, value, expected) in mutations {
        let mut mutant = lawful.clone();
        replace(&mut mutant["boundary_observation"], path, value.clone());
        let error = validate_native_boundary_observation(&mutant)
            .expect_err("boundary mutant must be rejected");
        assert!(error.contains(expected), "{path:?} failed for {error}");
    }
}

fn assert_control_point_mutants(lawful: &Value) {
    let mut pixel = lawful.clone();
    pixel["boundary_observation"]["client_control_points"][1]["rgba"] = json!([0, 0, 0, 0]);
    assert!(validate_native_boundary_observation(&pixel)
        .unwrap_err()
        .contains("client control point"));
    let mut duplicate = artifact();
    duplicate["boundary_observation"]["client_control_points"][1]["x"] = json!(60);
    duplicate["boundary_observation"]["client_control_points"][1]["y"] = json!(36);
    assert!(validate_native_boundary_observation(&duplicate)
        .unwrap_err()
        .contains("client control point"));
}

fn assert_terminal_census_mutants() {
    for class in worth_ui_host_native::UiNativeResourceCensus::field_names() {
        let mut held = artifact();
        held["boundary_observation"]["terminal_census"][class] = json!(1);
        assert!(
            validate_native_boundary_observation(&held)
                .unwrap_err()
                .contains(class),
            "terminal resource class {class} was omitted"
        );
    }
}

fn replace(root: &mut Value, path: &[&str], value: Value) {
    let (last, parents) = path.split_last().expect("nonempty mutation path");
    let mut target = root;
    for key in parents {
        target = &mut target[*key];
    }
    target[*last] = value;
}

fn artifact() -> Value {
    json!({"boundary_observation": boundary_observation()})
}

fn boundary_observation() -> Value {
    json!({
        "schema": "worth-ui-native-boundary-observation-v1",
        "os_version": "Microsoft Windows [Version 10.0.26100.4652]",
        "architecture": "x86_64",
        "product_processes": 1,
        "presented_source": [47, 129, 247, 255],
        "retained_center": [47, 129, 247, 255],
        "retained_baseline": [0, 0, 0, 0],
        "scale_factor_milli": 1_500,
        "logical_bounds_milli": [16_000, 12_000, 128_000, 72_000],
        "frame": 8,
        "surface": 1,
        "binding": 3,
        "mounted_instance": 7,
        "node_receipt": 6_018_028_539_936_990_355_u64,
        "presentation_attempt": 11,
        "runtime_attribution": attribution_fixture(),
        "client_physical_size": [240, 144],
        "client_control_points": client_points_fixture(),
        "quiescent_control_points_equal": true,
        "normal_os_close_requests": 1,
        "terminal_zero": true,
        "counters": counters_fixture(),
        "graphics": graphics_fixture(),
        "peak": resource_census_fixture(1, 2),
        "terminal_census": resource_census_fixture(0, 0),
    })
}

fn attribution_fixture() -> Value {
    json!({
        "frame": 8,
        "surface": 1,
        "binding": 3,
        "mounted_instance": 7,
        "node_receipt": 6_018_028_539_936_990_355_u64,
        "presentation_attempt": 11,
        "authored_provenance_digest": expected_native_seed_authored_provenance_digest(),
        "authored_semantic_identity_digest": expected_native_seed_authored_semantic_identity_digest()
    })
}

fn client_points_fixture() -> Value {
    json!([
        {"x": 60, "y": 36, "rgba": [47, 129, 247, 255]},
        {"x": 120, "y": 72, "rgba": [47, 129, 247, 255]},
        {"x": 180, "y": 108, "rgba": [47, 129, 247, 255]}
    ])
}

fn counters_fixture() -> Value {
    json!({
        "surface_acquisitions": 1,
        "queue_submissions": 1,
        "presents": 1,
        "readiness_signals": 1,
        "redraw_turns": 1,
        "idle_wait_turns": 1,
        "render_passes": 2,
        "coalesced_wakes": 0,
        "port_crossings": 4
    })
}

fn graphics_fixture() -> Value {
    json!({
        "device_type": "DiscreteGpu",
        "backend": "Dx12",
        "surface_format": "Bgra8UnormSrgb",
        "present_mode": "Fifo",
        "alpha_mode": "PreMultiplied",
        "retained_format": "Rgba8UnormSrgb",
        "max_texture_dimension_2d": 16_384,
        "event_loop_thread_matches_launch": true
    })
}

fn resource_census_fixture(count: u64, retained_targets: u64) -> Value {
    let mut census = serde_json::Map::new();
    for class in worth_ui_host_native::UiNativeResourceCensus::field_names() {
        let observed = if [
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
            "physical_signal_runtimes",
            "physical_signal_workers",
        ]
        .contains(&class)
        {
            count
        } else {
            0
        };
        census.insert(class.to_owned(), Value::from(observed));
    }
    census.insert("retained_targets".to_owned(), Value::from(retained_targets));
    Value::Object(census)
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
