use super::reload_storm_certification_test_support::{
    file_token_provider, invalid_file_provider, rich_file_provider, rich_rust_provider,
    rich_storm_app, runtime_with_rich_artifact, runtime_with_token, rust_token_provider, storm_app,
};
use crate::runtime::{
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormCertificationDenialReason,
    WorthUiReloadStormIterationOutcome, WorthUiReloadStormScenario,
    WorthUiSourceIngressDenialReason, WorthUiWatcherEvent,
};

#[test]
fn hostile_reload_storm_preserves_last_valid_active_plan() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before_active = runtime.inspect_active();
    let before_last_valid = runtime.last_valid();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("invalid-file-then-rust-prepared")
                .with_file_candidate("invalid file", invalid_file_provider("not valid worth ui"))
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("invalid candidates preserve and storm still certifies");

    assert_eq!(runtime.inspect_active(), before_active);
    assert_eq!(runtime.last_valid(), before_last_valid);
    assert_eq!(certification.counters().denied_candidate_count(), 1);
    assert_eq!(certification.counters().prepared_pending_cutover_count(), 1);
    assert_eq!(certification.counters().preservation_count(), 2);
    assert_eq!(
        certification
            .ordered_truth()
            .denied_preservation_plan_digests(),
        &[before_active.active_plan_digest()]
    );
    assert!(matches!(
        certification.bundle().iteration_outcomes()[0],
        WorthUiReloadStormIterationOutcome::DeniedPreserved(ref iteration)
            if matches!(
                iteration.candidate_denial_reason(),
                WorthUiReloadStormCandidateDenialReason::CandidateSubmissionDenied(_)
            )
    ));
    assert!(matches!(
        certification.bundle().iteration_outcomes()[1],
        WorthUiReloadStormIterationOutcome::PreparedPendingCutover(ref iteration)
            if iteration.ordering_receipt().matches_revision(iteration.source_revision())
    ));
}

#[test]
fn reload_storm_retains_equivalent_compositions_without_publishing_artifact_only_truth() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("equivalent-file-and-rust")
                .with_file_candidate("file prepared", file_token_provider("theme.text.primary"))
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("equivalent storm certifies");

    assert_eq!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().prepared_pending_cutover_count(), 2);
    assert_eq!(
        certification
            .ordered_truth()
            .prepared_preservation_plan_digests(),
        &[before.active_plan_digest(), before.active_plan_digest()]
    );
    for outcome in certification.bundle().iteration_outcomes() {
        assert!(matches!(
            outcome,
            WorthUiReloadStormIterationOutcome::PreparedPendingCutover(iteration)
                if iteration.ingress_counters().candidate_submissions_emitted() == 1
        ));
    }
}

#[test]
fn reload_storm_latency_counters_remain_iteration_shaped() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("mixed-counter-shape")
                .with_file_candidate("file prepared", file_token_provider("theme.text.primary"))
                .with_file_candidate("invalid file", invalid_file_provider("import ;"))
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("storm certifies");

    assert_eq!(certification.counters().iteration_count(), 3);
    assert_eq!(certification.counters().prepared_pending_cutover_count(), 2);
    assert_eq!(certification.counters().denied_candidate_count(), 1);
    assert_eq!(certification.counters().preservation_count(), 3);
    assert_eq!(certification.counters().foundational_receipt_count(), 1);
    assert_eq!(
        certification.bundle().foundational_receipt_count(),
        certification.counters().foundational_receipt_count()
    );
}

#[test]
fn reload_storm_source_ingress_denial_preserves_active_truth() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("partial-write-denial")
                .with_file_candidate_events(
                    "partial write",
                    file_token_provider("theme.text.secondary"),
                    [WorthUiWatcherEvent::write_started("app/main.wui.tmp")],
                )
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("source ingress denial preserves and storm still certifies");

    assert_eq!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().denied_candidate_count(), 1);
    assert!(matches!(
        certification.bundle().iteration_outcomes()[0],
        WorthUiReloadStormIterationOutcome::DeniedPreserved(ref iteration)
            if matches!(
                iteration.candidate_denial_reason(),
                WorthUiReloadStormCandidateDenialReason::SourceIngressDenied(denial)
                    if denial.reason()
                        == WorthUiSourceIngressDenialReason::PartialWriteWithoutStableSnapshot
            )
    ));
}

