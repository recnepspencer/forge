use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::facade::*;
use crate::tests::support::*;

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
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, source, &mut source_compute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let left = graph.diagnostics_summary(DiagnosticsProfile::Development);
    let right = graph.diagnostics_summary(DiagnosticsProfile::Development);
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
    let compute = |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(1, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph.execute_prepared_plan(&plan, &compute).unwrap();

    let diagnostics = graph.diagnostics();
    let summary = diagnostics.summary(DiagnosticsProfile::Operational);
    let history = diagnostics.history(DiagnosticsProfile::Operational);
    let latest_flow = diagnostics.latest_flow();
    let graph_inspector = diagnostics.inspect_graph();
    let execution_inspector = diagnostics.inspect_execution();

    assert_eq!(summary.active_node_count, 1);
    assert!(history.latest_execution_record_id.is_some());
    assert!(latest_flow.is_some());
    assert!(graph_inspector
        .nodes_with_execution_record()
        .contains(&node));
    assert_eq!(execution_inspector.nodes_with_trace_summaries(), vec![node]);
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
    graph_b.add_dependency(dependent, source, ASPECT_A).unwrap();
    evaluate(&mut graph_b, source, &mut compute).unwrap();
    evaluate(&mut graph_b, dependent, &mut compute).unwrap();
    mark_dirty(&mut graph_b, source, ASPECT_A).unwrap();

    let left = graph_a.diagnostics_summary(DiagnosticsProfile::Operational);
    let right = graph_b.diagnostics_summary(DiagnosticsProfile::Operational);
    let diff = compare_graphs(&left, &right);
    assert!(!diff.is_empty());
    assert!(!graphs_semantically_equivalent(&left, &right));
}

#[test]
fn inspectors_query_graph_plan_report_and_execution_history() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().on_demand().build();

    let source_compute =
        |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(1, 0)));
    let dependent_compute = |_node: NodeId, view: &ExecutionReadView<'_>| {
        let version = view.read_aspect_version(source, ASPECT_A)?;
        Ok(view.finish(NodeEvaluationResult::from_version(version)))
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|node, view| {
            if node == source {
                source_compute(node, view)
            } else {
                dependent_compute(node, view)
            }
        })
        .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &|node, view| {
            if node == source {
                source_compute(node, view)
            } else {
                dependent_compute(node, view)
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

    let compute = |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(2, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    let report = graph.execute_prepared_plan(&plan, &compute).unwrap();
    let explanation = graph.explain(node).unwrap();

    let flow = FlowSummary::new(
        DiagnosticsProfile::Development,
        ChangeInputSummary::new(
            vec![node],
            vec![ASPECT_A],
            0,
            Some("host-change".to_string()),
        ),
        InvalidationSummary::new(1, 0, 0),
        PlanningSummary::from_plan(&plan, DiagnosticsProfile::Development),
        PrecomputeSummary::from_report(&report, DiagnosticsProfile::Development),
        ApplySummary::from_report(&report, DiagnosticsProfile::Development),
        None,
        Some(explanation.diagnostics_summary(DiagnosticsProfile::Development)),
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
        Some(plan.summary.clone()),
        "precompute failed",
    );
    let rollback = RollbackDiagnostic::new(true, 3, 2, Some("rewound staged changes".to_string()));
    let failure_summary = failure.summarize(Some(&rollback), DiagnosticsProfile::Forensic);
    let failure_summary_2 = failure.summarize(None, DiagnosticsProfile::Forensic);
    assert!(!compare_failures(&failure_summary, &failure_summary_2).is_empty());
    assert!(render_failure_summary(&failure_summary).contains("FailureSummary"));
}

#[test]
fn successful_execution_automatically_records_flow_and_history_diagnostics() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let source_compute =
        |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(1, 0)));
    let dependent_compute = |_node: NodeId, view: &ExecutionReadView<'_>| {
        let version = view.read_aspect_version(source, ASPECT_A)?;
        Ok(view.finish(NodeEvaluationResult::from_version(version)))
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|node, view| {
            if node == source {
                source_compute(node, view)
            } else {
                dependent_compute(node, view)
            }
        })
        .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    graph
        .execute_prepared_plan(&plan, &|node, view| {
            if node == source {
                source_compute(node, view)
            } else {
                dependent_compute(node, view)
            }
        })
        .unwrap();

    let flow = graph
        .latest_flow_diagnostics()
        .expect("flow diagnostics should be recorded");
    assert_eq!(flow.change.changed_nodes, vec![source]);
    assert_eq!(flow.planning.plan.task_count, 2);
    assert_eq!(flow.apply.report.task_count, 2);
    assert!(!graph.recent_execution_history_diagnostics().is_empty());
}

