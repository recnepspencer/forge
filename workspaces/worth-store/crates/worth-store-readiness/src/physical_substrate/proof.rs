use super::{
    facts::{PhysicalSubstrateEvidenceCounts, PhysicalSubstrateHandoffEvidence},
    PhysicalSubstrateReadiness, PhysicalSubstrateReadinessDenial,
    PhysicalSubstrateReadinessDenialKind,
};
use worth_store_contracts::{AcceptedHandoffReadiness, ROADMAP_2_S1_SCOPE};
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId, PhysicalStoreRuntime,
    PhysicalStoreRuntimeCounterSnapshot, PlatformPhysicalAppendRequest,
    PlatformPhysicalOpenRequest, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug)]
pub struct PhysicalSubstrateCloseoutReceipt {
    scope: worth_store_contracts::RoadmapScope,
    evidence: PhysicalSubstrateHandoffEvidence,
}

impl PhysicalSubstrateCloseoutReceipt {
    pub const fn scope(&self) -> worth_store_contracts::RoadmapScope {
        self.scope
    }

    pub(crate) fn into_handoff_evidence(self) -> PhysicalSubstrateHandoffEvidence {
        self.evidence
    }
}

pub fn close_physical_substrate_readiness(
    readiness: AcceptedHandoffReadiness,
) -> Result<PhysicalSubstrateCloseoutReceipt, PhysicalSubstrateReadinessDenial> {
    if readiness.scope() != ROADMAP_2_S1_SCOPE {
        return Err(PhysicalSubstrateReadinessDenial::new(
            PhysicalSubstrateReadinessDenialKind::WrongRoadmapScope,
        ));
    }
    let scope = readiness.scope();
    let evidence = prove_physical_format_physical_handoff_evidence(readiness)?;
    Ok(PhysicalSubstrateCloseoutReceipt { scope, evidence })
}

pub fn prove_physical_substrate_readiness(
    closeout: PhysicalSubstrateCloseoutReceipt,
) -> Result<PhysicalSubstrateReadiness, PhysicalSubstrateReadinessDenial> {
    if closeout.scope() != ROADMAP_2_S1_SCOPE {
        return Err(PhysicalSubstrateReadinessDenial::new(
            PhysicalSubstrateReadinessDenialKind::WrongRoadmapScope,
        ));
    }
    let scope = closeout.scope();
    PhysicalSubstrateReadiness::from_physical_format_handoff_evidence(
        scope,
        closeout.into_handoff_evidence(),
    )
}

fn prove_physical_format_physical_handoff_evidence(
    readiness: AcceptedHandoffReadiness,
) -> Result<PhysicalSubstrateHandoffEvidence, PhysicalSubstrateReadinessDenial> {
    let reopen_readiness = readiness.clone();
    let mut facade = open_facade(readiness)?;
    let page_append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(1)?,
            b"s2-page-handoff",
        ))
        .map_err(|_| proof_rejected())?;
    let extent_append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            extent_cell(1)?,
            b"s2-extent-handoff",
        ))
        .map_err(|_| proof_rejected())?;
    facade
        .page_access()
        .locate_record(page_append.reference())
        .map_err(|_| proof_rejected())?;
    facade
        .extent_access()
        .read_record(extent_append.reference())
        .map_err(|_| proof_rejected())?;
    let published = facade
        .publish_physical_root()
        .map_err(|_| proof_rejected())?;
    let scan = facade
        .scan_physical_layout()
        .map_err(|_| proof_rejected())?;
    let mut reopened = PhysicalStoreRuntime::reopen(
        reopen_readiness,
        PlatformPhysicalOpenRequest::physical_format_canonical(),
        published.replay_artifact(),
    )
    .map_err(|_| proof_rejected())?;
    reopened
        .page_access()
        .locate_record(page_append.reference())
        .map_err(|_| proof_rejected())?;
    let shortcut_counters = rejected_shortcut_counters(&mut facade)?;
    let physical_references = [page_append.reference(), extent_append.reference()];
    let witnessed = witnessed_payload(11, b"s2-handoff")?;
    PhysicalSubstrateHandoffEvidence::from_physical_format_physical_witnesses(
        &physical_references,
        &[witnessed.0],
        &[witnessed.1],
        PhysicalSubstrateEvidenceCounts::from_physical_format_closeout_evidence(
            scan.runtime_report().discovered_references().len() as u32,
            no_materialization_evidence_count(shortcut_counters),
            counter_evidence_count(facade.counters(), reopened.counters()),
        ),
    )
}

