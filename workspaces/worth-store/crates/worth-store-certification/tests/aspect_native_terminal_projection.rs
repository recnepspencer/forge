use serde_json::{Map, Number, Value};
use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectLocator, AspectValue, BoundarySourceLocator,
    CanonicalBasisReadyArtifact, CanonicalDigestAlgorithmId, CanonicalizationRuleVersion,
    InternedString, LocatorAuthority, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    project_store_boundary_fact_to_terminal_json,
    readmit_external_terminal_projection_document_as_store_aspect_state,
    readmit_terminal_json_projection_as_store_aspect_state, StoreAspectAuthorityInput,
    StoreAspectBoundaryFact, StoreAspectIdentity, StoreCanonicalBasisConstruction,
    StoreCanonicalBasisFamily, StoreDigestAuthority, StoreDigestEvidence,
    StorePhysicalBoundaryWitness, StoreTerminalDocumentChecksum, StoreTerminalProjectionDenial,
    StoreTerminalProjectionDisplayLabel, StoreTerminalProjectionDocumentBytes,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

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
    let contract = segment_header_contract(aspect_key.clone());
    let fact = struct_boundary_fact(&contract, "segment-0009", 9);

    let projection = project_store_boundary_fact_to_terminal_json(&fact).unwrap();
    let outcome = readmit_terminal_json_projection_as_store_aspect_state(
        projection,
        contract,
        source_locator(aspect_key),
        physical_witness(),
    );

    assert!(matches!(outcome, TransitionOutcome::Success(_)));
}

#[test]
fn terminal_json_rendering_changes_checksum_not_store_digest_authority() {
    let aspect_key = aspect_key("store.physical.segment.header");
    let contract = segment_header_contract(aspect_key.clone());
    let fact = struct_boundary_fact(&contract, "segment-0011", 11);
    let expected_authority = basis_and_digest_for_boundary_fact(&fact);
    let projection = project_store_boundary_fact_to_terminal_json(&fact).unwrap();

    let compact_checksum = terminal_checksum(
        projection
            .to_compact_terminal_json_document_bytes()
            .unwrap(),
    );
    let pretty_checksum =
        terminal_checksum(projection.to_pretty_terminal_json_document_bytes().unwrap());
    let first_label = StoreTerminalProjectionDisplayLabel::new("Operator segment").unwrap();
    let second_label = StoreTerminalProjectionDisplayLabel::new("Review segment").unwrap();
    let first_labelled_checksum = terminal_checksum(
        projection
            .to_labelled_terminal_json_document_bytes(&first_label)
            .unwrap(),
    );
    let second_labelled_checksum = terminal_checksum(
        projection
            .to_labelled_terminal_json_document_bytes(&second_label)
            .unwrap(),
    );

    assert_ne!(
        compact_checksum.terminal_checksum_bytes(),
        pretty_checksum.terminal_checksum_bytes()
    );
    assert_ne!(
        first_labelled_checksum.terminal_checksum_bytes(),
        second_labelled_checksum.terminal_checksum_bytes()
    );

    let readmission = match readmit_terminal_json_projection_as_store_aspect_state(
        projection,
        contract,
        source_locator(aspect_key),
        physical_witness(),
    ) {
        TransitionOutcome::Success(readmission) => readmission,
        outcome => panic!("terminal projection should readmit: {outcome:?}"),
    };
    let readmitted_authority =
        basis_and_digest_for_boundary_fact(&readmission.rebuild_store_boundary_fact().unwrap());

    assert_same_native_basis_and_store_digest(&expected_authority, &readmitted_authority);
}