#[test]
fn diagnostics_profiles_control_retention_bounds() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compute = |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(1, 0)));

    graph.set_diagnostics_profile(DiagnosticsProfile::Operational);
    for _ in 0..8 {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &compute).unwrap();
    }
    assert!(graph.recent_execution_history_diagnostics().len() <= 4);

    graph.set_diagnostics_profile(DiagnosticsProfile::Forensic);
    for _ in 0..8 {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &compute).unwrap();
    }
    assert!(graph.recent_execution_history_diagnostics().len() > 4);
}

#[test]
fn operational_profile_repeated_waves_stay_bounded_and_shallow() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph.set_diagnostics_profile(DiagnosticsProfile::Operational);
    let compute = |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(1, 0)));

    for _ in 0..100 {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &compute).unwrap();
    }

    let diagnostics = graph.diagnostics();
    let history = diagnostics.recent_history();
    let policy = DiagnosticsPolicy::from_profile(DiagnosticsProfile::Operational);
    assert!(history.len() <= policy.history_limit);
    assert!(history.iter().all(|summary| summary.nodes.is_empty()));
    assert!(diagnostics.latest_failure().is_none());
}

#[test]
fn execution_failures_and_rollbacks_automatically_record_diagnostics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let node = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.evaluate_with_plan(
                node,
                &|_node, _view| Err(SignalError::internal("synthetic precompute failure")),
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap_err();
    assert!(format!("{err}").contains("synthetic precompute failure"));

    let failure = runtime
        .latest_failure_diagnostics()
        .expect("failure diagnostics should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::Precompute);

    let rollback = runtime
        .latest_rollback_diagnostics()
        .expect("rollback diagnostics should be retained");
    assert!(rollback.rolled_back);
}

#[test]
fn repeated_failure_capture_stays_current_and_bounded() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime
        .graph_mut()
        .set_diagnostics_profile(DiagnosticsProfile::Development);
    let node = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    for cycle in 0..100 {
        let err = runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(node, ASPECT_A)?;
                tx.evaluate_with_plan(
                    node,
                    &move |_node, _view| {
                        Err(SignalError::internal(format!(
                            "synthetic precompute failure cycle {cycle}"
                        )))
                    },
                    EvaluationRequestMode::Default,
                )?;
                Ok(())
            })
            .unwrap_err();
        assert!(format!("{err}").contains("synthetic precompute failure cycle"));
    }

    let diagnostics = runtime.diagnostics();
    let failure = diagnostics
        .latest_failure()
        .expect("latest failure should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::Precompute);
    assert!(failure.message.contains("cycle 99"));
    assert!(
        diagnostics.recent_history().len()
            <= DiagnosticsPolicy::from_profile(DiagnosticsProfile::Development).history_limit
    );
}

#[test]
fn repeated_rollbacks_keep_latest_rollback_current_and_bounded() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime
        .graph_mut()
        .set_diagnostics_profile(DiagnosticsProfile::Development);
    let node = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    for _ in 0..100 {
        let err = runtime.transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            Err(SignalError::invalid_input("force rollback"))
        });
        assert!(err.is_err());
    }

    let diagnostics = runtime.diagnostics();
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
            <= DiagnosticsPolicy::from_profile(DiagnosticsProfile::Development).history_limit
    );
}