fn open_facade(
    readiness: AcceptedHandoffReadiness,
) -> Result<PhysicalStoreRuntime, PhysicalSubstrateReadinessDenial> {
    PhysicalStoreRuntime::open_physical_format(
        readiness,
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .map_err(|_| proof_rejected())
}

fn rejected_shortcut_counters(
    facade: &mut PhysicalStoreRuntime,
) -> Result<PhysicalStoreRuntimeCounterSnapshot, PhysicalSubstrateReadinessDenial> {
    if facade.reject_full_store_heap_materialization().is_ok()
        || facade.reject_backend_residue_guess().is_ok()
    {
        return Err(proof_rejected());
    }
    Ok(facade.counters())
}

const fn no_materialization_evidence_count(counters: PhysicalStoreRuntimeCounterSnapshot) -> u32 {
    counters.full_store_materialization_rejections() + counters.backend_residue_guess_rejections()
}

const fn counter_evidence_count(
    facade: PhysicalStoreRuntimeCounterSnapshot,
    reopened: PhysicalStoreRuntimeCounterSnapshot,
) -> u32 {
    facade.opens()
        + facade.appends()
        + facade.reads()
        + facade.locates()
        + facade.scans()
        + facade.root_publications()
        + facade.writes()
        + facade.flushes()
        + facade.renames()
        + reopened.opens()
        + reopened.reopens()
        + reopened.locates()
}

fn witnessed_payload(
    generation_value: u64,
    payload: &[u8],
) -> Result<
    (
        worth_store_physical_format::PhysicalHeaderDecodeWitness,
        worth_store_physical_format::PhysicalPayloadViewAdmission<'static>,
    ),
    PhysicalSubstrateReadinessDenial,
> {
    let bytes = Box::leak(header_bytes(generation_value, payload)?.into_boxed_slice());
    let authority = PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().map_err(|_| proof_rejected())?,
    );
    let report = authority
        .decode_frame_header(
            validated_slot_reference(generation_value)?,
            bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .map_err(|_| proof_rejected())?;
    let payload = authority
        .payload_view(bytes, report.witness())
        .map_err(|_| proof_rejected())?;
    Ok((report.witness(), payload))
}

fn validated_slot_reference(
    generation_value: u64,
) -> Result<
    worth_store_physical_format::PhysicalReferenceValidationWitness,
    PhysicalSubstrateReadinessDenial,
> {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1)?, page(1)?, slot(11)?)
        .with_slot_generation(generation(generation_value)?);
    references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .map_err(|_| proof_rejected())
}

fn slot_cell(
    value: u16,
) -> Result<worth_store_physical_format::SlotGenerationCell, PhysicalSubstrateReadinessDenial> {
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1)?, page(1)?, slot(value)?)
        .with_slot_generation(generation(5)?))
}

fn extent_cell(
    value: u64,
) -> Result<worth_store_physical_format::ExtentGenerationCell, PhysicalSubstrateReadinessDenial> {
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .extent_cell(
            segment(1)?,
            worth_store_physical_format::PhysicalExtentId::from_raw(value)
                .map_err(|_| proof_rejected())?,
        )
        .with_extent_generation(generation(7)?))
}

fn header_bytes(
    generation_value: u64,
    payload: &[u8],
) -> Result<Vec<u8>, PhysicalSubstrateReadinessDenial> {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1)?, page(1)?, slot(11)?)
        .with_slot_generation(generation(generation_value)?);
    let binary = PhysicalBinaryEncodingWitness::physical_format_canonical()
        .map_err(|_| proof_rejected())?;
    let authority = PhysicalHeaderAuthority::for_canonical_physical_format(binary);
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(PHYSICAL_HEADER_LENGTH as usize + payload.len())
        .map_err(|_| proof_rejected())?;
    bytes.extend_from_slice(&authority.encode_record_frame_header(
        cell,
        payload.len().try_into().map_err(|_| proof_rejected())?,
    ));
    bytes.extend_from_slice(payload);
    Ok(bytes)
}

fn segment(value: u64) -> Result<PhysicalSegmentId, PhysicalSubstrateReadinessDenial> {
    PhysicalSegmentId::from_raw(value).map_err(|_| proof_rejected())
}

fn page(value: u64) -> Result<PhysicalPageId, PhysicalSubstrateReadinessDenial> {
    PhysicalPageId::from_raw(value).map_err(|_| proof_rejected())
}

fn slot(value: u16) -> Result<PhysicalRecordSlot, PhysicalSubstrateReadinessDenial> {
    PhysicalRecordSlot::from_raw(value).map_err(|_| proof_rejected())
}

fn generation(value: u64) -> Result<PhysicalGeneration, PhysicalSubstrateReadinessDenial> {
    PhysicalGeneration::from_raw(value).map_err(|_| proof_rejected())
}

const fn proof_rejected() -> PhysicalSubstrateReadinessDenial {
    PhysicalSubstrateReadinessDenial::new(
        PhysicalSubstrateReadinessDenialKind::PhysicalSubstrateProofRejected,
    )
}
