use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryDeclarationFamilyMarker, ForgeQueryDomainEntryMarker,
    ForgeQueryLowerAuthorityRouteFamily,
};
use worth_spatial::certification::geometry_support_posture::{
    geometry_public_support_matrix, GeometryPublicSupportStatus,
};
use worth_spatial::facade::anchor_selection::{
    spatial_anchor_selection_projection_facts, AuthorSpatialAnchorSelectionIntent,
    SpatialAnchorSelectionDeclarationEntry, SpatialAnchorSelectionDeclarationFamily,
    SpatialAnchorSelectionQueryDomain, SpatialAnchorSelectionQueryWorld,
    SpatialAnchorSelectionRequestedInput, SpatialAnchorSelectionStatus, SpatialMoveSpec,
};
use worth_spatial::facade::refs::EmptySpatialWitnessCatalog;
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialPointWitnessRef};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_anchor_selection_query_domain_entry_surface() {
    let domain = SpatialAnchorSelectionQueryDomain;

    assert_eq!(domain.domain_key(), "worth.spatial.anchor_selection");
    assert_eq!(domain.display_name(), "WorthSpatialAnchorSelectionDomain");
}

#[test]
fn spatial_public_facade_exports_single_route_anchor_selection_family_contract() {
    assert_eq!(
        SpatialAnchorSelectionDeclarationFamily::semantic_family_key(),
        "SpatialAnchorSelection"
    );
    assert_eq!(
        SpatialAnchorSelectionDeclarationFamily::route_contract().allowed_route_families(),
        [ForgeQueryLowerAuthorityRouteFamily::Relational]
    );
    assert_eq!(
        SpatialAnchorSelectionDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
}

#[test]
fn spatial_public_facade_exports_anchor_selection_projection_fact_surface() {
    let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::ShapeOrigin)
                .to([4.0, 2.0, 1.0]),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(SpatialAnchorSelectionQueryDomain)
        .with_operating_context(SpatialAnchorSelectionQueryWorld::new(
            "public-api-anchor-selection",
        ))
        .validate()
        .expect("validated anchor selection handle")
        .admit()
        .expect("admitted anchor selection handle");
    let facts =
        spatial_anchor_selection_projection_facts(&declaration, &handle).expect("projection facts");

    assert_eq!(facts.kind(), declaration.kind());
    assert_eq!(facts.status(), SpatialAnchorSelectionStatus::Admitted);
    assert_eq!(
        facts.requested_input(),
        &SpatialAnchorSelectionRequestedInput::PointWitness(SpatialPointWitnessRef::world_point([
            4.0, 2.0, 1.0,
        ]))
    );
    assert!(facts.resolution_class().is_some());
    assert!(facts.progression_digest().is_some());
}

#[test]
fn spatial_anchor_selection_is_present_in_public_support_and_applicability_contracts() {
    let support = geometry_public_support_matrix();
    let applicability = geometry_applicability_matrix();

    assert_eq!(
        support
            .row_for_surface(GeometryPublicSurface::SpatialAnchorSelection)
            .expect("anchor selection support row should exist")
            .status(),
        GeometryPublicSupportStatus::Supported
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::SpatialAnchorSelection,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("anchor selection routing applicability row should exist")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
}
