use crate::facade::*;
use crate::tests::support::*;
use std::hint::black_box;
use std::mem::size_of;
use std::time::Instant;

fn build_chain_graph(count: usize) -> (SignalGraph, Vec<NodeId>) {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<NodeId> = Vec::with_capacity(count);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..count {
        let node = graph.create_node();
        graph
            .append_dependency(node, chain[i - 1], ASPECT_B)
            .unwrap();
        chain.push(node);
    }

    (graph, chain)
}

fn with_perf_topology_asserts_disabled<T>(run: impl FnOnce() -> T) -> T {
    let previous = std::env::var_os("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS");
    std::env::set_var("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS", "1");
    let result = run();
    match previous {
        Some(value) => std::env::set_var("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS", value),
        None => std::env::remove_var("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS"),
    }
    result
}

fn evaluate_chain(graph: &mut SignalGraph, chain: &[NodeId]) {
    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(0, 1));
    for node in chain {
        evaluate(&mut *graph, *node, &mut compute).unwrap();
    }
}

fn evaluate_chain_bulk(graph: &mut SignalGraph, chain: &[NodeId]) {
    let plan = graph
        .build_evaluation_plan(chain, EvaluationRequestMode::Default)
        .unwrap();
    graph
        .execute_prepared_plan(&plan, &(), &|_ctx| Ok(version_ab(0, 1)))
        .unwrap();
}

fn evaluate_chain_bulk_profile(
    graph: &mut SignalGraph,
    chain: &[NodeId],
) -> (u128, u128, u64, u64, u128) {
    graph.reset_telemetry();
    let plan_start = Instant::now();
    let plan = graph
        .build_evaluation_plan(chain, EvaluationRequestMode::Default)
        .unwrap();
    let plan_elapsed_ms = plan_start.elapsed().as_millis();
    let execute_start = Instant::now();
    graph
        .execute_prepared_plan(&plan, &(), &|_ctx| Ok(version_ab(0, 1)))
        .unwrap();
    let execute_elapsed_ms = execute_start.elapsed().as_millis();
    let telemetry = *graph.telemetry();
    (
        plan_elapsed_ms,
        execute_elapsed_ms,
        telemetry.planner.plans_built,
        telemetry.planner.tasks_scheduled,
        telemetry.execution.stage_execution_nanos / 1_000_000,
    )
}

fn warm_up_bulk_evaluation_path() {
    let (mut graph, chain) = with_perf_topology_asserts_disabled(|| build_chain_graph(8));
    let _ = evaluate_chain_bulk_profile(&mut graph, &chain);
}

fn median_u128(values: &mut [u128]) -> u128 {
    values.sort_unstable();
    values[values.len() / 2]
}

#[test]
fn chain_1000_minimal_recomputation() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<NodeId> = Vec::with_capacity(1000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..1000 {
        let node = graph.create_node();
        graph
            .append_dependency(node, chain[i - 1], ASPECT_B)
            .unwrap();
        chain.push(node);
    }

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(0, 1));
    for node in &chain {
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }

    mark_dirty(&mut graph, chain[0], ASPECT_B).unwrap();

    let state_first = graph.get_state(chain[0]).unwrap();
    assert_eq!(state_first, NodeState::Dirty);

    let state_second = graph.get_state(chain[1]).unwrap();
    assert_eq!(state_second, NodeState::Dirty);

    let state_last = graph.get_state(chain[999]).unwrap();
    assert_eq!(state_last, NodeState::MaybeStale);
}

#[test]
#[ignore = "machine-sensitive structured performance profile"]
fn push_perf_10k_nodes() {
    let (mut graph, chain) = with_perf_topology_asserts_disabled(|| build_chain_graph(10_000));
    evaluate_chain_bulk(&mut graph, &chain);

    let start = Instant::now();
    mark_dirty(&mut graph, chain[0], ASPECT_B).unwrap();
    let elapsed = start.elapsed();

    let max_push_ms: u128 = 500;
    assert!(
        elapsed.as_millis() < max_push_ms,
        "Push propagation took {}ms, expected < {}ms",
        elapsed.as_millis(),
        max_push_ms
    );
}

