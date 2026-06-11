use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractDeclarationFamily,
    CoplanarOverlapContractExtractor, CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld, CoplanarOverlapPolicy,
};

#[test]
fn spatial_public_facade_exports_readable_overlap_surface() {
    let _: Option<CertifiedCoplanarOverlapFace2D> = None;
    let _: Option<CoplanarOverlapContractExtractor> = None;
    let _: CoplanarOverlapContractDeclarationFamily = CoplanarOverlapContractDeclarationFamily;
    let _: CoplanarOverlapContractQueryDomain = CoplanarOverlapContractQueryDomain;
    let _: CoplanarOverlapContractQueryWorld = CoplanarOverlapContractQueryWorld::new("public");
    let _: CoplanarOverlapPolicy = CoplanarOverlapPolicy::ExtractContractsOnly;
}

#[test]
fn overlap_family_is_query_native_retained_and_not_boolean() {
    let aspect_contract = CoplanarOverlapContractDeclarationFamily::aspect_contract();

    assert_eq!(
        CoplanarOverlapContractDeclarationFamily::semantic_family_key(),
        "CoplanarOverlapContractExtractor"
    );
    assert!(aspect_contract
        .required()
        .contains(&"geometry.coplanar_overlap.pair".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.coplanar_overlap.shared_interval".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.coplanar_overlap.ambiguous_contact".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.coplanar_overlap.policy_exit".to_string()));
}
