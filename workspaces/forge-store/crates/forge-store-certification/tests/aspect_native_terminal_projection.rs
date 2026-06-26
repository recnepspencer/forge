use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectLocator, AspectValue, BoundarySourceLocator,
    InternedString, LocatorAuthority, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    project_store_boundary_fact_to_terminal_json,
    readmit_external_terminal_projection_document_as_store_aspect_state,
    readmit_terminal_json_projection_as_store_aspect_state, StoreAspectAuthorityInput,
    StoreAspectBoundaryFact, StoreAspectIdentity, StorePhysicalBoundaryWitness,
    StoreTerminalProjectionDenial,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

#[test]
fn terminal_json_projection_is_one_way_until_readmission() {
    let aspect_key = aspect_key("store.physical.segment.identity");
    let contract = scalar_string_contract(aspect_key.clone());
    let fact = boundary_fact(&contract, "segment-0007");

    let projection = project_store_boundary_fact_to_terminal_json(&fact).unwrap();
    assert_eq!(projection.terminal_projection_identity(), fact.identity());

    let readmission = match readmit_terminal_json_projection_as_store_aspect_state(
        projection,
        contract,
        source_locator(aspect_key.clone()),
        physical_witness(),
    ) {
        TransitionOutcome::Success(readmission) => readmission,
        outcome => panic!("terminal projection should readmit: {outcome:?}"),
    };

    let readmitted_fact = StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(aspect_key),
        StoreAspectAuthorityInput::new(
            readmission.admitted_state().clone(),
            readmission.physical_witness(),
        ),
    )
    .unwrap();
    assert_eq!(readmitted_fact.identity(), readmission.identity());
}

#[test]
fn external_terminal_json_ingress_must_use_store_readmission() {
    let aspect_key = aspect_key("store.physical.segment.identity");
    let identity = StoreAspectIdentity::from_aspect_key(aspect_key.clone());
    let contract = scalar_string_contract(aspect_key.clone());

    let readmission = match readmit_external_terminal_projection_document_as_store_aspect_state(
        identity.clone(),
        "segment-0010".into(),
        contract,
        source_locator(aspect_key),
        physical_witness(),
    ) {
        TransitionOutcome::Success(readmission) => readmission,
        outcome => panic!("external terminal projection should readmit: {outcome:?}"),
    };

    assert_eq!(readmission.identity(), &identity);
}

#[test]
fn external_terminal_json_ingress_rejects_type_incompatible_projection_document() {
    let aspect_key = aspect_key("store.physical.segment.identity");
    let identity = StoreAspectIdentity::from_aspect_key(aspect_key.clone());
    let contract = scalar_string_contract(aspect_key.clone());

    let outcome = readmit_external_terminal_projection_document_as_store_aspect_state(
        identity,
        7.into(),
        contract,
        source_locator(aspect_key),
        physical_witness(),
    );

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(StoreTerminalProjectionDenial::JsonCompatibilityDenied(_))
    ));
}

#[test]
fn terminal_json_readmission_rejects_mismatched_contract_identity() {
    let identity_key = aspect_key("store.physical.segment.identity");
    let fact = boundary_fact(
        &scalar_string_contract(identity_key.clone()),
        "segment-0008",
    );
    let projection = project_store_boundary_fact_to_terminal_json(&fact).unwrap();
    let wrong_contract = scalar_string_contract(aspect_key("store.physical.segment.header"));

    let outcome = readmit_terminal_json_projection_as_store_aspect_state(
        projection,
        wrong_contract,
        source_locator(identity_key),
        physical_witness(),
    );

    assert_eq!(
        outcome,
        TransitionOutcome::Denied(StoreTerminalProjectionDenial::ContractIdentityMismatch)
    );
}

#[test]
fn terminal_json_projection_round_trips_validated_struct_state() {
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
        .identified_by(aspects().vocabulary().identity(30))
        .at_revision(aspects().vocabulary().revision(1))
        .struct_aspect(shape);
    let struct_value = aspects()
        .vocabulary()
        .struct_value()
        .with_field("segment", aspect_string("segment-0009"))
        .with_field("generation", AspectValue::UInt64(9))
        .finish()
        .unwrap();
    let validated = match aspects().validate().against(&contract).value(struct_value) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("struct validation should succeed: {outcome:?}"),
    };
    let state = match aspects().authoritative_state().admit([validated]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    let fact = StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(aspect_key.clone()),
        StoreAspectAuthorityInput::new(state, physical_witness()),
    )
    .unwrap();

    let projection = project_store_boundary_fact_to_terminal_json(&fact).unwrap();
    let outcome = readmit_terminal_json_projection_as_store_aspect_state(
        projection,
        contract,
        source_locator(aspect_key),
        physical_witness(),
    );

    assert!(matches!(outcome, TransitionOutcome::Success(_)));
}

fn boundary_fact(contract: &AspectContract, raw_value: &str) -> StoreAspectBoundaryFact {
    let validated = match aspects()
        .validate()
        .against(contract)
        .value(aspect_string(raw_value))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    };
    let state = match aspects().authoritative_state().admit([validated]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(contract.key().clone()),
        StoreAspectAuthorityInput::new(state, physical_witness()),
    )
    .unwrap()
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(29))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn aspect_string(value: &str) -> AspectValue {
    AspectValue::String(InternedString::from(value))
}

fn source_locator(aspect_key: AspectKey) -> BoundarySourceLocator {
    BoundarySourceLocator::Aspect(AspectLocator::new(LocatorAuthority::Projected, aspect_key))
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