#[test]
fn commit_promotion_failures_record_failure_and_rollback_diagnostics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
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
        .latest_failure_diagnostics()
        .expect("flush failure diagnostics should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::CommitPromotion);
    let rollback = runtime
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
        .latest_failure_diagnostics()
        .expect("begin failure diagnostics should be retained");
    assert_eq!(failure.phase, ExecutionFailurePhase::CommitPromotion);
    let rollback = runtime
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
    let compute = |_node: NodeId, view: &ExecutionReadView<'_>| Ok(view.finish(version_ab(3, 0)));
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph.execute_prepared_plan(&plan, &compute).unwrap();

    let history_a = graph.execution_history_summary(DiagnosticsProfile::Forensic);
    let history_b = graph.execution_history_summary(DiagnosticsProfile::Forensic);
    assert!(compare_execution_history(&history_a, &history_b).is_empty());
    assert!(repeat_run_summaries_equal(&[
        history_a.clone(),
        history_b.clone()
    ]));
    assert!(render_execution_history_summary(&history_a).contains("ExecutionHistorySummary"));

    let explanation_a = graph
        .explain(node)
        .unwrap()
        .diagnostics_summary(DiagnosticsProfile::Development);
    let explanation_b = graph
        .explain(node)
        .unwrap()
        .diagnostics_summary(DiagnosticsProfile::Development);
    assert!(compare_explanations(&explanation_a, &explanation_b).is_empty());
    assert!(explanations_semantically_equivalent(
        &explanation_a,
        &explanation_b
    ));
    assert!(render_explanation_summary(&explanation_a).contains("ExplanationSummary"));
}

