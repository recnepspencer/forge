use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, PlanarPrecisionBasis, PlanarPrecisionCertificationCase,
    PlanarPrecisionCertificationDeclarationFamily, PlanarPrecisionCertificationEntry,
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
    PlanarPrecisionPerformanceCounters,
};

use super::proof_fixture::{precision_basis, predicate_receipt};

#[test]
fn spatial_public_facade_exports_planar_precision_certification_surface() {
    let predicate = predicate_receipt(
        "movement:rotation-cancelled",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let basis = precision_basis(&predicate);
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
    );

    let _: PlanarPrecisionCertificationEntry = entry;
    let _: PlanarPrecisionBasis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:micro-feature-local-xy")
        .topology_basis_identity("topology:thin-slot-loop")
        .movement_rotation_posture_identity("movement:rotation-cancelled")
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect("basis");
    let _: PlanarPrecisionCertificationDeclarationFamily =
        PlanarPrecisionCertificationDeclarationFamily;
    let _: PlanarPrecisionCertificationQueryDomain = PlanarPrecisionCertificationQueryDomain;
    let _: PlanarPrecisionCertificationQueryWorld =
        PlanarPrecisionCertificationQueryWorld::new("public");
    let _: Option<PlanarPrecisionPerformanceCounters> = None;
}

#[test]
fn planar_precision_certification_family_is_query_native_and_relational() {
    let aspect_contract = PlanarPrecisionCertificationDeclarationFamily::aspect_contract();

    assert_eq!(
        PlanarPrecisionCertificationDeclarationFamily::semantic_family_key(),
        "PlanarPrecisionCertification"
    );
    assert_eq!(
        PlanarPrecisionCertificationDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
    assert!(aspect_contract
        .required()
        .contains(&"geometry.planar_precision.predicate_fact".to_string()));
    assert!(aspect_contract
        .required()
        .contains(&"geometry.planar_precision.normalization_scale".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.planar_precision.scale_separation".to_string()));
}
