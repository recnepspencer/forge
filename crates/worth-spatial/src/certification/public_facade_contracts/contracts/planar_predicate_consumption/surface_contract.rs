use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionCounters,
    PredicateCertificateConsumptionDeclarationFamily, PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld,
};

#[test]
fn spatial_public_facade_exports_readable_predicate_consumption_surface() {
    let _intent = PredicateCertificateConsumption::for_planar_workload();
    let _: PredicateCertificateConsumptionDeclarationFamily =
        PredicateCertificateConsumptionDeclarationFamily;
    let _: PredicateCertificateConsumptionQueryDomain = PredicateCertificateConsumptionQueryDomain;
    let _: PredicateCertificateConsumptionQueryWorld =
        PredicateCertificateConsumptionQueryWorld::new("public");
    let _: Option<PredicateCertificateConsumptionCounters> = None;

    let family =
        <PredicateCertificateConsumptionDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
            PredicateCertificateConsumptionQueryDomain,
        >>::semantic_family_key();
    assert_eq!(family, "PredicateCertificateConsumptionValidator");
    assert!(std::any::type_name::<PredicateCertificateConsumption>()
        .contains("PredicateCertificateConsumption"));
}

#[test]
fn predicate_consumption_family_declares_certification_and_substitute_rejection() {
    let aspect_contract = PredicateCertificateConsumptionDeclarationFamily::aspect_contract();

    assert!(aspect_contract
        .required()
        .contains(&"geometry.predicate_consumption.predicate_receipts".to_string()));
    assert!(aspect_contract
        .required()
        .contains(&"geometry.predicate_consumption.consumer_receipts".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.predicate_consumption.certified_rows".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.predicate_consumption.precision_metadata".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.predicate_consumption.substitute_rejection".to_string()));
}
