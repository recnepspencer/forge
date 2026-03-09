#![cfg(feature = "parallel")]

use forge_harness::facade::{ComparisonMode, ComparisonProfile, ExecutionRequest};

use crate::facade::*;
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
    .candidates([SignalProfileCatalog::staged_parallel("parallel-candidate")])
    .compare()
    .unwrap();

    assert!(report.matched);
}

#[test]
#[ignore = "stress coverage for wide-graph staged-parallel parity loops"]
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
        .candidates([SignalProfileCatalog::staged_parallel("parallel-candidate")])
        .compare()
        .unwrap();
        assert!(report.matched);
    }
}
