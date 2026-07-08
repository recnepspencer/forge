use crate::facade::WorthUi;
use crate::runtime::tests::source_ingress_test_support::{
    empty_artifact, file_import_provider, runtime_from_artifact, rust_import_artifact,
    rust_import_provider,
};
use crate::runtime::{
    WorthUiReloadDebounce, WorthUiSourceIngressDenialReason, WorthUiSourceProvider,
    WorthUiWatchedArtifactInput, WorthUiWatchedCandidateSubmissionDenial, WorthUiWatcherEvent,
};
use std::time::Duration;

#[test]
fn equivalent_file_event_bursts_debounce_to_equivalent_candidates() {
    let snapshot = WorthUi::app().freeze();
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
    assert_eq!(
        first.into_candidate().basis(),
        second.into_candidate().basis()
    );
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
fn file_watcher_uses_candidate_pipeline_for_file_and_rust_artifact_inputs() {
    let snapshot = WorthUi::app().freeze();
    let file_submission = lower_file_submission(
        file_import_provider(),
        [WorthUiWatcherEvent::modified("app/main.wui")],
        snapshot.capabilities(),
    );
    let rust_submission = lower_rust_submission(
        rust_import_provider(),
        [WorthUiWatcherEvent::provider_revision("rust-authored")],
        snapshot.capabilities(),
    );
    let file_candidate = file_submission.into_candidate();
    let rust_candidate = rust_submission.into_candidate();

    assert_eq!(file_candidate.basis(), rust_candidate.basis());
    assert_eq!(file_candidate.cause().kind_name(), "file-source-changed");
    assert_eq!(
        rust_candidate.cause().kind_name(),
        "rust-authored-input-changed"
    );
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
    let snapshot = WorthUi::app().freeze();
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
    let snapshot = WorthUi::app().freeze();
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
        file_submission.into_candidate().basis(),
        memory_submission.into_candidate().basis()
    );
}

#[test]
fn watched_artifact_without_material_cannot_be_candidate() {
    let snapshot = WorthUi::app().freeze();
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::rust_authored_artifact("rust-authored")
                .with_artifact_input(WorthUiWatchedArtifactInput::rust_authored("input", 42)),
        )
        .start();
    let batch = session
        .ingest([WorthUiWatcherEvent::provider_revision("rust-authored")])
        .expect("synthetic descriptor can debounce");

    let denial = batch
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("candidate material is required");

    assert!(matches!(
        denial,
        WorthUiWatchedCandidateSubmissionDenial::SourceIngress(_)
    ));
}

