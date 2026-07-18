use crate::facade::WorthUi;
use crate::runtime::tests::source_ingress_boundary_test_support::{
    assert_source_denial_reason, lower_file_submission,
};
use crate::runtime::tests::source_ingress_test_support::{
    empty_artifact, file_import_provider, runtime_from_artifact, rust_import_input,
};
use crate::runtime::{
    WorthUiReloadDebounce, WorthUiSourceIngressDenialReason, WorthUiSourceProvider,
    WorthUiWatcherEvent,
};
use std::time::Duration;

#[test]
fn equivalent_file_event_bursts_debounce_to_equivalent_candidates() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let provider = file_import_provider();
    let first = lower_file_submission(
        provider.clone(),
        [
            WorthUiWatcherEvent::modified("app/main.wui"),
            WorthUiWatcherEvent::atomic_rename("app/main.wui.tmp", "app/main.wui"),
        ],
        snapshot.capabilities(),
    );
    let second = lower_file_submission(
        provider,
        [
            WorthUiWatcherEvent::atomic_rename("app/main.wui.tmp", "app/main.wui"),
            WorthUiWatcherEvent::modified("app/main.wui"),
        ],
        snapshot.capabilities(),
    );

    assert_eq!(
        first.ordering_receipt().receipt_digest(),
        second.ordering_receipt().receipt_digest()
    );
    assert_eq!(
        first.source_revision().final_package_digest(),
        second.source_revision().final_package_digest()
    );
    assert_eq!(first.composition_basis(), second.composition_basis());
}

#[test]
fn watcher_event_without_lowered_candidate_cannot_mutate_active_runtime() {
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(file_import_provider())
        .start();
    let batch = session
        .ingest([WorthUiWatcherEvent::modified("app/main.wui")])
        .expect("event debounces to a batch");

    assert_eq!(batch.counters().active_runtime_mutations(), 0);
    assert_eq!(batch.counters().frame_path_work(), 0);
}

#[test]
fn watcher_event_reorder_does_not_change_final_candidate_sequence() {
    let provider = file_import_provider();
    let debounce = WorthUiReloadDebounce::stable_window(Duration::from_millis(20));
    let first = debounce
        .debounce(
            provider.clone(),
            &[
                WorthUiWatcherEvent::deleted("app/main.wui.tmp"),
                WorthUiWatcherEvent::write_completed("app/main.wui"),
            ],
            7,
        )
        .expect("first burst debounces");
    let second = debounce
        .debounce(
            provider,
            &[
                WorthUiWatcherEvent::write_completed("app/main.wui"),
                WorthUiWatcherEvent::deleted("app/main.wui.tmp"),
            ],
            7,
        )
        .expect("second burst debounces");

    assert_eq!(first.ordering_receipt(), second.ordering_receipt());
    assert_eq!(first.source_revision(), second.source_revision());
}

#[test]
fn partial_write_and_atomic_rename_emit_one_ordered_candidate() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let submission = lower_file_submission(
        file_import_provider(),
        [
            WorthUiWatcherEvent::write_started("app/main.wui.tmp"),
            WorthUiWatcherEvent::atomic_rename("app/main.wui.tmp", "app/main.wui"),
        ],
        snapshot.capabilities(),
    );

    assert_eq!(submission.counters().raw_events_observed(), 2);
    assert_eq!(submission.counters().events_coalesced(), 1);
    assert_eq!(submission.counters().candidate_submissions_emitted(), 1);
}

#[test]
fn partial_write_without_stable_snapshot_is_denied_before_candidate_submission() {
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(file_import_provider())
        .start();

    let denial = session
        .ingest([WorthUiWatcherEvent::write_started("app/main.wui.tmp")])
        .expect_err("unstable partial write is denied");

    assert_eq!(
        denial.reason(),
        WorthUiSourceIngressDenialReason::PartialWriteWithoutStableSnapshot
    );
}

