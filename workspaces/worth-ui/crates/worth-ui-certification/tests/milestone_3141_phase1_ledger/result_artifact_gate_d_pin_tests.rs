use serde_json::{json, Value};

use super::gate_d_pin;

#[test]
fn gate_d_pin_artifact_requires_exact_product_census_and_layout_key_transitions() {
    let artifact = gate_d_pin_artifact();
    gate_d_pin::validate("P5-ATLAS-PINNING-01", &artifact).unwrap();
    for field in [
        "mounted_bindings",
        "pinned_layouts",
        "presentations",
        "atlas_transactions",
        "native_peak_pin_count",
        "physical_signal_runtimes",
        "physical_signal_workers",
        "alpha_entries",
        "color_entries",
    ] {
        let mut mutant = artifact.clone();
        mutant["boundary_observation"][field] = json!(0);
        assert!(
            gate_d_pin::validate("P5-ATLAS-PINNING-01", &mutant).is_err(),
            "{field}"
        );
    }
    let mut mutant = artifact.clone();
    for index in 0..8 {
        mutant["boundary_observation"]["native_frame_pins"][0][9 + index]["raster_key"] =
            json!(digest('d', index));
    }
    assert!(gate_d_pin::validate("P5-ATLAS-PINNING-01", &mutant).is_err());
    let mut mutant = artifact;
    mutant["boundary_observation"]["native_frame_pins"][1][0]["layout"] = json!(digest('e', 0));
    assert!(gate_d_pin::validate("P5-ATLAS-PINNING-01", &mutant).is_err());
}

fn gate_d_pin_artifact() -> Value {
    let alpha = (0..9).map(|index| digest('a', index)).collect::<Vec<_>>();
    let mut first = frame('a', &alpha);
    let mut removed = alpha[..8].to_vec();
    removed.push(digest('b', 8));
    first.extend(frame('b', &removed));
    let pressure = (0..31).map(|index| digest('c', index)).collect::<Vec<_>>();
    first.extend(frame('c', &pressure));
    let second = first.clone();
    let mut third = frame('a', &alpha);
    third.extend(frame('c', &pressure));
    json!({
        "hostile_control": {"executed_test_count": 1},
        "boundary_observation": {
            "schema": "worth-ui-native-gate-d-pin-world-v3",
            "mounted_bindings": 1, "pinned_layouts": 3, "presentations": 4,
            "atlas_transactions": 4, "native_peak_pin_count": 49,
            "native_frame_pin_counts": [49, 49, 40, 0],
            "native_frame_pins": [first, second, third, []],
            "observation_history_complete": true,
            "physical_signal_runtimes": 1, "physical_signal_workers": 1,
            "alpha_entries": 40, "color_entries": 1,
            "closed_query_resources": 1, "query_close_complete": true,
            "terminal_zero": true
        },
        "construction_cost": "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1",
        "execution_cost": "executed-tests=2;presentations=4;atlas-transactions=4"
    })
}

fn frame(layout: char, keys: &[String]) -> Vec<Value> {
    keys.iter()
        .map(|key| json!({"layout": digest(layout, 0), "raster_key": key}))
        .collect()
}

fn digest(prefix: char, index: usize) -> String {
    format!("{prefix}{index:02x}{:0>61}", "")
}
