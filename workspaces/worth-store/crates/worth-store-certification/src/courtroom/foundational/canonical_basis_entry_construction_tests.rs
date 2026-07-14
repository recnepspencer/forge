use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, AuthoritativeRecordAspectPatch,
    CanonicalBasisEntryKind, CanonicalizationRuleVersion, ContractValidatedAspectArtifact,
    InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact, StoreCanonicalBasisConstruction,
    StoreCanonicalBasisConstructionDenial, StoreCanonicalBasisFamily, StorePhysicalBoundaryWitness,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    prepare_physical_page_header_canonical_basis, PhysicalBinaryEncodingWitness,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageKind, PhysicalPublicationState,
    PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn store_canonical_basis_is_native_and_order_stable() {
    let header = decoded_page_header();
    let physical = physical_witness();
    let version = basis_version();

    let first = prepared_entries(prepare_physical_page_header_canonical_basis(
        version.clone(),
        header,
        physical,
    ));
    let second = prepared_entries(prepare_physical_page_header_canonical_basis(
        version, header, physical,
    ));

    assert_eq!(first, second);
    assert_has_locus(&first, "boundary.authority.scope");
    assert_has_locus_and_kind(
        &first,
        "source.kind",
        CanonicalBasisEntryKind::Future("store-page-header-field"),
    );
    assert_has_locus_and_kind(
        &first,
        "boundary.authority.scope",
        CanonicalBasisEntryKind::Future("store-physical-boundary-witness"),
    );
}

#[test]
fn aspect_boundary_basis_preserves_store_physical_witness_and_order() {
    let fact = aspect_boundary_fact();
    let version = basis_version();

    let first = prepared_entries(
        StoreCanonicalBasisConstruction::for_family(StoreCanonicalBasisFamily::AspectBoundaryFact)
            .with_aspect_boundary_fact(&fact)
            .prepare(version.clone()),
    );
    let second = prepared_entries(
        StoreCanonicalBasisConstruction::for_family(StoreCanonicalBasisFamily::AspectBoundaryFact)
            .with_aspect_boundary_fact(&fact)
            .prepare(version),
    );

    assert_eq!(first, second);
    assert_has_locus(&first, "physical.boundary.authority.scope");
    assert_has_locus_prefix(&first, "foundational.aspect-state.");
    assert_has_locus_and_kind(
        &first,
        "source.kind",
        CanonicalBasisEntryKind::Future("store-aspect-boundary-field"),
    );
    assert_has_locus_and_kind(
        &first,
        "physical.boundary.authority.scope",
        CanonicalBasisEntryKind::Future("store-physical-boundary-witness"),
    );
}

#[test]
fn aspect_patch_basis_preserves_store_physical_witness_and_order() {
    let fact = aspect_patch_boundary_fact();
    let version = basis_version();

    let first = prepared_entries(
        StoreCanonicalBasisConstruction::for_family(
            StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
        )
        .with_aspect_patch_boundary_fact(&fact)
        .prepare(version.clone()),
    );
    let second = prepared_entries(
        StoreCanonicalBasisConstruction::for_family(
            StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
        )
        .with_aspect_patch_boundary_fact(&fact)
        .prepare(version),
    );

    assert_eq!(first, second);
    assert_has_locus(&first, "physical.boundary.authority.scope");
    assert_has_locus_prefix(&first, "foundational.aspect-patch.");
    assert_has_locus_and_kind(
        &first,
        "source.kind",
        CanonicalBasisEntryKind::Future("store-aspect-patch-field"),
    );
    assert_has_locus_and_kind(
        &first,
        "physical.boundary.authority.scope",
        CanonicalBasisEntryKind::Future("store-physical-boundary-witness"),
    );
}

#[test]
fn conflicting_native_sources_are_denied_explicitly() {
    let denial =
        StoreCanonicalBasisConstruction::for_family(StoreCanonicalBasisFamily::AspectBoundaryFact)
            .with_aspect_boundary_fact(&aspect_boundary_fact())
            .with_aspect_patch_boundary_fact(&aspect_patch_boundary_fact())
            .prepare(basis_version());

    assert_eq!(
        denied(denial),
        StoreCanonicalBasisConstructionDenial::ConflictingNativeSources {
            family: StoreCanonicalBasisFamily::AspectBoundaryFact,
        }
    );
}