#[test]
fn in_memory_source_provider_uses_same_candidate_admission() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let file_submission = lower_file_submission(
        file_import_provider(),
        [WorthUiWatcherEvent::modified("app/main.wui")],
        snapshot.capabilities(),
    );
    let memory_submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("editor-buffer")
            .with_file("app/main.wui", r#"import "app/panels/inspector.wui";"#)
            .with_file("app/panels/inspector.wui", ""),
        [WorthUiWatcherEvent::provider_revision("editor-buffer")],
        snapshot.capabilities(),
    );

    assert_eq!(
        file_submission.composition_basis(),
        memory_submission.composition_basis()
    );
}

#[test]
fn rust_authored_provider_without_composition_cannot_be_candidate() {
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(WorthUiSourceProvider::rust_authored("rust-authored"))
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::provider_revision("rust-authored")])
        .expect_err("an empty provider is denied before debounce");

    assert_eq!(
        denial.reason(),
        WorthUiSourceIngressDenialReason::EmptyProvider
    );
}

#[test]
fn mixed_file_and_rust_composition_provider_is_denied_before_candidate_selection() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(file_import_provider().with_rust_authored_input(rust_import_input()))
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::provider_revision("mixed")])
        .expect("mixed material can still debounce")
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("candidate material selection must not be ambiguous");

    assert_source_denial_reason(
        denial,
        WorthUiSourceIngressDenialReason::MixedCandidateMaterial,
    );
}

#[test]
fn multiple_rust_compositions_are_denied_instead_of_first_composition_winning() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::rust_authored("rust-authored")
                .with_rust_authored_input(rust_import_input())
                .with_rust_authored_input(
                    crate::source::WorthUiRustAuthoredArtifactInput::from_modules([
                        crate::source::WorthUiRustAuthoredArtifactInputModule::new("app/main.wui"),
                    ]),
                ),
        )
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::provider_revision("rust-authored")])
        .expect("multi-artifact material can still debounce")
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("multiple artifact inputs need explicit merge semantics");

    assert_source_denial_reason(
        denial,
        WorthUiSourceIngressDenialReason::MultipleRustAuthoredInputs,
    );
}

#[test]
fn empty_source_ingress_hook_is_denied_before_debounce() {
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(file_import_provider())
        .with_hook(crate::runtime::WorthUiSourceIngressHook::generated_source(
            "empty-generated",
            WorthUiSourceProvider::generated("empty-generated"),
        ))
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::modified("app/main.wui")])
        .expect_err("empty hooks are unsupported outputs");

    assert_eq!(
        denial.reason(),
        WorthUiSourceIngressDenialReason::UnsupportedHookOutput
    );
}

#[test]
fn duplicate_source_modules_report_source_package_rejection() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let provider = WorthUiSourceProvider::in_memory("duplicate-source")
        .with_file("app/main.wui", "")
        .with_file("app/./main.wui", "");
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(provider)
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::provider_revision("duplicate-source")])
        .expect("provider material can debounce before source package validation")
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("duplicate source module identity must fail package validation");

    assert_source_denial_reason(
        denial,
        WorthUiSourceIngressDenialReason::SourcePackageRejected,
    );
}

#[test]
fn malformed_source_reports_parse_rejection_not_missing_material() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let provider = WorthUiSourceProvider::in_memory("malformed-source")
        .with_file("app/main.wui", "component MissingBrace {");
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(provider)
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::provider_revision("malformed-source")])
        .expect("provider material can debounce before parse validation")
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("malformed source must fail parse validation");

    assert_source_denial_reason(
        denial,
        WorthUiSourceIngressDenialReason::SourceParseRejected,
    );
}

#[test]
fn ordering_receipt_sequence_drift_is_denied_before_candidate_lowering() {
    let snapshot = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(file_import_provider())
        .start();
    let batch = session
        .ingest([WorthUiWatcherEvent::modified("app/main.wui")])
        .expect("event debounces");
    let drifted_receipt = batch
        .ordering_receipt()
        .clone()
        .with_sequence_for_test(batch.source_revision().sequence() + 1);
    let denial = batch
        .with_ordering_receipt_for_test(drifted_receipt)
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("receipt drift must be denied before source lowering");

    assert_source_denial_reason(
        denial,
        WorthUiSourceIngressDenialReason::OrderingReceiptDrift,
    );
}
