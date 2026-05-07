use super::super::support::*;

#[test]
fn branches_can_be_created_and_switched() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap();
    assert_ne!(main_branch.id, feature_branch.id);

    runtime.switch_branch(feature_branch.id.0).unwrap();
    let active = runtime.current_branch();
    assert_eq!(active.id, feature_branch.id);

    let replay = runtime.replay_for_branch(feature_branch.id.0).unwrap();
    assert!(!replay.frames.is_empty());
}
