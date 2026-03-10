use crate::facade::*;
use crate::tests::support::*;
use std::hint::black_box;
use std::mem::size_of;

#[test]
fn chain_1000_minimal_recomputation() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<crate::facade::NodeId> = Vec::with_capacity(1000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..1000 {
        let node = graph.create_node();
        graph.add_dependency(node, chain[i - 1], ASPECT_B).unwrap();
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
fn push_perf_10k_nodes() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<crate::facade::NodeId> = Vec::with_capacity(10_000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..10_000 {
        let node = graph.create_node();
        graph.add_dependency(node, chain[i - 1], ASPECT_B).unwrap();
        chain.push(node);
    }

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(0, 1));
    for node in &chain {
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }

    let start = std::time::Instant::now();
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
fn ondemand_defer_perf_10k_nodes() {
    let mut graph = SignalGraph::new();
    let mut nodes: Vec<NodeId> = Vec::with_capacity(10_000);

    for _ in 0..10_000 {
        nodes.push(
            graph
                .node()
                .condition(EvaluationCondition::OnDemand)
                .build(),
        );
    }

    let start = std::time::Instant::now();
    for node in &nodes {
        let mut compute = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 1));
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }
    let elapsed = start.elapsed();

    let max_eval_ms: u128 = 500;
    assert!(
        elapsed.as_millis() < max_eval_ms,
        "On-demand defer path took {}ms, expected < {}ms",
        elapsed.as_millis(),
        max_eval_ms
    );
    assert_eq!(graph.telemetry().ondemand_deferred_count, 10_000);
}

#[test]
#[ignore = "layout report for slot occupancy experiments"]
fn slot_layout_report() {
    use crate::data::node::NodeEntry;

    #[allow(dead_code)]
    struct CurrentSlot {
        data: Option<NodeEntry>,
        generation: u32,
    }

    #[allow(dead_code)]
    struct BoxedSlot {
        data: Option<Box<NodeEntry>>,
        generation: u32,
    }

    #[allow(dead_code)]
    struct SplitOccupancySlot {
        generation: u32,
        occupied: bool,
        data: Box<NodeEntry>,
    }

    eprintln!("slot_size_current={}", size_of::<CurrentSlot>());
    eprintln!("slot_size_boxed={}", size_of::<BoxedSlot>());
    eprintln!("slot_size_split={}", size_of::<SplitOccupancySlot>());

    let count = 50_000usize;
    let current = (0..count)
        .map(|_| CurrentSlot {
            data: None,
            generation: 0,
        })
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();
    let occupied = current
        .iter()
        .filter(|slot: &&CurrentSlot| black_box(slot.data.is_some()))
        .count();
    eprintln!(
        "slot_scan_current_nanos={} occupied={occupied}",
        start.elapsed().as_nanos()
    );
}
