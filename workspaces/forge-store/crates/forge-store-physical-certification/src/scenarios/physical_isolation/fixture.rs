use forge_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

pub(super) fn physical_isolation_boundary_fact(
    label: &str,
    segment: u64,
) -> StoreAspectBoundaryFact {
    let key = aspects()
        .vocabulary()
        .key("store.physical.isolation.interleaving.fixture")
        .unwrap();
    let contract = scalar_string_contract(key.clone());
    let value = AspectValue::String(InternedString::from(format!("{label}:{segment}")));
    let validated = match aspects().validate().against(&contract).value(value) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("physical-isolation fixture validation failed: {outcome:?}"),
    };
    let state = match aspects().authoritative_state().admit([validated]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("physical-isolation fixture admission failed: {outcome:?}"),
    };
    let authority = StorePhysicalAuthorityWitness::for_aspect_native_boundary(
        ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    )
    .unwrap();
    let boundary = StorePhysicalBoundaryWitness::from_physical_authority(authority).unwrap();
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(state, boundary),
    )
    .unwrap()
}

fn scalar_string_contract(key: forge_foundational::AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(key)
        .identified_by(aspects().vocabulary().identity(51))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}
