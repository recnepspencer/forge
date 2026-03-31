use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::facade::*;
use crate::tests::support::*;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagnosticsEvent {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DiagnosticsDomain {
    Cache,
}

struct FailingSubscriber;
impl EventSubscriber for FailingSubscriber {
    type Event = DiagnosticsEvent;
    type DataId = DiagnosticsDomain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(99)
    }

    fn name(&self) -> &'static str {
        "failing"
    }

    fn requires(&self) -> &'static [Self::DataId] {
        &[]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[]
    }

    fn on_event(&mut self, _event: &Self::Event) {}

    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        Err(SignalError::internal("injected subscriber failure"))
    }
}

struct NeedsMissingProviderSubscriber;
impl EventSubscriber for NeedsMissingProviderSubscriber {
    type Event = DiagnosticsEvent;
    type DataId = DiagnosticsDomain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(100)
    }

    fn name(&self) -> &'static str {
        "missing-provider"
    }

    fn requires(&self) -> &'static [Self::DataId] {
        &[DiagnosticsDomain::Cache]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[]
    }

    fn on_event(&mut self, _event: &Self::Event) {}

    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        Ok(())
    }
}

#[test]
fn graph_diagnostics_summary_is_deterministic_and_serializable() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, source, &mut source_compute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let left = graph
        .observe()
        .diagnostics_summary(DiagnosticsTier::Development);
    let right = graph
        .observe()
        .diagnostics_summary(DiagnosticsTier::Development);
    assert_eq!(left, right);
    assert!(graphs_semantically_equivalent(&left, &right));

    let json = serde_json::to_string(&left).unwrap();
    let restored: GraphSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(left, restored);
    assert!(render_graph_summary(&left).contains("GraphSummary"));
}

#[test]
fn diagnostics_entrypoint_exposes_one_discoverable_surface() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph.execute_prepared_plan(&plan, &(), &compute).unwrap();

    let diagnostics = graph.observe().diagnostics();
    let summary = diagnostics.summary(DiagnosticsTier::Operational);
    let history = diagnostics.history(DiagnosticsTier::Operational);
    let latest_flow = diagnostics.latest_flow();
    let compare = diagnostics.compare();
    let graph_inspector = diagnostics.inspect_graph();
    let execution_inspector = diagnostics.inspect_execution();

    assert_eq!(summary.active_node_count, 1);
    assert!(history.latest_execution_record_id.is_some());
    assert!(latest_flow.is_some());
    assert!(compare.graphs(&summary, &summary).is_empty());
    assert!(graph_inspector
        .nodes_with_execution_record()
        .contains(&node));
    assert_eq!(execution_inspector.nodes_with_trace_summaries(), vec![node]);
}

#[test]
fn diagnostics_grouped_job_views_are_discoverable() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    let report = graph.execute_prepared_plan(&plan, &(), &compute).unwrap();

    let diagnostics = graph.observe().diagnostics();
    let health = diagnostics.health_view();
    let inspect = diagnostics.inspect();

    assert_eq!(health.current_now().active_node_count, 1);
    assert!(health.latest_flow().is_some());
    assert!(health.recent_history().back().is_some());
    assert!(inspect
        .graph()
        .nodes_with_execution_record()
        .contains(&node));
    assert_eq!(inspect.execution().nodes_with_trace_summaries(), vec![node]);
    assert_eq!(inspect.plan(&plan).stage_count(), 1);
    assert!(inspect.report(&report).task_record_for_node(node).is_some());
}

#[test]
fn diagnostics_plan_summary_reports_contract_pruning() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().reads_aspects(mask_a()).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let summary = plan.diagnostics_summary(DiagnosticsTier::Development);

    assert_eq!(summary.task_count, 0);
    assert_eq!(summary.contract_pruned_count, 1);
}

#[test]
fn explanation_summary_includes_contract_metadata() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .reads_aspects([ASPECT_A])
        .produces_aspects([ASPECT_B])
        .requires_context(ContextRequirement::DomainContext)
        .with_partition_scope(PartitionSubscription::whole_partition("wing"))
        .build();

    let explanation = graph.observe().explain(node).unwrap();
    let summary = explanation.diagnostics_summary(DiagnosticsTier::Development);

    assert_eq!(
        summary.contract_reads_mask,
        AspectMask::from([ASPECT_A]).bits() as u128
    );
    assert_eq!(
        summary.contract_produces_mask,
        AspectMask::from([ASPECT_B]).bits() as u128
    );
    assert_eq!(summary.contract_partition_scope_count, 1);
    assert_eq!(summary.required_context, ContextRequirement::DomainContext);
}

