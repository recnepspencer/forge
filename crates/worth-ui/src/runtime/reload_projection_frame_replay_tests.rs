use super::*;

#[test]
fn projection_frame_replay_rejects_certified_frame_digest_drift() {
    let original_change = admitted_validation_change(7);
    let replayed_change = admitted_validation_change(8);
    let (original_batch, original_breadth) = certified_single_rebuild_batch(&original_change, 12);
    let (replayed_batch, replayed_breadth) = certified_single_rebuild_batch(&replayed_change, 13);

    let denial = WorthUiProjectionFrameReplayCertification::certify(
        &original_breadth,
        &original_batch,
        &replayed_breadth,
        &replayed_batch,
    )
    .expect_err("certified projection frames with different digests cannot replay");

    assert_eq!(
        denial,
        WorthUiProjectionFrameReplayDenial::ProjectionFrameDigestMismatch
    );
}

fn admitted_validation_change(runtime_instance: u64) -> WorthUiAdmittedRuntimeChangeEvidence {
    let evidence = WorthUiValidationReloadEvidence::builder(runtime_instance, 10, 11)
        .record_candidate_plan(13)
        .finish(WorthUiValidationReloadStatus::ReadyForFrameBoundary, 12, 13)
        .mark_activated(12, 13);
    let classified = WorthUiClassifiedRuntimeChange::from_validation_reload(&evidence);
    WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(runtime_instance),
    )
    .expect("activated validation change carries changed facts")
}

fn certified_single_rebuild_batch(
    change: &WorthUiAdmittedRuntimeChangeEvidence,
    rebound_frame_digest: u64,
) -> (
    WorthUiProjectionRebindBatchReceipt,
    WorthUiReloadProjectionBreadthCertification,
) {
    let batch = WorthUiProjectionRebindBatchReceipt::single_row(
        change.runtime_instance(),
        change.digest(),
        WorthUiProjectionRebindCounters::after_rebuild(
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        ),
        WorthUiProjectionRebindRowReceipt::new(
            WorthUiProjectionIdentity::runtime("header.theme"),
            WorthUiProjectionFamily::HeaderTheme,
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
            11,
            rebound_frame_digest,
        ),
    );
    let breadth = WorthUiReloadProjectionBreadthCertification::certify(change, &batch)
        .expect("single projection rebuild breadth certifies");
    (batch, breadth)
}