#[test]
#[ignore = "machine-sensitive structured performance profile"]
fn push_build_perf_10k_nodes() {
    let (elapsed, chain_len) = with_perf_topology_asserts_disabled(|| {
        let start = Instant::now();
        let (_graph, chain) = build_chain_graph(10_000);
        (start.elapsed(), chain.len())
    });

    let max_build_ms: u128 = 1_500;
    assert_eq!(chain_len, 10_000);
    assert!(
        elapsed.as_millis() < max_build_ms,
        "10k-node graph build took {}ms, expected < {}ms",
        elapsed.as_millis(),
        max_build_ms
    );
}

#[test]
#[ignore = "diagnostic-only ad hoc profile; authoritative 10k bootstrap cert lives in performance_profiles"]
fn push_initial_eval_perf_10k_nodes() {
    warm_up_bulk_evaluation_path();
    let mut elapsed_samples = Vec::with_capacity(5);
    let mut plan_samples = Vec::with_capacity(5);
    let mut execute_samples = Vec::with_capacity(5);
    let mut stage_exec_samples = Vec::with_capacity(5);
    let mut plans_built = 0;
    let mut tasks_scheduled = 0;

    for _ in 0..5 {
        let (mut graph, chain) = with_perf_topology_asserts_disabled(|| build_chain_graph(10_000));
        let (plan_ms, execute_ms, sample_plans_built, sample_tasks_scheduled, stage_exec_ms) =
            evaluate_chain_bulk_profile(&mut graph, &chain);
        elapsed_samples.push(plan_ms + execute_ms);
        plan_samples.push(plan_ms);
        execute_samples.push(execute_ms);
        stage_exec_samples.push(stage_exec_ms);
        plans_built = sample_plans_built;
        tasks_scheduled = sample_tasks_scheduled;
    }

    let elapsed_ms = median_u128(&mut elapsed_samples);
    let plan_ms = median_u128(&mut plan_samples);
    let execute_ms = median_u128(&mut execute_samples);
    let stage_exec_ms = median_u128(&mut stage_exec_samples);

    eprintln!(
        "Initial 10k-node evaluation median={}ms plan={}ms execute={}ms plans_built={} tasks_scheduled={} stage_exec={}ms samples={:?}",
        elapsed_ms,
        plan_ms,
        execute_ms,
        plans_built,
        tasks_scheduled,
        stage_exec_ms,
        elapsed_samples,
    );
}

#[test]
#[ignore = "diagnostic comparison for 10k bootstrap path shape"]
fn push_initial_eval_compare_10k_nodes() {
    let count = 10_000;
    let (single_elapsed_ms, single_plans_built, single_tasks_scheduled, single_stage_exec_ms) =
        with_perf_topology_asserts_disabled(|| {
            let (mut graph, chain) = build_chain_graph(count);
            graph.reset_telemetry();
            let start = Instant::now();
            evaluate_chain(&mut graph, &chain);
            let elapsed = start.elapsed().as_millis();
            let telemetry = *graph.telemetry();
            (
                elapsed,
                telemetry.planner.plans_built,
                telemetry.planner.tasks_scheduled,
                telemetry.execution.stage_execution_nanos / 1_000_000,
            )
        });

    let (bulk_elapsed_ms, bulk_plans_built, bulk_tasks_scheduled, bulk_stage_exec_ms) =
        with_perf_topology_asserts_disabled(|| {
            let (mut graph, chain) = build_chain_graph(count);
            let (plan_ms, execute_ms, plans_built, tasks_scheduled, stage_exec_ms) =
                evaluate_chain_bulk_profile(&mut graph, &chain);
            (
                plan_ms + execute_ms,
                plans_built,
                tasks_scheduled,
                stage_exec_ms,
            )
        });

    eprintln!(
        "single_node_eval: elapsed_ms={single_elapsed_ms} plans_built={single_plans_built} tasks_scheduled={single_tasks_scheduled} stage_exec_ms={single_stage_exec_ms}"
    );
    eprintln!(
        "bulk_eval: elapsed_ms={bulk_elapsed_ms} plans_built={bulk_plans_built} tasks_scheduled={bulk_tasks_scheduled} stage_exec_ms={bulk_stage_exec_ms}"
    );
}