#[test]
fn graph_diff_detects_state_and_structure_mismatch() {
    let mut graph_a = SignalGraph::new();
    let node_a = graph_a.node().build();
    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph_a, node_a, &mut compute).unwrap();

    let mut graph_b = SignalGraph::new();
    let source = graph_b.node().build();
    let dependent = graph_b.node().build();
    graph_b
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    evaluate(&mut graph_b, source, &mut compute).unwrap();
    evaluate(&mut graph_b, dependent, &mut compute).unwrap();
    mark_dirty(&mut graph_b, source, ASPECT_A).unwrap();

    let left = graph_a.diagnostics_summary(DiagnosticsTier::Operational);
    let right = graph_b.diagnostics_summary(DiagnosticsTier::Operational);
    let diff = compare_graphs(&left, &right);
    assert!(!diff.is_empty());
    assert!(!graphs_semantically_equivalent(&left, &right));
}

#[test]
fn inspectors_query_graph_plan_report_and_execution_history() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().on_demand().build();

    let source_compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));
    let dependent_compute = |ctx: &mut EvaluationContext<'_, ()>| {
        let version = ctx.read_aspect_version(source, ASPECT_A)?;
        Ok(ctx.finish(NodeEvaluationResult::from_version(version)))
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| {
            if ctx.node() == source {
                source_compute(ctx)
            } else {
                dependent_compute(ctx)
            }
        })
        .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &(), &|ctx| {
            if ctx.node() == source {
                source_compute(ctx)
            } else {
                dependent_compute(ctx)
            }
        })
        .unwrap();

    let graph_inspector = inspect_graph(&graph);
    assert_eq!(
        graph_inspector.nodes_with_condition(&EvaluationCondition::OnDemand),
        vec![dependent]
    );
    assert_eq!(graph_inspector.nodes_with_execution_record().len(), 2);

    let plan_inspector = inspect_plan(&plan);
    assert_eq!(plan_inspector.stage_count(), 2);
    assert_eq!(plan_inspector.tasks_for_node(dependent).len(), 1);

    let report_inspector = inspect_report(&report);
    assert_eq!(
        report_inspector
            .tasks_with_outcome(TaskExecutionOutcome::Recomputed)
            .len(),
        2
    );

    let execution_inspector = inspect_execution(&graph);
    assert_eq!(execution_inspector.nodes_with_trace_summaries().len(), 2);
    assert!(execution_inspector.latest_execution_record_id().is_some());
}

#[test]
fn flow_and_failure_summaries_are_structured_and_diffable() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph
        .set_causality(
            node,
            Some(CausalityMetadata {
                kind: "host-change".to_string(),
                fields: Default::default(),
            }),
        )
        .unwrap();

    let compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(2, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    let report = graph.execute_prepared_plan(&plan, &(), &compute).unwrap();
    let explanation = graph.observe().explain(node).unwrap();

    let flow = FlowSummary::new(
        DiagnosticsTier::Development,
        ChangeInputSummary::new(
            vec![node],
            vec![ASPECT_A],
            0,
            Some("host-change".to_string()),
        ),
        InvalidationSummary::new(1, 0, 0, 0, 0),
        PlanningSummary::from_plan(&plan, DiagnosticsTier::Development),
        PrecomputeSummary::from_report(&report, DiagnosticsTier::Development),
        ApplySummary::from_report(&report, DiagnosticsTier::Development),
        Vec::new(),
        Vec::new(),
        None,
        Some(explanation.diagnostics_summary(DiagnosticsTier::Development)),
    );
    let flow_clone = flow.clone();
    assert!(compare_flows(&flow, &flow_clone).is_empty());
    assert_eq!(inspect_flow(&flow).changed_nodes(), &[node]);
    assert!(render_flow_summary(&flow).contains("FlowSummary"));

    let failure = ExecutionFailureContext::new(
        ExecutionFailurePhase::Precompute,
        Some(0),
        Some(node),
        Some(StageExecutor::Serial),
        Some(ExecutionRecordId(7)),
        Some(plan.summary),
        "precompute failed",
    );
    let rollback = RollbackDiagnostic::new(
        true,
        3,
        2,
        Some("rewound staged changes".to_string()),
        Vec::new(),
    );
    let failure_summary = failure.summarize(Some(&rollback), DiagnosticsTier::Forensic);
    let failure_summary_2 = failure.summarize(None, DiagnosticsTier::Forensic);
    assert!(!compare_failures(&failure_summary, &failure_summary_2).is_empty());
    assert!(render_failure_summary(&failure_summary).contains("FailureSummary"));
}

