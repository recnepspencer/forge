use super::super::support::*;

#[test]
fn time_only_pending_write_intent_retains_write_adjacent_trigger_through_delivery_and_inspection() {
    let mut runtime = stateful_bridge_task_runtime();
    let origin_identity = test_write_adjacent_origin_identity(
        ForgeQueryEffectWriteAdjacentTriggerClass::TimeOnlyWake,
        "time-only:cause:task-title",
    );
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.time-follow-on",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<ForgeQueryNativeRow>(
            ForgeQueryEffectDeclaration::write_intent(
                "effects.time-follow-on",
                ForgeQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
                "strategy.intent.reconcile",
            )
            .with_write_adjacent_trigger(
                ForgeQueryEffectWriteAdjacentTriggerClass::TimeOnlyWake,
                origin_identity.clone(),
            ),
        )
        .expect("time-only write-intent effect should declare");

    let write = runtime
        .write(insert_command(
            "Task",
            [
                (
                    "identity.id",
                    test_string_aspect_value("time-only-follow-on-1"),
                ),
                (
                    "title.value",
                    test_string_aspect_value("title from time-only wake"),
                ),
            ],
        ))
        .expect("write should queue time-only follow-on");

    let inspection = runtime
        .inspect_effect(&effect)
        .expect("effect inspection should succeed");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("pending write intent should drain");

    assert_eq!(
        inspection.write_adjacent_trigger_class(),
        ForgeQueryEffectWriteAdjacentTriggerClass::TimeOnlyWake
    );
    assert_eq!(write.delivered_effect_count(), 0);
    assert_eq!(write.pending_write_intent_count(), 1);
    assert_eq!(write.suppressed_effect_count(), 0);
    assert_eq!(write.effect_expression_failure_count(), 0);
    assert_eq!(
        inspection
            .feedback_graph()
            .expect("time-only follow-on should surface the pending intent boundary")
            .termination(),
        ForgeQueryFeedbackTermination::PendingIntentDeferred
    );
    assert_eq!(
        inspection
            .write_adjacent_trigger()
            .origin_evidence_identity(),
        &origin_identity
    );
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].write_adjacent_trigger().class(),
        ForgeQueryEffectWriteAdjacentTriggerClass::TimeOnlyWake
    );
    assert_eq!(
        deliveries[0]
            .write_adjacent_trigger()
            .origin_evidence_identity(),
        &origin_identity
    );
}
