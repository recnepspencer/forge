use crate::physical_scope_admission_test_support::{
    extent_validation, root_with_extent, scope_membership, with_checked_frame,
};
use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest,
    BackgroundWorkBudgetSnapshot, BackgroundWorkClass, FixedMetadataReservation,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ChunkDamageLocality, ChunkIntegrityAuthority, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityInspectionRequest, ChunkIntegrityStreamingWindow,
    ChunkIntegrityStreamingWindowDenial, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    ScopedPhysicalValidatorInput,
};

#[test]
fn independent_bounded_chunk_windows_converge_without_blob_lifecycle_claims() {
    let first = inspect_intact_chunk_window();
    let second = inspect_intact_chunk_window();

    assert_eq!(first, second);
    assert_eq!(first.counters().protected_window_reads(), 1);
    assert_eq!(first.counters().streaming_windows_planned(), 4);
    assert_eq!(first.counters().chunk_header_checks(), 1);
    assert_eq!(first.counters().chunk_payload_checks(), 1);
    assert_eq!(first.counters().chunk_boundary_checks(), 1);
    assert_eq!(first.counters().extent_boundary_checks(), 1);
    assert_eq!(first.counters().skipped_whole_object_reads(), 1);
    assert!(!first.lifecycle_claims().claims_dedupe_correctness());
    assert!(!first.lifecycle_claims().claims_reachability());
    assert!(!first.lifecycle_claims().claims_resumability());
    assert!(!first.lifecycle_claims().claims_blob_retention());
}

#[test]
fn chunk_damage_localizes_header_payload_chunk_boundary_and_extent_boundary() {
    assert_chunk_damage(
        chunk_payload("header-damage", b"head"),
        ChunkIntegrityDenialKind::ChunkHeaderDamage,
        (1, 0, 0, 0),
        |locality| matches!(locality, ChunkDamageLocality::ChunkHeader(_)),
    );
    assert_chunk_damage(
        chunk_payload("payload-damage", b"body"),
        ChunkIntegrityDenialKind::ChunkPayloadDamage,
        (1, 1, 0, 0),
        |locality| matches!(locality, ChunkDamageLocality::ChunkPayload(_)),
    );
    assert_chunk_damage(
        chunk_payload("chunk-boundary-damage", b"edge"),
        ChunkIntegrityDenialKind::ChunkBoundaryDamage,
        (1, 1, 1, 0),
        |locality| matches!(locality, ChunkDamageLocality::ChunkBoundary(_)),
    );
    assert_chunk_damage(
        chunk_payload("extent-boundary-damage", b"extent"),
        ChunkIntegrityDenialKind::ExtentBoundaryDamage,
        (1, 1, 1, 1),
        |locality| matches!(locality, ChunkDamageLocality::ExtentBoundary(_)),
    );
}

#[test]
fn whole_object_window_is_denied_before_chunk_inspection() {
    let envelope = admitted_streaming_envelope(4096, 4096);

    assert!(ChunkIntegrityStreamingWindow::from_admitted_streaming_envelope(envelope).is_err());
}

