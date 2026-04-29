use super::super::support::*;

#[test]
fn write_intent_effect_lowers_to_pending_intent_with_phase_evidence() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::write_intent(
                "intent.reconcile-title",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "reconcile-title-slug",
            )
            .with_condition(ForgeQueryEffectCondition::expression(
                "expr.title.needs-slug",
                ["title"],
                ["intent.slug"],
            )),
        )
        .expect("pending write-intent effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Intent task" },
            }),
        })
        .expect("write should route pending intent effect");
    let evidence = runtime
        .inspect_effect(&effect)
        .expect("pending intent effect should inspect");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("pending intent work should drain through effect queue");

    assert_eq!(
        effect.authority_lane(),
        ForgeQueryAuthorityLane::PendingWriteIntent
    );
    assert_eq!(write.considered_effect_count(), 1);
    assert_eq!(write.delivered_effect_count(), 0);
    assert_eq!(write.pending_write_intent_count(), 1);
    assert_eq!(evidence.pending_delivery_count(), 0);
    assert_eq!(evidence.pending_write_intent_count(), 1);
    assert_eq!(evidence.pending_delivered_count(), 0);
    assert_eq!(evidence.pending_suppressed_count(), 0);
    assert_eq!(evidence.pending_expression_failure_count(), 0);
    assert_eq!(
        evidence.latest_delivery_family(),
        Some(&ForgeQueryEffectDeliveryFamily::PendingWriteIntent)
    );
    assert!(!evidence.trigger_digest().is_empty());
    assert!(!evidence.condition_digest().is_empty());
    assert!(!evidence.declaration_digest().is_empty());
    assert!(!evidence.pending_delivery_digest().is_empty());
    assert!(evidence.latest_phase_digest().is_some());
    assert!(!evidence.inspection_digest().is_empty());
    assert_eq!(
        evidence
            .latest_phase_evidence()
            .expect("phase evidence should exist")
            .phases(),
        &[
            ForgeQueryEffectPhase::TruthRead,
            ForgeQueryEffectPhase::Derive,
            ForgeQueryEffectPhase::PendingWriteIntent,
        ]
    );
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].family(),
        &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
    );
    assert_eq!(
        deliveries[0].authority_lane(),
        ForgeQueryAuthorityLane::PendingWriteIntent
    );
    assert_eq!(deliveries[0].target(), "reconcile-title-slug");
    assert_eq!(
        deliveries[0].phase_evidence().loop_prevention(),
        ForgeQueryEffectLoopPrevention::PendingIntentExecutionDeferred
    );
    assert_eq!(
        deliveries[0].phase_evidence().loop_prevention().as_str(),
        "pending-intent-execution-deferred"
    );
    assert_eq!(
        deliveries[0].phase_evidence().idempotence(),
        ForgeQueryEffectIdempotence::PendingIntentReceiptIdentity
    );
    assert!(deliveries[0]
        .reason()
        .expect("pending intent explanation should exist")
        .contains("pending write intent"));
}

#[test]
fn write_intent_effect_rejects_authoritative_truth_target() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let declaration = ForgeQueryEffectDeclaration::write_intent(
        "intent.truth-smuggle",
        ForgeQueryEffectTrigger::live_view(&live, ["title"]),
        "direct-truth-write",
    )
    .with_effect_policy(ForgeQueryEffectPolicy::AuthoritativeAllowed)
    .with_target_lane(ForgeQueryAuthorityLane::AuthoritativeTruth);

    let error = runtime
        .declare_effect::<Value>(declaration)
        .expect_err("write intent cannot target truth directly");

    match error {
        ForgeQueryRuntimeError::EffectDeclaration { stage, message, .. } => {
            assert_eq!(stage, "write-intent-admission");
            assert!(message.contains("pending write intent authority"));
        }
        other => panic!("expected write intent admission denial, got {other:?}"),
    }
}