#[test]
fn successful_execution_automatically_records_flow_and_history_diagnostics() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let source_compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));
    let dependent_compute = |ctx: &mut EvaluationContext<'_, ()>| {
        let version = ctx.read_aspect_version(source, ASPECT_A)?;
        Ok(ctx.finish(NodeEvaluationResult::from_version(version)))
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| {
            if ctx.node() == source {
                source_compute(ctx)
            } else {
                dependent_compute(ctx)
            }
        })
        .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    graph
        .execute_prepared_plan(&plan, &(), &|ctx| {
            if ctx.node() == source {
                source_compute(ctx)
            } else {
                dependent_compute(ctx)
            }
        })
        .unwrap();

    let flow = graph
        .observe()
        .latest_flow_diagnostics()
        .expect("flow diagnostics should be recorded");
    let frontier = graph
        .observe()
        .latest_frontier_execution_summary()
        .expect("frontier execution summary should be retained");
    assert_eq!(flow.change.changed_nodes, vec![source]);
    assert_eq!(flow.planning.plan.task_count, 2);
    assert_eq!(flow.apply.report.task_count, 2);
    assert_eq!(
        flow.invalidation.frontier_seed_count as u64,
        frontier.counters.frontier_seed_count
    );
    assert_eq!(
        flow.invalidation.frontier_direct_wave_count as u64,
        frontier.counters.frontier_direct_wave_count
    );
    assert_eq!(
        flow.invalidation.frontier_transitive_wave_count as u64,
        frontier.counters.frontier_transitive_wave_count
    );
    assert!(!graph
        .observe()
        .recent_execution_history_diagnostics()
        .is_empty());
}

#[test]
fn diagnostics_profiles_control_retention_bounds() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));

    graph.reset_runtime_policy_to_tier(DiagnosticsTier::Operational);
    for _ in 0..8 {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &(), &compute).unwrap();
    }
    assert!(graph.observe().recent_execution_history_diagnostics().len() <= 4);

    graph.reset_runtime_policy_to_tier(DiagnosticsTier::Forensic);
    for _ in 0..8 {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &(), &compute).unwrap();
    }
    assert!(graph.observe().recent_execution_history_diagnostics().len() > 4);
}

#[test]
fn operational_profile_repeated_waves_stay_bounded_and_shallow() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph.reset_runtime_policy_to_tier(DiagnosticsTier::Operational);
    let compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));

    for _ in 0..100 {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &(), &compute).unwrap();
    }

    let diagnostics = graph.observe().diagnostics();
    let history = diagnostics.recent_history();
    let policy = SignalRuntimePolicy::for_tier(DiagnosticsTier::Operational);
    assert!(history.len() <= policy.retention_budget.history_limit);
    assert!(history.iter().all(|summary| summary.nodes.is_empty()));
    assert!(diagnostics.latest_failure().is_none());
}

#[test]
fn execution_failures_and_rollbacks_automatically_record_diagnostics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.evaluate_with_plan(
                node,
                &|_view| {
                    Err::<crate::logic::evaluation::EvaluationOutput, _>(SignalError::internal(
                        "synthetic precompute failure",
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap_err();
    assert!(format!("{err}").contains("synthetic precompute failure"));

    let failure = runtime
        .observe()
        .latest_failure_diagnostics()
        .expect("failure diagnostics should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::Precompute);

    let rollback = runtime
        .observe()
        .latest_rollback_diagnostics()
        .expect("rollback diagnostics should be retained");
    assert!(rollback.rolled_back);
}

#[test]
fn repeated_failure_capture_stays_current_and_bounded() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime
        .graph_mut()
        .reset_runtime_policy_to_tier(DiagnosticsTier::Development);
    let node = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    for cycle in 0..100 {
        let err = runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(node, ASPECT_A)?;
                tx.evaluate_with_plan(
                    node,
                    &move |_view| {
                        Err::<crate::logic::evaluation::EvaluationOutput, _>(SignalError::internal(
                            format!("synthetic precompute failure cycle {cycle}"),
                        ))
                    },
                    EvaluationRequestMode::Default,
                )?;
                Ok(())
            })
            .unwrap_err();
        assert!(format!("{err}").contains("synthetic precompute failure cycle"));
    }

    let diagnostics = runtime.observe().diagnostics();
    let failure = diagnostics
        .latest_failure()
        .expect("latest failure should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::Precompute);
    assert!(failure.message.contains("cycle 99"));
    assert!(
        diagnostics.recent_history().len()
            <= SignalRuntimePolicy::for_tier(DiagnosticsTier::Development)
                .retention_budget
                .history_limit
    );
}

#[test]
fn repeated_rollbacks_keep_latest_rollback_current_and_bounded() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime
        .graph_mut()
        .reset_runtime_policy_to_tier(DiagnosticsTier::Development);
    let node = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    for _ in 0..100 {
        let err = runtime.transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            Err(SignalError::invalid_input("force rollback"))
        });
        assert!(err.is_err());
    }

    let diagnostics = runtime.observe().diagnostics();
    let rollback = diagnostics
        .latest_rollback()
        .expect("latest rollback should be retained");
    assert!(rollback.rolled_back);
    assert!(rollback
        .reason
        .as_deref()
        .unwrap_or_default()
        .contains("explicit rollback"));
    assert!(
        diagnostics.recent_history().len()
            <= SignalRuntimePolicy::for_tier(DiagnosticsTier::Development)
                .retention_budget
                .history_limit
    );
}

