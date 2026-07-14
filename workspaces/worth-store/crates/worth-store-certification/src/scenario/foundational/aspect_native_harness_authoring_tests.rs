use worth_foundational::{AspectValue, ContractValidatedAspectValueView};
use worth_store_aspect_native::StoreAspectIdentity;
use worth_store_contracts::{
    PhysicalAuthorityScope, StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use worth_store_test_support::NativeStoreAspectFixture;

#[test]
fn ordinary_store_harness_authors_native_aspects() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0042", 42);

    assert_eq!(
        fixture.identity(),
        &StoreAspectIdentity::from_aspect_key(fixture.contract().key().clone())
    );
    assert!(fixture.scalar_value().is_none());
    assert!(fixture.struct_value().is_some());
    assert!(matches!(
        fixture.validated_value().payload().view(),
        ContractValidatedAspectValueView::Struct(_)
    ));
    assert!(fixture
        .authoritative_state()
        .payload()
        .get(fixture.identity().aspect_key())
        .is_some());
    assert_eq!(fixture.boundary_fact().identity(), fixture.identity());
    assert_eq!(fixture.patch_boundary_fact().identity(), fixture.identity());
    assert_eq!(fixture.aspect_locator().identity(), fixture.identity());
    assert_eq!(fixture.value_locator().identity(), fixture.identity());
    assert_eq!(
        fixture.field_locator().unwrap().identity(),
        fixture.identity()
    );
    assert_eq!(
        fixture.physical_witness().authority().authority_scope(),
        PhysicalAuthorityScope::AspectNativeBoundaryVocabulary
    );
    assert_eq!(
        fixture.physical_witness().authority(),
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE
        )
        .unwrap()
    );
}

#[test]
fn ordinary_scalar_fixture_authors_aspect_value_without_json() {
    let fixture = NativeStoreAspectFixture::scalar_string("segment-0043");

    assert!(matches!(
        fixture.scalar_value(),
        Some(AspectValue::String(_))
    ));
    assert!(fixture.struct_value().is_none());
    assert!(matches!(
        fixture.validated_value().payload().view(),
        ContractValidatedAspectValueView::Scalar(_)
    ));
    assert_eq!(fixture.boundary_fact().identity(), fixture.identity());
}
