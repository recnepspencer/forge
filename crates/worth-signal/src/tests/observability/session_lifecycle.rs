use crate::data::telemetry::InvalidationPerformedCounter;
use crate::facade::{
    DependencyEdge, SignalGraph, SignalObservationAdmissionDenial, SignalObservationRequest,
    SignalRuntimePolicy,
};
use crate::tests::support::ASPECT_A;

#[test]
fn on_demand_idle_does_not_capture_performed_counter_work() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();

    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();

    let bootstrap = graph
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();
    crate::facade::mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(2, 0)))
        })
        .unwrap();

    let counters = graph.invalidation_performed_counters();
    assert!(InvalidationPerformedCounter::ALL
        .into_iter()
        .all(|counter| counters.value(counter) == 0));
    assert!(graph.invalidation_performed_work().is_empty());
    assert!(graph.observe().lineage_records().is_empty());
    assert!(graph.observe().explanation_fact(consumer).is_none());
    assert!(graph
        .observe()
        .latest_frontier_execution_summary()
        .is_none());
    assert!(graph.observe().replay_events().is_empty());
    assert!(graph.observe().latest_flow_diagnostics().is_none());
}

#[test]
fn on_demand_clone_and_checkpoint_restore_keep_optional_capture_disabled() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    let bootstrap = graph
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();
    crate::facade::mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(2, 0)))
        })
        .unwrap();

    let mut cloned = graph.clone();
    let authority = graph.capture_checkpoint_authority();
    let mut restored = SignalGraph::restore_from_checkpoint_authority(&authority).unwrap();
    for candidate in [&mut cloned, &mut restored] {
        let telemetry_before = *candidate.observe().telemetry();
        crate::facade::mark_dirty(&mut *candidate, consumer, ASPECT_A).unwrap();
        let follow_up = candidate
            .build_evaluation_plan(
                &[consumer],
                crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
            )
            .unwrap();
        candidate
            .execute_prepared_plan(&follow_up, &(), &|context| {
                Ok(context.finish(crate::tests::support::version_ab(3, 0)))
            })
            .unwrap();
        assert_eq!(
            candidate.get_state(consumer).unwrap(),
            crate::data::node::NodeState::Clean
        );
        assert_eq!(
            *candidate.observe().telemetry(),
            telemetry_before,
            "OnDemand clone/restore must not retain optional telemetry after execution"
        );
        assert_eq!(
            candidate.runtime_policy().observation_activation(),
            worth_foundational::ObservationActivationProfile::OnDemand
        );
        assert!(InvalidationPerformedCounter::ALL
            .into_iter()
            .all(|counter| candidate.invalidation_performed_counters().value(counter) == 0));
        assert!(candidate.invalidation_performed_work().is_empty());
        assert!(candidate.observe().lineage_records().is_empty());
        assert!(candidate.observe().explanation_fact(consumer).is_none());
        assert!(candidate
            .observe()
            .latest_frontier_execution_summary()
            .is_none());
        assert!(candidate.observe().replay_events().is_empty());
        assert!(candidate.observe().latest_flow_diagnostics().is_none());
    }
}

#[test]
fn explicit_session_selects_counters_and_rejects_nested_admission() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();
    let session = graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();

    assert!(matches!(
        graph.begin_observation_session(SignalObservationRequest::operation()),
        Err(SignalObservationAdmissionDenial::SessionAlreadyActive)
    ));
    assert!(matches!(
        graph.try_set_runtime_policy(SignalRuntimePolicy::development()),
        Err(crate::runtime_policy::SignalRuntimePolicyCompilationDenial::ObservationSessionActive)
    ));
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    assert_eq!(
        graph
            .invalidation_performed_counters()
            .value(InvalidationPerformedCounter::TopologyRevisionRevalidations),
        1
    );
    assert!(graph.finish_observation_session(&session).is_err());
}

