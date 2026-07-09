use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceScope,
    PhysicalSegmentId,
};
use worth_store_physical_integrity::{
    AuthorityDamageBoundary, ExecutedQuarantineFinding, PhysicalQuarantineAuthority,
    QuarantineRecord, QuarantineSealRequest,
};
use worth_store_recovery_physics::{
    recovery_readmission_layout_family, RecoveryLayoutReadmissionWitness,
};

pub(super) fn import_witness(
    family: crate::PhysicalArtifactFamily,
    seed: &str,
) -> RecoveryLayoutReadmissionWitness {
    let record = unresolved_authority_record(seed);
    let authority = current_authority("store.s8.import", seed);
    recovery_readmission_layout_family()
        .admit_record_backed_witness(family.id(), &record, &authority)
        .expect("record-backed import witness should admit")
}

pub(super) fn quarantine_witness(
    family: crate::PhysicalArtifactFamily,
    seed: &str,
) -> RecoveryLayoutReadmissionWitness {
    let record = authoritative_quarantine_record(seed);
    record_backed_witness(family, &record, seed)
}

pub(super) fn record_backed_witness(
    family: crate::PhysicalArtifactFamily,
    record: &QuarantineRecord,
    seed: &str,
) -> RecoveryLayoutReadmissionWitness {
    let authority = current_authority("store.s8.corruption", seed);
    recovery_readmission_layout_family()
        .admit_record_backed_witness(family.id(), record, &authority)
        .expect("record-backed quarantine witness should admit")
}

pub(super) fn offline_witness(
    family: crate::PhysicalArtifactFamily,
    seed: &str,
) -> worth_store_recovery_physics::RecoveryLayoutReadmissionWitness {
    let admission = super::tests::offline_admission(seed);
    recovery_readmission_layout_family()
        .admit_offline_witness(family.id(), &admission)
        .expect("offline admission should mint a family-bound lower witness")
}

pub(super) fn authoritative_quarantine_record(seed: &str) -> QuarantineRecord {
    let finding = ExecutedQuarantineFinding::authoritative_quarantine(test_scope(seed));
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .expect("authoritative quarantine record should seal")
}

pub(super) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn unresolved_authority_record(seed: &str) -> QuarantineRecord {
    let finding = ExecutedQuarantineFinding::unresolved_authority(
        test_scope(seed),
        AuthorityDamageBoundary::BackendResidue,
    );
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .expect("unresolved-authority quarantine record should seal")
}

fn test_scope(seed: &str) -> PhysicalReferenceScope {
    PhysicalReferenceScope::derived_index(
        PhysicalGenerationAuthority::s1()
            .page_cell(segment(seed), page(seed))
            .with_page_generation(generation(seed)),
    )
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
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
    .expect("Store boundary fact should admit matching identity")
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
        .expect("test physical authority scope should be valid"),
    )
    .expect("test physical boundary witness should admit")
}

fn segment(seed: &str) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(seed_basis(seed) + 1).expect("test segment id is non-zero")
}

fn page(seed: &str) -> PhysicalPageId {
    PhysicalPageId::from_raw(seed_basis(seed) + 11).expect("test page id is non-zero")
}

fn generation(seed: &str) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(seed_basis(seed) + 5).expect("test generation is non-zero")
}

fn seed_basis(seed: &str) -> u64 {
    seed.bytes().enumerate().fold(17_u64, |acc, (index, byte)| {
        acc + ((index as u64 + 1) * byte as u64)
    })
}
