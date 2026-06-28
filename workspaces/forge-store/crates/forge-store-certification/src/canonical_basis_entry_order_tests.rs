use forge_foundational::{
    aspects, AspectKey, AspectValue, CanonicalBasisEntryKind, CanonicalizationRuleVersion,
    InternedString, ScalarAspectType, StructAspectValue,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily, StorePhysicalBoundaryWitness,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

#[test]
fn aspect_boundary_basis_is_stable_across_struct_field_authoring_order() {
    let version = basis_version();
    let first = prepared_entries(
        StoreCanonicalBasisConstruction::for_family(StoreCanonicalBasisFamily::AspectBoundaryFact)
            .with_aspect_boundary_fact(&aspect_boundary_struct_fact(
                FieldAuthoringOrder::SegmentFirst,
            ))
            .prepare(version.clone()),
    );
    let second = prepared_entries(
        StoreCanonicalBasisConstruction::for_family(StoreCanonicalBasisFamily::AspectBoundaryFact)
            .with_aspect_boundary_fact(&aspect_boundary_struct_fact(
                FieldAuthoringOrder::GenerationFirst,
            ))
            .prepare(version),
    );

    assert_eq!(first, second);
    assert_has_locus_prefix(&first, "foundational.aspect-state.");
    assert_has_locus_and_kind(
        &first,
        "physical.boundary.authority.scope",
        CanonicalBasisEntryKind::Future("store-physical-boundary-witness"),
    );
}

fn prepared_entries(
    outcome: forge_store_aspect_native::StoreCanonicalBasisConstructionOutcome,
) -> Vec<forge_foundational::CanonicalBasisEntry> {
    match outcome {
        TransitionOutcome::Success(ready) => ready.payload().entries().to_vec(),
        other => panic!("basis construction should succeed: {other:?}"),
    }
}

fn assert_has_locus_prefix(entries: &[forge_foundational::CanonicalBasisEntry], expected: &str) {
    assert!(
        entries.iter().any(|entry| {
            matches!(
                entry.locus(),
                forge_foundational::CanonicalBasisLocus::Named(InternedString::Raw(name))
                    if name.starts_with(expected)
            )
        }),
        "missing canonical basis locus prefix {expected}"
    );
}

fn assert_has_locus_and_kind(
    entries: &[forge_foundational::CanonicalBasisEntry],
    expected_locus: &str,
    expected_kind: CanonicalBasisEntryKind,
) {
    assert!(
        entries.iter().any(|entry| {
            matches!(
                entry.locus(),
                forge_foundational::CanonicalBasisLocus::Named(name)
                    if name == &InternedString::from(expected_locus)
                        && entry.kind() == expected_kind
            )
        }),
        "missing canonical basis locus {expected_locus} with kind {expected_kind:?}"
    );
}

#[derive(Debug, Clone, Copy)]
enum FieldAuthoringOrder {
    SegmentFirst,
    GenerationFirst,
}

fn aspect_boundary_struct_fact(order: FieldAuthoringOrder) -> StoreAspectBoundaryFact {
    let key = aspect_key("store.phase5.struct.aspect");
    let shape = aspects()
        .struct_fields()
        .required("segment", ScalarAspectType::String)
        .required("generation", ScalarAspectType::UInt64)
        .finish()
        .unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(2))
        .at_revision(aspects().vocabulary().revision(1))
        .struct_aspect(shape);
    let validated = match aspects()
        .validate()
        .against(&contract)
        .value(struct_value(order))
    {
        TransitionOutcome::Success(value) => value,
        other => panic!("struct validation should succeed: {other:?}"),
    };
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

fn struct_value(order: FieldAuthoringOrder) -> StructAspectValue {
    let builder = aspects().vocabulary().struct_value();
    let struct_value = match order {
        FieldAuthoringOrder::SegmentFirst => builder
            .with_field(
                "segment",
                AspectValue::String(InternedString::from("segment-0007")),
            )
            .with_field("generation", AspectValue::UInt64(7)),
        FieldAuthoringOrder::GenerationFirst => builder
            .with_field("generation", AspectValue::UInt64(7))
            .with_field(
                "segment",
                AspectValue::String(InternedString::from("segment-0007")),
            ),
    }
    .finish()
    .unwrap();

    struct_value
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
