use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{
    PhysicalAuthorityBoundaryInstance, ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

pub(crate) fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    current_authority_for_boundary(label, ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY)
}

pub(crate) fn current_authority_for_boundary(
    label: &str,
    boundary_instance: PhysicalAuthorityBoundaryInstance,
) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current", boundary_instance))
}

fn boundary_fact(
    identity_key: &str,
    value: &str,
    boundary_instance: PhysicalAuthorityBoundaryInstance,
) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness(boundary_instance)),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
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

fn physical_witness(
    boundary_instance: PhysicalAuthorityBoundaryInstance,
) -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary_instance(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
            boundary_instance,
        )
        .unwrap(),
    )
    .unwrap()
}
