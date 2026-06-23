use super::reload_storm_certification_test_support::{
    file_token_provider, invalid_file_provider, runtime_with_token, rust_token_provider, storm_app,
};
use super::*;

#[test]
fn replay_certification_accepts_identical_semantic_storms() {
    let app = storm_app();
    let mut original_runtime = runtime_with_token(&app, "theme.text.primary");
    let mut replay_runtime = runtime_with_token(&app, "theme.text.primary");
    let scenario = WorthUiReloadStormScenario::named("reload-closeout-replay")
        .with_file_candidate("invalid source", invalid_file_provider("not worth ui"))
        .with_file_candidate(
            "file activates",
            file_token_provider("theme.text.secondary"),
        )
        .with_rust_candidate(
            "rust no-op",
            rust_token_provider(&app, "theme.text.secondary"),
        );

    let original = original_runtime
        .certify_reload_storm_against_snapshot(scenario.clone(), app.capabilities())
        .expect("original storm certifies");
    let replayed = replay_runtime
        .certify_reload_storm_against_snapshot(scenario, app.capabilities())
        .expect("replayed storm certifies");
    let replay = WorthUiReloadReplayCertification::certify(&original, &replayed)
        .expect("same semantic storm converges under replay");

    assert_eq!(
        replay.final_active_artifact_digest(),
        original.ordered_truth().final_active_artifact_digest()
    );
    assert_eq!(
        replay.final_active_plan_digest(),
        original.ordered_truth().final_active_plan_digest()
    );
    assert_eq!(
        replay.final_capability_snapshot_digest(),
        original.ordered_truth().final_capability_snapshot_digest()
    );
    assert_eq!(
        replay.final_authoring_snapshot_digest(),
        original.ordered_truth().final_authoring_snapshot_digest()
    );
    assert_eq!(
        replay.final_last_valid_artifact_digest(),
        original.ordered_truth().final_last_valid_artifact_digest()
    );
    assert_eq!(replay.iteration_count(), 3);
    assert_eq!(
        replay.foundational_meaning_digest(),
        original.bundle().foundational_meaning_digest()
    );
}

