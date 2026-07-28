use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceScope,
    PhysicalSegmentId,
};
use worth_store_recovery_physics::{
    LogSequenceNumber, RecoveryCandidateDiscoveryTrace, RecoveryMemoryAllocation, WalLsnRange,
};
use worth_store_test_support::harness::physical_residency::PhysicalResidencyStoreWorld;

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

pub(super) fn with_recovery_memory_allocation<R>(
    run: impl FnOnce(RecoveryMemoryAllocation<'_>) -> R,
) -> R {
    let world =
        PhysicalResidencyStoreWorld::initialize("btree-recovery-allocation").expect("Store world");
    let allocation = world
        .serving()
        .physical_allocations()
        .admit_recovery(std::num::NonZeroU64::new(128).expect("fixture bytes are nonzero"))
        .expect("real Store recovery allocation should admit");
    let result = run(RecoveryMemoryAllocation::from_store_allocation(allocation));
    assert!(!world.close().residency().requires_inspection());
    result
}

fn seed_basis(seed: &str) -> u64 {
    seed.bytes().enumerate().fold(17_u64, |acc, (index, byte)| {
        acc + ((index as u64 + 1) * byte as u64)
    })
}
