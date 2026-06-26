use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectIdentity, StoreAspectNativeDenial, StoreAspectPatchAuthorityInput,
    StoreAspectPatchBoundaryFact, StorePhysicalBoundaryWitness, StoreValidatedAspectValueAdmission,
};
use forge_store_authority::admit_aspect_native_authority_record;
use forge_store_contracts::{
    PhysicalAuthorityScope, StableArtifactId, StorePhysicalAuthorityWitness,
    ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE, ROADMAP_2_S1_SCOPE,
};
use forge_store_readiness::{
    AspectNativeVocabularyFamily, AspectNativeVocabularyPosture,
    StoreAspectNativeVocabularyReadiness,
};

#[test]
fn store_boundary_values_are_foundational_aspect_values() {
    let aspect_key = aspect_key("store.physical.segment.identity");
    let contract = scalar_string_contract(aspect_key.clone());
    let validated_value = validated_scalar_value(&contract, "segment-0001");
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_value.clone()])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    let physical_witness = physical_witness();
    let identity = StoreAspectIdentity::from_aspect_key(aspect_key.clone());

    let boundary_fact = StoreAspectBoundaryFact::from_admitted_state(
        identity.clone(),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness),
    )
    .unwrap();
    let authority_record = admit_aspect_native_authority_record(
        StableArtifactId::new("store.aspect-native.segment.identity").unwrap(),
        boundary_fact.clone(),
    );
    let readiness =
        StoreAspectNativeVocabularyReadiness::from_boundary_fact(boundary_fact, physical_witness);

    assert_eq!(authority_record.boundary_fact().identity(), &identity);
    assert_eq!(
        authority_record.boundary_fact().identity().aspect_key(),
        &aspect_key
    );
    assert!(readiness.adopted_families().contains(&(
        AspectNativeVocabularyFamily::AspectValues,
        AspectNativeVocabularyPosture::FoundationalSharedVocabulary
    )));
    assert!(readiness.adopted_families().contains(&(
        AspectNativeVocabularyFamily::StorePhysicalWitness,
        AspectNativeVocabularyPosture::StoreOwnedPhysicalWitness
    )));
}

#[test]
fn store_boundary_struct_values_are_validated_before_authority() {
    let aspect_key = aspect_key("store.physical.segment.header");
    let shape = aspects()
        .struct_fields()
        .required("segment", ScalarAspectType::String)
        .required("generation", ScalarAspectType::UInt64)
        .finish()
        .unwrap();
    let contract = aspects()
        .contract()
        .for_key(aspect_key.clone())
        .identified_by(aspects().vocabulary().identity(2))
        .at_revision(aspects().vocabulary().revision(1))
        .struct_aspect(shape);
    let struct_value = aspects()
        .vocabulary()
        .struct_value()
        .with_field("segment", aspect_string("segment-0001"))
        .with_field("generation", AspectValue::UInt64(7))
        .finish()
        .unwrap();
    let validated_value = match aspects().validate().against(&contract).value(struct_value) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("struct validation should succeed: {outcome:?}"),
    };
    let admitted_state = match aspects().authoritative_state().admit([validated_value]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    let fact = StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(aspect_key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .unwrap();

    assert!(fact
        .authority_input()
        .admitted_state()
        .payload()
        .aspects()
        .entries()
        .next()
        .is_some());
}

#[test]
fn authoritative_patches_require_foundational_patch_and_store_witness() {
    let aspect_key = aspect_key("store.physical.segment.identity");
    let contract = scalar_string_contract(aspect_key.clone());
    let validated_value = validated_scalar_value(&contract, "segment-0002");
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value)
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch construction should succeed: {outcome:?}"),
    };
    let identity = StoreAspectIdentity::from_aspect_key(aspect_key);
    let patch_fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity.clone(),
        StoreAspectPatchAuthorityInput::new(patch, physical_witness()),
    )
    .unwrap();

    assert_eq!(patch_fact.identity(), &identity);
}