#[test]
#[ignore = "diagnostic comparison for repeated 10k bulk bootstrap in one process"]
fn push_initial_eval_bulk_twice_10k_nodes() {
    let count = 10_000;
    let (first_elapsed_ms, first_plans_built, first_tasks_scheduled, first_stage_exec_ms) =
        with_perf_topology_asserts_disabled(|| {
            let (mut graph, chain) = build_chain_graph(count);
            let (plan_ms, execute_ms, plans_built, tasks_scheduled, stage_exec_ms) =
                evaluate_chain_bulk_profile(&mut graph, &chain);
            (
                plan_ms + execute_ms,
                plans_built,
                tasks_scheduled,
                stage_exec_ms,
            )
        });

    let (second_elapsed_ms, second_plans_built, second_tasks_scheduled, second_stage_exec_ms) =
        with_perf_topology_asserts_disabled(|| {
            let (mut graph, chain) = build_chain_graph(count);
            let (plan_ms, execute_ms, plans_built, tasks_scheduled, stage_exec_ms) =
                evaluate_chain_bulk_profile(&mut graph, &chain);
            (
                plan_ms + execute_ms,
                plans_built,
                tasks_scheduled,
                stage_exec_ms,
            )
        });

    eprintln!(
        "bulk_first: elapsed_ms={first_elapsed_ms} plans_built={first_plans_built} tasks_scheduled={first_tasks_scheduled} stage_exec_ms={first_stage_exec_ms}"
    );
    eprintln!(
        "bulk_second: elapsed_ms={second_elapsed_ms} plans_built={second_plans_built} tasks_scheduled={second_tasks_scheduled} stage_exec_ms={second_stage_exec_ms}"
    );
}

#[test]
#[ignore = "machine-sensitive defer budget; tracked in performance_profiles baseline suite"]
fn ondemand_defer_perf_10k_nodes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let mut nodes: Vec<NodeId> = Vec::with_capacity(10_000);
    {
        let mut graph = runtime.graph_mut();
        for _ in 0..10_000 {
            nodes.push(
                graph
                    .node()
                    .condition(EvaluationCondition::OnDemand)
                    .build(),
            );
        }
    }

    let start = std::time::Instant::now();
    let plan = runtime
        .build_evaluation_plan(&nodes, EvaluationRequestMode::Default)
        .unwrap();
    assert_eq!(plan.summary.task_count, 10_000);
    runtime
        .execute_prepared_plan(&plan, &(), &|_ctx| Ok(version_ab(0, 1)))
        .unwrap();
    let elapsed = start.elapsed();

    let max_eval_ms: u128 = 500;
    assert!(
        elapsed.as_millis() < max_eval_ms,
        "On-demand defer path took {}ms, expected < {}ms",
        elapsed.as_millis(),
        max_eval_ms
    );
}

