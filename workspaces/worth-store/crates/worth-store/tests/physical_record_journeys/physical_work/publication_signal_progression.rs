use std::{
    collections::HashSet,
    sync::mpsc::{self, TryRecvError},
    time::Duration,
};

use worth_signal::facade::{AsyncNodeAdmissionClass, AsyncNodeConditionBlockClass, NodeState};
use worth_store::physical_runtime::{
    PhysicalWorkEffectFate, RecordAppendBatch, RecordPublicationStage,
};
use worth_store_physical_backend::{MediaOperationIdentity, MediaPauseGate};

use super::{configuration, serving_from_initialization};

#[test]
fn root_publication_waits_for_settled_child_signal_completion_without_repeating_media() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    serving_from_initialization(&root).close();
    let (serving, media_gate) = super::fault_fixture::serving_from_open_with_one_write_pause(&root);
    let prepared = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::try_from_iter([b"Signal-gated publication".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let graph_nodes_before = serving
        .physical_signal_observation()
        .unwrap()
        .active_graph_node_count();
    let counters_before = serving.media_counters();
    let replacements_before = counters_before.replacements();
    let retries_before = counters_before.retry_attempts();
    let fault_matches_before = counters_before.fault_matches();
    let causal_overflow_before = serving.physical_work_observer().causal().overflow();
    let (result_tx, result_rx) = mpsc::sync_channel(1);
    let publication = std::thread::spawn(move || {
        let result = prepared.publish();
        let _ = result_tx.send(result);
    });

    if !reaches_within(&media_gate, Duration::from_secs(3)) {
        media_gate.release();
        let _ = publication.join();
        panic!(
            "C5_PREDICATE:signal-readiness the candidate data write must cross admitted Signal readiness and reach the real backend"
        );
    }
    let paused_operation = media_gate
        .reached_context()
        .and_then(|context| context.operation())
        .expect("the paused production write must carry backend operation identity");
    let signal_during_write = serving.physical_signal_observation().unwrap();
    assert_eq!(
        signal_during_write.active_graph_node_count(),
        graph_nodes_before + 2,
        "catalog replacement preparation must precede the first payload write; active locality={}",
        signal_during_write.active_locality_count()
    );
    let dependencies = serving.certification_publication_dependencies();
    let [dependency] = dependencies.as_slice() else {
        panic!(
            "one catalog replacement must remain Signal-blocked behind the physical stages, observed {}",
            dependencies.len()
        );
    };
    let dependency = *dependency;
    assert_eq!(
        dependency.class(),
        AsyncNodeAdmissionClass::BlockedByCondition
    );
    assert_eq!(
        dependency.condition(),
        Some(AsyncNodeConditionBlockClass::DependencyNotReady)
    );
    assert_ne!(
        dependency.node_state(),
        NodeState::Clean,
        "the dependent node must remain dirty before settled child completion"
    );
    assert_eq!(
        dependency.upstream_dependencies(),
        1,
        "C5_PREDICATE:signal-readiness the replacement capability must retain one real Signal dependency edge"
    );
    let signal_gate = serving.certification_pause_physical_signal_after_dequeue();
    media_gate.release();
    if !signal_gate.await_arrivals(1) {
        signal_gate.release();
        let _ = publication.join();
        panic!("settled child work must enqueue derived Signal completion");
    }

    assert_eq!(
        result_rx.try_recv(),
        Err(TryRecvError::Empty),
        "root publication must not finish while child Signal completion is paused"
    );
    assert_eq!(
        serving.media_counters().replacements(),
        replacements_before,
        "physical publication typestate must forbid root replacement before child completion"
    );
    let paused = causal_record_for_backend_operation(&serving, paused_operation);
    assert_eq!(
        paused.effect_fate(),
        PhysicalWorkEffectFate::PublicationCompleted
    );
    assert_eq!(
        paused.derived_completion(),
        None,
        "the child must be physically settled but not derived-complete at the pause"
    );

    signal_gate.release();
    let published = result_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("publication must resume after derived completion")
        .expect("publication must remain successful");
    publication.join().unwrap();
    assert_eq!(published.root_generation(), 2);
    assert_eq!(
        serving.media_counters().replacements(),
        replacements_before + 1
    );
    assert_eq!(
        published.physical_work().effects()[0].stage(),
        RecordPublicationStage::CandidateDataWrite
    );
    let replacement = published
        .physical_work()
        .effects()
        .iter()
        .find(|effect| effect.stage() == RecordPublicationStage::CatalogReplacement)
        .expect("the published trace must retain its catalog replacement");
    assert_eq!(
        replacement.identity(),
        dependency.identity(),
        "the blocked Signal dependency must unlock the same physical replacement identity"
    );
    assert!(
        serving.certification_publication_dependencies().is_empty(),
        "settlement must retire operation-local Signal dependency nodes"
    );
    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .active_graph_node_count(),
        graph_nodes_before,
        "settlement must restore the live Signal graph to its pre-publication breadth"
    );

    let causal = serving.physical_work_observer().causal().records();
    let publication_work = published
        .physical_work()
        .effects()
        .iter()
        .map(|effect| effect.identity())
        .collect::<HashSet<_>>();
    let publication_causal = causal
        .iter()
        .filter(|record| publication_work.contains(&record.identity()))
        .collect::<Vec<_>>();
    assert_eq!(
        publication_causal.len(),
        publication_work.len(),
        "every traced publication effect must retain one causal record"
    );
    assert_eq!(
        serving.physical_work_observer().causal().overflow(),
        causal_overflow_before,
        "the courtroom must not prove completeness after causal-ledger eviction"
    );
    assert_eq!(
        serving.media_counters().retry_attempts(),
        retries_before,
        "the settled child must not trigger a mechanical media retry"
    );
    assert_eq!(
        serving.media_counters().fault_matches(),
        fault_matches_before + 1,
        "the exact paused media boundary must be selected once"
    );
    assert_eq!(
        publication_causal
            .iter()
            .filter_map(|record| record.backend_operation())
            .collect::<HashSet<_>>()
            .len(),
        publication_causal.len(),
        "publication effects must not reuse or repeat backend operation identity"
    );
    let completed_child = causal_record_for_backend_operation(&serving, paused_operation);
    assert!(
        completed_child.derived_completion().is_some(),
        "releasing Signal must complete the exact settled child"
    );
    assert!(!serving.close_plan().execute().requires_inspection());
}

fn causal_record_for_backend_operation(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    operation: MediaOperationIdentity,
) -> worth_store::physical_runtime::PhysicalWorkCausalRecord {
    let matches = serving
        .physical_work_observer()
        .causal()
        .records()
        .iter()
        .copied()
        .filter(|record| record.backend_operation() == Some(operation))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "the paused backend operation must join exactly one physical work record"
    );
    matches[0]
}

fn reaches_within(gate: &MediaPauseGate, timeout: Duration) -> bool {
    let (reached, waiting) = mpsc::channel();
    let gate = gate.clone();
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let _ = reached.send(());
    });
    waiting.recv_timeout(timeout).is_ok()
}
