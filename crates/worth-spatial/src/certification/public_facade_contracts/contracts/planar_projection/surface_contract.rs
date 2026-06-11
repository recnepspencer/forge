use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
};
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, ProjectPointToCertifiedPlane2DBasis,
    ProjectPointToCertifiedPlane2DCase, ProjectPointToCertifiedPlane2DDeclarationFamily,
    ProjectPointToCertifiedPlane2DEntry, ProjectPointToCertifiedPlane2DMutationEvidence,
    ProjectPointToCertifiedPlane2DPerformanceCounters, ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld,
};

use super::proof_fixture::{certified_frame, projection_basis};

#[test]
fn spatial_public_facade_exports_certified_plane_projection_surface() {
    let frame = certified_frame(
        "projection-surface",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let basis = projection_basis(&frame, "point:thin-slot-corner-a", [1.0e-9, 0.0, 0.0]);
    let entry = project_point_to_certified_plane_2d_entry(
        ProjectPointToCertifiedPlane2DCase::from_local_frame(basis.clone()),
    );

    let _: ProjectPointToCertifiedPlane2DEntry = entry;
    let _: ProjectPointToCertifiedPlane2DBasis = basis;
    let _: ProjectPointToCertifiedPlane2DDeclarationFamily =
        ProjectPointToCertifiedPlane2DDeclarationFamily;
    let _: ProjectPointToCertifiedPlane2DQueryDomain = ProjectPointToCertifiedPlane2DQueryDomain;
    let _: ProjectPointToCertifiedPlane2DQueryWorld =
        ProjectPointToCertifiedPlane2DQueryWorld::new("public");
    let _: Option<ProjectPointToCertifiedPlane2DPerformanceCounters> = None;
    let _: Option<ProjectPointToCertifiedPlane2DMutationEvidence> = None;
}

#[test]
fn certified_plane_projection_family_is_query_native_and_relational() {
    let aspect_contract = ProjectPointToCertifiedPlane2DDeclarationFamily::aspect_contract();

    assert_eq!(
        ProjectPointToCertifiedPlane2DDeclarationFamily::semantic_family_key(),
        "ProjectPointToCertifiedPlane2D"
    );
    assert_eq!(
        ProjectPointToCertifiedPlane2DDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
    assert!(aspect_contract
        .required()
        .contains(&"geometry.planar_projection.local_frame_fact".to_string()));
    assert!(aspect_contract
        .required()
        .contains(&"geometry.planar_projection.source_point_basis".to_string()));
    assert!(aspect_contract
        .required()
        .contains(&"geometry.planar_projection.local_delta".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.planar_projection.point_2d".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.planar_projection.signed_distance".to_string()));
}

#[test]
fn certified_plane_projection_declaration_commits_frame_and_point_basis() {
    let frame = certified_frame(
        "projection-declaration",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let basis = projection_basis(&frame, "point:thin-slot-corner-a", [1.0e-9, 0.0, 0.0]);
    let entry = project_point_to_certified_plane_2d_entry(
        ProjectPointToCertifiedPlane2DCase::from_local_frame(basis.clone()),
    );
    let canonical_entries = entry.canonical_declaration_entries();

    assert_eq!(
        canonical_entry_text(
            &canonical_entries,
            "geometry.planar_projection.local_frame_fact"
        ),
        frame.fact_digest()
    );
    assert_eq!(
        canonical_entry_text(
            &canonical_entries,
            "geometry.planar_projection.source_point_basis"
        ),
        "point-basis:thin-slot-local-normalized"
    );
    assert_eq!(
        canonical_entry_text(&canonical_entries, "geometry.planar_projection.u_axis"),
        format!("{:?}", basis.u_axis())
    );
    assert_eq!(
        canonical_entry_text(&canonical_entries, "geometry.planar_projection.v_axis"),
        format!("{:?}", basis.v_axis())
    );
    assert_eq!(
        canonical_entry_text(
            &canonical_entries,
            "geometry.planar_projection.frame_origin"
        ),
        format!("{:?}", basis.frame_origin())
    );
    assert_eq!(
        canonical_entry_text(&canonical_entries, "geometry.planar_projection.point_2d"),
        format!("{:?}", [0.0, -1.0e-9])
    );
}

fn canonical_entry_text(entries: &[ForgeQueryDeclarationCanonicalEntry], locus: &str) -> String {
    let entry = entries
        .iter()
        .find(|entry| entry.locus() == locus)
        .unwrap_or_else(|| panic!("missing canonical declaration entry for {locus}"));
    match entry.value() {
        ForgeQueryDeclarationCanonicalValue::ExactText(text) => text.clone(),
        other => panic!("expected exact text canonical value for {locus}, got {other:?}"),
    }
}
