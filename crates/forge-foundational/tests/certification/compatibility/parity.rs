use forge_foundational::{
    admit_authoritative_record_aspect_state, lower_json_record_aspect_state, validate_aspect_value,
    AspectValue, JsonCompatibilityAspectInput, ScalarAspectType, StructAspectValue,
};
use forge_proof::TransitionOutcome;
use serde_json::json;

use super::json_lowering_fixtures::{scalar_input, source_for, task_summary_contract};
use crate::foundational_vocabulary::{field, key};

#[test]
fn json_lowering_matches_native_authoritative_state_for_equivalent_struct_truth() {
    let contract = task_summary_contract();
    let json_state = lower_json_record_aspect_state([JsonCompatibilityAspectInput::new(
        contract.clone(),
        source_for("task.summary"),
        json!({
            "done": true,
            "title": "Ship it",
            "note": "native parity"
        }),
    )]);

    let native_value = StructAspectValue::new([
        (field("title"), AspectValue::String("Ship it".into())),
        (field("done"), AspectValue::Bool(true)),
        (field("note"), AspectValue::String("native parity".into())),
    ])
    .expect("unique native fields");
    let TransitionOutcome::Success(native_artifact) =
        validate_aspect_value(&contract, native_value.into())
    else {
        panic!("expected native validation");
    };
    let TransitionOutcome::Success(native_state) =
        admit_authoritative_record_aspect_state([native_artifact])
    else {
        panic!("expected native state admission");
    };

    let TransitionOutcome::Success(json_state) = json_state else {
        panic!("expected JSON compatibility lowering");
    };
    assert_eq!(json_state.payload(), native_state.payload());
}

#[test]
fn json_lowering_admits_unambiguous_scalar_wrapper_shapes() {
    let decimal = scalar_input("decimal", ScalarAspectType::Decimal, json!("12.50"));
    let date = scalar_input("date", ScalarAspectType::Date, json!(19_000));
    let uuid = scalar_input(
        "uuid",
        ScalarAspectType::Uuid,
        json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    );
    let rational = scalar_input("ratio", ScalarAspectType::Rational, json!("3/4"));

    let outcome = lower_json_record_aspect_state([decimal, date, uuid, rational]);

    let TransitionOutcome::Success(state) = outcome else {
        panic!("expected unambiguous scalar wrapper lowering");
    };
    assert!(state.payload().get(&key("decimal")).is_some());
    assert!(state.payload().get(&key("date")).is_some());
    assert!(state.payload().get(&key("uuid")).is_some());
    assert!(state.payload().get(&key("ratio")).is_some());
}
