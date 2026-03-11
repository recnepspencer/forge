use std::sync::atomic::{AtomicU32, Ordering};

use crate::facade::{KeyedComputation, MemoizedResultOrigin, NodeEvaluationResult, StageExecutor};

use super::aspects::{ALERT, PRICE, RISK};
use super::scales::FintechScale;
use super::scenarios::setup_seeded_world;

#[test]
fn fintech_keyed_audit_cache_reuses_stable_memo_entries_without_cross_shape_corruption() {
    let mut world = setup_seeded_world();
    world.assert_shape(FintechScale::smoke());

    world
        .read_top_desk_with_executor(StageExecutor::Serial)
        .unwrap();
    world
        .read_top_scenario_with_executor(StageExecutor::Serial)
        .unwrap();

    let family = world
        .runtime
        .register_computation_family("fintech-audit-cache");
    let cache = world.runtime.keyed_node(&family, "desk-0");
    let baseline = KeyedComputation::new(family.clone(), "desk-0").with_memo_key("baseline");
    let stressed = KeyedComputation::new(family.clone(), "desk-0").with_memo_key("stress");
    let compute_calls = AtomicU32::new(0);

    let top_desk = world.top_desk();
    let top_scenario = world.top_scenario();
    let primary_market = world.primary_market_source();
    let evaluation = world.evaluation_shape();
    let precompute = evaluation.precompute();

    let run_cache = |world: &mut super::fixture::FintechWorld,
                     memo: &KeyedComputation,
                     compute_calls: &AtomicU32|
     -> Result<(), crate::facade::SignalError> {
        world.runtime.transaction(&mut (), |tx| {
            tx.evaluate_keyed(cache, memo, &|node, view| {
                if node == cache {
                    compute_calls.fetch_add(1, Ordering::Relaxed);
                    let desk = view.read_aspect_version(top_desk, RISK)?.get(RISK);
                    let scenario = view.read_aspect_version(top_scenario, RISK)?.get(RISK);
                    let market = view.read_aspect_version(primary_market, PRICE)?.get(PRICE);
                    let total = desk + scenario + market;
                    return Ok(view.finish(
                        NodeEvaluationResult::from_version(
                            crate::facade::AspectVersion::from_updates([
                                (RISK, total),
                                (ALERT, u64::from(total > 40_000)),
                            ]),
                        )
                        .with_output_identity(format!(
                            "audit-cache-{}",
                            memo.memo_key.as_ref().unwrap().as_str()
                        )),
                    ));
                }
                precompute(node, view)
            })?;
            Ok(())
        })?;
        Ok(())
    };

    run_cache(&mut world, &baseline, &compute_calls).unwrap();
    let baseline_version = world
        .runtime
        .graph()
        .get_entry(cache)
        .unwrap()
        .get_aspect_version();

    world
        .bump_primary_market(8, 3, 2, 1, StageExecutor::Serial)
        .unwrap();
    run_cache(&mut world, &stressed, &compute_calls).unwrap();
    let stressed_version = world
        .runtime
        .graph()
        .get_entry(cache)
        .unwrap()
        .get_aspect_version();
    assert_ne!(stressed_version, baseline_version);

    world
        .runtime
        .transaction(&mut (), |tx| {
            tx.mark_dirty(cache, RISK)?;
            Ok(())
        })
        .unwrap();
    run_cache(&mut world, &baseline, &compute_calls).unwrap();

    let final_version = world
        .runtime
        .graph()
        .get_entry(cache)
        .unwrap()
        .get_aspect_version();
    assert_eq!(final_version, baseline_version);
    assert_eq!(compute_calls.load(Ordering::Relaxed), 2);

    let explanation = world.runtime.explain(cache).unwrap();
    assert_eq!(
        explanation.memoized_origin,
        Some(MemoizedResultOrigin::MemoizedFromCache)
    );

    let metrics = world.runtime.metrics();
    assert_eq!(metrics.memoization_misses, 2);
    assert_eq!(metrics.memoization_hits, 1);
}