fn prepared_entries<E: std::fmt::Debug>(
    outcome: TransitionOutcome<
        worth_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisReadyArtifact,
        E,
    >,
) -> Vec<worth_foundational::CanonicalBasisEntry> {
    match outcome {
        TransitionOutcome::Success(ready) => ready.payload().entries().to_vec(),
        other => panic!("basis construction should succeed: {other:?}"),
    }
}

fn assert_has_locus(entries: &[worth_foundational::CanonicalBasisEntry], expected: &str) {
    assert!(
        entries.iter().any(|entry| {
            matches!(
                entry.locus(),
                worth_foundational::CanonicalBasisLocus::Named(name)
                    if name == &InternedString::from(expected)
            )
        }),
        "missing canonical basis locus {expected}"
    );
}

fn assert_has_locus_prefix(entries: &[worth_foundational::CanonicalBasisEntry], expected: &str) {
    assert!(
        entries.iter().any(|entry| {
            matches!(
                entry.locus(),
                worth_foundational::CanonicalBasisLocus::Named(InternedString::Raw(name))
                    if name.starts_with(expected)
            )
        }),
        "missing canonical basis locus prefix {expected}"
    );
}

fn assert_has_locus_and_kind(
    entries: &[worth_foundational::CanonicalBasisEntry],
    expected_locus: &str,
    expected_kind: CanonicalBasisEntryKind,
) {
    assert!(
        entries.iter().any(|entry| {
            matches!(
                entry.locus(),
                worth_foundational::CanonicalBasisLocus::Named(name)
                    if name == &InternedString::from(expected_locus)
                        && entry.kind() == expected_kind
            )
        }),
        "missing canonical basis locus {expected_locus} with kind {expected_kind:?}"
    );
}

fn denied(
    outcome: worth_store_aspect_native::StoreCanonicalBasisConstructionOutcome,
) -> StoreCanonicalBasisConstructionDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("basis construction should be denied: {other:?}"),
    }
}

fn decoded_page_header() -> PhysicalHeaderDecodeWitness {
    let generation = generation(7);
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment(11), page(13))
        .with_page_generation(generation);
    let bytes = page_bytes(generation, b"native-basis");
    header_authority()
        .decode_page_header(cell, &bytes, PhysicalPageKind::DataPage)
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical()
            .expect("static S.1 fixture encoding witness is valid"),
    )
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn aspect_boundary_fact() -> StoreAspectBoundaryFact {
    let key = aspect_key("store.phase5.semantic.aspect");
    let contract = scalar_string_contract(key.clone());
    let validated = validated_aspect_value(&contract, "semantic-only");
    let state = match aspects().authoritative_state().admit([validated]) {
        TransitionOutcome::Success(state) => state,
        other => panic!("state admission should succeed: {other:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(state, physical_witness()),
    )
    .unwrap()
}

fn aspect_patch_boundary_fact() -> StoreAspectPatchBoundaryFact {
    let key = aspect_key("store.phase5.patch.aspect");
    let contract = scalar_string_contract(key.clone());
    let validated = validated_aspect_value(&contract, "patched");
    let patch = match AuthoritativeRecordAspectPatch::whole_aspect([validated], []) {
        TransitionOutcome::Success(patch) => patch,
        other => panic!("patch construction should succeed: {other:?}"),
    };

    StoreAspectPatchBoundaryFact::from_authoritative_patch(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectPatchAuthorityInput::new(patch, physical_witness()),
    )
    .unwrap()
}

fn validated_aspect_value(
    contract: &AspectContract,
    value: &str,
) -> ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(value)))
    {
        TransitionOutcome::Success(value) => value,
        other => panic!("validation should succeed: {other:?}"),
    }
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
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

fn basis_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.native-basis.test.v1").unwrap()
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}
fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
