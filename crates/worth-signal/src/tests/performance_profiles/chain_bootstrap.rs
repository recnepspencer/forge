use std::sync::Once;
use std::time::Instant;

use serde_json::{json, Value};

use crate::facade::{mark_dirty, EvaluationRequestMode};
use crate::tests::performance_support::{
    build_chain_graph, capture_and_certify_perf_samples, with_perf_topology_asserts_disabled,
    PerfMeasurement, PerfTimingPolicy,
};
use crate::tests::support::version_ab;

use super::{graph_metrics_delta, hot_family_contract, ZERO_BROAD_ENTRY_ACCESS};

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_chain_10k_bootstrap_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        static CHAIN_10K_WARMUP: Once = Once::new();
        CHAIN_10K_WARMUP.call_once(|| {
            let (mut warm_graph, warm_chain) = build_chain_graph(10_000);
            let warm_plan = warm_graph
                .build_evaluation_plan(&warm_chain, EvaluationRequestMode::Default)
                .unwrap();
            warm_graph
                .execute_prepared_plan(&warm_plan, &(), &|_ctx| Ok(version_ab(0, 1)))
                .unwrap();
        });

        capture_and_certify_perf_samples(
            hot_family_contract(
                "chain_10k_bootstrap",
                "balanced",
                PerfTimingPolicy::MedianOnly,
                &[
                    "build_nanos",
                    "bootstrap_plan_nanos",
                    "bootstrap_execute_nanos",
                ],
                ZERO_BROAD_ENTRY_ACCESS,
            ),
            || {
                let build_start = Instant::now();
                let (mut graph, chain) = build_chain_graph(10_000);
                let build_nanos = build_start.elapsed().as_nanos();

                graph.reset_telemetry();
                let before = graph.observe().metrics();
                let plan_start = Instant::now();
                let plan = graph
                    .build_evaluation_plan(&chain, EvaluationRequestMode::Default)
                    .unwrap();
                let bootstrap_plan_nanos = plan_start.elapsed().as_nanos();

                let execute_start = Instant::now();
                graph
                    .execute_prepared_plan(&plan, &(), &|_ctx| Ok(version_ab(0, 1)))
                    .unwrap();
                let bootstrap_execute_nanos = execute_start.elapsed().as_nanos();

                let push_start = Instant::now();
                mark_dirty(&mut graph, chain[0], crate::tests::support::ASPECT_B).unwrap();
                let push_nanos = push_start.elapsed().as_nanos();
                let after = graph.observe().metrics();

                let mut metrics = graph_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("build_nanos".into(), json!(build_nanos));
                    map.insert("bootstrap_plan_nanos".into(), json!(bootstrap_plan_nanos));
                    map.insert(
                        "bootstrap_execute_nanos".into(),
                        json!(bootstrap_execute_nanos),
                    );
                    map.insert("push_nanos".into(), json!(push_nanos));
                    map.insert(
                        "plans_built".into(),
                        json!(graph.telemetry().planner.plans_built),
                    );
                    map.insert(
                        "tasks_scheduled".into(),
                        json!(graph.telemetry().planner.tasks_scheduled),
                    );
                    map.insert(
                        "stage_execution_nanos".into(),
                        json!(graph.telemetry().execution.stage_execution_nanos),
                    );
                }

                PerfMeasurement::new(
                    (build_nanos + bootstrap_plan_nanos + bootstrap_execute_nanos + push_nanos)
                        / 1_000,
                    metrics,
                )
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}