#[cfg(feature = "parallel")]
#[test]
fn serial_and_parallel_reports_are_semantically_equivalent() {
    let mut graph_serial = SignalGraph::new();
    let a = graph_serial.node().build();
    let b = graph_serial.node().build();
    let c = graph_serial.node().build();
    graph_serial.add_dependency(c, a, ASPECT_A).unwrap();
    graph_serial.add_dependency(c, b, ASPECT_A).unwrap();

    let mut graph_parallel = graph_serial.clone();

    let precompute = |node: NodeId, view: &ExecutionReadView<'_>| {
        let result = if node == c {
            let a_v = view.read_aspect_version(a, ASPECT_A)?;
            let b_v = view.read_aspect_version(b, ASPECT_A)?;
            NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                ASPECT_A,
                a_v.get(ASPECT_A) + b_v.get(ASPECT_A),
            )]))
        } else {
            NodeEvaluationResult::from_version(version_ab(1, 0))
        };
        Ok(view.finish(result))
    };

    let bootstrap = graph_serial
        .build_evaluation_plan(&[a, b, c], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph_serial
        .execute_prepared_plan(&bootstrap, &precompute)
        .unwrap();
    graph_parallel
        .execute_prepared_plan(&bootstrap, &precompute)
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
        .execute_prepared_plan_with_executor(&plan_serial, &precompute, StageExecutor::Serial)
        .unwrap();
    let report_parallel = graph_parallel
        .execute_prepared_plan_with_executor(
            &plan_parallel,
            &precompute,
            StageExecutor::parallel(1),
        )
        .unwrap();

    let summary_serial = report_serial.diagnostics_summary(DiagnosticsProfile::Development);
    let summary_parallel = report_parallel.diagnostics_summary(DiagnosticsProfile::Development);
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
    graph_serial
        .add_partition_dependency(wing, source, ASPECT_A, "wing")
        .unwrap();
    graph_serial
        .add_partition_dependency(tail, source, ASPECT_A, "tail")
        .unwrap();
    let mut graph_parallel = graph_serial.clone();

    let precompute = |node: NodeId, view: &ExecutionReadView<'_>| {
        let result = if node == source {
            NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_changed_region(ChangedRegion::new("wing"))
        } else {
            let version = view.read_aspect_version(source, ASPECT_A)?;
            NodeEvaluationResult::from_version(version)
        };
        Ok(view.finish(result))
    };

    let bootstrap = graph_serial
        .build_evaluation_plan(&[source, wing, tail], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph_serial
        .execute_prepared_plan(&bootstrap, &precompute)
        .unwrap();
    graph_parallel
        .execute_prepared_plan(&bootstrap, &precompute)
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
            .execute_prepared_plan_with_executor(&plan_serial, &precompute, StageExecutor::Serial)
            .unwrap();
        let report_parallel = graph_parallel
            .execute_prepared_plan_with_executor(
                &plan_parallel,
                &precompute,
                StageExecutor::parallel(1),
            )
            .unwrap();

        assert!(serial_parallel_reports_equivalent(
            &report_serial.diagnostics_summary(DiagnosticsProfile::Development),
            &report_parallel.diagnostics_summary(DiagnosticsProfile::Development),
        ));
        let serial_flow = graph_serial.latest_flow_diagnostics().unwrap();
        let parallel_flow = graph_parallel.latest_flow_diagnostics().unwrap();
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
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime
        .graph_mut()
        .set_diagnostics_profile(DiagnosticsProfile::Operational);
    let family = runtime.register_computation_family("projection");
    let node = runtime.keyed_node(&family, "bulkhead");
    let computation = KeyedComputation::new(family.clone(), "bulkhead").with_memo_key("shape-v1");
    let mut runtime_ctx = ();

    for cycle in 0..50 {
        let compute_value = cycle as u64 + 1;
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.evaluate_keyed(node, &computation, &|_id, view| {
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

    let diagnostics = runtime.diagnostics();
    assert!(
        diagnostics.recent_history().len()
            <= DiagnosticsPolicy::from_profile(DiagnosticsProfile::Operational).history_limit
    );
    assert!(diagnostics
        .recent_history()
        .iter()
        .all(|summary| summary.nodes.is_empty()));
    assert!(runtime.metrics().memoization_hits >= 1);
}

#[test]
fn repeated_partition_heavy_invalidation_retains_bounded_diagnostics() {
    let mut graph = SignalGraph::new();
    graph.set_diagnostics_profile(DiagnosticsProfile::Development);
    let source = graph.node().partitioned_output().build();
    let wing = graph.node().build();
    let tail = graph.node().build();
    graph
        .add_partition_dependency(wing, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .add_partition_dependency(tail, source, ASPECT_A, "tail")
        .unwrap();

    let precompute = |node: NodeId, view: &ExecutionReadView<'_>| {
        let result = if node == source {
            NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_changed_region(ChangedRegion::new("wing"))
        } else {
            NodeEvaluationResult::from_version(view.read_aspect_version(source, ASPECT_A)?)
        };
        Ok(view.finish(result))
    };

    let bootstrap = graph
        .build_evaluation_plan(&[source, wing, tail], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &precompute)
        .unwrap();

    for _ in 0..50 {
        mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")])
            .unwrap();
        let plan = graph
            .build_evaluation_plan(&[wing, tail], EvaluationRequestMode::Default)
            .unwrap();
        graph.execute_prepared_plan(&plan, &precompute).unwrap();
    }

    let diagnostics = graph.diagnostics();
    let flow = diagnostics
        .latest_flow()
        .expect("flow diagnostics should be retained");
    assert_eq!(flow.change.changed_nodes, vec![source]);
    assert_eq!(flow.change.changed_region_count, 1);
    assert_eq!(
        flow.invalidation.invalidated_direct_subscribers
            + flow.invalidation.maybe_stale_direct_subscribers,
        2
    );
    assert!(
        diagnostics.recent_history().len()
            <= DiagnosticsPolicy::from_profile(DiagnosticsProfile::Development).history_limit
    );
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

    let diagnostics = graph.diagnostics();
    assert!(diagnostics.recent_history().len() <= 2);
    assert!(
        diagnostics
            .recent_history()
            .iter()
            .all(|summary| summary.nodes.len() <= 1),
        "detail limit override should trim retained node detail"
    );
}
