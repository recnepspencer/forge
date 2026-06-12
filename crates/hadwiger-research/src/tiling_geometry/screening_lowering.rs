use crate::candidate_screening::{
    draft_candidate_screening_invariant_catalog_checked,
    evaluate_boundary_ownership_screening_checked,
    evaluate_exact_unit_distance_conflict_screening_checked,
    evaluate_minkowski_difference_screening_checked,
    evaluate_same_color_separation_screening_checked, evaluate_tile_diameter_screening_checked,
    BoundaryOwnedRegion, BoundaryOwnershipCertificate, ExactUnitDistanceConflictCertificate,
    MinkowskiUnitIntersectionCertificate, SameColorSeparationCertificate,
    ScreeningSolverTranscript, TileDiameterCertificate,
};
use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{
    HadwigerArtifactReference, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
};
use crate::domain_declarations::{declare_research_request_checked, TileContactWitnessDeclaration};
use crate::query_entry::HadwigerResearchHandle;

use super::cell_artifacts::TilingCell;
use super::contact_facts::{
    TilingBoundaryOwnershipReport, TilingContactFact, TilingContactReplayReport, TilingContactRole,
};
use super::tiling_geometry_errors::TilingGeometryError;

pub fn evaluate_tiling_boundary_ownership_checked(
    handle: &HadwigerResearchHandle,
    cell: &TilingCell,
) -> Result<TilingBoundaryOwnershipReport, TilingGeometryError> {
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let mut rows = Vec::new();
    for tile in cell.tiles() {
        let region = tile.to_screening_region()?;
        rows.push(BoundaryOwnedRegion::new(
            region,
            tile.color_id().as_str(),
            tile.boundary_ownership()
                .map(|policy| policy.owns_boundary())
                .unwrap_or(false),
        )?);
    }
    let certificate = BoundaryOwnershipCertificate::new(
        format!("boundary:{}", cell.reference().stable_token()),
        rows,
        transcript("boundary_ownership"),
    )?;
    let evaluation = evaluate_boundary_ownership_screening_checked(
        handle,
        &catalog,
        cell.reference(),
        certificate,
    )?;
    TilingBoundaryOwnershipReport::checked(cell, evaluation)
}

pub fn evaluate_tiling_tile_diameter_checked(
    handle: &HadwigerResearchHandle,
    cell: &TilingCell,
    tile_id: impl AsRef<str>,
) -> Result<TilingContactReplayReport, TilingGeometryError> {
    let tile = cell.require_tile(tile_id.as_ref())?;
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let certificate =
        TileDiameterCertificate::new(tile.to_screening_region()?, transcript("tile_diameter"))?;
    let evaluation =
        evaluate_tile_diameter_screening_checked(handle, &catalog, cell.reference(), certificate)?;
    let fact = TilingContactFact::exact_replay(
        tile.tile_id().as_str(),
        format!("{}#internal_diameter", tile.tile_id().as_str()),
        TilingContactRole::DiameterSafety,
    )?;
    TilingContactReplayReport::checked(cell, fact, evaluation, None)
}

pub fn evaluate_tiling_same_color_contact_checked(
    handle: &HadwigerResearchHandle,
    cell: &TilingCell,
    left_tile_id: impl AsRef<str>,
    right_tile_id: impl AsRef<str>,
) -> Result<TilingContactReplayReport, TilingGeometryError> {
    let (left, right, subject, contact_witness_digest) = declared_contact_subject(
        handle,
        cell,
        left_tile_id.as_ref(),
        right_tile_id.as_ref(),
        TilingContactRole::SameColorConflictCandidate,
    )?;
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let certificate = SameColorSeparationCertificate::new(
        left.to_screening_region()?,
        right.to_screening_region()?,
        transcript("same_color_separation"),
    )?;
    let evaluation =
        evaluate_same_color_separation_screening_checked(handle, &catalog, subject, certificate)?;
    let fact = TilingContactFact::exact_replay(
        left.tile_id().as_str(),
        right.tile_id().as_str(),
        TilingContactRole::SameColorConflictCandidate,
    )?;
    TilingContactReplayReport::checked(cell, fact, evaluation, Some(contact_witness_digest))
}

