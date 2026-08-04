use crate::facade::*;

use crate::easy::ReactiveGraph;

use super::source_corpus::{
    HOT_EASY_OBSERVATION_SOURCE, HOT_RUNTIME_OBSERVATION_SOURCE,
    HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE,
};

#[test]
fn easy_mode_failed_batch_restores_downstream_invalidation_state() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);

    assert_eq!(graph.get(doubled), 4);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(source, 9)?;
        reactive.try_get(doubled)?;
        Err(SignalError::invalid_input(
            "force rollback after dirty propagation",
        ))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(source), 2);
    assert_eq!(graph.get(doubled), 4);
}

#[test]
fn observation_hot_paths_use_index_iteration_without_snapshot_allocations() {
    assert!(
        HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE.contains("for_each_matching_observer_for_node("),
        "transaction observation staging should iterate the node index directly instead of materializing temporary observer-id snapshots"
    );
    assert!(
        !HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE.contains(
            "matching_observers_for_node(node)"
        ),
        "transaction observation staging should not allocate matching-observer snapshots on the hot path"
    );
    assert!(
        HOT_EASY_OBSERVATION_SOURCE.contains("for_each_matching_observer_for_node("),
        "easy observation delivery should iterate the node index directly instead of materializing temporary observer-id snapshots"
    );
    assert!(
        HOT_EASY_OBSERVATION_SOURCE.contains("has_matching_observers_for_node("),
        "easy observation traversal should use the zero-allocation node-watch check instead of snapshotting observer ids"
    );
    assert!(
        HOT_EASY_OBSERVATION_SOURCE.contains(".filter(|node| app.computed.contains_key(node))"),
        "easy observation recompute prepass should narrow itself to impacted computed nodes instead of cloning the whole impacted set"
    );
}

#[test]
fn committed_observation_delivery_avoids_recloning_boundary_summaries() {
    assert!(
        HOT_RUNTIME_OBSERVATION_SOURCE.contains("for delivery in deliveries"),
        "committed observation delivery should stream committed packets directly instead of rebuilding a second summary vector"
    );
    assert!(
        !HOT_RUNTIME_OBSERVATION_SOURCE.contains(".collect::<Vec<_>>();\r\n        self.deliver_boundary_summaries(graph, &summaries)")
            && !HOT_RUNTIME_OBSERVATION_SOURCE.contains(".collect::<Vec<_>>();\n        self.deliver_boundary_summaries(graph, &summaries)"),
        "committed observation delivery should not clone whole boundary summaries into a second vector before listener dispatch"
    );
}

#[test]
fn easy_mode_watch_and_effect_use_observation_substrate() {
    let mut graph = ReactiveGraph::new();
    let count = graph.input(1_i32);
    let doubled = graph.computed({
        let count = count;
        move |context| context.get(count) * 2
    });

    let watch_hits = std::sync::Arc::new(std::sync::Mutex::new(Vec::<usize>::new()));
    let watch_hits_clone = std::sync::Arc::clone(&watch_hits);
    let effect_hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let effect_hits_clone = std::sync::Arc::clone(&effect_hits);

    let watch_handle = graph.watch(doubled, move |notice| {
        watch_hits_clone
            .lock()
            .expect("easy watch mutex poisoned")
            .push(notice.matched_nodes().len());
        assert!(notice.trigger_matched());
        assert!(notice.meaningful_change());
    });
    let effect_handle = graph.effect(doubled, move || {
        *effect_hits_clone
            .lock()
            .expect("easy effect mutex poisoned") += 1;
    });

    graph.set(count, 2);
    assert_eq!(graph.get(doubled), 4);

    assert_eq!(
        watch_hits
            .lock()
            .expect("easy watch mutex poisoned")
            .as_slice(),
        &[1]
    );
    assert_eq!(*effect_hits.lock().expect("easy effect mutex poisoned"), 1);

    assert!(graph.unobserve(watch_handle));
    assert!(graph.unobserve(effect_handle));
}

#[test]
fn easy_mode_meaningful_change_watch_suppresses_recomputed_but_unchanged_values() {
    let mut graph = ReactiveGraph::new();
    let count = graph.input(1_i32);
    let parity = graph.computed({
        let count = count;
        move |context| context.get(count) % 2
    });

    let watch_hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let effect_hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let watch_hits_clone = std::sync::Arc::clone(&watch_hits);
    let effect_hits_clone = std::sync::Arc::clone(&effect_hits);

    let watch_handle = graph.watch(parity, move |notice| {
        *watch_hits_clone
            .lock()
            .expect("easy meaningful-change watch mutex poisoned") += 1;
        assert!(notice.recomputed());
        assert!(notice.meaningful_change());
    });
    let effect_handle = graph.effect(parity, move || {
        *effect_hits_clone
            .lock()
            .expect("easy meaningful-change effect mutex poisoned") += 1;
    });

    graph.set(count, 3);
    assert_eq!(graph.get(parity), 1);

    assert_eq!(
        *watch_hits
            .lock()
            .expect("easy meaningful-change watch mutex poisoned"),
        0
    );
    assert_eq!(
        *effect_hits
            .lock()
            .expect("easy meaningful-change effect mutex poisoned"),
        0
    );

    assert!(graph.unobserve(watch_handle));
    assert!(graph.unobserve(effect_handle));
}

#[test]
fn easy_mode_unobserve_stops_future_notifications() {
    let mut graph = ReactiveGraph::new();
    let count = graph.input(1_i32);
    let doubled = graph.computed({
        let count = count;
        move |context| context.get(count) * 2
    });

    let hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let hits_clone = std::sync::Arc::clone(&hits);
    let handle = graph.watch(doubled, move |_notice| {
        *hits_clone
            .lock()
            .expect("easy unobserve watch mutex poisoned") += 1;
    });

    graph.set(count, 2);
    assert_eq!(
        *hits.lock().expect("easy unobserve watch mutex poisoned"),
        1
    );

    assert!(graph.unobserve(handle));
    graph.set(count, 3);
    assert_eq!(
        *hits.lock().expect("easy unobserve watch mutex poisoned"),
        1
    );
}
