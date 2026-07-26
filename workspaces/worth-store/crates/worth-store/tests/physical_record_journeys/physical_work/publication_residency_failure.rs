use std::{
    sync::mpsc::{self, TryRecvError},
    time::{Duration, Instant},
};

use worth_store::physical_runtime::{
    PhysicalWorkEffectFate, RecordAppendBatch, RecordAppendError, RecordPublicationStage,
    UnpublishedRecordBatchCause, UnpublishedRecordEffectFate, UnpublishedRecordWorldFate,
};
use worth_store_physical_backend::MediaPauseGate;

use super::{configuration, serving_from_initialization};

#[test]
fn post_effect_candidate_rejection_retains_staged_physical_identity() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    serving_from_initialization(&root).close();
    let (serving, media_gate) = super::fault_fixture::serving_from_open_with_one_write_pause(&root);
    let graph_nodes_before = serving
        .physical_signal_observation()
        .unwrap()
        .active_graph_node_count();
    serving.certification_reject_next_candidate_publication_after_physical_write();
    let before = serving.media_counters();
    let submission = serving.record_submission();
    let (result_tx, result_rx) = mpsc::sync_channel(1);
    let publication = std::thread::spawn(move || {
        let result = submission.append_batch(
            RecordAppendBatch::try_from_iter([b"post-effect residency".as_slice()]).unwrap(),
            placement,
        );
        let _ = result_tx.send(result);
    });

    if !reaches_within(&media_gate, Duration::from_secs(3)) {
        media_gate.release();
        let _ = publication.join();
        panic!("the rejected candidate write must reach the real backend");
    }
    assert_eq!(
        result_rx.try_recv(),
        Err(TryRecvError::Empty),
        "residency rejection must occur after the paused physical write"
    );
    let dependencies = serving.certification_publication_dependencies();
    let [dependency] = dependencies.as_slice() else {
        panic!(
            "the failure path must prepare exactly one blocked replacement, observed {}",
            dependencies.len()
        );
    };
    let replacement_identity = dependency.identity();
    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .active_graph_node_count(),
        graph_nodes_before + 2
    );
    media_gate.release();
    let result = result_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("residency rejection must return after the physical write");
    publication.join().unwrap();

    let RecordAppendError::Unpublished(failure) = result.unwrap_err() else {
        panic!("post-effect residency rejection must remain unpublished")
    };
    let UnpublishedRecordBatchCause::Residency { stage, denial } = failure.cause() else {
        panic!("post-effect rejection must retain residency causality");
    };
    assert_eq!(*stage, RecordPublicationStage::CandidateDataWrite);
    let worth_store::physical_runtime::RecordAppendDenial::ResidencyUnavailable(residency) = denial
    else {
        panic!("post-effect rejection must expose Store-owned residency meaning");
    };
    assert_eq!(
        residency.kind(),
        worth_store::physical_runtime::PhysicalRecordResidencyFailureKind::PublicationConflict
    );
    assert_eq!(
        failure.effect_fate(),
        UnpublishedRecordEffectFate::EffectPossible
    );
    assert_eq!(
        failure.world_fate(),
        UnpublishedRecordWorldFate::InspectionRequired
    );
    let [effect] = failure.physical_work().effects() else {
        panic!("the real settled candidate write must survive in failure evidence")
    };
    assert_eq!(effect.stage(), RecordPublicationStage::CandidateDataWrite);
    let causal = serving
        .physical_work_observer()
        .causal()
        .records()
        .iter()
        .copied()
        .filter(|record| record.identity() == effect.identity())
        .collect::<Vec<_>>();
    assert_eq!(causal.len(), 1);
    assert_eq!(
        causal[0].effect_fate(),
        PhysicalWorkEffectFate::PublicationCompleted
    );
    assert!(causal[0].backend_operation().is_some());
    assert!(causal[0].derived_completion().is_some());
    assert_eq!(
        serving.media_counters().positioned_write_attempts(),
        before.positioned_write_attempts() + 1
    );
    assert_eq!(
        serving.media_counters().replacements(),
        before.replacements()
    );
    await_publication_signal_cleanup(&serving, graph_nodes_before);
    let closed = serving.close_plan().execute();
    assert!(closed.requires_inspection());
    assert_eq!(
        closed
            .shutdown()
            .work()
            .drain()
            .released_before_dispatch()
            .iter()
            .filter(|identity| **identity == replacement_identity)
            .count(),
        1,
        "the exact prepared replacement must terminate once without dispatch"
    );
}

fn await_publication_signal_cleanup(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    graph_nodes_before: usize,
) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let observation = serving.physical_signal_observation().unwrap();
        let dependencies = serving.certification_publication_dependencies();
        if observation.active_locality_count() == 0
            && observation.active_graph_node_count() == graph_nodes_before
            && dependencies.is_empty()
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "abandoned replacement retained Signal state: locality={}, graph_nodes={}, baseline={}, dependencies={}",
            observation.active_locality_count(),
            observation.active_graph_node_count(),
            graph_nodes_before,
            dependencies.len()
        );
        std::thread::yield_now();
    }
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
