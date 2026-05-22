use crate::spatial_intent::lowering::{admit_spatial_placement, SpatialPlacementSpec};
use crate::spatial_intent::refs::{
    SpatialAnchorRef, SpatialCarrierDirectionRole, SpatialCarrierPointRole,
    SpatialCatalogResolvedGeometricTag, SpatialDirectionWitnessRef,
    SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, resolve_spatial_direction_witness, SpatialFrameError,
    SpatialWitnessFailureClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoweredPointAnchorOrigin {
    ShapeOrigin,
    ExternalReference,
    FeatureOwned,
    GeometricTag,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoweredDirectionAnchorOrigin {
    ShapeAxis,
    FrameAxis,
    FeatureAxis,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoweredPointAnchor {
    origin: LoweredPointAnchorOrigin,
    world_point: [f64; 3],
}

impl LoweredPointAnchor {
    pub fn origin(&self) -> LoweredPointAnchorOrigin {
        self.origin
    }

    pub fn world_point(&self) -> [f64; 3] {
        self.world_point
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoweredDirectionAnchor {
    origin: LoweredDirectionAnchorOrigin,
    world_direction: [f64; 3],
}

impl LoweredDirectionAnchor {
    pub fn origin(&self) -> LoweredDirectionAnchorOrigin {
        self.origin
    }

    pub fn world_direction(&self) -> [f64; 3] {
        self.world_direction
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoweredReorientAnchor {
    PointLike(LoweredPointAnchor),
    Directional(LoweredDirectionAnchor),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoweredSubjectAnchor(LoweredPointAnchor);

impl LoweredSubjectAnchor {
    pub fn into_point(self) -> LoweredPointAnchor {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoweringAnchorDenial {
    Ambiguous,
    Unsupported,
    Undefined,
    Degenerate,
    Coincident,
    NonPointLike,
    NonDirectionLike,
    WitnessFailure(SpatialWitnessFailureClass),
    TagFailure(SpatialGeometricTagFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
    InvalidExistingPlacement,
}

pub fn lower_translation_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredPointAnchor, LoweringAnchorDenial> {
    lower_point_anchor_with_catalog(placement, anchor, catalog)
}

pub fn lower_rotation_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredPointAnchor, LoweringAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::WorldOrigin
        | SpatialAnchorRef::FrameOrigin(_)
        | SpatialAnchorRef::FeatureOwned(_) => {
            lower_point_anchor_with_catalog(placement, anchor, catalog)
        }
        _ => Err(LoweringAnchorDenial::Unsupported),
    }
}

pub fn lower_subject_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredSubjectAnchor, LoweringAnchorDenial> {
    let lowered = match anchor {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::FeatureOwned(_)
        | SpatialAnchorRef::GeometricTag(_) => {
            lower_point_anchor_with_catalog(placement, anchor, catalog)?
        }
        _ => return Err(LoweringAnchorDenial::NonPointLike),
    };
    Ok(LoweredSubjectAnchor(lowered))
}

pub fn lower_reorient_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredReorientAnchor, LoweringAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::WorldOrigin
        | SpatialAnchorRef::FrameOrigin(_) => Ok(LoweredReorientAnchor::PointLike(
            lower_point_anchor_with_catalog(placement, anchor, catalog)?,
        )),
        SpatialAnchorRef::ShapeAxis(_) | SpatialAnchorRef::FrameAxis { .. } => Ok(
            LoweredReorientAnchor::Directional(lower_direction_anchor(placement, anchor)?),
        ),
        SpatialAnchorRef::FeatureOwned(feature) => {
            lower_feature_owned_reorient_anchor(feature, catalog)
        }
        _ => Err(LoweringAnchorDenial::NonDirectionLike),
    }
}

fn lower_point_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredPointAnchor, LoweringAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeOrigin => Ok(LoweredPointAnchor {
            origin: LoweredPointAnchorOrigin::ShapeOrigin,
            world_point: resolve_shape_origin_world_point(placement)?,
        }),
        SpatialAnchorRef::WorldOrigin | SpatialAnchorRef::FrameOrigin(_) => {
            Ok(LoweredPointAnchor {
                origin: LoweredPointAnchorOrigin::ExternalReference,
                world_point: resolve_external_reference_world_point(anchor)?,
            })
        }
        SpatialAnchorRef::FeatureOwned(feature) => Ok(LoweredPointAnchor {
            origin: LoweredPointAnchorOrigin::FeatureOwned,
            world_point: catalog
                .resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor)
                .map(|resolved| resolved.world_point())
                .map_err(LoweringAnchorDenial::WitnessFailure)?,
        }),
        SpatialAnchorRef::GeometricTag(tag) => Ok(LoweredPointAnchor {
            origin: LoweredPointAnchorOrigin::GeometricTag,
            world_point: resolve_geometric_tag_anchor_world_point(tag, catalog)?,
        }),
        _ => Err(LoweringAnchorDenial::NonPointLike),
    }
}

fn lower_direction_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
) -> Result<LoweredDirectionAnchor, LoweringAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeAxis(_) => Ok(LoweredDirectionAnchor {
            origin: LoweredDirectionAnchorOrigin::ShapeAxis,
            world_direction: resolve_axis_world_direction(placement, anchor)?,
        }),
        SpatialAnchorRef::FrameAxis { .. } => Ok(LoweredDirectionAnchor {
            origin: LoweredDirectionAnchorOrigin::FrameAxis,
            world_direction: resolve_axis_world_direction(placement, anchor)?,
        }),
        _ => Err(LoweringAnchorDenial::NonDirectionLike),
    }
}

fn lower_feature_owned_reorient_anchor(
    feature: &str,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredReorientAnchor, LoweringAnchorDenial> {
    let point_result =
        catalog.resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor);
    let direction_result =
        catalog.resolve_feature_owned_direction(feature, SpatialCarrierDirectionRole::Axis);
    match (point_result, direction_result) {
        (Ok(point), Err(_)) => Ok(LoweredReorientAnchor::PointLike(LoweredPointAnchor {
            origin: LoweredPointAnchorOrigin::FeatureOwned,
            world_point: point.world_point(),
        })),
        (Err(SpatialWitnessFailureClass::Unsupported), Ok(direction)) => {
            Ok(LoweredReorientAnchor::Directional(LoweredDirectionAnchor {
                origin: LoweredDirectionAnchorOrigin::FeatureAxis,
                world_direction: direction.world_direction(),
            }))
        }
        (Err(SpatialWitnessFailureClass::Ambiguous), Ok(_)) | (Ok(_), Ok(_)) => {
            Err(LoweringAnchorDenial::Ambiguous)
        }
        (Err(point_error), Ok(_)) => Err(LoweringAnchorDenial::WitnessFailure(point_error)),
        (Err(point_error), Err(direction_error)) => {
            classify_feature_owned_failure(point_error, direction_error)
        }
    }
}

fn resolve_shape_origin_world_point(
    placement: &SpatialPlacementSpec,
) -> Result<[f64; 3], LoweringAnchorDenial> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
        .map_err(LoweringAnchorDenial::InvalidReferenceFrame)?;
    Ok(reference_frame.basis().embed_point(placement.origin()))
}