#[test]
fn replay_certification_rejects_different_scenario_identity() {
    let app = storm_app();
    let mut left_runtime = runtime_with_token(&app, "theme.text.primary");
    let mut right_runtime = runtime_with_token(&app, "theme.text.primary");
    let left = left_runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("left-replay")
                .with_file_candidate("file no-op", file_token_provider("theme.text.primary"))
                .with_rust_candidate(
                    "rust no-op",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("left certifies");
    let right = right_runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("right-replay")
                .with_file_candidate("file no-op", file_token_provider("theme.text.primary"))
                .with_rust_candidate(
                    "rust no-op",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("right certifies");

    let denial = WorthUiReloadReplayCertification::certify(&left, &right)
        .expect_err("scenario identity participates in replay proof");

    assert_eq!(
        denial,
        WorthUiReloadReplayCertificationDenial::ScenarioDigestMismatch
    );
}

#[test]
fn projection_breadth_certifies_sparse_changed_fact_intersections() {
    let change_evidence = admitted_validation_change(7);
    let batch = projection_batch(
        &change_evidence,
        WorthUiProjectionRebindCounters::aggregate([
            WorthUiProjectionRebindCounters::after_rebuild(
                WorthUiProjectionRebindStatus::ReboundAfterActivation,
            ),
            WorthUiProjectionRebindCounters::inspected_without_intersection(
                WorthUiProjectionRebindStatus::PreservedEquivalentReload,
            ),
        ]),
        [
            projection_row(
                "header.theme",
                WorthUiProjectionRebindStatus::ReboundAfterActivation,
            ),
            projection_row(
                "header.menu",
                WorthUiProjectionRebindStatus::PreservedEquivalentReload,
            ),
        ],
    );

    let certification =
        WorthUiReloadProjectionBreadthCertification::certify(&change_evidence, &batch)
            .expect("only intersecting projection rebuilt");

    assert_eq!(certification.changed_fact_count(), 2);
    assert_eq!(certification.inspected_projection_count(), 2);
    assert_eq!(certification.dependency_intersection_count(), 1);
    assert_eq!(certification.rebuild_attempt_count(), 1);
    assert_eq!(certification.preserved_frame_count(), 1);
}

#[test]
fn projection_breadth_rejects_global_rebuild_disguised_as_sparse_change() {
    let change_evidence = admitted_validation_change(7);
    let batch = projection_batch(
        &change_evidence,
        WorthUiProjectionRebindCounters::from_counts_for_test(2, 1, 2, 0, 0, 2),
        [
            projection_row(
                "header.theme",
                WorthUiProjectionRebindStatus::ReboundAfterActivation,
            ),
            projection_row(
                "header.menu",
                WorthUiProjectionRebindStatus::ReboundAfterActivation,
            ),
        ],
    );

    let denial = WorthUiReloadProjectionBreadthCertification::certify(&change_evidence, &batch)
        .expect_err("rebuild attempts must equal dependency intersections");

    assert_eq!(
        denial,
        WorthUiReloadProjectionBreadthDenial::RebuildCountDoesNotMatchDependencyIntersections
    );
}

#[test]
fn projection_breadth_allows_mixed_denied_and_rebuilt_rows_when_counts_are_bounded() {
    let change_evidence = admitted_validation_change(7);
    let batch = projection_batch(
        &change_evidence,
        WorthUiProjectionRebindCounters::aggregate([
            WorthUiProjectionRebindCounters::after_rebuild(
                WorthUiProjectionRebindStatus::ReboundAfterActivation,
            ),
            WorthUiProjectionRebindCounters::inspected_without_intersection(
                WorthUiProjectionRebindStatus::DeniedReloadNotActivated,
            ),
        ]),
        [
            projection_row(
                "header.theme",
                WorthUiProjectionRebindStatus::ReboundAfterActivation,
            ),
            projection_row(
                "page.host",
                WorthUiProjectionRebindStatus::DeniedReloadNotActivated,
            ),
        ],
    );

    let certification =
        WorthUiReloadProjectionBreadthCertification::certify(&change_evidence, &batch)
            .expect("mixed denied and rebuilt rows are valid when rebuilds stay bounded");

    assert_eq!(certification.dependency_intersection_count(), 1);
    assert_eq!(certification.rebuilt_frame_count(), 1);
    assert_eq!(certification.denied_frame_count(), 1);
}

#[test]
fn projection_breadth_rejects_batch_from_different_runtime_change() {
    let change_evidence = admitted_validation_change(7);
    let foreign_change = admitted_validation_change(8);
    let batch = projection_batch(
        &foreign_change,
        WorthUiProjectionRebindCounters::after_rebuild(
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        ),
        [projection_row(
            "header.theme",
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        )],
    );

    let denial = WorthUiReloadProjectionBreadthCertification::certify(&change_evidence, &batch)
        .expect_err("projection batch must belong to the admitted runtime change");

    assert_eq!(
        denial,
        WorthUiReloadProjectionBreadthDenial::RuntimeInstanceMismatch
    );
}

#[test]
fn projection_breadth_rejects_batch_with_same_runtime_but_different_change_digest() {
    let change_evidence = admitted_validation_change(7);
    let different_change = admitted_validation_change_with_plan(7, 31);
    let batch = projection_batch(
        &different_change,
        WorthUiProjectionRebindCounters::after_rebuild(
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        ),
        [projection_row(
            "header.theme",
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        )],
    );

    let denial = WorthUiReloadProjectionBreadthCertification::certify(&change_evidence, &batch)
        .expect_err("same runtime witness cannot replace exact change evidence digest");

    assert_eq!(
        denial,
        WorthUiReloadProjectionBreadthDenial::ChangeEvidenceDigestMismatch
    );
}

#[test]
fn visual_capture_receipt_requires_prior_runtime_and_projection_certification() {
    let change_evidence = admitted_validation_change(7);
    let batch = projection_batch(
        &change_evidence,
        WorthUiProjectionRebindCounters::after_rebuild(
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        ),
        [projection_row(
            "header.theme",
            WorthUiProjectionRebindStatus::ReboundAfterActivation,
        )],
    );
    let breadth = WorthUiReloadProjectionBreadthCertification::certify(&change_evidence, &batch)
        .expect("batch certifies");

    let receipt = WorthUiHotReloadVisualCaptureReceipt::from_certified_capture(
        &breadth,
        "sha256:visual-proof",
    )
    .expect("non-empty capture attaches to certified proof");

    assert_eq!(receipt.image_artifact_digest(), "sha256:visual-proof");
    assert_eq!(
        receipt.projection_rebind_digest(),
        breadth.projection_rebind_batch_digest()
    );
    assert_eq!(
        WorthUiHotReloadVisualCaptureReceipt::from_certified_capture(&breadth, " ",)
            .expect_err("blank screenshot digest is not evidence"),
        WorthUiHotReloadVisualCaptureDenial::EmptyImageArtifactDigest
    );
}

#[test]
fn steady_frame_after_reload_rejects_source_or_registry_work() {
    let mut counters = WorthUiOrdinaryLaneCounters::default();
    counters.record_frame_row_touch();
    counters.record_source_parse();

    let denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(91)
        .minimal_diagnostics()
        .record_ordinary_counters_for_test(counters)
        .seal()
        .expect_err("steady frame cannot perform source parse after reload");

    assert_eq!(
        denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}

fn admitted_validation_change(runtime_instance: u64) -> WorthUiAdmittedRuntimeChangeEvidence {
    admitted_validation_change_with_plan(runtime_instance, 13)
}

fn admitted_validation_change_with_plan(
    runtime_instance: u64,
    active_plan_digest_after: u64,
) -> WorthUiAdmittedRuntimeChangeEvidence {
    let evidence = WorthUiValidationReloadEvidence::builder(runtime_instance, 10, 11)
        .record_candidate_plan(active_plan_digest_after)
        .finish(
            WorthUiValidationReloadStatus::ReadyForFrameBoundary,
            12,
            active_plan_digest_after,
        )
        .mark_activated(12, active_plan_digest_after);
    let classified = WorthUiClassifiedRuntimeChange::from_validation_reload(&evidence);
    WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(runtime_instance),
    )
    .expect("activated validation change carries changed facts")
}

fn projection_batch<const N: usize>(
    change_evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    counters: WorthUiProjectionRebindCounters,
    rows: [WorthUiProjectionRebindRowReceipt; N],
) -> WorthUiProjectionRebindBatchReceipt {
    let change_evidence_digest = change_evidence.digest();
    let mut receipts = rows.into_iter().map(|row| {
        WorthUiProjectionRebindBatchReceipt::single_row(
            change_evidence.runtime_instance(),
            change_evidence_digest,
            counters_for_row(row.status()),
            row,
        )
    });
    let first = receipts.next().expect("at least one row");
    let aggregate =
        WorthUiProjectionRebindBatchReceipt::aggregate(std::iter::once(first).chain(receipts))
            .expect("batch aggregates");
    WorthUiProjectionRebindBatchReceipt::from_rows_for_test(
        aggregate.runtime_instance(),
        aggregate.change_evidence_digest(),
        counters,
        aggregate.rows().iter().cloned(),
    )
}

fn counters_for_row(status: WorthUiProjectionRebindStatus) -> WorthUiProjectionRebindCounters {
    match status {
        WorthUiProjectionRebindStatus::ReboundAfterActivation => {
            WorthUiProjectionRebindCounters::after_rebuild(status)
        }
        _ => WorthUiProjectionRebindCounters::inspected_without_intersection(status),
    }
}

fn projection_row(
    identity: &str,
    status: WorthUiProjectionRebindStatus,
) -> WorthUiProjectionRebindRowReceipt {
    WorthUiProjectionRebindRowReceipt::new(
        WorthUiProjectionIdentity::runtime(identity),
        WorthUiProjectionFamily::HeaderTheme,
        status,
        11,
        if status == WorthUiProjectionRebindStatus::ReboundAfterActivation {
            12
        } else {
            11
        },
    )
}
