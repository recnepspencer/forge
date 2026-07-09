use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectLocator, AspectValue, BoundarySourceLocator,
    InternedString, LocatorAuthority, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    project_store_boundary_fact_to_terminal_json,
    readmit_external_terminal_projection_document_as_store_aspect_state, StoreAspectAuthorityInput,
    StoreAspectBoundaryFact, StoreAspectIdentity, StorePhysicalBoundaryWitness,
    StoreTerminalProjectionDenial, StoreTerminalProjectionDisplayLabel,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

#[test]
fn labelled_terminal_json_document_cannot_readmit_as_store_authority() {
    let aspect_key = aspect_key("store.physical.segment.header");
    let contract = segment_header_contract(aspect_key.clone());
    let fact = struct_boundary_fact(&contract, "segment-0014", 14);
    let projection = project_store_boundary_fact_to_terminal_json(&fact).unwrap();
    let label = StoreTerminalProjectionDisplayLabel::new("Operator segment").unwrap();

    let labelled_document = serde_json::from_slice(
        projection
            .to_labelled_terminal_json_document_bytes(&label)
            .unwrap()
            .terminal_projection_bytes(),
    )
    .unwrap();

    let readmission = readmit_external_terminal_projection_document_as_store_aspect_state(
        StoreAspectIdentity::from_aspect_key(aspect_key.clone()),
        labelled_document,
        contract,
        source_locator(aspect_key),
        physical_witness(),
    );

    assert!(matches!(
        readmission,
        TransitionOutcome::Denied(StoreTerminalProjectionDenial::JsonCompatibilityDenied(_))
    ));
}

fn struct_boundary_fact(
    contract: &AspectContract,
    segment: &str,
    generation: u64,
) -> StoreAspectBoundaryFact {
    let struct_value = aspects()
        .vocabulary()
        .struct_value()
        .with_field(
            "segment",
            AspectValue::String(InternedString::from(segment)),
        )
        .with_field("generation", AspectValue::UInt64(generation))
        .finish()
        .unwrap();
    let validated = match aspects().validate().against(contract).value(struct_value) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("struct validation should succeed: {outcome:?}"),
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

fn segment_header_contract(aspect_key: AspectKey) -> AspectContract {
    let shape = aspects()
        .struct_fields()
        .required("segment", ScalarAspectType::String)
        .required("generation", ScalarAspectType::UInt64)
        .finish()
        .unwrap();
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(31))
        .at_revision(aspects().vocabulary().revision(1))
        .struct_aspect(shape)
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
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
