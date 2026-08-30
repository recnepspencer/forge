use super::super::support::*;

#[test]
fn unknown_merge_target_does_not_move_the_active_branch() {
    let (mut runtime, main_branch, feature_branch) = runtime_with_two_branches();
    runtime.switch_branch(feature_branch).unwrap();
    let before = runtime.read_value("counter").unwrap();

    let error = runtime
        .merge_branches(feature_branch, u64::MAX)
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert_eq!(runtime.current_branch().id.0, feature_branch);
    assert_eq!(runtime.read_value("counter").unwrap(), before);
    assert_ne!(main_branch, feature_branch);
}

#[test]
fn denied_merge_restores_the_callers_branch_and_state() {
    let (mut runtime, _, feature_branch) = runtime_with_two_branches();
    runtime.switch_branch(feature_branch).unwrap();
    let before = runtime.read_value("counter").unwrap();

    let error = runtime
        .merge_branches(feature_branch, feature_branch)
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert_eq!(runtime.current_branch().id.0, feature_branch);
    assert_eq!(runtime.read_value("counter").unwrap(), before);
}

#[test]
fn successful_merge_preserves_an_active_target_branch() {
    let (mut runtime, main_branch, feature_branch) = runtime_with_two_branches();
    runtime.switch_branch(feature_branch).unwrap();
    set_counter(&mut runtime, 2.0);
    runtime.switch_branch(main_branch).unwrap();

    runtime.merge_branches(feature_branch, main_branch).unwrap();

    assert_eq!(runtime.current_branch().id.0, main_branch);
    assert_eq!(
        runtime.read_value("counter").unwrap(),
        SignalValue::Number(2.0)
    );
}

#[test]
fn successful_merge_preserves_an_unrelated_active_branch() {
    let (mut runtime, main_branch, feature_branch) = runtime_with_two_branches();
    let observer_branch = runtime.create_branch("observer".to_owned()).unwrap().id.0;
    runtime.switch_branch(feature_branch).unwrap();
    set_counter(&mut runtime, 2.0);
    runtime.switch_branch(observer_branch).unwrap();
    set_counter(&mut runtime, 3.0);

    runtime.merge_branches(feature_branch, main_branch).unwrap();

    assert_eq!(runtime.current_branch().id.0, observer_branch);
    assert_eq!(
        runtime.read_value("counter").unwrap(),
        SignalValue::Number(3.0)
    );
    runtime.switch_branch(main_branch).unwrap();
    assert_eq!(
        runtime.read_value("counter").unwrap(),
        SignalValue::Number(2.0)
    );
}

fn runtime_with_two_branches() -> (RuntimeCore, u64, u64) {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    let main_branch = runtime.current_branch().id.0;
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap().id.0;
    (runtime, main_branch, feature_branch)
}

fn set_counter(runtime: &mut RuntimeCore, value: f64) {
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "counter".to_owned(),
            value: SignalValue::Number(value),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
}
