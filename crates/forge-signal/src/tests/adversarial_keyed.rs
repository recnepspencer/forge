use std::sync::atomic::{AtomicU32, Ordering};

use crate::facade::*;
use crate::tests::support::{version_ab, ASPECT_A};

#[test]
fn many_families_sharing_same_key_remain_distinct() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let families: Vec<_> = (0..64)
        .map(|index| runtime.register_computation_family(format!("family-{index}")))
        .collect();

    let mut seen = std::collections::BTreeSet::new();
    for family in &families {
        let node = runtime.keyed_node(family, "shared-key");
        assert!(seen.insert(node));
    }
}

#[test]
fn many_keys_in_one_family_reuse_stably_across_large_lookup_sequences() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("airframe");
    let mut first_pass = Vec::new();

    for index in 0..256 {
        first_pass.push(runtime.keyed_node(&family, format!("component-{index}")));
    }

    for index in 0..256 {
        let reused = runtime.keyed_node(&family, format!("component-{index}"));
        assert_eq!(reused, first_pass[index]);
    }
}

#[test]
fn repeated_failed_transactions_do_not_promote_memoized_results() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("fintech-pricing");
    let node = runtime.keyed_node(&family, "book");
    let computation = KeyedComputation::new(family.clone(), "book").with_memo_key("session-a");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    for _ in 0..20 {
        let err = runtime.transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Err(SignalError::invalid_input("rollback memo write"))
        });
        assert!(err.is_err());
    }

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("committed-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.evaluate_keyed(node, &computation, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(99, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let metrics = runtime.metrics();
    assert_eq!(metrics.memoization_misses, 21);
    assert_eq!(metrics.memoization_hits, 1);
    assert_eq!(compute_calls.load(Ordering::Relaxed), 21);
}

#[test]
fn keyed_telemetry_stays_coherent_under_mixed_hit_and_miss_workload() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("kernel");
    let mut runtime_ctx = ();

    for index in 0..24 {
        let node = runtime.keyed_node(&family, format!("part-{index}"));
        let computation =
            KeyedComputation::new(family.clone(), format!("part-{index}")).with_memo_key("shape");

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.evaluate_keyed(node, &computation, &|_id, view| {
                    Ok(view.finish(NodeEvaluationResult::from_version(version_ab(index + 1, 0))))
                })?;
                Ok(())
            })
            .unwrap();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(node, ASPECT_A)?;
                tx.evaluate_keyed(node, &computation, &|_id, view| {
                    Ok(view.finish(NodeEvaluationResult::from_version(version_ab(999, 0))))
                })?;
                Ok(())
            })
            .unwrap();
    }

    let metrics = runtime.metrics();
    assert_eq!(metrics.keyed_evaluation_count, 48);
    assert_eq!(metrics.memoization_misses, 24);
    assert_eq!(metrics.memoization_hits, 24);
}

#[test]
#[ignore = "stress coverage for keyed-cardinality and memoization churn"]
fn stress_thousands_of_keyed_lookups_and_memo_keys() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let families: Vec<_> = (0..64)
        .map(|index| runtime.register_computation_family(format!("family-{index}")))
        .collect();
    let mut runtime_ctx = ();

    for round in 0..8 {
        for family in &families {
            for key_index in 0..128 {
                let key = format!("key-{key_index}");
                let node = runtime.keyed_node(family, key.clone());
                let computation = KeyedComputation::new(family.clone(), key)
                    .with_memo_key(format!("memo-{round}-{key_index}"));
                runtime
                    .transaction(&mut runtime_ctx, |tx| {
                        tx.evaluate_keyed(node, &computation, &|_id, view| {
                            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(
                                round + key_index as u64 + 1,
                                0,
                            ))))
                        })?;
                        Ok(())
                    })
                    .unwrap();
            }
        }
    }

    assert!(runtime.metrics().keyed_evaluation_count > 1_000);
}
