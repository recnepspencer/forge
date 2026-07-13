use crate::{SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow};
use forge_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationRequest,
    AllocationScope, DirtyPageCount, FixedMetadataReservation, PrefetchRequest, PrefetchWindow,
    ReadAheadRequest, SpeculativePhysicalWorkAdmission, SpeculativePhysicalWorkDenialKind,
    WriteBehindRequest,
};

#[test]
fn speculative_work_evidence_accepts_replay_lowering_and_no_qos_claim() {
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();
    let table = speculative_work_evidence_tests_support::resident_frame_table();
    let plan = admission
        .lower_read_ahead(
            ReadAheadRequest::new(PrefetchWindow::resident_frames(1).unwrap(), None),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let plan_report = SpeculativeWorkEvidenceReport::from_read_ahead_plan(
        SpeculativeWorkEvidenceRow::ReplayStablePlanLoweredBeforeExecution,
        &plan,
    )
    .unwrap();
    let receipt = admission
        .record_read_ahead_admitted(plan, &mut allocation)
        .unwrap();
    let qos_report = SpeculativeWorkEvidenceReport::from_read_ahead_admission(
        SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim,
        receipt,
    )
    .unwrap();

    assert_eq!(
        plan_report.row(),
        SpeculativeWorkEvidenceRow::ReplayStablePlanLoweredBeforeExecution
    );
    assert_eq!(
        qos_report.row(),
        SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim
    );
}

#[test]
fn speculative_work_evidence_accepts_prefetch_and_write_behind_no_qos_honesty() {
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();
    let mut table = speculative_work_evidence_tests_support::resident_frame_table();
    let dirty =
        speculative_work_evidence_tests_support::admit_payload_frame(&mut table, 7, 2, b"dirty");
    table.mark_dirty(dirty.resident_frame_token()).unwrap();
    let prefetch = admission
        .lower_prefetch(
            PrefetchRequest::new(PrefetchWindow::resident_frames(1).unwrap(), None),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let write_behind = admission
        .lower_write_behind(
            WriteBehindRequest::dirty_pages(DirtyPageCount::from_observed_pages(1), None).unwrap(),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let prefetch_receipt = admission
        .record_prefetch_admitted(prefetch, &mut allocation)
        .unwrap();
    let write_behind_receipt = admission
        .record_write_behind_admitted(write_behind, &mut allocation)
        .unwrap();

    let prefetch_report = SpeculativeWorkEvidenceReport::from_prefetch_honesty(
        SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim,
        prefetch_receipt,
    )
    .unwrap();
    let write_behind_report = SpeculativeWorkEvidenceReport::from_write_behind_honesty(
        SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim,
        write_behind_receipt,
    )
    .unwrap();

    assert_eq!(
        prefetch_report.row(),
        SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim
    );
    assert_eq!(
        write_behind_report.row(),
        SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim
    );
}

#[test]
fn speculative_work_evidence_accepts_denial_before_scheduling() {
    let table = speculative_work_evidence_tests_support::resident_frame_table();
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();
    let request = ReadAheadRequest::new(
        PrefetchWindow::resident_frames(1).unwrap(),
        Some(AllocationRequest::background_work_memory(AllocationScope::Foreground, 8).unwrap()),
    );

    let denial = admission
        .lower_read_ahead(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap_err();
    let report = SpeculativeWorkEvidenceReport::from_denial(
        SpeculativeWorkEvidenceRow::DenialBeforeScheduling,
        denial,
    )
    .unwrap();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::ForegroundAllocationInterference { requested_bytes: 8 }
    );
    assert_eq!(report.counters().read_ahead_denied_count(), 1);
}

#[test]
fn speculative_work_evidence_rejects_unsupported_qos_as_physical_substrate_denial_proof() {
    let mut admission = SpeculativePhysicalWorkAdmission::new();
    let denial = admission.reject_unsupported_qos_claim();

    let report = SpeculativeWorkEvidenceReport::from_denial(
        SpeculativeWorkEvidenceRow::DenialBeforeScheduling,
        denial,
    );

    assert!(report.is_err());
    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::UnsupportedQosClaim
    );
}

fn allocation_admission(bytes: u64) -> AllocationAdmission {
    let budget = AllocationByteBudget::bytes(bytes).unwrap();
    let declaration = AllocationEnvelopeDeclaration::declare()
        .foreground(budget)
        .maintenance(budget)
        .recovery(budget)
        .scrub(budget)
        .import_export(budget)
        .streaming(budget)
        .fixed_metadata(FixedMetadataReservation::constant_bytes(8).unwrap())
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(declaration)
}

mod speculative_work_evidence_tests_support {
    use forge_store_buffer_pool::{
        BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameAdmission,
        ResidentFrameLoadRequest, ResidentFrameTable, ResidentFrameTableCapacity,
        ResidentMemoryBudget, S2PhysicalResidencyEntry,
    };
    use forge_store_contracts::{
        AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
    };
    use forge_store_physical_format::{
        PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
        PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
        PhysicalPageId, PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
        PhysicalReferenceValidationWitness, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
    };
    use forge_store_readiness::{
        close_physical_substrate_readiness, prove_physical_substrate_readiness,
    };

    pub fn resident_frame_table() -> ResidentFrameTable {
        let readiness = prove_physical_substrate_readiness(
            close_physical_substrate_readiness(accepted_physical_format_readiness()).unwrap(),
        )
        .unwrap();
        let budget = BufferPoolBudget::declare(
            ResidentMemoryBudget::bytes(8192).unwrap(),
            PinnedPageBudget::pages(4).unwrap(),
            DirtyPageBudget::pages(2).unwrap(),
        );
        let admitted = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
            readiness.physical_substrate_snapshot(),
        )
        .unwrap()
        .with_budget(budget)
        .admit()
        .unwrap();
        ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(4).unwrap())
    }

    pub fn admit_payload_frame(
        table: &mut ResidentFrameTable,
        generation_value: u64,
        page_value: u64,
        payload: &[u8],
    ) -> ResidentFrameAdmission {
        let frame = frame_bytes(generation_value, payload);
        let request = load_request_from_frame(generation_value, page_value, &frame);
        let payload = header_authority()
            .payload_view(&frame, request.header())
            .unwrap();
        table.admit_resident_frame_bytes(request, payload).unwrap()
    }

    fn load_request_from_frame(
        generation_value: u64,
        page_value: u64,
        frame_bytes: &[u8],
    ) -> ResidentFrameLoadRequest {
        ResidentFrameLoadRequest::from_physical_format_physical_frame(
            validated_slot_reference(generation_value, page_value),
            frame_header_witness(generation_value, page_value, frame_bytes),
        )
        .unwrap()
    }

    fn validated_slot_reference(
        generation_value: u64,
        page_value: u64,
    ) -> PhysicalReferenceValidationWitness {
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
        let cell = generations
            .slot_cell(segment(1), page(page_value), slot(3))
            .with_slot_generation(generation(generation_value));
        let admitted = references.admit_page_slot(cell);
        references.validate_page_slot(admitted, cell).unwrap()
    }

    fn frame_header_witness(
        generation_value: u64,
        page_value: u64,
        bytes: &[u8],
    ) -> PhysicalHeaderDecodeWitness {
        header_authority()
            .decode_frame_header(
                validated_slot_reference(generation_value, page_value),
                bytes,
                PhysicalFrameKind::RecordFrame,
            )
            .unwrap()
            .witness()
    }

    fn header_authority() -> PhysicalHeaderAuthority {
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
        )
    }

    fn frame_bytes(generation_value: u64, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
        bytes.push(PhysicalFrameKind::RecordFrame.tag());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&generation_value.to_le_bytes());
        bytes.push(PhysicalPublicationState::Published.code());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(payload);
        bytes
    }

    fn accepted_physical_format_readiness() -> AcceptedHandoffReadiness {
        AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
            ROADMAP_2_S1_SCOPE,
            HandoffEvidenceDigestSet::new(
                digest("backend"),
                digest("deferred"),
                digest("harness"),
                digest("terms"),
                digest("audit"),
                digest("complexity"),
                digest("provenance"),
            ),
        )
        .unwrap()
    }

    fn digest(name: &str) -> StableDigest {
        StableDigest::new(format!("sha256:{name}")).unwrap()
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
}
