#![cfg(feature = "parallel")]

use std::num::NonZeroUsize;

use forge_harness::facade::{ComparisonMode, ComparisonProfile, ExecutionRequest};

use crate::facade::*;
use crate::logic::planner::{ParallelApplyMode, ParallelExecutionPolicy};
use crate::tests::support::{version_ab, ASPECT_A};

use crate::presentation::harness::{signal_parity_suite, SignalProfileCatalog, SignalScenario};

#[test]
fn many_thin_stages_remain_serial_under_parallel_threshold() {
    let mut graph = SignalGraph::new();
    let mut chain = Vec::new();
    for _ in 0..32 {
        chain.push(graph.node().build());
    }
    for index in 1..chain.len() {
        graph
            .add_dependency(chain[index], chain[index - 1], ASPECT_A)
            .unwrap();
    }

    let bootstrap = graph
        .build_evaluation_plan(&chain, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|_node, view| Ok(view.finish(version_ab(1, 0))))
        .unwrap();

    mark_dirty(&mut graph, chain[0], ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&[chain[chain.len() - 1]], EvaluationRequestMode::Default)
        .unwrap();
    let before = graph.telemetry().parallel_stage_dispatch_count;
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &|_node, view| Ok(view.finish(version_ab(2, 0))),
            StageExecutor::parallel(3),
        )
        .unwrap();

    assert_eq!(graph.telemetry().parallel_stage_dispatch_count, before);
    assert!(report.stages.iter().all(|stage| {
        matches!(
            stage.outcome,
            crate::facade::StageExecutionOutcome::CompletedSerial
        )
    }));
}

#[test]
fn wide_stage_crosses_parallel_threshold() {
    let mut graph = SignalGraph::new();
    let left = graph.node().build();
    let right = graph.node().build();
    let requested = [left, right];

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|_node, view| Ok(view.finish(version_ab(1, 0))))
        .unwrap();

    mark_dirty(&mut graph, left, ASPECT_A).unwrap();
    mark_dirty(&mut graph, right, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    assert_eq!(plan.summary.max_stage_width, 2);
    let before = graph.telemetry().parallel_stage_dispatch_count;
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &|_node, view| Ok(view.finish(version_ab(2, 0))),
            StageExecutor::parallel(2),
        )
        .unwrap();

    assert_eq!(graph.telemetry().parallel_stage_dispatch_count, before + 1);
    assert_eq!(report.stages.len(), 1);
    assert!(report.stages.iter().any(|stage| {
        matches!(
            stage.outcome,
            crate::facade::StageExecutionOutcome::CompletedParallel
        )
    }));
}

#[test]
fn full_parallel_splits_wide_stage_into_deterministic_apply_groups() {
    let mut graph = SignalGraph::new();
    let requested: Vec<_> = (0..4).map(|_| graph.node().build()).collect();

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|_node, view| Ok(view.finish(version_ab(1, 0))))
        .unwrap();

    for &node in &requested {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    }

    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let policy = ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
        .with_apply_group_min_width(2)
        .with_max_concurrent_apply_groups(2);
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &|_node, view| Ok(view.finish(version_ab(2, 0))),
            StageExecutor::full_parallel(1).with_parallel_policy(policy),
        )
        .unwrap();

    assert_eq!(report.stages.len(), 1);
    let stage = &report.stages[0];
    assert_eq!(
        stage.apply_mode,
        Some(ParallelApplyMode::GroupedConcurrentApply)
    );
    assert_eq!(stage.apply_group_count, 2);
    assert_eq!(stage.concurrent_apply_task_count, requested.len() as u32);
}