#[test]
fn reload_storm_with_interleaved_invalid_and_valid_compositions_preserves_active_truth() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("invalid-valid-invalid")
                .with_file_candidate("invalid file first", invalid_file_provider("???"))
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.secondary"),
                )
                .with_file_candidate("invalid file last", invalid_file_provider("import ;")),
            app.capabilities(),
        )
        .expect("interleaved storm certifies");

    assert_eq!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().prepared_pending_cutover_count(), 1);
    assert_eq!(certification.counters().denied_candidate_count(), 2);
    assert_eq!(
        certification
            .ordered_truth()
            .prepared_preservation_plan_digests(),
        &[before.active_plan_digest()]
    );
    assert_eq!(
        *certification
            .ordered_truth()
            .denied_preservation_plan_digests()
            .last()
            .expect("last invalid candidate preserves active plan"),
        certification.ordered_truth().final_active_plan_digest()
    );
}

#[test]
fn declaration_bearing_reload_storm_retains_whole_compositions_pending_cutover() {
    let app = rich_storm_app();
    let mut runtime = runtime_with_rich_artifact(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("rich-query-bound-platform-spine")
                .with_file_candidate("invalid rich file", invalid_file_provider("surface ;"))
                .with_file_candidate(
                    "file rich prepared",
                    rich_file_provider("theme.text.secondary"),
                )
                .with_rust_candidate(
                    "rust rich prepared",
                    rich_rust_provider(&app, "theme.text.secondary"),
                ),
            app.capabilities(),
        )
        .expect("declaration-bearing compositions remain whole while awaiting cutover");

    assert_eq!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().denied_candidate_count(), 1);
    assert_eq!(certification.counters().prepared_pending_cutover_count(), 2);
    let WorthUiReloadStormIterationOutcome::PreparedPendingCutover(file_iteration) =
        &certification.bundle().iteration_outcomes()[1]
    else {
        panic!("valid file composition should remain pending cutover");
    };
    let WorthUiReloadStormIterationOutcome::PreparedPendingCutover(rust_iteration) =
        &certification.bundle().iteration_outcomes()[2]
    else {
        panic!("valid Rust composition should remain pending cutover");
    };
    assert_eq!(
        file_iteration.composition_basis(),
        rust_iteration.composition_basis()
    );
    assert_eq!(
        file_iteration
            .composition_basis()
            .semantic_handoff()
            .identity(),
        rust_iteration
            .composition_basis()
            .semantic_handoff()
            .identity()
    );
    assert_ne!(
        file_iteration
            .composition_basis()
            .semantic_handoff()
            .authored_mode(),
        rust_iteration
            .composition_basis()
            .semantic_handoff()
            .authored_mode()
    );
    assert_ne!(
        file_iteration.authoring_lane(),
        rust_iteration.authoring_lane()
    );
    assert!(file_iteration
        .ordering_receipt()
        .matches_revision(file_iteration.source_revision()));
    assert!(rust_iteration
        .ordering_receipt()
        .matches_revision(rust_iteration.source_revision()));
}

#[test]
fn reload_storm_rejects_forged_receipt_reuse_across_candidates() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let denial = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("forged-reuse")
                .with_file_candidate("file prepared", file_token_provider("theme.text.secondary"))
                .with_forged_receipt_reuse_probe(
                    "rust tries reused receipt",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect_err("receipt reuse across candidate meaning is denied");

    assert_eq!(runtime.inspect_active(), before);
    assert!(matches!(
        denial.reason(),
        WorthUiReloadStormCertificationDenialReason::ForgedReceiptReuseAcrossCandidates
    ));
    assert_eq!(denial.counters().forged_receipt_reuse_denial_count(), 1);
}

#[test]
fn reload_storm_foundational_bundle_comparison_uses_full_meaning() {
    let app = storm_app();
    let mut left_runtime = runtime_with_token(&app, "theme.text.primary");
    let mut right_runtime = runtime_with_token(&app, "theme.text.primary");

    let left = left_runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("left")
                .with_file_candidate("file prepared", file_token_provider("theme.text.primary"))
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("left certifies");
    let right = right_runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("right")
                .with_file_candidate("invalid file", invalid_file_provider("not ui"))
                .with_rust_candidate(
                    "rust prepared",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("right certifies");

    let comparison = left
        .bundle()
        .compare_foundational_bundle_meaning(right.bundle())
        .expect("both storm certifications contain foundational bundles");
    assert!(
        !comparison.is_equivalent(),
        "Foundational bundle comparison must inspect counter specs and evidence rows"
    );
    assert!(matches!(
        right.bundle().iteration_outcomes()[0],
        WorthUiReloadStormIterationOutcome::DeniedPreserved(_)
    ));
}
