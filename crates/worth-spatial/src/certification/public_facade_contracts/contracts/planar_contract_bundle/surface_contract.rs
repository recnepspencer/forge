use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarBooleanReadinessBundle, PlanarContractBundleValidationDeclarationFamily,
    PlanarContractBundleValidationQueryDomain, PlanarContractBundleValidator,
};

#[test]
fn spatial_public_facade_exports_readable_bundle_validator_surface() {
    let _builder = PlanarBooleanReadinessBundle::builder();
    let family =
        <PlanarContractBundleValidationDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
            PlanarContractBundleValidationQueryDomain,
        >>::semantic_family_key();
    assert_eq!(family, "PlanarContractBundleValidator");
    assert!(std::any::type_name::<PlanarContractBundleValidator>()
        .contains("PlanarContractBundleValidator"));
}
