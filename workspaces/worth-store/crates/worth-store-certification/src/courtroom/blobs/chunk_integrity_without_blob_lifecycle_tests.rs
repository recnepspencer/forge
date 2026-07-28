use std::num::NonZeroU64;

use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    extent_validation, root_with_extent, scope_membership, with_store_checked_frame,
};
use worth_store::physical_runtime::{BlobPhysicalAllocation, ServingPhysicalRuntime};
use worth_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use worth_store_physical_integrity::{
    ChunkDamageLocality, ChunkIntegrityAuthority, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityInspectionRequest, ChunkIntegrityStreamingWindow,
    ChunkIntegrityStreamingWindowDenial, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    ScopedPhysicalValidatorInput,
};
use worth_store_test_support::harness::physical_residency::PhysicalResidencyStoreWorld;

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
    with_blob_allocation(4096, |allocation| {
        let denial =
            ChunkIntegrityStreamingWindow::admit(allocation, 4096, nonzero(4096)).unwrap_err();
        assert_eq!(
            denial,
            ChunkIntegrityStreamingWindowDenial::WholeObjectWindow
        );
    });
}

#[test]
fn window_larger_than_the_exact_blob_allocation_is_denied() {
    with_blob_allocation(512, |allocation| {
        let denial =
            ChunkIntegrityStreamingWindow::admit(allocation, 4096, nonzero(1024)).unwrap_err();
        assert_eq!(
            denial,
            ChunkIntegrityStreamingWindowDenial::WindowExceedsBlobAllocation {
                requested: 1024,
                allocation: 512,
            }
        );
    });
}

#[test]
fn blob_allocation_from_another_store_is_denied_before_chunk_inspection() {
    let mut observed = None;
    with_store_chunk_input(chunk_payload("ok", b"DATA"), |_, input| {
        let other = PhysicalResidencyStoreWorld::initialize("foreign-blob-allocation").unwrap();
        let allocation = other
            .serving()
            .physical_allocations()
            .admit_blob(nonzero(1024))
            .unwrap();
        let window = ChunkIntegrityStreamingWindow::admit(&allocation, 4096, nonzero(1024))
            .expect("the foreign allocation is internally large enough");
        observed = Some(
            ChunkIntegrityInspectionRequest::from_store_blob_window(input, window).unwrap_err(),
        );
        drop(allocation);
        assert!(!other.close().residency().requires_inspection());
    });

    assert_eq!(
        observed.unwrap().kind(),
        ChunkIntegrityDenialKind::BlobAllocationStoreMismatch
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
    with_chunk_input(
        chunk_payload(status, body),
        4096,
        window_bytes,
        window_bytes,
        |input, window| {
            let request =
                ChunkIntegrityInspectionRequest::from_store_blob_window(input, window).unwrap();
            denial = Some(ChunkIntegrityAuthority::new().inspect(request).unwrap_err());
        },
    );
    denial.unwrap()
}

fn inspect_intact_chunk_window() -> worth_store_physical_integrity::ChunkIntegrityReport {
    let mut report = None;
    with_chunk_input(
        chunk_payload("ok", b"DATA"),
        4096,
        1024,
        1024,
        |input, window| {
            let request =
                ChunkIntegrityInspectionRequest::from_store_blob_window(input, window).unwrap();
            report = Some(ChunkIntegrityAuthority::new().inspect(request).unwrap());
        },
    );
    report.unwrap()
}

fn assert_chunk_damage(
    payload: Vec<u8>,
    expected_kind: ChunkIntegrityDenialKind,
    expected_checks: (u32, u32, u32, u32),
    expected_locality: impl FnOnce(ChunkDamageLocality) -> bool,
) {
    let mut denial = None;
    with_chunk_input(payload, 4096, 1024, 1024, |input, window| {
        let request =
            ChunkIntegrityInspectionRequest::from_store_blob_window(input, window).unwrap();
        denial = Some(ChunkIntegrityAuthority::new().inspect(request).unwrap_err());
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

fn with_chunk_input(
    payload: Vec<u8>,
    object_bytes: u64,
    window_bytes: u64,
    allocation_bytes: u64,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>, ChunkIntegrityStreamingWindow<'_, '_>),
) {
    with_store_chunk_input(payload, |serving, input| {
        let allocation = serving
            .physical_allocations()
            .admit_blob(nonzero(allocation_bytes))
            .unwrap();
        let window =
            ChunkIntegrityStreamingWindow::admit(&allocation, object_bytes, nonzero(window_bytes))
                .unwrap();
        run(input, window);
    });
}

fn with_store_chunk_input(
    payload: Vec<u8>,
    run: impl FnOnce(&ServingPhysicalRuntime, ScopedPhysicalValidatorInput<'_>),
) {
    let validation = extent_validation(1, 9, 7);
    let scope = PhysicalReferenceScope::chunk_like(validation);
    let root = root_with_extent(1, 9, 7);
    let membership = scope_membership(&root, scope);
    with_store_checked_frame(&payload, validation, |serving, checked| {
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        run(
            serving,
            ScopedPhysicalValidatorInput::chunk_like(admission).unwrap(),
        );
    });
}

fn with_blob_allocation(bytes: u64, run: impl FnOnce(&BlobPhysicalAllocation<'_>)) {
    let world = PhysicalResidencyStoreWorld::initialize("blob-window-allocation").unwrap();
    let allocation = world
        .serving()
        .physical_allocations()
        .admit_blob(nonzero(bytes))
        .unwrap();
    run(&allocation);
    drop(allocation);
    assert!(!world.close().residency().requires_inspection());
}

fn chunk_payload(status: &str, body: &[u8]) -> Vec<u8> {
    let mut payload = format!("CHNK|{status}|").into_bytes();
    payload.extend_from_slice(body);
    payload
}

fn nonzero(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}
