#![cfg_attr(not(any(test, feature = "test-support-lowering")), allow(dead_code))]

use super::placement_types::SpatialPlacementFrame;
use crate::authored_refs::{
    EmptySpatialWitnessCatalog, SpatialAnchorRef, SpatialCarrierDirectionRole,
    SpatialCarrierPointRole, SpatialCatalogResolvedGeometricTag, SpatialDirectionWitnessRef,
    SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::witness_resolution::resolve_spatial_direction_witness_with_catalog;
use crate::witness_resolution::{
    admit_spatial_frame, SpatialFrameError, SpatialWitnessFailureClass,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum LoweredReorientAnchor {
    PointLike([f64; 3]),
    Directional([f64; 3]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MotionAnchorDenial {
    Ambiguous,
    Unsupported,
    WitnessFailure(SpatialWitnessFailureClass),
    TagFailure(SpatialGeometricTagFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
}

pub(crate) fn lower_move_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], MotionAnchorDenial> {
    lower_translation_anchor_with_catalog(placement, anchor, catalog)
}

pub(crate) fn lower_offset_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], MotionAnchorDenial> {
    lower_translation_anchor_with_catalog(placement, anchor, catalog)
}

pub(crate) fn lower_rotate_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], MotionAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::WorldOrigin
        | SpatialAnchorRef::FrameOrigin(_)
        | SpatialAnchorRef::FeatureOwned(_) => {
            lower_translation_anchor_with_catalog(placement, anchor, catalog)
        }
        _ => Err(MotionAnchorDenial::Unsupported),
    }
}

pub(crate) fn lower_reorient_anchor(
    placement_frame: SpatialPlacementFrame,
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredReorientAnchor, MotionAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::WorldOrigin
        | SpatialAnchorRef::FrameOrigin(_) => Ok(LoweredReorientAnchor::PointLike(
            lower_translation_anchor_with_catalog(placement, anchor, catalog)?,
        )),
        SpatialAnchorRef::ShapeAxis(_) | SpatialAnchorRef::FrameAxis { .. } => {
            Ok(LoweredReorientAnchor::Directional(
                resolve_axis_world_direction(placement_frame, anchor)?,
            ))
        }
        SpatialAnchorRef::FeatureOwned(feature) => {
            lower_feature_owned_reorient_anchor(feature, catalog)
        }
        _ => Err(MotionAnchorDenial::Unsupported),
    }
}

fn lower_translation_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], MotionAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin => resolve_shape_origin_world_point(placement),
        SpatialAnchorRef::WorldOrigin | SpatialAnchorRef::FrameOrigin(_) => {
            resolve_external_reference_world_point(anchor)
        }
        SpatialAnchorRef::FeatureOwned(feature) => catalog
            .resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor)
            .map(|resolved| resolved.world_point())
            .map_err(MotionAnchorDenial::WitnessFailure),
        SpatialAnchorRef::GeometricTag(tag) => {
            resolve_geometric_tag_anchor_world_point(tag, catalog)
        }
        _ => Err(MotionAnchorDenial::Unsupported),
    }
}

fn lower_feature_owned_reorient_anchor(
    feature: &str,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredReorientAnchor, MotionAnchorDenial> {
    let point_result =
        catalog.resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor);
    let direction_result =
        catalog.resolve_feature_owned_direction(feature, SpatialCarrierDirectionRole::Axis);
    match (point_result, direction_result) {
        (Ok(point), Err(_)) => Ok(LoweredReorientAnchor::PointLike(point.world_point())),
        (Err(SpatialWitnessFailureClass::Unsupported), Ok(direction)) => Ok(
            LoweredReorientAnchor::Directional(direction.world_direction()),
        ),
        (Err(SpatialWitnessFailureClass::Ambiguous), Ok(_)) | (Ok(_), Ok(_)) => {
            Err(MotionAnchorDenial::Ambiguous)
        }
        (Err(point_error), Ok(_)) => Err(MotionAnchorDenial::WitnessFailure(point_error)),
        (Err(point_error), Err(direction_error)) => {
            classify_feature_owned_failure(point_error, direction_error)
        }
    }
}

fn resolve_shape_origin_world_point(
    placement: &SpatialPlacementSpec,
) -> Result<[f64; 3], MotionAnchorDenial> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
        .map_err(MotionAnchorDenial::InvalidReferenceFrame)?;
    Ok(reference_frame.basis().embed_point(placement.origin()))
}

fn resolve_external_reference_world_point(
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], MotionAnchorDenial> {
    match anchor {
        SpatialAnchorRef::WorldOrigin => Ok([0.0, 0.0, 0.0]),
        SpatialAnchorRef::FrameOrigin(frame) => admit_spatial_frame(frame.clone())
            .map(|frame| frame.basis().origin())
            .map_err(MotionAnchorDenial::InvalidReferenceFrame),
        _ => Err(MotionAnchorDenial::Unsupported),
    }
}

fn resolve_axis_world_direction(
    placement_frame: SpatialPlacementFrame,
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], MotionAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeAxis(axis) => Ok(match axis {
            crate::authored_refs::SpatialAxis::U => placement_frame.u_axis(),
            crate::authored_refs::SpatialAxis::V => placement_frame.v_axis(),
            crate::authored_refs::SpatialAxis::W => placement_frame.w_axis(),
        }),
        SpatialAnchorRef::FrameAxis { frame, axis } => {
            resolve_spatial_direction_witness_with_catalog(
                SpatialDirectionWitnessRef::frame_axis(frame.clone(), *axis),
                &EmptySpatialWitnessCatalog,
            )
            .map(|resolved| resolved.resolved_world_direction())
            .map_err(MotionAnchorDenial::WitnessFailure)
        }
        _ => Err(MotionAnchorDenial::Unsupported),
    }
}

fn resolve_geometric_tag_anchor_world_point(
    tag: &str,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], MotionAnchorDenial> {
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
        .map_err(MotionAnchorDenial::TagFailure)
}

fn classify_feature_owned_failure(
    point_error: SpatialWitnessFailureClass,
    direction_error: SpatialWitnessFailureClass,
) -> Result<LoweredReorientAnchor, MotionAnchorDenial> {
    if point_error == SpatialWitnessFailureClass::Unsupported {
        return Err(MotionAnchorDenial::WitnessFailure(direction_error));
    }
    if direction_error == SpatialWitnessFailureClass::Unsupported {
        return Err(MotionAnchorDenial::WitnessFailure(point_error));
    }
    if point_error == direction_error {
        return Err(MotionAnchorDenial::WitnessFailure(point_error));
    }
    Err(MotionAnchorDenial::Ambiguous)
}