#[test]
fn commit_promotion_failures_record_failure_and_rollback_diagnostics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .with_events::<DiagnosticsEvent>()
        .with_domains::<DiagnosticsDomain>()
        .build();
    let node = runtime.graph_mut().node().build();
    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();

    let mut runtime_ctx = ();
    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.emit_event(DiagnosticsEvent::Tick);
            tx.flush_events(CheckpointBarrier::PerOperation)?;
            Ok(())
        })
        .unwrap_err();
    assert!(format!("{err}").contains("event bus flush failed"));

    let failure = runtime
        .observe()
        .latest_failure_diagnostics()
        .expect("flush failure diagnostics should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::CommitPromotion);
    let rollback = runtime
        .observe()
        .latest_rollback_diagnostics()
        .expect("flush rollback diagnostics should be retained");
    assert!(rollback.rolled_back);
    assert!(rollback
        .reason
        .as_deref()
        .unwrap_or_default()
        .contains("event bus flush failed"));
}

#[test]
fn event_bus_begin_failures_record_failure_and_rollback_diagnostics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .with_events::<DiagnosticsEvent>()
        .with_domains::<DiagnosticsDomain>()
        .build();
    let node = runtime.graph_mut().node().build();
    runtime
        .event_bus_mut()
        .subscribe(Box::new(NeedsMissingProviderSubscriber))
        .unwrap();

    let mut runtime_ctx = ();
    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            Ok(())
        })
        .unwrap_err();
    assert!(format!("{err}").contains("event bus begin failed"));

    let failure = runtime
        .observe()
        .latest_failure_diagnostics()
        .expect("begin failure diagnostics should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::CommitPromotion);
    let rollback = runtime
        .observe()
        .latest_rollback_diagnostics()
        .expect("begin rollback diagnostics should be retained");
    assert!(rollback.rolled_back);
    assert!(rollback
        .reason
        .as_deref()
        .unwrap_or_default()
        .contains("event bus begin failed"));
}

#[test]
fn history_and_explanation_summaries_are_deterministic() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(3, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph.execute_prepared_plan(&plan, &(), &compute).unwrap();

    let history_a = graph
        .observe()
        .execution_history_summary(DiagnosticsTier::Forensic);
    let history_b = graph
        .observe()
        .execution_history_summary(DiagnosticsTier::Forensic);
    assert!(compare_execution_history(&history_a, &history_b).is_empty());
    assert!(repeat_run_summaries_equal(&[
        history_a.clone(),
        history_b.clone()
    ]));
    assert!(render_execution_history_summary(&history_a).contains("ExecutionHistorySummary"));

    let explanation_a = graph
        .observe()
        .explain(node)
        .unwrap()
        .diagnostics_summary(DiagnosticsTier::Development);
    let explanation_b = graph
        .observe()
        .explain(node)
        .unwrap()
        .diagnostics_summary(DiagnosticsTier::Development);
    assert!(compare_explanations(&explanation_a, &explanation_b).is_empty());
    assert!(explanations_semantically_equivalent(
        &explanation_a,
        &explanation_b
    ));
    assert!(render_explanation_summary(&explanation_a).contains("ExplanationSummary"));
}

#[test]
fn diagnostics_history_and_replay_preserve_typed_advanced_reuse_origins() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .runtime_policy(SignalRuntimePolicy::kernel())
        .build();
    let compute_calls = AtomicU32::new(0);
    let projection = runtime
        .define(Recipe {
            family: "diagnostics-projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching()
                .with_partial_artifact_splicing()
                .with_partition_scope(PartitionSubscription::whole_partition("wing")),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("diagnostics-artifact")
                        .with_output_change(OutputChange::Refreshed)
                        .with_changed_region(ChangedRegion::new("wing")),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias");
    let wing = projection.keyed("wing");
    let alias_node = alias.node(&mut runtime);
    let wing_node = wing.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            source.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity(tx, "source", "shape-v1", "mesh-001")
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    mark_dirty(runtime.graph_mut(), wing_node, ASPECT_A).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_partial_splice(
                tx,
                "shape-v1",
                [PartitionSubscription::whole_partition("wing")],
            )
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 2);

    let replay = runtime.graph().replay_events();
    assert!(replay.iter().any(|event| {
        event.kind == ReplayEventKind::TaskApplied
            && event.node == Some(alias_node)
            && event.reuse_origin == Some(ReuseOrigin::CrossIdentityPersistentReuse)
    }));
    assert!(replay.iter().any(|event| {
        event.kind == ReplayEventKind::TaskApplied
            && event.node == Some(wing_node)
            && event.reuse_origin == Some(ReuseOrigin::PartialArtifactSplice)
    }));

    let history = runtime
        .observe()
        .execution_history_summary(DiagnosticsTier::Development);
    assert_eq!(
        history
            .reuse_origin_counts
            .get(&ReuseOrigin::CrossIdentityPersistentReuse)
            .copied(),
        Some(1)
    );
    assert_eq!(
        history
            .reuse_origin_counts
            .get(&ReuseOrigin::PartialArtifactSplice)
            .copied(),
        Some(1)
    );
    assert!(history.nodes.iter().any(|node| {
        node.node == alias_node
            && node.reuse_origin == Some(ReuseOrigin::CrossIdentityPersistentReuse)
    }));
    assert!(history.nodes.iter().any(|node| {
        node.node == wing_node && node.reuse_origin == Some(ReuseOrigin::PartialArtifactSplice)
    }));

    let recent = runtime.observe().recent_execution_history_diagnostics();
    let latest = recent.back().expect("recent history entry");
    assert_eq!(
        latest
            .reuse_origin_counts
            .get(&ReuseOrigin::CrossIdentityPersistentReuse)
            .copied(),
        Some(1)
    );
    assert_eq!(
        latest
            .reuse_origin_counts
            .get(&ReuseOrigin::PartialArtifactSplice)
            .copied(),
        Some(1)
    );
}