#[test]
fn terminal_json_field_order_changes_do_not_change_readmitted_native_digest() {
    let aspect_key = aspect_key("store.physical.segment.header");
    let identity = StoreAspectIdentity::from_aspect_key(aspect_key.clone());
    let first_contract = segment_header_contract(aspect_key.clone());
    let second_contract = segment_header_contract(aspect_key.clone());

    let first_readmission = readmitted_external_struct_projection(
        identity.clone(),
        first_contract,
        [
            ("segment", Value::String("segment-0012".to_string())),
            ("generation", Number::from(12).into()),
        ],
    );
    let second_readmission = readmitted_external_struct_projection(
        identity,
        second_contract,
        [
            ("generation", Number::from(12).into()),
            ("segment", Value::String("segment-0012".to_string())),
        ],
    );

    let first_authority = basis_and_digest_for_boundary_fact(
        &first_readmission.rebuild_store_boundary_fact().unwrap(),
    );
    let second_authority = basis_and_digest_for_boundary_fact(
        &second_readmission.rebuild_store_boundary_fact().unwrap(),
    );

    assert_same_native_basis_and_store_digest(&first_authority, &second_authority);
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

fn struct_boundary_fact(
    contract: &AspectContract,
    segment: &str,
    generation: u64,
) -> StoreAspectBoundaryFact {
    let struct_value = aspects()
        .vocabulary()
        .struct_value()
        .with_field("segment", aspect_string(segment))
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
        .identified_by(aspects().vocabulary().identity(30))
        .at_revision(aspects().vocabulary().revision(1))
        .struct_aspect(shape)
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

fn terminal_checksum(bytes: StoreTerminalProjectionDocumentBytes) -> StoreTerminalDocumentChecksum {
    StoreTerminalDocumentChecksum::for_terminal_projection_document_bytes(&bytes)
}

fn readmitted_external_struct_projection<const N: usize>(
    identity: StoreAspectIdentity,
    contract: AspectContract,
    fields: [(&str, Value); N],
) -> worth_store_aspect_native::StoreTerminalJsonReadmission {
    let mut document = Map::new();
    for (field, value) in fields {
        document.insert(field.to_string(), value);
    }

    match readmit_external_terminal_projection_document_as_store_aspect_state(
        identity.clone(),
        Value::Object(document),
        contract,
        source_locator(identity.aspect_key().clone()),
        physical_witness(),
    ) {
        TransitionOutcome::Success(readmission) => readmission,
        outcome => panic!("external terminal projection should readmit: {outcome:?}"),
    }
}

struct NativeBasisAndStoreDigestEvidence {
    native_basis: CanonicalBasisReadyArtifact,
    store_digest: StoreDigestEvidence,
}

fn basis_and_digest_for_boundary_fact(
    fact: &StoreAspectBoundaryFact,
) -> NativeBasisAndStoreDigestEvidence {
    let basis = match StoreCanonicalBasisConstruction::for_family(
        StoreCanonicalBasisFamily::AspectBoundaryFact,
    )
    .with_aspect_boundary_fact(fact)
    .prepare(canonical_rule_version())
    {
        TransitionOutcome::Success(basis) => basis,
        outcome => panic!("canonical basis construction should succeed: {outcome:?}"),
    };

    let digest = match StoreDigestAuthority::for_native_basis(
        StoreCanonicalBasisFamily::AspectBoundaryFact,
        basis.clone(),
    )
    .derive(CanonicalDigestAlgorithmId::sha256())
    {
        TransitionOutcome::Success(digest) => digest,
        outcome => panic!("Store digest derivation should succeed: {outcome:?}"),
    };

    NativeBasisAndStoreDigestEvidence {
        native_basis: basis,
        store_digest: digest,
    }
}

fn assert_same_native_basis_and_store_digest(
    left: &NativeBasisAndStoreDigestEvidence,
    right: &NativeBasisAndStoreDigestEvidence,
) {
    assert_eq!(left.native_basis.payload(), right.native_basis.payload());
    assert_eq!(left.store_digest.family(), right.store_digest.family());
    assert_eq!(
        left.store_digest.source_kind(),
        right.store_digest.source_kind()
    );
    assert_eq!(
        left.store_digest.equivalence_basis_identity(),
        right.store_digest.equivalence_basis_identity()
    );
    assert_eq!(
        left.store_digest.canonical_digest().value().bytes(),
        right.store_digest.canonical_digest().value().bytes()
    );
}

fn canonical_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth.store.phase7.terminal-projection")
        .expect("valid canonical rule version")
}
