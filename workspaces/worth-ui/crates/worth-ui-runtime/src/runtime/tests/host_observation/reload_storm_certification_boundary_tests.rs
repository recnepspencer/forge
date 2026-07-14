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
            WorthUiReloadStormScenario::named("invalid-file-then-rust-noop")
                .with_file_candidate("invalid file", invalid_file_provider("not valid worth ui"))
                .with_rust_candidate(
                    "rust no-op",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("invalid candidates preserve and storm still certifies");

    assert_eq!(runtime.inspect_active(), before_active);
    assert_eq!(runtime.last_valid(), before_last_valid);
    assert_eq!(certification.counters().denied_candidate_count(), 1);
    assert_eq!(certification.counters().preservation_count(), 1);
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
}

#[test]
fn reload_storm_equivalent_edits_do_not_rebuild_or_swap_needlessly() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("equivalent-file-and-rust")
                .with_file_candidate("file no-op", file_token_provider("theme.text.primary"))
                .with_rust_candidate(
                    "rust no-op",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("equivalent storm certifies");

    assert_eq!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().no_op_candidate_count(), 2);
    assert_eq!(certification.counters().plan_swap_count(), 0);
    assert!(certification
        .ordered_truth()
        .activated_plan_digests()
        .is_empty());
}

#[test]
fn reload_storm_latency_counters_remain_iteration_shaped() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("mixed-counter-shape")
                .with_file_candidate("file no-op", file_token_provider("theme.text.primary"))
                .with_file_candidate("invalid file", invalid_file_provider("import ;"))
                .with_rust_candidate(
                    "rust no-op",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect("storm certifies");

    assert_eq!(certification.counters().iteration_count(), 3);
    assert_eq!(certification.counters().candidate_admission_count(), 2);
    assert_eq!(certification.counters().artifact_comparison_count(), 2);
    assert_eq!(certification.counters().plan_lowering_count(), 0);
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
                    "rust no-op",
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
fn reload_storm_with_interleaved_invalid_and_valid_candidates_preserves_ordered_truth() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("invalid-valid-invalid")
                .with_file_candidate("invalid file first", invalid_file_provider("???"))
                .with_rust_candidate(
                    "rust activates",
                    rust_token_provider(&app, "theme.text.secondary"),
                )
                .with_file_candidate("invalid file last", invalid_file_provider("import ;")),
            app.capabilities(),
        )
        .expect("interleaved storm certifies");

    assert_ne!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().activated_candidate_count(), 1);
    assert_eq!(certification.counters().denied_candidate_count(), 2);
    assert_eq!(
        certification.counters().candidate_admission_count(),
        1,
        "meaningful activation must carry the admitted candidate forward without re-admission"
    );
    assert_eq!(
        certification.ordered_truth().final_active_plan_digest(),
        *certification
            .ordered_truth()
            .activated_plan_digests()
            .last()
            .expect("one activation")
    );
    assert_eq!(
        *certification
            .ordered_truth()
            .denied_preservation_plan_digests()
            .last()
            .expect("last invalid candidate preserves activated active plan"),
        certification.ordered_truth().final_active_plan_digest()
    );
}

#[test]
fn rich_platform_reload_storm_proves_query_bound_file_and_rust_replacement_spine() {
    let app = rich_storm_app();
    let mut runtime = runtime_with_rich_artifact(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let certification = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("rich-query-bound-platform-spine")
                .with_file_candidate("invalid rich file", invalid_file_provider("surface ;"))
                .with_file_candidate(
                    "file rich activates",
                    rich_file_provider("theme.text.secondary"),
                )
                .with_rust_candidate(
                    "rust rich no-op",
                    rich_rust_provider(&app, "theme.text.secondary"),
                ),
            app.capabilities(),
        )
        .expect("rich platform storm certifies");

    assert_ne!(runtime.inspect_active(), before);
    assert_eq!(certification.counters().denied_candidate_count(), 1);
    assert_eq!(certification.counters().activated_candidate_count(), 1);
    assert_eq!(certification.counters().no_op_candidate_count(), 1);
    assert_eq!(certification.counters().lane_admission_count(), 1);
    assert_eq!(
        certification.ordered_truth().final_active_plan_digest(),
        *certification
            .ordered_truth()
            .activated_plan_digests()
            .last()
            .expect("rich file candidate activates")
    );
    assert!(matches!(
        certification.bundle().iteration_outcomes()[2],
        WorthUiReloadStormIterationOutcome::EquivalentNoOp(_)
    ));
    let WorthUiReloadStormIterationOutcome::Activated(activated) =
        &certification.bundle().iteration_outcomes()[1]
    else {
        panic!("rich file candidate should activate");
    };
    assert_eq!(
        activated
            .report()
            .counters()
            .query_binding_comparison_count(),
        1
    );
    assert_eq!(activated.report().counters().query_live_rebind_count(), 1);
    assert_eq!(
        activated
            .report()
            .counters()
            .durable_state_reconciliation_count(),
        1
    );
    assert_eq!(activated.report().counters().topology_assembly_count(), 1);
}

#[test]
fn reload_storm_rejects_forged_receipt_reuse_across_candidates() {
    let app = storm_app();
    let mut runtime = runtime_with_token(&app, "theme.text.primary");
    let before = runtime.inspect_active();

    let denial = runtime
        .certify_reload_storm_against_snapshot(
            WorthUiReloadStormScenario::named("forged-reuse")
                .with_file_candidate(
                    "file activates",
                    file_token_provider("theme.text.secondary"),
                )
                .with_forged_receipt_reuse_probe(
                    "rust tries reused receipt",
                    rust_token_provider(&app, "theme.text.primary"),
                ),
            app.capabilities(),
        )
        .expect_err("receipt reuse across candidate meaning is denied");

    assert_ne!(runtime.inspect_active(), before);
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
            WorthUiReloadStormScenario::named("right")
                .with_file_candidate("invalid file", invalid_file_provider("not ui"))
                .with_rust_candidate(
                    "rust no-op",
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