#[test]
fn non_streaming_background_envelope_cannot_mint_chunk_window() {
    let denial = ChunkIntegrityStreamingWindow::from_admitted_streaming_envelope(
        admitted_non_streaming_envelope(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ChunkIntegrityStreamingWindowDenial::WrongBackgroundEnvelopeClass {
            actual: BackgroundWorkClass::ScrubPlanning
        }
    );
}

#[test]
fn protected_bytes_cannot_exceed_admitted_streaming_window() {
    let denial = inspect_unknown_chunk_denial();

    assert_eq!(
        denial.kind(),
        ChunkIntegrityDenialKind::ProtectedWindowExceedsStreamingWindow
    );
    assert!(matches!(
        denial.damage_locality(),
        Some(ChunkDamageLocality::Unknown(_))
    ));
    assert_eq!(denial.counters().inspected_bytes(), 12);
    assert_eq!(denial.counters().streaming_windows_planned(), 1024);
    assert_eq!(denial.counters().chunk_header_checks(), 0);
    assert_eq!(denial.counters().chunk_payload_checks(), 0);
    assert_eq!(denial.counters().chunk_boundary_checks(), 0);
    assert_eq!(denial.counters().extent_boundary_checks(), 0);
}

pub(crate) fn inspect_unknown_chunk_denial() -> ChunkIntegrityDenial {
    inspect_chunk_denial("ok", b"DATA", 4)
}

pub(crate) fn inspect_chunk_denial(
    status: &str,
    body: &[u8],
    window_bytes: u64,
) -> ChunkIntegrityDenial {
    let mut denial = None;
    with_chunk_input(chunk_payload(status, body), |input| {
        let request = ChunkIntegrityInspectionRequest::from_admitted_chunk_window(
            input,
            admitted_chunk_window(4096, window_bytes),
        )
        .unwrap();
        denial = Some(ChunkIntegrityAuthority::s3().inspect(request).unwrap_err());
    });
    denial.unwrap()
}

fn inspect_intact_chunk_window() -> forge_store_physical_integrity::ChunkIntegrityReport {
    let mut report = None;
    with_chunk_input(chunk_payload("ok", b"DATA"), |input| {
        let request = ChunkIntegrityInspectionRequest::from_admitted_chunk_window(
            input,
            admitted_chunk_window(4096, 1024),
        )
        .unwrap();
        report = Some(ChunkIntegrityAuthority::s3().inspect(request).unwrap());
    });
    report.unwrap()
}

fn assert_chunk_damage(
    payload: Vec<u8>,
    expected_kind: ChunkIntegrityDenialKind,
    expected_checks: (u32, u32, u32, u32),
    expected_locality: impl FnOnce(ChunkDamageLocality) -> bool,
) {
    let mut denial = None;
    with_chunk_input(payload, |input| {
        let request = ChunkIntegrityInspectionRequest::from_admitted_chunk_window(
            input,
            admitted_chunk_window(4096, 1024),
        )
        .unwrap();
        denial = Some(ChunkIntegrityAuthority::s3().inspect(request).unwrap_err());
    });
    let denial = denial.unwrap();

    assert_eq!(denial.kind(), expected_kind);
    assert!(expected_locality(denial.damage_locality().unwrap()));
    assert_eq!(denial.counters().chunk_header_checks(), expected_checks.0);
    assert_eq!(denial.counters().chunk_payload_checks(), expected_checks.1);
    assert_eq!(denial.counters().chunk_boundary_checks(), expected_checks.2);
    assert_eq!(
        denial.counters().extent_boundary_checks(),
        expected_checks.3
    );
    assert_eq!(denial.counters().skipped_whole_object_reads(), 1);
}

fn with_chunk_input(payload: Vec<u8>, run: impl FnOnce(ScopedPhysicalValidatorInput<'_>)) {
    let validation = extent_validation(1, 9, 7);
    let scope = PhysicalReferenceScope::chunk_like(validation);
    let root = root_with_extent(1, 9, 7);
    let membership = scope_membership(&root, scope);
    with_checked_frame(&payload, validation, |checked| {
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        run(ScopedPhysicalValidatorInput::chunk_like(admission).unwrap());
    });
}

fn chunk_payload(status: &str, body: &[u8]) -> Vec<u8> {
    let mut payload = format!("CHNK|{status}|").into_bytes();
    payload.extend_from_slice(body);
    payload
}

fn admitted_chunk_window(object_bytes: u64, window_bytes: u64) -> ChunkIntegrityStreamingWindow {
    ChunkIntegrityStreamingWindow::from_admitted_streaming_envelope(admitted_streaming_envelope(
        object_bytes,
        window_bytes,
    ))
    .unwrap()
}

fn admitted_streaming_envelope(object_bytes: u64, window_bytes: u64) -> AdmittedBackgroundEnvelope {
    BackgroundEnvelopeAdmission::new()
        .admit(
            BackgroundEnvelopeRequest::large_record_streaming()
                .resident_frames(1)
                .resident_bytes(window_bytes)
                .pin_pages_for_bounded_step(1)
                .allocation_bytes(window_bytes)
                .copied_bytes(window_bytes)
                .streaming_window(object_bytes, window_bytes)
                .finish(),
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation_admission(window_bytes),
        )
        .unwrap()
}

fn admitted_non_streaming_envelope() -> AdmittedBackgroundEnvelope {
    BackgroundEnvelopeAdmission::new()
        .admit(
            BackgroundEnvelopeRequest::scrub_planning()
                .resident_frames(1)
                .resident_bytes(128)
                .pin_pages_for_bounded_step(1)
                .allocation_bytes(128)
                .finish(),
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation_admission(512),
        )
        .unwrap()
}

fn allocation_admission(window_bytes: u64) -> AllocationAdmission {
    AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(window_bytes))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    )
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}
