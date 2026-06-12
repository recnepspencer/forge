use super::super::support::*;

#[test]
fn effect_delivery_routes_from_live_trigger_with_expression_metadata() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.title-badges",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.badges",
            )
            .with_condition(ForgeQueryEffectCondition::expression(
                "expr.title.badge",
                ["title"],
                ["ui.badge"],
            )),
        )
        .expect("effect should declare");

    let write = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Effect task")),
            ],
        ))
        .expect("write should route effect");
    let evidence = runtime
        .inspect_effect(&effect)
        .expect("effect should inspect before drain");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("effect deliveries should drain");

    assert_eq!(write.considered_effect_count(), 1);
    assert_eq!(write.delivered_effect_count(), 1);
    assert_eq!(write.suppressed_effect_count(), 0);
    assert_eq!(write.effect_expression_failure_count(), 0);
    assert_eq!(evidence.name(), "ui.title-badges");
    assert_eq!(evidence.trigger_source(), "tasks.table");
    assert_eq!(
        evidence.trigger_source_kind(),
        ForgeQueryEffectTriggerSourceKind::LiveView
    );
    assert_eq!(evidence.condition_descriptor(), "expr.title.badge");
    assert_eq!(evidence.condition_inputs(), &["title".to_string()]);
    assert_eq!(evidence.condition_outputs(), &["ui.badge".to_string()]);
    assert_eq!(evidence.pending_delivery_count(), 1);
    assert_eq!(evidence.pending_delivered_count(), 1);
    assert_eq!(evidence.pending_suppressed_count(), 0);
    assert_eq!(evidence.pending_expression_failure_count(), 0);
    assert_eq!(
        evidence.latest_delivery_family(),
        Some(&ForgeQueryEffectDeliveryFamily::Delivered)
    );
    assert!(!evidence.trigger_digest().is_empty());
    assert!(!evidence.condition_digest().is_empty());
    assert!(!evidence.declaration_digest().is_empty());
    assert!(!evidence.pending_delivery_digest().is_empty());
    assert!(evidence.latest_phase_digest().is_some());
    assert!(!evidence.inspection_digest().is_empty());
    let feedback_graph = evidence
        .feedback_graph()
        .expect("effect inspection should carry feedback graph");
    assert_eq!(
        feedback_graph.phase_nodes(),
        &[
            ForgeQueryFeedbackPhaseNode::TruthRead,
            ForgeQueryFeedbackPhaseNode::Derive,
            ForgeQueryFeedbackPhaseNode::EffectDelivery,
        ]
    );
    assert_eq!(
        feedback_graph.termination(),
        ForgeQueryFeedbackTermination::Delivered
    );
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].family(),
        &ForgeQueryEffectDeliveryFamily::Delivered
    );
    assert_eq!(deliveries[0].target(), "ui.badges");
    assert_eq!(
        deliveries[0].authority_lane(),
        ForgeQueryAuthorityLane::EffectDeliveryState
    );
    assert_eq!(deliveries[0].aspect_paths(), &["title.value".to_string()]);
    assert_eq!(deliveries[0].payload()["condition"], "expr.title.badge");

    let graph = runtime
        .inspect_feedback_path(&effect)
        .expect("delivered effect should expose feedback graph");
    assert_eq!(graph.effect_name(), "ui.title-badges");
    assert_eq!(
        graph.phase_nodes(),
        &[
            ForgeQueryFeedbackPhaseNode::TruthRead,
            ForgeQueryFeedbackPhaseNode::Derive,
            ForgeQueryFeedbackPhaseNode::EffectDelivery,
        ]
    );
    assert_eq!(
        graph.termination(),
        ForgeQueryFeedbackTermination::Delivered
    );
    assert_eq!(
        graph.effect_policy(),
        Some(ForgeQueryEffectPolicy::AuthoritativeAllowed)
    );
    assert!(!graph.graph_digest().is_empty());
    assert!(!graph.inspection_digest().is_empty());
}

#[test]
fn effect_delivery_routes_from_computed_trigger_after_computed_patch() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.titles.effect", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.summary-badges",
            ForgeQueryEffectTrigger::computed_view(&titles, ["title.summary"]),
            "ui.summary",
        ))
        .expect("computed-triggered effect should declare");

    let write = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Computed effect task")),
            ],
        ))
        .expect("write should route computed effect");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("effect deliveries should drain");

    assert_eq!(write.considered_computed_view_count(), 1);
    assert_eq!(write.considered_effect_count(), 1);
    assert_eq!(write.delivered_effect_count(), 1);
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].trigger_source_kind(),
        ForgeQueryEffectTriggerSourceKind::ComputedView
    );
    assert_eq!(deliveries[0].trigger_source(), "computed.titles.effect");
    assert_eq!(deliveries[0].aspect_paths(), &["title.summary".to_string()]);
    assert_eq!(
        runtime.read_derived(&titles),
        vec![Value::String(write.deltas()[0].entity_identity.to_string())]
    );
}

