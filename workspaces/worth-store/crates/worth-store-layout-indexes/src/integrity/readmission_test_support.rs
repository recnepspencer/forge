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
use worth_store_recovery_physics::RecoveryLayoutReadmissionWitness;
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub(super) fn import_witness(
    family: crate::PhysicalArtifactFamily,
    seed: &str,
) -> RecoveryLayoutReadmissionWitness {
    let record = unresolved_authority_record(seed);
    let authority = current_authority("store.new.strategy", seed);
    let security = current_security_scope("store.new.strategy", seed);
    worth_store_recovery_physics::layout_readmission()
        .admit_import(family.id(), &record, &authority, security.witnesses())
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
    record_backed_witness_for_store(family, record, "store.new.strategy", seed)
}

pub(super) fn record_backed_witness_for_store(
    family: crate::PhysicalArtifactFamily,
    record: &QuarantineRecord,
    store_authority_key: &str,
    seed: &str,
) -> RecoveryLayoutReadmissionWitness {
    record_backed_witness_for_scope(
        family,
        record,
        store_authority_key,
        seed,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
    )
}

pub(super) fn record_backed_witness_for_scope(
    family: crate::PhysicalArtifactFamily,
    record: &QuarantineRecord,
    store_authority_key: &str,
    seed: &str,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
) -> RecoveryLayoutReadmissionWitness {
    let authority = current_authority(store_authority_key, seed);
    let security = current_security_scope_with(store_authority_key, seed, key_scope, tenant_scope);
    worth_store_recovery_physics::layout_readmission()
        .admit_quarantine(family.id(), record, &authority, security.witnesses())
        .expect("record-backed quarantine witness should admit")
}

pub(super) fn offline_witness(
    family: crate::PhysicalArtifactFamily,
    seed: &str,
) -> worth_store_recovery_physics::RecoveryLayoutReadmissionWitness {
    let admission = super::tests::offline_admission(seed);
    worth_store_recovery_physics::layout_readmission()
        .admit_offline(family.id(), &admission)
        .expect("offline recovery admission issues readmission")
}

pub(super) fn authoritative_quarantine_record(seed: &str) -> QuarantineRecord {
    let finding = ExecutedQuarantineFinding::authoritative_quarantine(test_scope(seed));
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .expect("authoritative quarantine record should seal")
}

pub(super) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

pub(super) fn current_security_scope(
    identity_key: &str,
    value: &str,
) -> StoreAdmittedSecurityScope {
    current_security_scope_with(
        identity_key,
        value,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
    )
}

pub(super) fn current_security_scope_with(
    identity_key: &str,
    value: &str,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, value);
    let authenticity = StoreAuthenticityRequirement::not_required();
    let custody = StoreCustodyPosture::InternalStoreCustody;
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        &authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("current test security scope should admit: {outcome:?}"),
    }
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
        PhysicalGenerationAuthority::for_canonical_physical_format()
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
