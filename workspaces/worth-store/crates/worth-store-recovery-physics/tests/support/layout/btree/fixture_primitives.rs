use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_buffer_pool::{
    PhysicalOperationAllocationScope, PhysicalResidencyLimits, PhysicalResidencyPool,
    PhysicalSpeculativeWorkKind,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceScope,
    PhysicalSegmentId,
};
use worth_store_recovery_physics::{
    LogSequenceNumber, RecoveryCandidateDiscoveryTrace, RecoveryMemoryAllocation, WalLsnRange,
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

pub(super) fn recovery_memory_allocation() -> RecoveryMemoryAllocation {
    let pool = PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0x42; 16]).unwrap(),
        )
        .published_identity(),
        recovery_limits(),
    )
    .unwrap();
    let allocation = pool
        .begin_operation(
            PhysicalOperationAllocationScope::Recovery,
            std::num::NonZeroU64::new(128).unwrap(),
        )
        .unwrap();
    RecoveryMemoryAllocation::from_allocation_grant(allocation).unwrap()
}

fn recovery_limits() -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Speculation;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(5632))
        .resident_bytes(nonzero_bytes(512))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(1))
        .pinned_frames(nonzero_count(1))
        .pin_leases(nonzero_count(1))
        .dirty_frames(nonzero_count(1))
        .dirty_replacement_bytes(nonzero_bytes(512))
        .operation_bytes(nonzero_bytes(512))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(512))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(512))
        .scope_bytes(Scope::Recovery, nonzero_bytes(512))
        .scope_bytes(Scope::Scrub, nonzero_bytes(512))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(512))
        .scope_bytes(Scope::Verification, nonzero_bytes(512))
        .scope_bytes(Scope::Blob, nonzero_bytes(512))
        .speculative_frames(Speculation::Prefetch, nonzero_count(1))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(1))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(1))
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}

fn seed_basis(seed: &str) -> u64 {
    seed.bytes().enumerate().fold(17_u64, |acc, (index, byte)| {
        acc + ((index as u64 + 1) * byte as u64)
    })
}