#[test]
fn empty_request_is_denied_before_runtime_state_changes() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    assert!(matches!(
        graph.begin_observation_session(SignalObservationRequest::empty()),
        Err(SignalObservationAdmissionDenial::EmptyRequest)
    ));
}

#[test]
fn public_runtime_session_finishes_real_counter_execution_and_reports_selection() {
    let mut runtime = crate::facade::SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = runtime.graph_mut().node().build();
    let target = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .set_dependencies(target, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    let bootstrap = runtime
        .graph_mut()
        .build_evaluation_plan(
            &[target],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    runtime
        .graph_mut()
        .execute_prepared_plan(&bootstrap, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();
    crate::facade::mark_dirty(runtime.graph_mut(), target, ASPECT_A).unwrap();

    let (_, receipt) = runtime
        .observe_execution(SignalObservationRequest::counters(), |runtime| {
            let plan = runtime.graph_mut().build_evaluation_plan(
                &[target],
                crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
            )?;
            crate::logic::planner::execute_prepared_plan(
                &mut runtime.graph_mut(),
                &plan,
                &(),
                &|context: &mut crate::logic::context::EvaluationContext<'_, ()>| {
                    Ok(context.finish(crate::tests::support::version_ab(1, 0)))
                },
            )?;
            Ok(())
        })
        .unwrap();

    assert!(
        receipt
            .realized_counters()
            .value(InvalidationPerformedCounter::NodesEvaluated)
            > 0
    );
    assert!(!receipt
        .request()
        .includes(crate::facade::SignalObservationSurface::PerformedWork));
    assert_eq!(
        receipt.completion(),
        crate::facade::SignalObservationCompletion::Completed
    );
}

#[test]
fn selected_surfaces_are_reciprocal_for_counters_and_lineage() {
    let mut runtime = crate::facade::SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = runtime.graph_mut().node().build();
    let target = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .set_dependencies(target, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    let bootstrap = runtime
        .graph_mut()
        .build_evaluation_plan(
            &[target],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    runtime
        .graph_mut()
        .execute_prepared_plan(&bootstrap, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();

    crate::facade::mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();
    let (_, counters) = runtime
        .observe_execution(SignalObservationRequest::counters(), |runtime| {
            let plan = runtime.graph_mut().build_evaluation_plan(
                &[target],
                crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
            )?;
            crate::logic::planner::execute_prepared_plan(
                &mut runtime.graph_mut(),
                &plan,
                &(),
                &|context: &mut crate::logic::context::EvaluationContext<'_, ()>| {
                    Ok(context.finish(crate::tests::support::version_ab(2, 0)))
                },
            )?;
            Ok(())
        })
        .unwrap();
    assert!(counters
        .realized_counters()
        .values()
        .into_iter()
        .any(|value| value > 0));
    assert!(runtime.graph().observe().lineage_records().is_empty());
    assert!(runtime
        .graph()
        .observe()
        .latest_flow_diagnostics()
        .is_none());

    crate::facade::mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();
    let (_, lineage) = runtime
        .observe_execution(SignalObservationRequest::lineage(), |runtime| {
            let plan = runtime.graph_mut().build_evaluation_plan(
                &[target],
                crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
            )?;
            crate::logic::planner::execute_prepared_plan(
                &mut runtime.graph_mut(),
                &plan,
                &(),
                &|context: &mut crate::logic::context::EvaluationContext<'_, ()>| {
                    Ok(context.finish(crate::tests::support::version_ab(3, 0)))
                },
            )?;
            Ok(())
        })
        .unwrap();
    assert!(InvalidationPerformedCounter::ALL
        .into_iter()
        .all(|counter| lineage.realized_counters().value(counter) == 0));
    assert!(!runtime.graph().observe().lineage_records().is_empty());
    assert!(runtime.graph().invalidation_performed_work().is_empty());
    assert!(lineage
        .request()
        .includes(crate::facade::SignalObservationSurface::DescriptiveLineage));
    assert!(!lineage
        .request()
        .includes(crate::facade::SignalObservationSurface::PerformedCounters));
}