fn resolve_external_reference_world_point(
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], LoweringAnchorDenial> {
    match anchor {
        SpatialAnchorRef::WorldOrigin => Ok([0.0, 0.0, 0.0]),
        SpatialAnchorRef::FrameOrigin(frame) => admit_spatial_frame(frame.clone())
            .map(|frame| frame.basis().origin())
            .map_err(LoweringAnchorDenial::InvalidReferenceFrame),
        _ => Err(LoweringAnchorDenial::Unsupported),
    }
}

fn resolve_geometric_tag_anchor_world_point(
    tag: &str,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<[f64; 3], LoweringAnchorDenial> {
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
        .map_err(LoweringAnchorDenial::TagFailure)
}

fn resolve_axis_world_direction(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
) -> Result<[f64; 3], LoweringAnchorDenial> {
    match anchor {
        SpatialAnchorRef::ShapeAxis(axis) => {
            let admitted = admit_spatial_placement(placement.clone())
                .map_err(|_| LoweringAnchorDenial::InvalidExistingPlacement)?;
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
        .map_err(LoweringAnchorDenial::WitnessFailure),
        _ => Err(LoweringAnchorDenial::Unsupported),
    }
}

fn classify_feature_owned_failure(
    point_error: SpatialWitnessFailureClass,
    direction_error: SpatialWitnessFailureClass,
) -> Result<LoweredReorientAnchor, LoweringAnchorDenial> {
    if point_error == SpatialWitnessFailureClass::Unsupported {
        return Err(LoweringAnchorDenial::WitnessFailure(direction_error));
    }
    if direction_error == SpatialWitnessFailureClass::Unsupported {
        return Err(LoweringAnchorDenial::WitnessFailure(point_error));
    }
    if point_error == direction_error {
        return Err(LoweringAnchorDenial::WitnessFailure(point_error));
    }
    Err(LoweringAnchorDenial::Ambiguous)
}