pub fn evaluate_tiling_minkowski_contact_checked(
    handle: &HadwigerResearchHandle,
    cell: &TilingCell,
    left_tile_id: impl AsRef<str>,
    right_tile_id: impl AsRef<str>,
) -> Result<TilingContactReplayReport, TilingGeometryError> {
    let role = TilingContactRole::MinkowskiUnitContact;
    let (left, right, subject, contact_witness_digest) = declared_contact_subject(
        handle,
        cell,
        left_tile_id.as_ref(),
        right_tile_id.as_ref(),
        role,
    )?;
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let certificate = MinkowskiUnitIntersectionCertificate::new(
        left.to_screening_region()?,
        right.to_screening_region()?,
        transcript("minkowski_difference"),
    )?;
    let evaluation =
        evaluate_minkowski_difference_screening_checked(handle, &catalog, subject, certificate)?;
    let fact =
        TilingContactFact::exact_replay(left.tile_id().as_str(), right.tile_id().as_str(), role)?;
    TilingContactReplayReport::checked(cell, fact, evaluation, Some(contact_witness_digest))
}

pub fn evaluate_tiling_exact_unit_contact_checked(
    handle: &HadwigerResearchHandle,
    cell: &TilingCell,
    left_tile_id: impl AsRef<str>,
    right_tile_id: impl AsRef<str>,
) -> Result<TilingContactReplayReport, TilingGeometryError> {
    let (left, right, subject, contact_witness_digest) = declared_contact_subject(
        handle,
        cell,
        left_tile_id.as_ref(),
        right_tile_id.as_ref(),
        TilingContactRole::BoundaryContact,
    )?;
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let certificate = ExactUnitDistanceConflictCertificate::new(
        left.to_screening_region()?,
        right.to_screening_region()?,
        transcript("exact_unit_distance_conflict"),
    )?;
    let evaluation = evaluate_exact_unit_distance_conflict_screening_checked(
        handle,
        &catalog,
        subject,
        certificate,
    )?;
    let fact = TilingContactFact::exact_replay(
        left.tile_id().as_str(),
        right.tile_id().as_str(),
        TilingContactRole::BoundaryContact,
    )?;
    TilingContactReplayReport::checked(cell, fact, evaluation, Some(contact_witness_digest))
}

fn declared_contact_subject<'a>(
    handle: &HadwigerResearchHandle,
    cell: &'a TilingCell,
    left_tile_id: &str,
    right_tile_id: &str,
    role: TilingContactRole,
) -> Result<
    (
        &'a super::rectangular_regions::RectangularTileRegion,
        &'a super::rectangular_regions::RectangularTileRegion,
        HadwigerArtifactReference,
        String,
    ),
    TilingGeometryError,
> {
    let left = cell.require_tile(left_tile_id)?;
    let right = cell.require_tile(right_tile_id)?;
    if left.tile_id() == right.tile_id() {
        return Err(TilingGeometryError::SameTileContact {
            tile_id: left.tile_id().as_str().to_string(),
        });
    }
    let (left, right) = if left.tile_id().as_str() <= right.tile_id().as_str() {
        (left, right)
    } else {
        (right, left)
    };
    let contact_signature = format!(
        "{}:{}:{}:{}",
        cell.reference().stable_token(),
        left.tile_id().as_str(),
        right.tile_id().as_str(),
        role.as_str()
    );
    let checked = declare_research_request_checked(
        handle,
        TileContactWitnessDeclaration::new(contact_signature.clone())
            .with_left_tile_ref(left.tile_id().as_str())
            .with_right_tile_ref(right.tile_id().as_str())
            .with_contact_signature(contact_signature),
    );
    let Some(admitted) = checked.admitted() else {
        return Err(TilingGeometryError::QueryContactDeclarationNotAdmitted);
    };
    Ok((
        left,
        right,
        cell.reference(),
        canonical_digest_token(admitted.declaration_digest()),
    ))
}

fn transcript(label: &'static str) -> ScreeningSolverTranscript {
    ScreeningSolverTranscript::new(
        "hadwiger-rectangular-geometry",
        "phase3",
        format!("transcript:{label}"),
        "exact_replay",
    )
    .expect("static transcript fields are non-empty")
}