#[test]
fn rendered_execution_report_summary_names_advanced_reuse_families() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, source, &mut source_compute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let mut report = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    let _ = &mut report;

    let summary = ExecutionReportSummary {
        profile: DiagnosticsTier::Development,
        stage_count: 1,
        task_count: 4,
        tasks_executed: 4,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 1,
        tasks_with_suppressed_propagation: 0,
        prepared_evaluations_produced: 4,
        prepared_evaluations_applied: 4,
        dependency_capture_updates: 0,
        semantic_segment_count: 4,
        task_outcome_counts: [
            (
                crate::logic::planner::TaskExecutionOutcome::MemoizedReuse,
                1,
            ),
            (
                crate::logic::planner::TaskExecutionOutcome::SnapshotRestoreReuse,
                1,
            ),
            (
                crate::logic::planner::TaskExecutionOutcome::CrossIdentityPersistentReuse,
                1,
            ),
            (
                crate::logic::planner::TaskExecutionOutcome::PartialArtifactSplice,
                1,
            ),
        ]
        .into_iter()
        .collect(),
        stage_outcome_counts: [(
            crate::logic::planner::StageExecutionOutcome::CompletedSerial,
            1,
        )]
        .into_iter()
        .collect(),
    };

    let rendered = render_execution_report_summary(&summary);
    assert!(rendered.contains("memoized=1"));
    assert!(rendered.contains("advanced_reuse=["));
    assert!(rendered.contains("snapshot_restore=1"));
    assert!(rendered.contains("cross_identity=1"));
    assert!(rendered.contains("partial_splice=1"));
}

#[test]
fn rendered_execution_history_summary_surfaces_correspondence_and_splice_detail() {
    let summary = ExecutionHistorySummary {
        profile: DiagnosticsTier::Development,
        traced_node_count: 2,
        execution_record_count: 2,
        latest_execution_record_id: Some(9),
        reuse_origin_counts: [
            (ReuseOrigin::CrossIdentityPersistentReuse, 1),
            (ReuseOrigin::PartialArtifactSplice, 1),
        ]
        .into_iter()
        .collect(),
        nodes: vec![
            ExecutionHistoryNodeSummary {
                node: NodeId::new(1, 0),
                execution_record_id: Some(8),
                semantic_segment_id: Some(1),
                output_change: Some(OutputChange::Refreshed),
                memoized_origin: None,
                reuse_basis: None,
                reuse_origin: Some(ReuseOrigin::CrossIdentityPersistentReuse),
                persistent_correspondence_kind: Some(
                    crate::data::reuse::PersistentCorrespondenceKind::LineageBackedMapping,
                ),
                composition_region_count: 0,
                reuse_certification_proof_count: 1,
                changed_partition_count: 0,
                causality_kind: None,
            },
            ExecutionHistoryNodeSummary {
                node: NodeId::new(2, 0),
                execution_record_id: Some(9),
                semantic_segment_id: Some(2),
                output_change: Some(OutputChange::Refreshed),
                memoized_origin: None,
                reuse_basis: None,
                reuse_origin: Some(ReuseOrigin::PartialArtifactSplice),
                persistent_correspondence_kind: None,
                composition_region_count: 3,
                reuse_certification_proof_count: 1,
                changed_partition_count: 2,
                causality_kind: None,
            },
        ],
    };

    let rendered = render_execution_history_summary(&summary);
    assert!(rendered.contains("LineageBackedMapping"));
    assert!(rendered.contains("partial_splice_nodes=1"));
    assert!(rendered.contains("partial_splice_regions=3"));
}

