use crate::{
    s2_readiness_facts::{S2PhysicalSubstrateEvidenceCounts, S2PhysicalSubstrateHandoffEvidence},
    S2PhysicalSubstrateReadiness, S2ReadinessDenial, S2ReadinessDenialKind,
};
use forge_store_contracts::{AcceptedHandoffReadiness, ROADMAP_2_S1_SCOPE};
use forge_store_layout_indexes::layout_strategy_admission::{
    phase19_extent_rule, phase19_page_rule,
};
use forge_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId, PhysicalPublicationState,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalOpenRequest, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug)]
pub struct S1PhysicalSubstrateCloseoutReceipt {
    scope: forge_store_contracts::RoadmapScope,
    evidence: S2PhysicalSubstrateHandoffEvidence,
}

impl S1PhysicalSubstrateCloseoutReceipt {
    pub const fn scope(&self) -> forge_store_contracts::RoadmapScope {
        self.scope
    }

    pub(crate) fn into_handoff_evidence(self) -> S2PhysicalSubstrateHandoffEvidence {
        self.evidence
    }
}

pub fn close_s1_physical_substrate_readiness(
    readiness: AcceptedHandoffReadiness,
) -> Result<S1PhysicalSubstrateCloseoutReceipt, S2ReadinessDenial> {
    if readiness.scope() != ROADMAP_2_S1_SCOPE {
        return Err(S2ReadinessDenial::new(
            S2ReadinessDenialKind::WrongRoadmapScope,
        ));
    }
    let scope = readiness.scope();
    let evidence = prove_s1_physical_handoff_evidence(readiness)?;
    Ok(S1PhysicalSubstrateCloseoutReceipt { scope, evidence })
}

pub fn prove_s2_physical_substrate_readiness(
    closeout: S1PhysicalSubstrateCloseoutReceipt,
) -> Result<S2PhysicalSubstrateReadiness, S2ReadinessDenial> {
    if closeout.scope() != ROADMAP_2_S1_SCOPE {
        return Err(S2ReadinessDenial::new(
            S2ReadinessDenialKind::WrongRoadmapScope,
        ));
    }
    let scope = closeout.scope();
    S2PhysicalSubstrateReadiness::from_s1_handoff_evidence(scope, closeout.into_handoff_evidence())
}

fn prove_s1_physical_handoff_evidence(
    readiness: AcceptedHandoffReadiness,
) -> Result<S2PhysicalSubstrateHandoffEvidence, S2ReadinessDenial> {
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
    let page_rule = phase19_page_rule().map_err(|_| proof_rejected())?;
    let extent_rule = phase19_extent_rule().map_err(|_| proof_rejected())?;
    facade
        .page_layout(&page_rule)
        .map_err(|_| proof_rejected())?
        .locate_record(page_append.reference())
        .map_err(|_| proof_rejected())?;
    facade
        .extent_layout(&extent_rule)
        .map_err(|_| proof_rejected())?
        .read_record(extent_append.reference())
        .map_err(|_| proof_rejected())?;
    let published = facade
        .publish_physical_root()
        .map_err(|_| proof_rejected())?;
    let scan = facade
        .scan_physical_layout()
        .map_err(|_| proof_rejected())?;
    let mut reopened = PlatformPhysicalFacade::reopen_s1(
        reopen_readiness,
        PlatformPhysicalOpenRequest::s1_canonical(),
        published.replay_artifact(),
    )
    .map_err(|_| proof_rejected())?;
    reopened
        .page_layout(&page_rule)
        .map_err(|_| proof_rejected())?
        .locate_record(page_append.reference())
        .map_err(|_| proof_rejected())?;
    let shortcut_counters = rejected_shortcut_counters(&mut facade)?;
    let physical_references = [page_append.reference(), extent_append.reference()];
    let witnessed = witnessed_payload(11, b"s2-handoff")?;
    S2PhysicalSubstrateHandoffEvidence::from_s1_physical_witnesses(
        &physical_references,
        &[witnessed.0],
        &[witnessed.1],
        S2PhysicalSubstrateEvidenceCounts::from_s1_closeout_evidence(
            scan.runtime_report().discovered_references().len() as u32,
            no_materialization_evidence_count(shortcut_counters),
            counter_evidence_count(facade.counters(), reopened.counters()),
        ),
    )
}