#[test]
#[ignore = "layout report for slot occupancy experiments"]
fn slot_layout_report() {
    use crate::data::node::NodeEntry;

    struct CurrentSlot {
        data: Option<NodeEntry>,
        generation: u32,
    }

    struct BoxedSlot {
        data: Option<Box<NodeEntry>>,
        generation: u32,
    }

    struct SplitOccupancySlot {
        generation: u32,
        occupied: bool,
        data: Box<NodeEntry>,
    }

    eprintln!("slot_size_current={}", size_of::<CurrentSlot>());
    eprintln!("slot_size_boxed={}", size_of::<BoxedSlot>());
    eprintln!("slot_size_split={}", size_of::<SplitOccupancySlot>());
    let split_probe = SplitOccupancySlot {
        generation: 7,
        occupied: true,
        data: Box::new(NodeEntry::new()),
    };
    black_box(split_probe.generation);
    black_box(split_probe.occupied);
    black_box(split_probe.data.is_tombstoned());

    let count = 200_000usize;
    let current_vacant = (0..count)
        .map(|_| CurrentSlot {
            data: None,
            generation: 0,
        })
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();
    let occupied = current_vacant
        .iter()
        .filter(|slot: &&CurrentSlot| black_box(slot.data.is_some()))
        .count();
    eprintln!(
        "slot_scan_current_vacant_nanos={} occupied={occupied}",
        start.elapsed().as_nanos()
    );

    let boxed_vacant = (0..count)
        .map(|_| BoxedSlot {
            data: None,
            generation: 0,
        })
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();
    let occupied = boxed_vacant
        .iter()
        .filter(|slot: &&BoxedSlot| black_box(slot.data.is_some()))
        .count();
    eprintln!(
        "slot_scan_boxed_vacant_nanos={} occupied={occupied}",
        start.elapsed().as_nanos()
    );

    let current_occupied = (0..count)
        .map(|i| {
            let mut entry = NodeEntry::new();
            if i % 2 == 0 {
                entry.set_tombstoned(true);
            }
            CurrentSlot {
                data: Some(entry),
                generation: i as u32,
            }
        })
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();
    let dirty_or_tombstoned = current_occupied
        .iter()
        .filter(|slot| {
            let entry = black_box(slot.data.as_ref().expect("occupied slot"));
            entry.is_tombstoned() || matches!(entry.get_state(), NodeState::Dirty)
        })
        .count();
    eprintln!(
        "slot_scan_current_occupied_nanos={} marked={dirty_or_tombstoned}",
        start.elapsed().as_nanos()
    );

    let boxed_occupied = (0..count)
        .map(|i| {
            let mut entry = NodeEntry::new();
            if i % 2 == 0 {
                entry.set_tombstoned(true);
            }
            BoxedSlot {
                data: Some(Box::new(entry)),
                generation: i as u32,
            }
        })
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();
    let dirty_or_tombstoned = boxed_occupied
        .iter()
        .filter(|slot| {
            let entry = black_box(slot.data.as_deref().expect("occupied slot"));
            entry.is_tombstoned() || matches!(entry.get_state(), NodeState::Dirty)
        })
        .count();
    eprintln!(
        "slot_scan_boxed_occupied_nanos={} marked={dirty_or_tombstoned}",
        start.elapsed().as_nanos()
    );

    let churn_rounds = 10usize;
    let churn_width = 50_000usize;
    let mut current_churn = Vec::with_capacity(churn_width);
    let start = std::time::Instant::now();
    for round in 0..churn_rounds {
        current_churn.clear();
        current_churn.extend((0..churn_width).map(|i| CurrentSlot {
            data: Some(NodeEntry::new()),
            generation: (round * churn_width + i) as u32,
        }));
        for slot in &mut current_churn {
            slot.data = None;
            slot.generation += 1;
        }
        black_box(&current_churn);
    }
    eprintln!(
        "slot_churn_current_nanos={} rounds={churn_rounds} width={churn_width}",
        start.elapsed().as_nanos()
    );

    let mut boxed_churn = Vec::with_capacity(churn_width);
    let start = std::time::Instant::now();
    for round in 0..churn_rounds {
        boxed_churn.clear();
        boxed_churn.extend((0..churn_width).map(|i| BoxedSlot {
            data: Some(Box::new(NodeEntry::new())),
            generation: (round * churn_width + i) as u32,
        }));
        for slot in &mut boxed_churn {
            slot.data = None;
            slot.generation += 1;
        }
        black_box(&boxed_churn);
    }
    eprintln!(
        "slot_churn_boxed_nanos={} rounds={churn_rounds} width={churn_width}",
        start.elapsed().as_nanos()
    );
}
