use crate::data::trace::TraceSummary;
use crate::diagnostics::policy::DiagnosticsPolicy;
use crate::facade::*;
use crate::tests::support::{version_ab, ASPECT_A};

#[test]
fn operational_profile_stays_bounded_under_snapshot_and_dependency_churn() {
    let mut graph = SignalGraph::new();
    graph.set_diagnostics_profile(DiagnosticsProfile::Operational);
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let dependent = graph.node().build();

    let bootstrap = |graph: &mut SignalGraph, use_source_b: bool| {
        graph.remove_dependency(dependent, source_a, ASPECT_A).ok();
        graph.remove_dependency(dependent, source_b, ASPECT_A).ok();
        graph
            .add_dependency(
                dependent,
                if use_source_b { source_b } else { source_a },
                ASPECT_A,
            )
            .unwrap();
    };

    for wave in 0..40 {
        let use_source_b = wave % 2 == 1;
        bootstrap(&mut graph, use_source_b);
        let target_source = if use_source_b { source_b } else { source_a };
        mark_dirty(&mut graph, target_source, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &|ctx| {
                let result = if ctx.node() == source_a {
                    ctx.finish(version_ab(1 + wave as u64, 0))
                } else if ctx.node() == source_b {
                    ctx.finish(version_ab(10 + wave as u64, 0))
                } else {
                    let version = ctx.read_aspect_version(target_source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(version))
                };
                Ok(result)
            })
            .unwrap();
    }

    let diagnostics = graph.observe().diagnostics();
    let policy = DiagnosticsPolicy::from_profile(DiagnosticsProfile::Operational);
    assert!(diagnostics.recent_history().len() <= policy.history_limit);
    assert!(diagnostics.latest_failure().is_none());
    assert!(diagnostics.latest_rollback().is_none());
    assert!(diagnostics.latest_flow().is_some());
}

#[test]
fn repeated_failure_and_rollback_loops_preserve_explanation_after_churn() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime
        .graph_mut()
        .set_diagnostics_profile(DiagnosticsProfile::Development);
    let source_a = runtime.graph_mut().node().build();
    let source_b = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_with_plan(
                dependent,
                &|view| {
                    let result = if view.node() == source_a {
                        view.finish(version_ab(1, 0))
                    } else if view.node() == source_b {
                        view.finish(version_ab(2, 0))
                    } else {
                        let version = view.read_aspect_version(source_a, ASPECT_A)?;
                        view.finish(NodeEvaluationResult::from_version(version))
                    };
                    Ok(result)
                },
                EvaluationRequestMode::ForceOnDemand,
            )?;
            Ok(())
        })
        .unwrap();

    for wave in 0..16 {
        let err = runtime.transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(if wave % 2 == 0 { source_a } else { source_b }, ASPECT_A)?;
            tx.evaluate_with_plan(
                dependent,
                &|view| {
                    let result = if view.node() == source_a {
                        view.finish(version_ab(1 + wave as u64, 0))
                    } else if view.node() == source_b {
                        view.finish(version_ab(100 + wave as u64, 0))
                    } else if wave % 2 == 0 {
                        let version = view.read_aspect_version(source_b, ASPECT_A)?;
                        view.finish(NodeEvaluationResult::from_version(version))
                    } else {
                        let version = view.read_aspect_version(source_a, ASPECT_A)?;
                        view.finish(NodeEvaluationResult::from_version(version))
                    };
                    Ok(result)
                },
                EvaluationRequestMode::Default,
            )?;
            Err(SignalError::invalid_input("force rollback after churn"))
        });
        assert!(err.is_err());
    }

    let diagnostics = runtime.observe().diagnostics();
    assert!(diagnostics.latest_rollback().is_some());
    let explanation = runtime.observe().explain(dependent).unwrap();
    assert!(!explanation.upstream.is_empty());
}

#[test]
#[ignore = "stress coverage for repeated development-profile diagnostics waves"]
fn stress_development_profile_repeated_waves_remains_semantically_stable() {
    let mut graph = SignalGraph::new();
    graph.set_diagnostics_profile(DiagnosticsProfile::Development);
    let source = graph.node().output_identity().build();
    let dependents: Vec<_> = (0..64)
        .map(|_| graph.node().partitioned_output().build())
        .collect();
    for (index, &dependent) in dependents.iter().enumerate() {
        graph
            .add_partition_detail_dependency(
                dependent,
                source,
                ASPECT_A,
                "wing",
                format!("rib-{index}"),
            )
            .unwrap();
    }

    for wave in 0..200 {
        mark_dirty_with_regions(
            &mut graph,
            source,
            ASPECT_A,
            &[ChangedRegion::new("wing").with_detail(format!("rib-{}", wave % 64))],
        )
        .unwrap();
        let plan = graph
            .build_evaluation_plan(&dependents, EvaluationRequestMode::Default)
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &|ctx| {
                let result = if ctx.node() == source {
                    ctx.finish(
                        NodeEvaluationResult::from_version(version_ab(wave as u64 + 1, 0))
                            .with_output_identity("wing-artifact")
                            .with_changed_region(
                                ChangedRegion::new("wing")
                                    .with_detail(format!("rib-{}", wave % 64)),
                            ),
                    )
                } else {
                    let version = ctx.read_partitioned_aspect_version(
                        source,
                        ASPECT_A,
                        PartitionSubscription::partition_and_detail(
                            "wing",
                            format!("rib-{}", wave % 64),
                        ),
                    )?;
                    ctx.finish(NodeEvaluationResult::from_version(version))
                };
                Ok(result)
            })
            .unwrap();
    }

    assert!(graph.observe().diagnostics().recent_history().len() > 1);
    assert!(graph.observe().latest_failure_diagnostics().is_none());
}

#[test]
fn execution_history_prefers_most_recent_records_over_low_arena_indices() {
    let mut graph = SignalGraph::new();
    graph.set_diagnostics_profile(DiagnosticsProfile::Development);
    let mut nodes = Vec::new();
    for _ in 0..96 {
        nodes.push(graph.node().build());
    }

    for (record_id, &node) in nodes.iter().enumerate() {
        let mut trace = TraceSummary::default();
        trace.execution_record_id = Some(record_id as u64 + 1);
        graph
            .get_entry_mut(node)
            .unwrap()
            .set_trace_summary(Some(trace));
    }

    let history = graph.observe().execution_history_summary(DiagnosticsProfile::Development);
    let retained = history
        .nodes
        .iter()
        .map(|summary| summary.node)
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        retained.contains(&nodes[79]),
        "history detail should retain newest high-index node executions instead of truncating by arena order: {history:?}"
    );
    assert!(
        !retained.contains(&nodes[0]),
        "history detail should not be dominated by stale low-index nodes when detail_limit is exceeded: {history:?}"
    );
}
