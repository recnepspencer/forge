use worth_foundational::{
    aspects, AspectMask, AspectValue, ContractValidationInput, InternedString, MutationMask,
    ProjectionMask, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store::aspect_native::{
    StoreAspectContractAdmission, StoreAspectIdentity, StorePhysicalAuthorityWitness,
    StorePhysicalBoundaryWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use worth_store_contracts::ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY;
use worth_store_security::StoreAuthorityBoundSecurityScopeReceipt;

pub(in crate::physical_work) fn admitted_contract(
    revision: u64,
) -> (
    worth_foundational::AspectContract,
    StoreAspectIdentity,
    StoreAspectContractAdmission,
    StorePhysicalBoundaryWitness,
) {
    admitted_named_contract("store.physical.work.lifecycle", 71, revision)
}

pub(in crate::physical_work) fn admitted_named_contract(
    key: &str,
    identity_value: u64,
    revision: u64,
) -> (
    worth_foundational::AspectContract,
    StoreAspectIdentity,
    StoreAspectContractAdmission,
    StorePhysicalBoundaryWitness,
) {
    let key = aspects().vocabulary().key(key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(identity_value))
        .at_revision(aspects().vocabulary().revision(revision))
        .scalar(ScalarAspectType::String);
    let witness = StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap();
    let identity = StoreAspectIdentity::from_aspect_key(key);
    let admission = StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
        .unwrap()
        .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
        .unwrap()
        .admit_mutation_mask(AspectMask::<MutationMask>::whole_aspect())
        .unwrap();
    (contract, identity, admission, witness)
}

pub(in crate::physical_work) fn alternative_physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary_instance(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
            ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY,
        )
        .unwrap(),
    )
    .unwrap()
}

pub(in crate::physical_work) fn security_scope(
    witness: StorePhysicalBoundaryWitness,
) -> StoreAuthorityBoundSecurityScopeReceipt {
    worth_store_security::admitted_store_internal_security_scope_for_physical_witness_test(witness)
        .authority_bound_receipt()
}

pub(in crate::physical_work) fn security_scope_from_authority(
    authority_key: &str,
    witness: StorePhysicalBoundaryWitness,
) -> StoreAuthorityBoundSecurityScopeReceipt {
    worth_store_security::admitted_store_internal_security_scope_for_named_physical_witness_test(
        authority_key,
        witness,
    )
    .authority_bound_receipt()
}

pub(in crate::physical_work) fn validated_value(
    contract: &worth_foundational::AspectContract,
    value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(ContractValidationInput::from(AspectValue::String(
            InternedString::from(value),
        ))) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("value should validate: {outcome:?}"),
    }
}