#[test]
fn contract_admission_rejects_mismatched_store_identity() {
    let identity =
        StoreAspectIdentity::from_aspect_key(aspect_key("store.physical.segment.identity"));
    let contract = scalar_string_contract(aspect_key("store.physical.segment.header"));

    assert_eq!(
        StoreAspectContractAdmission::new(identity, contract, physical_witness()),
        Err(StoreAspectNativeDenial::IdentityMismatch)
    );
}

#[test]
fn validated_value_admission_rejects_mismatched_store_identity() {
    let contract = scalar_string_contract(aspect_key("store.physical.segment.header"));
    let validated_value = validated_scalar_value(&contract, "segment-0003");
    let identity =
        StoreAspectIdentity::from_aspect_key(aspect_key("store.physical.segment.identity"));

    assert_eq!(
        StoreValidatedAspectValueAdmission::new(identity, validated_value, physical_witness()),
        Err(StoreAspectNativeDenial::IdentityMismatch)
    );
}

#[test]
fn boundary_fact_rejects_state_without_store_identity_key() {
    let contract = scalar_string_contract(aspect_key("store.physical.segment.header"));
    let validated_value = validated_scalar_value(&contract, "segment-0004");
    let admitted_state = match aspects().authoritative_state().admit([validated_value]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    let identity =
        StoreAspectIdentity::from_aspect_key(aspect_key("store.physical.segment.identity"));

    assert_eq!(
        StoreAspectBoundaryFact::from_admitted_state(
            identity,
            StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
        ),
        Err(StoreAspectNativeDenial::IdentityMismatch)
    );
}

#[test]
fn boundary_fact_rejects_extra_authoritative_state_keys() {
    let identity_key = aspect_key("store.physical.segment.identity");
    let extra_key = aspect_key("store.physical.segment.header");
    let identity_contract = scalar_string_contract(identity_key.clone());
    let extra_contract = scalar_string_contract(extra_key);
    let identity_value = validated_scalar_value(&identity_contract, "segment-0006");
    let extra_value = validated_scalar_value(&extra_contract, "segment-header-0006");
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([identity_value, extra_value])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("multi-key state admission should succeed: {outcome:?}"),
    };
    let identity = StoreAspectIdentity::from_aspect_key(identity_key);

    assert_eq!(
        StoreAspectBoundaryFact::from_admitted_state(
            identity,
            StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
        ),
        Err(StoreAspectNativeDenial::IdentityMismatch)
    );
}

#[test]
fn patch_fact_rejects_patch_without_store_identity_key() {
    let contract = scalar_string_contract(aspect_key("store.physical.segment.header"));
    let validated_value = validated_scalar_value(&contract, "segment-0005");
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value)
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch construction should succeed: {outcome:?}"),
    };
    let identity =
        StoreAspectIdentity::from_aspect_key(aspect_key("store.physical.segment.identity"));

    assert_eq!(
        StoreAspectPatchBoundaryFact::from_authoritative_patch(
            identity,
            StoreAspectPatchAuthorityInput::new(patch, physical_witness()),
        ),
        Err(StoreAspectNativeDenial::IdentityMismatch)
    );
}

#[test]
fn store_physical_boundary_witness_rejects_non_aspect_native_scope() {
    let s1_witness = StorePhysicalAuthorityWitness::for_s1_vocabulary(ROADMAP_2_S1_SCOPE).unwrap();

    assert_eq!(
        StorePhysicalBoundaryWitness::from_physical_authority(s1_witness),
        Err(StoreAspectNativeDenial::PhysicalAuthorityScopeMismatch(
            PhysicalAuthorityScope::PhysicalFoundationVocabulary
        ))
    );
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
) -> forge_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(aspect_string(raw_value))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn aspect_string(value: &str) -> AspectValue {
    AspectValue::String(InternedString::from(value))
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