#[test]
fn full_parallel_rewires_dynamic_dependencies_without_losing_parity() {
    fn bootstrap_graph() -> Result<(SignalGraph, NodeId, NodeId, NodeId, [NodeId; 2]), SignalError>
    {
        let mut graph = SignalGraph::new();
        let selector = graph.node().build();
        let left = graph.node().build();
        let right = graph.node().build();
        let target_a = graph.node().build();
        let target_b = graph.node().build();
        let targets = [target_a, target_b];

        let bootstrap = graph.build_evaluation_plan(
            &[selector, left, right, target_a, target_b],
            EvaluationRequestMode::ForceOnDemand,
        )?;
        graph.execute_prepared_plan(&bootstrap, &move |node, view| {
            let result = if node == selector {
                view.finish(version_ab(0, 0))
            } else if node == left {
                view.finish(version_ab(10, 0))
            } else if node == right {
                view.finish(version_ab(20, 0))
            } else {
                let selector_version = view.read_aspect_version(selector, ASPECT_A)?;
                let source = if selector_version.get(ASPECT_A) == 0 {
                    left
                } else {
                    right
                };
                let chosen = view.read_aspect_version(source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(chosen))
            };
            Ok(result)
        })?;

        Ok((graph, selector, left, right, targets))
    }

    fn rewire_targets(
        graph: &mut SignalGraph,
        selector: NodeId,
        right: NodeId,
        targets: &[NodeId; 2],
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError> {
        mark_dirty(graph, selector, ASPECT_A)?;
        let plan = graph.build_evaluation_plan(targets, EvaluationRequestMode::Default)?;
        graph.execute_prepared_plan_with_executor(
            &plan,
            &move |node, view| {
                let result = if node == selector {
                    view.finish(version_ab(1, 0))
                } else {
                    let selector_version = view.read_aspect_version(selector, ASPECT_A)?;
                    let source = if selector_version.get(ASPECT_A) == 0 {
                        unreachable!("selector should have recomputed before dynamic targets")
                    } else {
                        right
                    };
                    let chosen = view.read_aspect_version(source, ASPECT_A)?;
                    view.finish(NodeEvaluationResult::from_version(chosen))
                };
                Ok(result)
            },
            executor,
        )
    }

    let (base_graph, selector, left, right, targets) = bootstrap_graph().unwrap();
    let mut serial_graph = base_graph.clone();
    let mut parallel_graph = base_graph;

    let serial_report = rewire_targets(
        &mut serial_graph,
        selector,
        right,
        &targets,
        StageExecutor::Serial,
    )
    .unwrap();
    let parallel_report = rewire_targets(
        &mut parallel_graph,
        selector,
        right,
        &targets,
        StageExecutor::full_parallel(2),
    )
    .unwrap();

    for target in targets {
        assert_eq!(
            serial_graph.dependencies_of(target).unwrap(),
            parallel_graph.dependencies_of(target).unwrap()
        );
        assert!(parallel_graph
            .dependencies_of(target)
            .unwrap()
            .iter()
            .any(|edge| edge.source() == right));
        assert!(!parallel_graph
            .dependencies_of(target)
            .unwrap()
            .iter()
            .any(|edge| edge.source() == left));
        assert_eq!(
            serial_graph.get_entry(target).unwrap().get_aspect_version(),
            parallel_graph
                .get_entry(target)
                .unwrap()
                .get_aspect_version()
        );
    }

    assert_eq!(
        serial_graph.subscribers_of(left).unwrap(),
        parallel_graph.subscribers_of(left).unwrap()
    );
    assert_eq!(
        serial_graph.subscribers_of(right).unwrap(),
        parallel_graph.subscribers_of(right).unwrap()
    );
    assert!(parallel_graph.subscribers_of(left).unwrap().is_empty());
    assert_eq!(
        parallel_graph.subscribers_of(right).unwrap().len(),
        targets.len()
    );
    assert!(parallel_report
        .stages
        .iter()
        .any(|stage| { stage.apply_mode == Some(ParallelApplyMode::GroupedConcurrentApply) }));
    assert_eq!(serial_report.tasks_executed, parallel_report.tasks_executed);
}

#[test]
fn harness_parity_holds_for_branchy_partitioned_output_identity_graph() {
    let mut scenario = SignalScenario::new("adversarial-branchy-parity");
    let source = scenario.build_node("source", |graph| graph.node().output_identity().build());
    let left = scenario.build_node("left", |graph| graph.node().partitioned_output().build());
    let right = scenario.build_node("right", |graph| graph.node().partitioned_output().build());
    let _dependent = scenario.node("dependent");
    scenario
        .partition_detail_dependency("left", "source", ASPECT_A, "wing", "rib-a")
        .unwrap();
    scenario
        .partition_detail_dependency("right", "source", ASPECT_A, "wing", "rib-b")
        .unwrap();
    scenario.dependency("dependent", "left", ASPECT_A).unwrap();
    scenario.dependency("dependent", "right", ASPECT_A).unwrap();

    let fixture = scenario
        .observe("dependent")
        .with_evaluator(move |node: NodeId, view: &ExecutionReadView<'_>| {
            let result = if node == source {
                view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("wing-artifact")
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-a")),
                )
            } else if node == left || node == right {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            } else {
                let left_v = view.read_aspect_version(left, ASPECT_A)?;
                let right_v = view.read_aspect_version(right, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(
                        ASPECT_A,
                        left_v.get(ASPECT_A) + right_v.get(ASPECT_A),
                    )]),
                ))
            };
            Ok(result)
        })
        .fixture()
        .unwrap();

    let request = ExecutionRequest::target("observe-dependent", "dependent".to_string());
    let report = signal_parity_suite(
        fixture,
        request,
        SignalProfileCatalog::serial("serial-baseline"),
    )
    .comparison_profile(ComparisonProfile {
        mode: ComparisonMode::Semantic,
        include_extensions: false,
        numeric_tolerance: None,
    })
    .candidates([
        SignalProfileCatalog::staged_parallel("staged-parallel-candidate"),
        SignalProfileCatalog::full_parallel("full-parallel-candidate"),
    ])
    .compare()
    .unwrap();

    assert!(report.matched);
}

