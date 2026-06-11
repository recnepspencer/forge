use forge_query::facade::{
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
};
use worth_spatial::certification::geometry_support_posture::{
    admit_geometry_public_surface, geometry_public_support_matrix, GeometryPublicSupportStatus,
};
use worth_spatial::facade::support::GeometryPublicSurface;

#[test]
fn spatial_public_geometry_support_matrix_covers_every_admitted_surface() {
    let matrix = geometry_public_support_matrix();
    let surfaces = GeometryPublicSurface::all();

    assert_eq!(matrix.rows().len(), surfaces.len());
    assert!(matrix.row("GeometryIntent").is_none());
    assert!(!matrix.matrix_digest().is_empty());

    for surface in surfaces {
        let row = matrix
            .row_for_surface(surface)
            .expect("every admitted geometry surface must have a support row");
        assert_eq!(row.status(), GeometryPublicSupportStatus::Supported);
        assert_eq!(row.surface(), surface);
        assert!(!row.row_digest().is_empty());
        assert!(!row.admission_rule().is_empty());
    }
}

#[test]
fn spatial_public_geometry_support_matrix_keeps_family_contracts_and_surface_admission_in_sync() {
    let matrix = geometry_public_support_matrix();
    let legality = ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact();
    let route = ForgeQueryDeclarationRouteContract::relational_only();
    let rebinding_route = ForgeQueryDeclarationRouteContract::relational_and_bridge();

    let target_identity = matrix
        .row_for_surface(GeometryPublicSurface::GeometryTargetIdentity)
        .expect("target identity support row should exist");
    assert_eq!(target_identity.declared_family_key(), None);
    assert_eq!(target_identity.legality_contract(), None);
    assert_eq!(target_identity.route_contract(), None);

    for surface in [
        GeometryPublicSurface::SpatialAnchorSelection,
        GeometryPublicSurface::PrimitiveBinding,
        GeometryPublicSurface::PrimitiveAnchorBinding,
        GeometryPublicSurface::PrimitiveRebinding,
        GeometryPublicSurface::TopologyNeighborhoodReplacement,
        GeometryPublicSurface::ToleranceAndPrecisionCertification,
        GeometryPublicSurface::HistoricalGeometryInspection,
        GeometryPublicSurface::BranchLocalGeometryInspection,
        GeometryPublicSurface::GeometryReplayParity,
        GeometryPublicSurface::GeometryRecoveryAction,
        GeometryPublicSurface::GeometryProjectionConsumption,
    ] {
        let row = matrix
            .row_for_surface(surface)
            .expect("family support row should exist");
        assert!(row.declared_family_key().is_some());
        assert_eq!(row.legality_contract(), Some(legality));
        let expected_route = match surface {
            GeometryPublicSurface::PrimitiveRebinding => rebinding_route,
            _ => route,
        };
        assert_eq!(row.route_contract(), Some(expected_route));

        let admission = admit_geometry_public_surface(surface);
        assert_eq!(admission.surface(), surface);
        assert_eq!(admission.support_row_digest(), row.row_digest());
        assert_eq!(admission.matrix_digest(), matrix.matrix_digest());
    }
}
