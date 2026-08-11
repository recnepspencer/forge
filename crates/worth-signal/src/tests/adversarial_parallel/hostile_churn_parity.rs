use crate::facade::{AspectVersion, ChangedRegion, EvaluationContext, NodeEvaluationResult};
use crate::presentation::harness::{signal_parity_suite, SignalProfileCatalog, SignalScenario};
use crate::tests::support::{version_ab, ASPECT_A};
use worth_harness::facade::{ComparisonMode, ComparisonProfile, ExecutionRequest};

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
        .with_evaluator(move |ctx: &mut EvaluationContext<'_, ()>| {
            let node = ctx.node();
            let result = if node == source {
                ctx.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("wing-artifact")
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-a")),
                )
            } else if node == left || node == right {
                let version = ctx.read_aspect_version(source, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(version))
            } else {
                let left_v = ctx.read_aspect_version(left, ASPECT_A)?;
                let right_v = ctx.read_aspect_version(right, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(
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
        .with_evaluator(move |ctx: &mut EvaluationContext<'_, ()>| {
            let node = ctx.node();
            let result = if node == source {
                ctx.finish(version_ab(1, 0))
            } else if mids.contains(&node) {
                let version = ctx.read_aspect_version(source, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(version))
            } else {
                let mut total = 0_u64;
                for &mid in &mids {
                    total += ctx.read_aspect_version(mid, ASPECT_A)?.get(ASPECT_A);
                }
                ctx.finish(NodeEvaluationResult::from_version(
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
