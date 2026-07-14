use crate::{
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
    PhysicalRuntimeVerifierComparison, RuntimeVerifierComparisonClassification,
    RuntimeVerifierDiagnosticDenial, RuntimeVerifierDiagnosticKind,
    RuntimeVerifierDiagnosticReport, RuntimeVerifierRelationship, RuntimeVerifierSupportDenial,
    RuntimeVerifierSupportReport,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    OfflinePhysicalVerifier, OfflineVerifierLayoutObservation, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId,
    PhysicalShortcutBoundary, PhysicalStoreRuntime, PhysicalStoreRuntimeDenial,
    PhysicalStoreRuntimeDenialKind, PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest,
    RuntimeLayoutObservation,
};

#[test]
fn runtime_and_offline_verifier_reports_compare_through_structured_parity() {
    let open_request = PlatformPhysicalOpenRequest::physical_format_canonical();
    let mut facade = open_facade(open_request.clone());
    append_slot(&mut facade, 1);
    let published = facade.publish_physical_root().expect("publish root");
    let runtime_scan = facade.scan_physical_layout().expect("runtime scan");
    let offline_report =
        OfflinePhysicalVerifier::for_canonical_physical_format(open_request.headers().clone())
            .verify(published.persisted_layout())
            .expect("offline verifier");

    let runtime = RuntimeLayoutObservation::from_facade_scan(&runtime_scan);
    let offline = OfflineVerifierLayoutObservation::from_verifier_report(&offline_report);
    let comparison = PhysicalRuntimeVerifierComparison::compare(&runtime, &offline)
        .expect("runtime and verifier agree");
    let support = RuntimeVerifierSupportReport::from_comparison(&comparison)
        .expect("support report from parity");
    let diagnostic = RuntimeVerifierDiagnosticReport::from_comparison(&comparison)
        .expect("diagnostic from parity");
    let evidence = PhysicalOfflineVerifierEvidenceReport::from_runtime_verifier_comparison(
        PhysicalOfflineVerifierEvidenceRow::RuntimeLayoutMatch,
        &comparison,
    )
    .expect("offline evidence from structured comparison");

    assert_eq!(
        comparison.classification(),
        RuntimeVerifierComparisonClassification::Equivalent
    );
    assert_eq!(
        comparison.parity_trace().relationship(),
        RuntimeVerifierRelationship::RuntimeMustMatchVerifier
    );
    assert_eq!(comparison.runtime_semantic_decode_attempts(), 0);
    assert_eq!(comparison.offline_semantic_decode_attempts(), 0);
    assert_eq!(support.semantic_decode_attempts(), 0);
    assert_eq!(
        diagnostic.kind(),
        RuntimeVerifierDiagnosticKind::LayoutParity
    );
    assert_eq!(
        evidence.comparison(),
        Some(RuntimeVerifierComparisonClassification::Equivalent)
    );
}

#[test]
fn controlled_runtime_verifier_mismatch_gets_typed_support_report() {
    let runtime = observed_runtime_layout_with_slot(1);
    let offline = observed_offline_layout_with_slot(2);
    let denial = PhysicalRuntimeVerifierComparison::compare(&runtime, &offline)
        .expect_err("different admitted physical references deny parity");
    let support = RuntimeVerifierSupportReport::from_mismatch(&denial);
    let diagnostic = RuntimeVerifierDiagnosticReport::from_mismatch(&denial);
    let evidence = PhysicalOfflineVerifierEvidenceReport::from_runtime_verifier_mismatch(
        PhysicalOfflineVerifierEvidenceRow::RuntimeDisagreementReported,
        &denial,
    )
    .expect("offline evidence from typed mismatch");

    assert_eq!(
        denial.classification(),
        RuntimeVerifierComparisonClassification::MissingInRuntime
    );
    assert_eq!(
        denial.parity_trace().relationship(),
        RuntimeVerifierRelationship::RuntimeMustDisagreeWithVerifier
    );
    assert_eq!(support.classification(), denial.classification());
    assert_eq!(
        diagnostic.kind(),
        RuntimeVerifierDiagnosticKind::LayoutMismatch
    );
    assert_eq!(evidence.comparison(), Some(denial.classification()));
}

