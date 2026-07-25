use std::time::{Duration, Instant};

use worth_store::physical_runtime::{
    RecordAppendBatch, RecordAppendDenial, RecordAppendError, RecordPublicationStage,
    UnpublishedRecordBatchCause, UnpublishedRecordEffectFate, UnpublishedRecordWorldFate,
};

use super::{configuration, serving_from_initialization};

#[test]
fn post_effect_catalog_eligibility_mismatch_is_never_a_reusable_denial() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let graph_nodes_before = serving
        .physical_signal_observation()
        .unwrap()
        .active_graph_node_count();
    serving.certification_reject_next_catalog_eligibility_join();
    let before = serving.media_counters();

    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"eligibility mismatch".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let RecordAppendError::Unpublished(failure) = error else {
        panic!("post-effect eligibility mismatch must remain unpublished: {error:?}")
    };
    assert!(matches!(
        failure.cause(),
        UnpublishedRecordBatchCause::Semantic {
            stage: RecordPublicationStage::CatalogReplacement,
            denial: RecordAppendDenial::CatalogReplacementEligibilityMismatch,
        }
    ));
    assert_eq!(
        failure.effect_fate(),
        UnpublishedRecordEffectFate::EffectPossible
    );
    assert_eq!(
        failure.world_fate(),
        UnpublishedRecordWorldFate::InspectionRequired
    );
    assert!(failure.physical_work().effect_count() > 0);
    assert!(failure
        .physical_work()
        .effects()
        .iter()
        .any(|effect| effect.stage() == RecordPublicationStage::CatalogCandidateSynchronization));
    assert!(failure
        .physical_work()
        .effects()
        .iter()
        .all(|effect| effect.stage() != RecordPublicationStage::CatalogReplacement));
    assert!(
        serving.media_counters().positioned_write_attempts() > before.positioned_write_attempts(),
        "the mismatch must occur after real candidate writes"
    );
    assert_eq!(
        serving.media_counters().replacements(),
        before.replacements(),
        "eligibility mismatch must prevent catalog replacement"
    );

    await_signal_cleanup(&serving, graph_nodes_before);
    assert!(serving.close_plan().execute().requires_inspection());
}

fn await_signal_cleanup(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    graph_nodes_before: usize,
) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let observation = serving.physical_signal_observation().unwrap();
        if observation.active_locality_count() == 0
            && observation.active_graph_node_count() == graph_nodes_before
            && serving.certification_publication_dependencies().is_empty()
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "eligibility mismatch retained Signal work: locality={}, graph_nodes={}, baseline={}",
            observation.active_locality_count(),
            observation.active_graph_node_count(),
            graph_nodes_before,
        );
        std::thread::yield_now();
    }
}