#[cfg(feature = "parallel")]
#[test]
fn serial_and_parallel_reports_are_semantically_equivalent() {
    let mut graph_serial = SignalGraph::new();
    let a = graph_serial.node().build();
    let b = graph_serial.node().build();
    let c = graph_serial.node().build();
    let mut dependencies = DependencyBatchBuilder::new(&mut graph_serial);
    dependencies
        .append_dependency(c, a, ASPECT_A)
        .unwrap()
        .append_dependency(c, b, ASPECT_A)
        .unwrap();
    dependencies.commit().unwrap();

    let mut graph_parallel = graph_serial.clone();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| {
        let result = if ctx.node() == c {
            let a_v = ctx.read_aspect_version(a, ASPECT_A)?;
            let b_v = ctx.read_aspect_version(b, ASPECT_A)?;
            NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                ASPECT_A,
                a_v.get(ASPECT_A) + b_v.get(ASPECT_A),
            )]))
        } else {
            NodeEvaluationResult::from_version(version_ab(1, 0))
        };
        Ok(ctx.finish(result))
    };

    let bootstrap = graph_serial
        .build_evaluation_plan(&[a, b, c], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph_serial
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();
    graph_parallel
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();

    mark_dirty(&mut graph_serial, a, ASPECT_A).unwrap();
    mark_dirty(&mut graph_parallel, a, ASPECT_A).unwrap();

    let plan_serial = graph_serial
        .build_evaluation_plan(&[c], EvaluationRequestMode::Default)
        .unwrap();
    let plan_parallel = graph_parallel
        .build_evaluation_plan(&[c], EvaluationRequestMode::Default)
        .unwrap();

    let report_serial = graph_serial
        .execute_prepared_plan_with_executor(&plan_serial, &(), &evaluator, StageExecutor::Serial)
        .unwrap();
    let report_parallel = graph_parallel
        .execute_prepared_plan_with_executor(
            &plan_parallel,
            &(),
            &evaluator,
            StageExecutor::parallel(1),
        )
        .unwrap();

    let summary_serial = report_serial.diagnostics_summary(DiagnosticsTier::Development);
    let summary_parallel = report_parallel.diagnostics_summary(DiagnosticsTier::Development);
    assert!(serial_parallel_reports_equivalent(
        &summary_serial,
        &summary_parallel
    ));
}

#[cfg(feature = "parallel")]
#[test]
fn repeated_serial_parallel_lifecycle_parity_stays_stable() {
    let mut graph_serial = SignalGraph::new();
    let source = graph_serial.node().build();
    let wing = graph_serial.node().build();
    let tail = graph_serial.node().build();
    let mut dependencies = DependencyBatchBuilder::new(&mut graph_serial);
    dependencies
        .append_partition_dependency(wing, source, ASPECT_A, "wing")
        .unwrap()
        .append_partition_dependency(tail, source, ASPECT_A, "tail")
        .unwrap();
    dependencies.commit().unwrap();
    let mut graph_parallel = graph_serial.clone();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| {
        let result = if ctx.node() == source {
            NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_changed_region(ChangedRegion::new("wing"))
        } else {
            let version = ctx.read_aspect_version(source, ASPECT_A)?;
            NodeEvaluationResult::from_version(version)
        };
        Ok(ctx.finish(result))
    };

    let bootstrap = graph_serial
        .build_evaluation_plan(&[source, wing, tail], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph_serial
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();
    graph_parallel
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();

    for _ in 0..25 {
        mark_dirty_with_regions(
            &mut graph_serial,
            source,
            ASPECT_A,
            &[ChangedRegion::new("wing")],
        )
        .unwrap();
        mark_dirty_with_regions(
            &mut graph_parallel,
            source,
            ASPECT_A,
            &[ChangedRegion::new("wing")],
        )
        .unwrap();

        let plan_serial = graph_serial
            .build_evaluation_plan(&[wing, tail], EvaluationRequestMode::Default)
            .unwrap();
        let plan_parallel = graph_parallel
            .build_evaluation_plan(&[wing, tail], EvaluationRequestMode::Default)
            .unwrap();

        let report_serial = graph_serial
            .execute_prepared_plan_with_executor(
                &plan_serial,
                &(),
                &evaluator,
                StageExecutor::Serial,
            )
            .unwrap();
        let report_parallel = graph_parallel
            .execute_prepared_plan_with_executor(
                &plan_parallel,
                &(),
                &evaluator,
                StageExecutor::parallel(1),
            )
            .unwrap();

        assert!(serial_parallel_reports_equivalent(
            &report_serial.diagnostics_summary(DiagnosticsTier::Development),
            &report_parallel.diagnostics_summary(DiagnosticsTier::Development),
        ));
        let serial_flow = graph_serial.observe().latest_flow_diagnostics().unwrap();
        let parallel_flow = graph_parallel.observe().latest_flow_diagnostics().unwrap();
        assert_eq!(serial_flow.change, parallel_flow.change);
        assert_eq!(serial_flow.invalidation, parallel_flow.invalidation);
        assert_eq!(serial_flow.planning, parallel_flow.planning);
        assert!(serial_parallel_reports_equivalent(
            &serial_flow.apply.report,
            &parallel_flow.apply.report,
        ));
        assert_eq!(
            serial_flow.apply.prepared_evaluations_applied,
            parallel_flow.apply.prepared_evaluations_applied
        );
        assert_eq!(
            serial_flow.apply.dependency_capture_updates,
            parallel_flow.apply.dependency_capture_updates
        );
        assert_eq!(
            serial_flow.apply.tasks_validated_clean,
            parallel_flow.apply.tasks_validated_clean
        );
        assert_eq!(
            serial_flow.apply.tasks_pruned,
            parallel_flow.apply.tasks_pruned
        );
        assert_eq!(
            serial_flow.apply.tasks_with_suppressed_propagation,
            parallel_flow.apply.tasks_with_suppressed_propagation
        );
    }
}

#[test]
fn repeated_memoized_execution_retains_bounded_diagnostics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime
        .graph_mut()
        .reset_runtime_policy_to_tier(DiagnosticsTier::Operational);
    let family = define_keyed_computation(&mut runtime, "projection", ());
    let keyed = family.keyed("bulkhead");
    let node = keyed.node(&mut runtime);
    let computation = keyed.memoized("shape-v1");
    let mut runtime_ctx = ();

    for cycle in 0..50 {
        let compute_value = cycle as u64 + 1;
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.evaluate_keyed(node, &computation, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(compute_value, 0))
                            .with_output_identity("bulkhead-artifact")
                            .with_output_change(OutputChange::Refreshed),
                    ))
                })?;
                Ok(())
            })
            .unwrap();
        mark_dirty(runtime.graph_mut(), node, ASPECT_A).unwrap();
    }

    let diagnostics = runtime.observe().diagnostics();
    assert!(
        diagnostics.recent_history().len()
            <= SignalRuntimePolicy::for_tier(DiagnosticsTier::Operational)
                .retention_budget
                .history_limit
    );
    assert!(diagnostics
        .recent_history()
        .iter()
        .all(|summary| summary.nodes.is_empty()));
    assert!(runtime.observe().metrics().evaluation.memoization_hits >= 1);
}

