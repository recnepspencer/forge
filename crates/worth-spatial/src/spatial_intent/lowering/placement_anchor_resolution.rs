use crate::spatial_intent::lowering::placement_anchor_directions::SpatialPlacementDirectionalAnchorError;
use crate::spatial_intent::lowering::placement_anchor_points::SpatialPlacementPointAnchorError;
use crate::spatial_intent::lowering::{admit_spatial_placement, SpatialPlacementSpec};
use crate::spatial_intent::refs::{
    SpatialAnchorRef, SpatialCarrierPointRole, SpatialCatalogResolvedGeometricTag,
    SpatialDirectionWitnessRef, SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, resolve_spatial_direction_witness,
    resolve_spatial_direction_witness_with_catalog, SpatialWitnessFailureClass,
};

pub(crate) fn resolve_shape_origin_world_point(
    placement: &SpatialPlacementSpec,
) -> Result<[f64; 3], SpatialPlacementPointAnchorError> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
        .map_err(SpatialPlacementPointAnchorError::InvalidReferenceFrame)?;
    Ok(reference_frame.basis().embed_point(placement.origin()))
}

pub(crate) fn resolve_external_reference_world_point(
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], SpatialPlacementPointAnchorError> {
    match anchor {
        SpatialAnchorRef::WorldOrigin => Ok([0.0, 0.0, 0.0]),
        SpatialAnchorRef::FrameOrigin(frame) => admit_spatial_frame(frame.clone())
            .map(|frame| frame.basis().origin())
            .map_err(SpatialPlacementPointAnchorError::InvalidReferenceFrame),
        _ => Err(SpatialPlacementPointAnchorError::UnsupportedAnchor),
    }
}

pub(crate) fn resolve_feature_owned_anchor_world_point(
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], SpatialPlacementPointAnchorError> {
    match anchor {
        SpatialAnchorRef::FeatureOwned(feature) => catalog
            .resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor)
            .map(|resolved| resolved.world_point())
            .map_err(SpatialPlacementPointAnchorError::AnchorWitnessFailure),
        _ => Err(SpatialPlacementPointAnchorError::UnsupportedAnchor),
    }
}

pub(crate) fn resolve_geometric_tag_anchor_world_point(
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], SpatialPlacementPointAnchorError> {
    match anchor {
        SpatialAnchorRef::GeometricTag(tag) => catalog
            .resolve_geometric_tag(tag)
            .map_err(SpatialGeometricTagFailureClass::Resolution)
            .and_then(|resolved| match resolved {
                SpatialCatalogResolvedGeometricTag::PointLike(resolved) => {
                    Ok(resolved.world_point())
                }
                SpatialCatalogResolvedGeometricTag::DirectionLike(_) => {
                    Err(SpatialGeometricTagFailureClass::ResolvedDirectionLike)
                }
                SpatialCatalogResolvedGeometricTag::UnsupportedClass => {
                    Err(SpatialGeometricTagFailureClass::ResolvedUnsupportedClass)
                }
            })
            .map_err(SpatialPlacementPointAnchorError::AnchorTagFailure),
        _ => Err(SpatialPlacementPointAnchorError::UnsupportedAnchor),
    }
}

pub(crate) fn resolve_axis_world_direction(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], SpatialPlacementDirectionalAnchorError> {
    match anchor {
        SpatialAnchorRef::ShapeAxis(axis) => {
            let admitted = admit_spatial_placement(placement.clone())
                .map_err(|_| SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)?;
            let frame = admitted.frame();
            Ok(match axis {
                crate::spatial_intent::refs::SpatialAxis::U => frame.u_axis(),
                crate::spatial_intent::refs::SpatialAxis::V => frame.v_axis(),
                crate::spatial_intent::refs::SpatialAxis::W => frame.w_axis(),
            })
        }
        SpatialAnchorRef::FrameAxis { frame, axis } => resolve_spatial_direction_witness(
            SpatialDirectionWitnessRef::frame_axis(frame.clone(), *axis),
        )
        .map(|resolved| resolved.resolved_world_direction())
        .map_err(SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure),
        _ => Err(SpatialPlacementDirectionalAnchorError::UnsupportedAnchor),
    }
}

pub(crate) fn resolve_feature_axis_world_direction(
    feature: String,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], SpatialPlacementDirectionalAnchorError> {
    resolve_spatial_direction_witness_with_catalog(
        SpatialDirectionWitnessRef::feature_axis(feature),
        catalog,
    )
    .map(|resolved| resolved.resolved_world_direction())
    .map_err(SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure)
}

pub(crate) fn classify_feature_owned_failure(
    point_error: SpatialWitnessFailureClass,
    direction_error: SpatialWitnessFailureClass,
) -> SpatialPlacementDirectionalAnchorError {
    if point_error == SpatialWitnessFailureClass::Unsupported {
        return SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure(direction_error);
    }
    if direction_error == SpatialWitnessFailureClass::Unsupported {
        return SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure(point_error);
    }
    if point_error == direction_error {
        return SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure(point_error);
    }
    SpatialPlacementDirectionalAnchorError::AmbiguousAnchorMeaning
}