fn open_facade(
    readiness: AcceptedHandoffReadiness,
) -> Result<PlatformPhysicalFacade, S2ReadinessDenial> {
    PlatformPhysicalFacade::open_s1(readiness, PlatformPhysicalOpenRequest::s1_canonical())
        .map_err(|_| proof_rejected())
}

fn rejected_shortcut_counters(
    facade: &mut PlatformPhysicalFacade,
) -> Result<PlatformPhysicalFacadeCounterSnapshot, S2ReadinessDenial> {
    if facade.reject_full_store_heap_materialization().is_ok()
        || facade.reject_backend_residue_guess().is_ok()
    {
        return Err(proof_rejected());
    }
    Ok(facade.counters())
}

const fn no_materialization_evidence_count(counters: PlatformPhysicalFacadeCounterSnapshot) -> u32 {
    counters.full_store_materialization_rejections() + counters.backend_residue_guess_rejections()
}

const fn counter_evidence_count(
    facade: PlatformPhysicalFacadeCounterSnapshot,
    reopened: PlatformPhysicalFacadeCounterSnapshot,
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
        forge_store_physical_format::PhysicalHeaderDecodeWitness,
        forge_store_physical_format::PhysicalPayloadViewAdmission<'static>,
    ),
    S2ReadinessDenial,
> {
    let bytes = Box::leak(header_bytes(generation_value, payload).into_boxed_slice());
    let authority = PhysicalHeaderAuthority::s1(
        PhysicalBinaryEncodingWitness::s1_canonical().map_err(|_| proof_rejected())?,
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
) -> Result<forge_store_physical_format::PhysicalReferenceValidationWitness, S2ReadinessDenial> {
    let references = PhysicalReferenceAuthority::s1();
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(segment(1)?, page(1)?, slot(11)?)
        .with_slot_generation(generation(generation_value)?);
    references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .map_err(|_| proof_rejected())
}

fn slot_cell(
    value: u16,
) -> Result<forge_store_physical_format::SlotGenerationCell, S2ReadinessDenial> {
    Ok(PhysicalGenerationAuthority::s1()
        .slot_cell(segment(1)?, page(1)?, slot(value)?)
        .with_slot_generation(generation(5)?))
}

fn extent_cell(
    value: u64,
) -> Result<forge_store_physical_format::ExtentGenerationCell, S2ReadinessDenial> {
    Ok(PhysicalGenerationAuthority::s1()
        .extent_cell(
            segment(1)?,
            forge_store_physical_format::PhysicalExtentId::from_raw(value)
                .map_err(|_| proof_rejected())?,
        )
        .with_extent_generation(generation(7)?))
}

fn header_bytes(generation_value: u64, payload: &[u8]) -> Vec<u8> {
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

fn segment(value: u64) -> Result<PhysicalSegmentId, S2ReadinessDenial> {
    PhysicalSegmentId::from_raw(value).map_err(|_| proof_rejected())
}

fn page(value: u64) -> Result<PhysicalPageId, S2ReadinessDenial> {
    PhysicalPageId::from_raw(value).map_err(|_| proof_rejected())
}

fn slot(value: u16) -> Result<PhysicalRecordSlot, S2ReadinessDenial> {
    PhysicalRecordSlot::from_raw(value).map_err(|_| proof_rejected())
}

fn generation(value: u64) -> Result<PhysicalGeneration, S2ReadinessDenial> {
    PhysicalGeneration::from_raw(value).map_err(|_| proof_rejected())
}

const fn proof_rejected() -> S2ReadinessDenial {
    S2ReadinessDenial::new(S2ReadinessDenialKind::S1PhysicalSubstrateProofRejected)
}
