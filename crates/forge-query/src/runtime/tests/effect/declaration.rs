use super::super::support::*;

#[test]
fn effect_declaration_rejects_missing_triggers_before_registration() {
    let mut runtime = stateful_bridge_task_runtime();
    let missing = ForgeQueryEffectDeclaration::deliver(
        "ui.missing",
        ForgeQueryEffectTrigger::live_view_name("tasks.missing", ["title"]),
        "ui.badges",
    );
    let error = runtime
        .declare_effect::<Value>(missing)
        .expect_err("missing live trigger should reject");

    match error {
        ForgeQueryRuntimeError::EffectDeclaration {
            effect_name,
            stage,
            message,
        } => {
            assert_eq!(effect_name, "ui.missing");
            assert_eq!(stage, "trigger-admission");
            assert!(message.contains("tasks.missing"));
        }
        other => panic!("expected effect declaration denial, got {other:?}"),
    }
}

#[test]
fn effect_declaration_rejects_truth_delivery_without_intent_boundary() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let declaration = ForgeQueryEffectDeclaration::deliver(
        "ui.truth-smuggle",
        ForgeQueryEffectTrigger::live_view(&live, ["title"]),
        "Task",
    )
    .with_target_lane(ForgeQueryAuthorityLane::AuthoritativeTruth);

    let error = runtime
        .declare_effect::<Value>(declaration)
        .expect_err("effect delivery must not target truth");

    match error {
        ForgeQueryRuntimeError::EffectDeclaration { stage, message, .. } => {
            assert_eq!(stage, "authority-admission");
            assert!(message.contains("intent authority"));
        }
        other => panic!("expected authority admission denial, got {other:?}"),
    }
}