#[test]
fn computed_effect_does_not_replay_stale_undrained_computed_patch() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.titles.stale-effect", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.stale-summary-badges",
            ForgeQueryEffectTrigger::computed_view(&titles, ["title.summary"]),
            "ui.summary",
        ))
        .expect("computed-triggered effect should declare");

    let first_write = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("First effect task")),
            ],
        ))
        .expect("first write should route computed effect");
    let first_deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("first effect deliveries should drain");
    assert_eq!(first_deliveries.len(), 1);

    let unrelated = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: first_write.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("irrelevant".to_string()),
        })
        .expect("irrelevant write should not replay stale computed patch");
    let stale_deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("stale effect deliveries should drain");

    assert_eq!(unrelated.considered_computed_view_count(), 1);
    assert!(unrelated.affected_derived_view_ids().is_empty());
    assert_eq!(unrelated.considered_effect_count(), 0);
    assert!(stale_deliveries.is_empty());
}

#[test]
fn effect_expression_suppression_and_failure_are_typed_and_counted() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let suppressed_effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.suppressed",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.suppressed",
            )
            .with_condition(ForgeQueryEffectCondition::expression(
                "expr.needs-validation",
                ["validation.state"],
                ["ui.badge"],
            )),
        )
        .expect("suppressed effect should declare");
    let failing_effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.failing",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.failing",
            )
            .with_condition(ForgeQueryEffectCondition::failing_expression(
                "expr.fail.validation",
                ["title"],
                ["ui.badge"],
            )),
        )
        .expect("failing effect should declare");

    let write = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Conditional task")),
            ],
        ))
        .expect("write should route effects");
    let suppressed_evidence = runtime
        .inspect_effect(&suppressed_effect)
        .expect("suppressed terminal artifact should inspect before drain");
    let suppressed = runtime
        .drain_effect_deliveries(&suppressed_effect)
        .expect("suppressed effect should drain");
    let failed = runtime
        .drain_effect_deliveries(&failing_effect)
        .expect("failing effect should drain");

    assert_eq!(write.considered_effect_count(), 2);
    assert_eq!(write.delivered_effect_count(), 0);
    assert_eq!(write.suppressed_effect_count(), 1);
    assert_eq!(write.effect_expression_failure_count(), 1);
    assert_eq!(
        suppressed[0].family(),
        &ForgeQueryEffectDeliveryFamily::Suppressed
    );
    assert_eq!(suppressed_evidence.pending_delivery_count(), 1);
    assert_eq!(suppressed_evidence.pending_delivered_count(), 0);
    assert_eq!(suppressed_evidence.pending_suppressed_count(), 1);
    assert_eq!(suppressed_evidence.pending_expression_failure_count(), 0);
    assert_eq!(
        suppressed_evidence.latest_delivery_family(),
        Some(&ForgeQueryEffectDeliveryFamily::Suppressed)
    );
    assert!(suppressed_evidence.latest_phase_digest().is_some());
    assert!(suppressed[0]
        .reason()
        .expect("suppression reason should exist")
        .contains("inputs were not changed"));
    assert_eq!(
        failed[0].family(),
        &ForgeQueryEffectDeliveryFamily::ExpressionFailed
    );
    assert!(failed[0]
        .reason()
        .expect("failure reason should exist")
        .contains("deterministic failure"));
}

#[test]
fn meaningful_change_suppression_counts_semantic_delta_suppression() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.meaningful-title",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.badges",
            )
            .with_meaningful_change_suppression(),
        )
        .expect("meaningful effect should declare");

    let inserted = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Meaningful task")),
            ],
        ))
        .expect("insert should deliver because whole-row delta is meaningful");
    assert_eq!(inserted.delivered_effect_count(), 1);
    assert_eq!(inserted.meaningful_effect_suppression_count(), 0);
    assert_eq!(
        runtime
            .drain_effect_deliveries(&effect)
            .expect("insert delivery should drain")
            .len(),
        1
    );

    let churn = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: inserted.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("semantic-churn".to_string()),
        })
        .expect("irrelevant aspect update should be suppressed as churn");
    let evidence = runtime
        .inspect_effect(&effect)
        .expect("meaningful effect should inspect");
    let suppressed = runtime
        .drain_effect_deliveries(&effect)
        .expect("suppressed effect should drain");

    assert_eq!(churn.considered_effect_count(), 1);
    assert_eq!(churn.delivered_effect_count(), 0);
    assert_eq!(churn.suppressed_effect_count(), 1);
    assert_eq!(churn.meaningful_effect_suppression_count(), 1);
    assert_eq!(
        evidence.suppression_policy(),
        ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
    );
    assert_eq!(evidence.counters().meaningful_suppressions(), 1);
    assert_eq!(evidence.pending_delivered_count(), 0);
    assert_eq!(evidence.pending_suppressed_count(), 1);
    assert_eq!(evidence.pending_expression_failure_count(), 0);
    assert_eq!(
        evidence.latest_delivery_family(),
        Some(&ForgeQueryEffectDeliveryFamily::Suppressed)
    );
    assert_eq!(suppressed.len(), 1);
    assert_eq!(
        suppressed[0].family(),
        &ForgeQueryEffectDeliveryFamily::Suppressed
    );
    assert_eq!(
        suppressed[0].suppression_policy(),
        ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
    );
    assert!(suppressed[0]
        .reason()
        .expect("meaningful suppression should explain itself")
        .contains("meaningful semantic delta suppression"));
}
