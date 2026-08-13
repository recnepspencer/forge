use serde_json::{json, Value};

use crate::milestone_3141_phase1_ledger::{
    command_binding::ControlBinding, result_artifact_control, result_artifact_cost,
    result_artifact_counter,
};

#[test]
fn artifact_requires_its_exact_mutation_control() {
    let binding = ControlBinding {
        package: "worth-ui-host-native".to_owned(),
        target_kind: "lib".to_owned(),
        target_name: "lib".to_owned(),
        features: Vec::new(),
        test_name: "native::presentation::hostile".to_owned(),
    };
    let mut artifact = json!({"hostile_control": {
        "package": binding.package,
        "target_kind": binding.target_kind,
        "target_name": binding.target_name,
        "test_name": binding.test_name,
        "features": [], "matched_test_count": 1, "executed_test_count": 1,
        "passed_test_count": 1, "ignored_test_count": 0, "exit_posture": "passed",
        "list_exit_code": 0, "test_exit_code": 0, "list_duration_ms": 1,
        "test_duration_ms": 1, "test_budget_ms": 10_000,
        "list_command": result_artifact_control::cargo_command(&binding, true),
        "test_command": result_artifact_control::cargo_command(&binding, false),
        "mutation_control": {
            "requirement": "P3-BASELINE-REPLAY-01",
            "case": "opaque-baseline-clear"
        }
    }});
    result_artifact_control::validate(&artifact, Some(&binding), 10_000, "P3-BASELINE-REPLAY-01")
        .unwrap();
    for value in [
        Value::Null,
        json!({"requirement": "P3-BASELINE-REPLAY-01", "case": "wrong"}),
        json!({"requirement": "P3-DAMAGE-REPLAY-01", "case": "opaque-baseline-clear"}),
    ] {
        artifact["hostile_control"]["mutation_control"] = value;
        assert!(result_artifact_control::validate(
            &artifact,
            Some(&binding),
            10_000,
            "P3-BASELINE-REPLAY-01",
        )
        .is_err());
    }
}

#[test]
fn structural_counter_must_come_from_main_execution_stdout() {
    let mut artifact = json!({
        "test_stdout": concat!(
            "running 1 test\n",
            "WORTH_UI_LEDGER_COUNTERS={\"P3-BASELINE-REPLAY-01\":1}\n"
        )
    });
    result_artifact_counter::validate("P3-BASELINE-REPLAY-01", &artifact, 1).unwrap();
    artifact["test_stdout"] =
        Value::from("WORTH_UI_LEDGER_COUNTERS={\"P3-BASELINE-REPLAY-01\":0}\n");
    assert!(result_artifact_counter::validate("P3-BASELINE-REPLAY-01", &artifact, 1).is_err());
}

#[test]
fn shared_cost_distinguishes_mounted_and_native_worlds() {
    let mut artifact = json!({
        "hostile_control": {"executed_test_count": 1},
        "shared_main_artifact": "world.json",
        "construction_cost": concat!(
            "main-tests=0;hostile-controls=1;product-processes=0;",
            "compile-sessions=0;courtroom-worlds=0;shared-mounted-worlds=1"
        ),
        "execution_cost": "executed-tests=1;presentations=0;shared-presentations=5"
    });
    result_artifact_cost::validate("P3-HEADLESS-COST-01", &artifact).unwrap();
    artifact["construction_cost"] = Value::from(concat!(
        "main-tests=0;hostile-controls=1;product-processes=0;",
        "compile-sessions=0;courtroom-worlds=0;shared-native-worlds=1"
    ));
    assert!(result_artifact_cost::validate("P3-HEADLESS-COST-01", &artifact).is_err());
}

#[test]
fn world_cost_is_derived_from_main_and_supporting_observations() {
    let mut artifact = json!({
        "hostile_control": {"executed_test_count": 1},
        "supporting_world": {"worlds": 1, "presentations": 5},
        "test_stdout": concat!(
            "WORTH_UI_LEDGER_WORLD=1\n",
            "WORTH_UI_LEDGER_PRESENTATIONS=7\n"
        ),
        "construction_cost": concat!(
            "main-tests=1;hostile-controls=1;product-processes=1;",
            "compile-sessions=0;courtroom-worlds=1;shared-mounted-worlds=1"
        ),
        "execution_cost": "executed-tests=2;presentations=7;shared-presentations=5"
    });
    result_artifact_cost::validate("P3-HP02-WORLD-01", &artifact).unwrap();
    artifact["supporting_world"]["presentations"] = Value::from(4);
    assert!(result_artifact_cost::validate("P3-HP02-WORLD-01", &artifact).is_err());
}
