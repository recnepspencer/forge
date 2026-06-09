#![cfg_attr(not(any(test, feature = "test-support-lowering")), allow(dead_code))]

use crate::authored_refs::{
    SpatialAnchorRef, SpatialCarrierPointRole, SpatialCatalogResolvedGeometricTag,
    SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::{
    admit_spatial_frame, SpatialFrameError, SpatialWitnessFailureClass,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConstraintAnchorDenial {
    Unsupported,
    NonPointLike,
    WitnessFailure(SpatialWitnessFailureClass),
    TagFailure(SpatialGeometricTagFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
}

pub(crate) fn lower_points_toward_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    lower_translation_anchor_with_catalog(placement, anchor, catalog)
}

pub(crate) fn lower_anchor_match_target_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    lower_translation_anchor_with_catalog(placement, anchor, catalog)
}

pub(crate) fn lower_subject_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::FeatureOwned(_)
        | SpatialAnchorRef::GeometricTag(_) => {
            lower_subject_point_anchor_with_catalog(placement, anchor, catalog)
        }
        _ => Err(ConstraintAnchorDenial::NonPointLike),
    }
}

fn lower_translation_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin => resolve_shape_origin_world_point(placement),
        SpatialAnchorRef::WorldOrigin | SpatialAnchorRef::FrameOrigin(_) => {
            resolve_external_reference_world_point(anchor)
        }
        SpatialAnchorRef::FeatureOwned(feature) => catalog
            .resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor)
            .map(|resolved| resolved.world_point())
            .map_err(ConstraintAnchorDenial::WitnessFailure),
        SpatialAnchorRef::GeometricTag(tag) => {
            resolve_geometric_tag_anchor_world_point(tag, catalog)
        }
        _ => Err(ConstraintAnchorDenial::NonPointLike),
    }
}

fn lower_subject_point_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin => resolve_shape_origin_world_point(placement),
        SpatialAnchorRef::FeatureOwned(feature) => catalog
            .resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor)
            .map(|resolved| resolved.world_point())
            .map_err(ConstraintAnchorDenial::WitnessFailure),
        SpatialAnchorRef::GeometricTag(tag) => {
            resolve_geometric_tag_anchor_world_point(tag, catalog)
        }
        _ => Err(ConstraintAnchorDenial::NonPointLike),
    }
}

fn resolve_shape_origin_world_point(
    placement: &SpatialPlacementSpec,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
        .map_err(ConstraintAnchorDenial::InvalidReferenceFrame)?;
    Ok(reference_frame.basis().embed_point(placement.origin()))
}

fn resolve_external_reference_world_point(
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    match anchor {
        SpatialAnchorRef::WorldOrigin => Ok([0.0, 0.0, 0.0]),
        SpatialAnchorRef::FrameOrigin(frame) => admit_spatial_frame(frame.clone())
            .map(|frame| frame.basis().origin())
            .map_err(ConstraintAnchorDenial::InvalidReferenceFrame),
        _ => Err(ConstraintAnchorDenial::Unsupported),
    }
}

fn resolve_geometric_tag_anchor_world_point(
    tag: &str,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], ConstraintAnchorDenial> {
    catalog
        .resolve_geometric_tag(tag)
        .map_err(SpatialGeometricTagFailureClass::Resolution)
        .and_then(|resolved| match resolved {
            SpatialCatalogResolvedGeometricTag::PointLike(resolved) => Ok(resolved.world_point()),
            SpatialCatalogResolvedGeometricTag::DirectionLike(_) => {
                Err(SpatialGeometricTagFailureClass::ResolvedDirectionLike)
            }
            SpatialCatalogResolvedGeometricTag::UnsupportedClass => {
                Err(SpatialGeometricTagFailureClass::ResolvedUnsupportedClass)
            }
        })
        .map_err(ConstraintAnchorDenial::TagFailure)
}