#[test]
fn repeated_partition_heavy_invalidation_retains_bounded_diagnostics() {
    let mut graph = SignalGraph::new();
    graph.reset_runtime_policy_to_tier(DiagnosticsTier::Development);
    let source = graph.node().partitioned_output().build();
    let wing = graph.node().build();
    let tail = graph.node().build();
    let mut dependencies = DependencyBatchBuilder::new(&mut graph);
    dependencies
        .append_partition_dependency(wing, source, ASPECT_A, "wing")
        .unwrap()
        .append_partition_dependency(tail, source, ASPECT_A, "tail")
        .unwrap();
    dependencies.commit().unwrap();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| {
        let result = if ctx.node() == source {
            NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_changed_region(ChangedRegion::new("wing"))
        } else {
            NodeEvaluationResult::from_version(ctx.read_aspect_version(source, ASPECT_A)?)
        };
        Ok(ctx.finish(result))
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, wing, tail], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();

    for _ in 0..50 {
        mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")])
            .unwrap();
        let plan = graph
            .build_evaluation_plan(&[wing, tail], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &(), &evaluator).unwrap();
    }

    let diagnostics = graph.observe().diagnostics();
    let flow = diagnostics
        .latest_flow()
        .expect("flow diagnostics should be retained");
    let frontier = diagnostics
        .latest_frontier_execution()
        .expect("frontier execution summary should be retained");
    assert_eq!(flow.change.changed_nodes, vec![source]);
    assert_eq!(flow.change.changed_region_count, 1);
    assert_eq!(
        flow.invalidation.invalidated_direct_subscribers
            + flow.invalidation.maybe_stale_direct_subscribers,
        2
    );
    assert_eq!(
        flow.invalidation.frontier_seed_count as u64,
        frontier.counters.frontier_seed_count
    );
    assert_eq!(
        flow.invalidation.frontier_trace_retained_count as usize,
        diagnostics.latest_invalidation_trace_records().len()
    );
    assert!(
        diagnostics.recent_history().len()
            <= SignalRuntimePolicy::for_tier(DiagnosticsTier::Development)
                .retention_budget
                .history_limit
    );
}

