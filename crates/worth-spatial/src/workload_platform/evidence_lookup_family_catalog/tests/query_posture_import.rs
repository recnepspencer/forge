use super::super::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyQueryPostureKind,
    EvidenceLookupProjectionConsumptionSurface, EvidenceLookupProjectionFactFamily,
    EvidenceLookupQueryImportEvidence,
};

#[test]
fn query_required_families_import_query_evidence_instead_of_owning_it() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");

    for family in closeout
        .declarations()
        .iter()
        .filter(|family| family.query_posture().requires_query_evidence())
    {
        assert!(matches!(
            family.query_posture().kind(),
            EvidenceLookupFamilyQueryPostureKind::ImportedSupportPinRequired
                | EvidenceLookupFamilyQueryPostureKind::ImportedProjectionConsumptionRequired
        ));
        assert!(family.query_posture().imported_evidence_digest().is_some());
        assert!(family.query_posture().imported_evidence().is_some());
    }
}

#[test]
fn not_required_query_family_has_no_imported_query_digest() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let family = closeout
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("event ledger family exists");

    assert_eq!(
        family.query_posture().kind(),
        EvidenceLookupFamilyQueryPostureKind::NotRequired
    );
    assert!(family.query_posture().imported_evidence_digest().is_none());
    assert!(family.query_posture().imported_evidence().is_none());
}

#[test]
fn query_import_posture_names_real_query_surfaces() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let overlap = closeout
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("overlap family exists");
    let projection = closeout
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("projection family exists");

    assert!(matches!(
        overlap.query_posture().imported_evidence(),
        Some(EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin { .. })
    ));
    assert_eq!(
        overlap
            .query_posture()
            .imported_evidence()
            .expect("support pin evidence")
            .query_surface_type_name(),
        "forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin"
    );
    assert!(matches!(
        projection.query_posture().imported_evidence(),
        Some(EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt { .. })
    ));
    let projection_evidence = projection
        .query_posture()
        .imported_evidence()
        .expect("projection evidence");
    assert_eq!(
        projection_evidence.projection_consumption_surface(),
        Some(EvidenceLookupProjectionConsumptionSurface::ForgeQueryProjectionConsumptionReceipt)
    );
    assert_eq!(
        projection_evidence.projection_fact_family(),
        Some(EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection)
    );
    assert_eq!(
        projection_evidence.query_surface_type_name(),
        "forge_query::facade::ProjectionConsumptionReceipt"
    );
}