#[test]
fn shortcut_lanes_are_rejected_at_named_boundaries() {
    let mut facade = open_facade(PlatformPhysicalOpenRequest::physical_format_canonical());
    let live_cache = facade
        .reject_live_runtime_cache_shortcut()
        .expect_err("live runtime cache shortcut denied");
    let backend_map = facade
        .reject_backend_private_map_shortcut()
        .expect_err("backend private map shortcut denied");
    let raw_dump = facade
        .reject_raw_debug_dump_shortcut()
        .expect_err("raw debug dump shortcut denied");

    let support = RuntimeVerifierSupportReport::from_shortcut_facade_denial(&live_cache)
        .expect("support from live cache facade denial");
    let diagnostic = RuntimeVerifierDiagnosticReport::from_shortcut_facade_denial(&backend_map)
        .expect("diagnostic from backend map facade denial");
    let raw_dump_diagnostic =
        RuntimeVerifierDiagnosticReport::from_shortcut_facade_denial(&raw_dump)
            .expect("diagnostic from raw dump facade denial");

    assert_eq!(
        live_cache.kind(),
        PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected
    );
    assert_eq!(
        backend_map.kind(),
        PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected
    );
    assert_eq!(
        raw_dump.kind(),
        PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected
    );
    assert_eq!(
        support.forbidden_shortcuts(),
        &[PhysicalShortcutBoundary::LiveRuntimeCache]
    );
    assert_eq!(
        diagnostic.kind(),
        RuntimeVerifierDiagnosticKind::ShortcutRejected
    );
    assert_eq!(
        diagnostic.shortcut_boundary(),
        Some(PhysicalShortcutBoundary::BackendPrivateMap)
    );
    assert_eq!(
        raw_dump_diagnostic.shortcut_boundary(),
        Some(PhysicalShortcutBoundary::RawDebugDump)
    );
}

#[test]
fn shortcut_support_and_diagnostics_require_facade_shortcut_boundary() {
    let missing_record =
        PhysicalStoreRuntimeDenial::new(PhysicalStoreRuntimeDenialKind::MissingPhysicalRecord);

    let support_denial = RuntimeVerifierSupportReport::from_shortcut_facade_denial(&missing_record)
        .expect_err("non-shortcut facade denial rejected for shortcut support");
    let diagnostic_denial =
        RuntimeVerifierDiagnosticReport::from_shortcut_facade_denial(&missing_record)
            .expect_err("non-shortcut facade denial rejected for shortcut diagnostic");

    assert_eq!(
        support_denial,
        RuntimeVerifierSupportDenial::UnexpectedFacadeDenial(
            PhysicalStoreRuntimeDenialKind::MissingPhysicalRecord
        )
    );
    assert_eq!(
        diagnostic_denial,
        RuntimeVerifierDiagnosticDenial::UnexpectedFacadeDenial(
            PhysicalStoreRuntimeDenialKind::MissingPhysicalRecord
        )
    );
}

fn observed_runtime_layout_with_slot(slot_number: u16) -> RuntimeLayoutObservation {
    let mut facade = open_facade(PlatformPhysicalOpenRequest::physical_format_canonical());
    append_slot(&mut facade, slot_number);
    facade.publish_physical_root().expect("publish root");
    let scan = facade.scan_physical_layout().expect("runtime scan");
    RuntimeLayoutObservation::from_facade_scan(&scan)
}

fn observed_offline_layout_with_slot(slot_number: u16) -> OfflineVerifierLayoutObservation {
    let open_request = PlatformPhysicalOpenRequest::physical_format_canonical();
    let mut facade = open_facade(open_request.clone());
    append_slot(&mut facade, slot_number);
    let published = facade.publish_physical_root().expect("publish root");
    let report =
        OfflinePhysicalVerifier::for_canonical_physical_format(open_request.headers().clone())
            .verify(published.persisted_layout())
            .expect("offline verifier");
    OfflineVerifierLayoutObservation::from_verifier_report(&report)
}

fn append_slot(facade: &mut PhysicalStoreRuntime, slot_number: u16) {
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(slot_number),
            b"cert",
        ))
        .expect("append through facade");
}

fn open_facade(open_request: PlatformPhysicalOpenRequest) -> PhysicalStoreRuntime {
    PhysicalStoreRuntime::open_physical_format(readiness(), open_request).expect("open S.1 facade")
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(ROADMAP_2_S1_SCOPE, digest_set())
        .expect("S.1 handoff readiness")
}

fn digest_set() -> HandoffEvidenceDigestSet {
    HandoffEvidenceDigestSet::new(
        digest("backend"),
        digest("deferred"),
        digest("harness"),
        digest("terms"),
        digest("audit"),
        digest("complexity"),
        digest("provenance"),
    )
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).expect("non-empty digest")
}

fn slot_cell(slot_number: u16) -> forge_store_physical_format::SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1), page(1), slot(slot_number))
        .with_slot_generation(generation(5))
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