#[test]
fn mixed_direct_and_transitive_frontier_counters_stay_aligned_with_flow_diagnostics() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct = graph.node().build();
    let maybe_stale = graph.node().build();
    let transitive = graph.node().build();

    graph
        .append_partition_detail_dependency(direct, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(maybe_stale, source, ASPECT_A, "wing", "rib-13")
        .unwrap();
    graph
        .append_dependency(transitive, direct, ASPECT_A)
        .unwrap();
    graph
        .append_dependency(transitive, maybe_stale, ASPECT_A)
        .unwrap();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| {
        let result = if ctx.node() == source {
            ctx.finish(
                NodeEvaluationResult::from_version(version_ab(1, 0))
                    .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
            )
        } else {
            let version = ctx.read_aspect_version(source, ASPECT_A)?;
            ctx.finish(NodeEvaluationResult::from_version(version))
        };
        Ok(result)
    };

    let bootstrap = graph
        .build_evaluation_plan(
            &[source, direct, maybe_stale, transitive],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let plan = graph
        .build_evaluation_plan(
            &[direct, maybe_stale, transitive],
            EvaluationRequestMode::Default,
        )
        .unwrap();
    graph.execute_prepared_plan(&plan, &(), &evaluator).unwrap();

    let diagnostics = graph.observe().diagnostics();
    let flow = diagnostics
        .latest_flow()
        .expect("flow diagnostics should be retained");
    let frontier = diagnostics
        .latest_frontier_execution()
        .expect("frontier execution summary should be retained");

    assert_eq!(
        flow.invalidation.frontier_seed_count as u64,
        frontier.counters.frontier_seed_count
    );
    assert_eq!(
        flow.invalidation.frontier_direct_wave_count as u64,
        frontier.counters.frontier_direct_wave_count
    );
    assert_eq!(
        flow.invalidation.frontier_transitive_wave_count as u64,
        frontier.counters.frontier_transitive_wave_count
    );
    assert_eq!(
        flow.invalidation.frontier_cycle_check_candidate_count as u64,
        frontier.counters.frontier_cycle_check_candidate_count
    );
    assert_eq!(
        flow.invalidation.frontier_cycle_check_visited_count as u64,
        frontier.counters.frontier_cycle_check_visited_count
    );
    assert_eq!(
        flow.invalidation.invalidated_direct_subscribers,
        frontier
            .direct_waves
            .iter()
            .flat_map(|wave| wave.entries.iter())
            .filter(|entry| matches!(
                entry.classification,
                FrontierEntryClassification::DirectDirty
            ))
            .count() as u32
    );
    assert_eq!(
        flow.invalidation.maybe_stale_direct_subscribers,
        frontier
            .direct_waves
            .iter()
            .flat_map(|wave| wave.entries.iter())
            .filter(|entry| matches!(
                entry.classification,
                FrontierEntryClassification::MaybeStale
            ))
            .count() as u32
    );
}

#[test]
fn flow_diagnostics_report_zero_realized_transitive_waves_when_frontier_has_none() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct = graph.node().build();

    graph
        .append_partition_detail_dependency(direct, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| {
        let result = if ctx.node() == source {
            ctx.finish(
                NodeEvaluationResult::from_version(version_ab(1, 0))
                    .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
            )
        } else {
            let version = ctx.read_aspect_version(source, ASPECT_A)?;
            ctx.finish(NodeEvaluationResult::from_version(version))
        };
        Ok(result)
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, direct], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &evaluator)
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let plan = graph
        .build_evaluation_plan(&[direct], EvaluationRequestMode::Default)
        .unwrap();
    graph.execute_prepared_plan(&plan, &(), &evaluator).unwrap();

    let diagnostics = graph.observe().diagnostics();
    let flow = diagnostics
        .latest_flow()
        .expect("flow diagnostics should be retained");
    let frontier = diagnostics
        .latest_frontier_execution()
        .expect("frontier execution summary should be retained");

    assert!(frontier
        .transitive_waves
        .iter()
        .all(|wave| wave.entries.is_empty()));
    assert_eq!(frontier.counters.frontier_transitive_wave_count, 0);
    assert_eq!(flow.invalidation.frontier_transitive_wave_count, 0);
}

#[test]
fn runtime_policy_history_budget_overrides_are_enforced() {
    let policy = SignalRuntimePolicy::development()
        .with_history_limit(2)
        .with_detail_limit(1)
        .with_history_details(true);

    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(policy);
    let source = graph.node().output_identity().build();

    for version in 0..5 {
        mark_dirty(&mut graph, source, ASPECT_A).unwrap_or(());
        evaluate(&mut graph, source, &mut |_id, _graph| {
            Ok(
                NodeEvaluationResult::from_version(version_ab(version + 1, 0))
                    .with_output_identity(format!("budget-{version}")),
            )
        })
        .unwrap();
    }

    let diagnostics = graph.observe().diagnostics();
    assert!(diagnostics.recent_history().len() <= 2);
    assert!(
        diagnostics
            .recent_history()
            .iter()
            .all(|summary| summary.nodes.len() <= 1),
        "detail limit override should trim retained node detail"
    );
}