#[test]
fn mixed_file_and_artifact_provider_is_denied_before_candidate_selection() {
    let snapshot = WorthUi::app().freeze();
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(file_import_provider().with_artifact_input(
            WorthUiWatchedArtifactInput::from_rust_authored_artifact(
                "import-provider",
                rust_import_artifact(),
            ),
        ))
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
fn multiple_artifact_inputs_are_denied_instead_of_first_artifact_winning() {
    let snapshot = WorthUi::app().freeze();
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::rust_authored_artifact("rust-authored")
                .with_artifact_input(WorthUiWatchedArtifactInput::from_rust_authored_artifact(
                    "first",
                    rust_import_artifact(),
                ))
                .with_artifact_input(WorthUiWatchedArtifactInput::from_rust_authored_artifact(
                    "second",
                    empty_artifact(),
                )),
        )
        .start();
    let denial = session
        .ingest([WorthUiWatcherEvent::provider_revision("rust-authored")])
        .expect("multi-artifact material can still debounce")
        .lower_to_candidate_submission(snapshot.capabilities())
        .expect_err("multiple artifact inputs need explicit merge semantics");

    assert_source_denial_reason(
        denial,
        WorthUiSourceIngressDenialReason::MultipleArtifactInputs,
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
    let snapshot = WorthUi::app().freeze();
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
    let snapshot = WorthUi::app().freeze();
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
fn file_authored_source_ingress_emits_the_sealed_source_backed_package_on_the_ordinary_lane() {
    let snapshot = WorthUi::app()
        .register_component(source_backed_package_component(
            "workspace.component.workflow_editor",
        ))
        .register_component(source_backed_package_component(
            "workspace.component.workflow_editor.peer_a",
        ))
        .register_component(source_backed_package_component(
            "workspace.component.workflow_editor.peer_b",
        ))
        .register_mosaic_region_kind(source_backed_package_region())
        .register_mosaic_sizing_contract(source_backed_package_sizing())
        .freeze();
    let provider = WorthUiSourceProvider::in_memory("source-backed-package").with_file(
        "app/source_backed_package.wui",
        r#"
component workspace.component.workflow_editor {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
component workspace.component.workflow_editor.peer_a {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
component workspace.component.workflow_editor.peer_b {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
"#,
    );
    let submission = lower_file_submission(
        provider,
        [WorthUiWatcherEvent::provider_revision(
            "source-backed-package",
        )],
        snapshot.capabilities(),
    );
    let source_backed_package = submission
        .source_backed_dsl_package()
        .expect("ordinary file-authored ingress should emit the sealed source-backed package");

    let mut observed = source_backed_package
        .dsl_package()
        .admitted_declarations()
        .iter()
        .map(|receipt| {
            (
                receipt.source_provenance().module_path().to_owned(),
                receipt.source_provenance().declaration_index(),
            )
        })
        .collect::<Vec<_>>();
    observed.sort();

    assert_eq!(
        observed,
        vec![
            ("app/source_backed_package.wui".to_owned(), 0),
            ("app/source_backed_package.wui".to_owned(), 1),
            ("app/source_backed_package.wui".to_owned(), 2),
        ]
    );
    assert_eq!(
        source_backed_package
            .declaration_witness()
            .claims_for("app/source_backed_package.wui", 0)
            .map(|claims| claims.mosaic_sizing_contract_id().as_str()),
        Some("workspace.sizing.mosaic_support")
    );
}

#[test]
fn ordering_receipt_sequence_drift_is_denied_before_candidate_lowering() {
    let snapshot = WorthUi::app().freeze();
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

fn lower_file_submission<const N: usize>(
    provider: WorthUiSourceProvider,
    events: [WorthUiWatcherEvent; N],
    snapshot: &crate::capability::CapabilitySnapshot,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(provider)
        .start();
    session
        .ingest(events)
        .expect("events debounce")
        .lower_to_candidate_submission(snapshot)
        .expect("candidate submission lowers")
}

fn lower_rust_submission<const N: usize>(
    provider: WorthUiSourceProvider,
    events: [WorthUiWatcherEvent; N],
    snapshot: &crate::capability::CapabilitySnapshot,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    lower_file_submission(provider, events, snapshot)
}

fn assert_source_denial_reason(
    denial: WorthUiWatchedCandidateSubmissionDenial,
    expected_reason: WorthUiSourceIngressDenialReason,
) {
    match denial {
        WorthUiWatchedCandidateSubmissionDenial::SourceIngress(source_denial) => {
            assert_eq!(source_denial.reason(), expected_reason);
        }
        WorthUiWatchedCandidateSubmissionDenial::Candidate(candidate_denial) => {
            panic!("expected source ingress denial, got {candidate_denial:?}");
        }
    }
}

fn source_backed_package_component(id: &str) -> crate::capability::ComponentDescriptor {
    crate::capability::ComponentDescriptor::new(
        crate::capability::ComponentId::new(id).unwrap(),
        crate::capability::ComponentPropSchema::named(format!("{id}.props")),
        crate::capability::ComponentChildPolicy::no_children(),
        crate::capability::ComponentStateOwnership::runtime_owned(),
    )
}

fn source_backed_package_region() -> crate::capability::MosaicRegionKindDescriptor {
    crate::capability::MosaicRegionKindDescriptor::new(
        crate::capability::MosaicRegionKindId::new("workspace.region.primary").unwrap(),
        crate::capability::MosaicRegionRole::primary(),
    )
    .with_sizing_behavior(crate::capability::MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(crate::capability::MosaicScrollOwnership::region_owned())
    .with_focus_scope(crate::capability::MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(crate::capability::MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(crate::capability::SurfacePlacementClass::primary_region())
    .with_persistence(crate::capability::MosaicRegionPersistence::restorable())
    .with_clipping(crate::capability::MosaicClippingPosture::clip_to_region())
    .with_hit_test(crate::capability::MosaicHitTestPosture::participates())
}

fn source_backed_package_sizing() -> crate::capability::MosaicSizingContractDescriptor {
    crate::capability::MosaicSizingContractDescriptor::new(
        crate::capability::MosaicSizingContractId::new("workspace.sizing.mosaic_support").unwrap(),
        crate::capability::MosaicSizingKind::fill(),
    )
    .with_measurement_authority(crate::capability::MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(crate::capability::MosaicResizePermission::user_resizable())
    .with_persistence(crate::capability::MosaicSizingPersistence::restorable())
    .with_overflow_behavior(crate::capability::MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(
        crate::capability::MosaicParentGrowthBehavior::does_not_force_parent(),
    )
    .with_viewport_constraint(crate::capability::MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(crate::capability::NamedMeasurementDefinition::new(
        crate::capability::NamedMeasurementToken::new("workspace.measurement.mosaic_support")
            .unwrap(),
        crate::capability::MeasurementValue::logical_pixels(320),
        crate::capability::MeasurementConstraint::between(
            crate::capability::MeasurementValue::logical_pixels(200),
            crate::capability::MeasurementValue::logical_pixels(640),
        ),
    ))
}
