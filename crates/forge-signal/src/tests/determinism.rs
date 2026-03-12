use crate::facade::*;
use crate::tests::support::*;

#[test]
fn kv63_circular_reference_detected() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();

    graph.add_dependency(b, a, ASPECT_B).unwrap();
    graph.add_dependency(a, b, ASPECT_B).unwrap();

    let result = mark_dirty(&mut graph, a, ASPECT_B);
    assert!(
        result.is_err(),
        "Circular reference A↔B should produce an error"
    );

    match result.unwrap_err() {
        SignalError::CycleDetected { path } => {
            assert!(
                path.contains(&a) && path.contains(&b),
                "cycle path should retain both nodes, got: {path:?}"
            );
        }
        err => panic!("expected typed cycle error, got: {err}"),
    }
}

#[test]
fn kv64_parallel_branches_deterministic() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();

    let mut branches: Vec<Vec<NodeId>> = Vec::new();
    for _ in 0..5 {
        let mut branch = Vec::new();
        let first = graph.node().build();
        graph.add_dependency(first, root, ASPECT_B).unwrap();
        branch.push(first);

        for j in 1..10 {
            let node = graph.node().build();
            graph.add_dependency(node, branch[j - 1], ASPECT_B).unwrap();
            branch.push(node);
        }
        branches.push(branch);
    }

    let mut compute_counter = 0u64;
    let mut compute = |_id, _g: &SignalGraph| {
        compute_counter += 1;
        Ok(version_ab(0, compute_counter))
    };

    evaluate(&mut graph, root, &mut compute).unwrap();
    for branch in &branches {
        for node in branch {
            evaluate(&mut graph, *node, &mut compute).unwrap();
        }
    }

    mark_dirty(&mut graph, root, ASPECT_B).unwrap();

    let mut recompute_counter = 0u64;
    let mut recompute = |_id, _g: &SignalGraph| {
        recompute_counter += 1;
        Ok(version_ab(0, 100 + recompute_counter))
    };

    evaluate(&mut graph, root, &mut recompute).unwrap();
    for branch in &branches {
        for node in branch {
            evaluate(&mut graph, *node, &mut recompute).unwrap();
        }
    }

    assert_eq!(
        recompute_counter, 51,
        "All 51 nodes (root + 5×10) should recompute after root dirty"
    );
}