#[test]
#[ignore = "stress coverage for wide-graph full-parallel parity loops"]
fn stress_repeated_parallel_parity_on_wide_branch_graph() {
    let mut scenario = SignalScenario::new("stress-parity");
    let source = scenario.node("source");
    let mids: Vec<_> = (0..24)
        .map(|index| scenario.node(format!("mid-{index}")))
        .collect();
    let _target = scenario.node("target");
    for (index, _) in mids.iter().enumerate() {
        scenario
            .dependency(&format!("mid-{index}"), "source", ASPECT_A)
            .unwrap();
        scenario
            .dependency("target", &format!("mid-{index}"), ASPECT_A)
            .unwrap();
    }

    let fixture = scenario
        .observe("target")
        .with_evaluator(move |node: NodeId, view: &ExecutionReadView<'_>| {
            let result = if node == source {
                view.finish(version_ab(1, 0))
            } else if mids.contains(&node) {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            } else {
                let mut total = 0_u64;
                for &mid in &mids {
                    total += view.read_aspect_version(mid, ASPECT_A)?.get(ASPECT_A);
                }
                view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(ASPECT_A, total)]),
                ))
            };
            Ok(result)
        })
        .fixture()
        .unwrap();

    let request = ExecutionRequest::target("target", "target".to_string());
    for _ in 0..25 {
        let report = signal_parity_suite(
            fixture.clone(),
            request.clone(),
            SignalProfileCatalog::serial("serial-baseline"),
        )
        .comparison_profile(ComparisonProfile {
            mode: ComparisonMode::Semantic,
            include_extensions: false,
            numeric_tolerance: None,
        })
        .candidates([
            SignalProfileCatalog::staged_parallel("staged-parallel-candidate"),
            SignalProfileCatalog::full_parallel("full-parallel-candidate"),
        ])
        .compare()
        .unwrap();
        assert!(report.matched);
    }
}
