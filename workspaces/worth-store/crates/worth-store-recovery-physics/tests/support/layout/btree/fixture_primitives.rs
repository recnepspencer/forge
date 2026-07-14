use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration,
    BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest, BackgroundWorkBudgetSnapshot,
    FixedMetadataReservation,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceScope,
    PhysicalSegmentId,
};
use worth_store_recovery_physics::{
    LogSequenceNumber, RecoveryCandidateDiscoveryTrace, RecoveryMemoryEnvelope, WalLsnRange,
};

pub fn wal_range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap()
}

pub(super) fn trace(label: &str, order: u64) -> RecoveryCandidateDiscoveryTrace {
    RecoveryCandidateDiscoveryTrace::new("btree-recovery-profile", label, order)
}

pub(super) fn test_scope(seed: &str) -> PhysicalReferenceScope {
    PhysicalReferenceScope::derived_index(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .page_cell(segment(seed_basis(seed) + 1), page(seed_basis(seed) + 11))
            .with_page_generation(generation(seed_basis(seed) + 5)),
    )
}

pub(super) fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .unwrap()
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

pub(super) fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

pub(super) fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

pub(super) fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}

pub(super) fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    RecoveryMemoryEnvelope::from_admitted(admit_background()).unwrap()
}

fn admit_background() -> worth_store_buffer_pool::AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    admission
        .admit(
            BackgroundEnvelopeRequest::recovery_planning()
                .resident_frames(1)
                .resident_bytes(128)
                .pin_pages_for_bounded_step(1)
                .allocation_bytes(128)
                .finish(),
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation,
        )
        .unwrap()
}

fn allocation_admission() -> AllocationAdmission {
    AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(512))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    )
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}

fn seed_basis(seed: &str) -> u64 {
    seed.bytes().enumerate().fold(17_u64, |acc, (index, byte)| {
        acc + ((index as u64 + 1) * byte as u64)
    })
}
