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
    assert_eq!(graph.telemetry().evaluation.ondemand_deferred_count, 10_000);
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
            entry.is_tombstoned() || matches!(entry.get_state(), crate::facade::NodeState::Dirty)
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
            entry.is_tombstoned() || matches!(entry.get_state(), crate::facade::NodeState::Dirty)
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
