use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, PlanarPredicateAuthorityCase,
    PlanarPredicateAuthorityDeclarationFamily, PlanarPredicateAuthorityEntry,
    PlanarPredicateAuthorityQueryDomain, PlanarPredicateAuthorityQueryWorld,
    PlanarPredicateCoincidencePolicy, PlanarPredicateFactReceipt, PlanarPredicateKind,
};

use super::proof_fixture::orient_basis;

#[test]
fn spatial_public_facade_exports_planar_predicate_authority_surface() {
    let basis = orient_basis("movement:identity", [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));

    let _: PlanarPredicateAuthorityEntry = entry;
    let _: PlanarPredicateAuthorityDeclarationFamily = PlanarPredicateAuthorityDeclarationFamily;
    let _: PlanarPredicateAuthorityQueryDomain = PlanarPredicateAuthorityQueryDomain;
    let _: PlanarPredicateAuthorityQueryWorld = PlanarPredicateAuthorityQueryWorld::new("public");
    let _: PlanarPredicateKind = PlanarPredicateKind::Orient2d;
    let _: PlanarPredicateCoincidencePolicy = PlanarPredicateCoincidencePolicy::AdmitCertifiedZero;
    let _: Option<PlanarPredicateFactReceipt> = None;
}

#[test]
fn planar_predicate_authority_family_is_query_native_and_relational() {
    let aspect_contract = PlanarPredicateAuthorityDeclarationFamily::aspect_contract();

    assert_eq!(
        PlanarPredicateAuthorityDeclarationFamily::semantic_family_key(),
        "PlanarPredicateAuthority"
    );
    assert_eq!(
        PlanarPredicateAuthorityDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_predicate.projected_points"
        )));
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_predicate.coincidence_policy"
        )));
    assert!(aspect_contract.preserved().contains(
        &crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_predicate.certified_sign"
        )
    ));
}
