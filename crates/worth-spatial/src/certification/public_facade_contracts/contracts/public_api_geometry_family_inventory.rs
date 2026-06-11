use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract,
};
use worth_spatial::facade::anchor_binding::PrimitiveAnchorBindingDeclarationFamily;
use worth_spatial::facade::anchor_selection::SpatialAnchorSelectionDeclarationFamily;
use worth_spatial::facade::binding::PrimitiveBindingDeclarationFamily;
use worth_spatial::facade::inspection::{
    BranchLocalGeometryInspectionDeclarationFamily, GeometryReplayParityDeclarationFamily,
    HistoricalGeometryInspectionDeclarationFamily,
};
use worth_spatial::facade::neighborhood::TopologyNeighborhoodReplacementDeclarationFamily;
use worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateDeclarationFamily;
use worth_spatial::facade::planar_precision::PlanarPrecisionCertificationDeclarationFamily;
use worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DDeclarationFamily;
use worth_spatial::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsDeclarationFamily;
use worth_spatial::facade::planar_recovery::PlanarRecoveryPostureDeclarationFamily;
use worth_spatial::facade::projection::GeometryProjectionConsumptionDeclarationFamily;
use worth_spatial::facade::rebinding::PrimitiveRebindingDeclarationFamily;
use worth_spatial::facade::recovery::GeometryRecoveryActionDeclarationFamily;
use worth_spatial::facade::tolerance::ToleranceAndPrecisionCertificationDeclarationFamily;

#[test]
fn spatial_public_family_inventory_uses_direct_query_native_runtime_names() {
    assert_eq!(
        SpatialAnchorSelectionDeclarationFamily::semantic_family_key(),
        "SpatialAnchorSelection"
    );
    assert_eq!(
        PrimitiveBindingDeclarationFamily::semantic_family_key(),
        "PrimitiveBinding"
    );
    assert_eq!(
        PrimitiveAnchorBindingDeclarationFamily::semantic_family_key(),
        "PrimitiveAnchorBinding"
    );
    assert_eq!(
        PrimitiveRebindingDeclarationFamily::semantic_family_key(),
        "PrimitiveRebinding"
    );
    assert_eq!(
        TopologyNeighborhoodReplacementDeclarationFamily::semantic_family_key(),
        "TopologyNeighborhoodReplacement"
    );
    assert_eq!(
        PlanarPrecisionCertificationDeclarationFamily::semantic_family_key(),
        "PlanarPrecisionCertification"
    );
    assert_eq!(
        PlanarLocalFrameCertificateDeclarationFamily::semantic_family_key(),
        "PlanarLocalFrameCertificate"
    );
    assert_eq!(
        ProjectPointToCertifiedPlane2DDeclarationFamily::semantic_family_key(),
        "ProjectPointToCertifiedPlane2D"
    );
    assert_eq!(
        ProjectionConsumedPlanarFactsDeclarationFamily::semantic_family_key(),
        "ProjectionConsumedPlanarFacts"
    );
    assert_eq!(
        PlanarRecoveryPostureDeclarationFamily::semantic_family_key(),
        "PlanarRecoveryPosture"
    );
    assert_eq!(
        ToleranceAndPrecisionCertificationDeclarationFamily::semantic_family_key(),
        "ToleranceAndPrecisionCertification"
    );
    assert_eq!(
        HistoricalGeometryInspectionDeclarationFamily::semantic_family_key(),
        "HistoricalGeometryInspection"
    );
    assert_eq!(
        BranchLocalGeometryInspectionDeclarationFamily::semantic_family_key(),
        "BranchLocalGeometryInspection"
    );
    assert_eq!(
        GeometryReplayParityDeclarationFamily::semantic_family_key(),
        "GeometryReplayParity"
    );
    assert_eq!(
        GeometryRecoveryActionDeclarationFamily::semantic_family_key(),
        "GeometryRecoveryAction"
    );
    assert_eq!(
        GeometryProjectionConsumptionDeclarationFamily::semantic_family_key(),
        "GeometryProjectionConsumption"
    );
}

#[test]
fn spatial_public_family_inventory_exposes_explicit_legality_and_route_contracts() {
    let legality = ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact();
    let route = ForgeQueryDeclarationRouteContract::relational_only();
    let rebinding_route = ForgeQueryDeclarationRouteContract::relational_and_bridge();

    assert_eq!(
        SpatialAnchorSelectionDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        PrimitiveBindingDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        PrimitiveAnchorBindingDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        PrimitiveRebindingDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        TopologyNeighborhoodReplacementDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        PlanarPrecisionCertificationDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        PlanarLocalFrameCertificateDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        ProjectPointToCertifiedPlane2DDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        ProjectionConsumedPlanarFactsDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        PlanarRecoveryPostureDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        ToleranceAndPrecisionCertificationDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        HistoricalGeometryInspectionDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        BranchLocalGeometryInspectionDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        GeometryReplayParityDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        GeometryRecoveryActionDeclarationFamily::legality_contract(),
        legality
    );
    assert_eq!(
        GeometryProjectionConsumptionDeclarationFamily::legality_contract(),
        legality
    );

    assert_eq!(
        SpatialAnchorSelectionDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(PrimitiveBindingDeclarationFamily::route_contract(), route);
    assert_eq!(
        PrimitiveAnchorBindingDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        PrimitiveRebindingDeclarationFamily::route_contract(),
        rebinding_route
    );
    assert_eq!(
        TopologyNeighborhoodReplacementDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        PlanarPrecisionCertificationDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        PlanarLocalFrameCertificateDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        ProjectPointToCertifiedPlane2DDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        ProjectionConsumedPlanarFactsDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        PlanarRecoveryPostureDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        ToleranceAndPrecisionCertificationDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        HistoricalGeometryInspectionDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        BranchLocalGeometryInspectionDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        GeometryReplayParityDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        GeometryRecoveryActionDeclarationFamily::route_contract(),
        route
    );
    assert_eq!(
        GeometryProjectionConsumptionDeclarationFamily::route_contract(),
        route
    );
}
