#![cfg_attr(not(any(test, feature = "test-support-lowering")), allow(dead_code))]

use super::placement_motion_anchors::{
    lower_move_anchor_with_catalog, lower_offset_anchor_with_catalog, lower_reorient_anchor,
    lower_rotate_anchor, LoweredReorientAnchor, MotionAnchorDenial,
};
use super::placement_types::{SpatialPlacementFrame, SpatialPlacementMotionError};
use crate::anchor_selection::{
    SpatialMoveSpec, SpatialOffsetSpec, SpatialReorientSpec, SpatialRotateSpec,
};
use crate::authored_refs::SpatialDirectionWitnessRef;
use crate::authored_refs::SpatialWitnessCatalog;
use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::admit_spatial_frame;
use worth_math::{canonical_perpendicular_unit_vector, UnitVector3};

pub(crate) fn apply_move_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    spec: &SpatialMoveSpec,
    destination_point: [f64; 3],
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let anchor = lower_move_anchor_with_catalog(&placement, spec.anchor(), catalog)
        .map_err(map_move_anchor_failure)?;
    translate_anchor_to_world_point(placement, anchor, destination_point)
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}

pub(crate) fn apply_offset_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    spec: &SpatialOffsetSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let _anchor = lower_offset_anchor_with_catalog(&placement, spec.anchor(), catalog)
        .map_err(map_offset_anchor_failure)?;
    translate_placement_world_offset(placement, spec.offset())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}

pub(crate) fn apply_rotate_to_placement_with_catalog(
    facing_vector: [f64; 3],
    placement: SpatialPlacementSpec,
    spec: &SpatialRotateSpec,
    normalized_axis: [f64; 3],
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match spec.anchor() {
        crate::authored_refs::SpatialAnchorRef::ShapeOrigin => Ok(placement.facing(rotate_vector(
            facing_vector,
            normalized_axis,
            spec.angle_radians(),
        ))),
        _ => {
            let pivot = lower_rotate_anchor(&placement, spec.anchor(), catalog)
                .map_err(map_rotate_anchor_failure)?;
            rotate_origin_and_facing(
                placement,
                facing_vector,
                pivot,
                normalized_axis,
                spec.angle_radians(),
            )
            .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
        }
    }
}

pub(crate) fn apply_reorient_to_placement_with_catalog(
    placement_frame: SpatialPlacementFrame,
    facing_vector: [f64; 3],
    placement: SpatialPlacementSpec,
    spec: &SpatialReorientSpec,
    direction_witness: &SpatialDirectionWitnessRef,
    normalized_facing: [f64; 3],
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match lower_reorient_anchor(placement_frame, &placement, spec.anchor(), catalog)
        .map_err(map_reorient_anchor_failure)?
    {
        LoweredReorientAnchor::PointLike(_) => {
            Ok(placement.facing_witness(direction_witness.clone()))
        }
        LoweredReorientAnchor::Directional(anchor) => {
            rotate_vector_to_align_source(facing_vector, anchor, normalized_facing)
                .map(|facing| placement.facing(facing))
                .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
        }
    }
}

fn map_move_anchor_failure(denial: MotionAnchorDenial) -> SpatialPlacementMotionError {
    match denial {
        MotionAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        MotionAnchorDenial::TagFailure(value) => {
            SpatialPlacementMotionError::AnchorTagFailure(value)
        }
        MotionAnchorDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedMoveAnchor,
    }
}

fn map_offset_anchor_failure(denial: MotionAnchorDenial) -> SpatialPlacementMotionError {
    match denial {
        MotionAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        MotionAnchorDenial::TagFailure(value) => {
            SpatialPlacementMotionError::AnchorTagFailure(value)
        }
        MotionAnchorDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedOffsetAnchor,
    }
}

fn map_rotate_anchor_failure(denial: MotionAnchorDenial) -> SpatialPlacementMotionError {
    match denial {
        MotionAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        MotionAnchorDenial::TagFailure(value) => {
            SpatialPlacementMotionError::AnchorTagFailure(value)
        }
        MotionAnchorDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedRotateAnchor,
    }
}

fn map_reorient_anchor_failure(denial: MotionAnchorDenial) -> SpatialPlacementMotionError {
    match denial {
        MotionAnchorDenial::Ambiguous => {
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        }
        MotionAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        MotionAnchorDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedReorientAnchor,
    }
}

fn translate_anchor_to_world_point(
    placement: SpatialPlacementSpec,
    anchor_world_point: [f64; 3],
    target_world_point: [f64; 3],
) -> Option<SpatialPlacementSpec> {
    translate_placement_world_offset(
        placement,
        [
            target_world_point[0] - anchor_world_point[0],
            target_world_point[1] - anchor_world_point[1],
            target_world_point[2] - anchor_world_point[2],
        ],
    )
}

fn translate_placement_world_offset(
    placement: SpatialPlacementSpec,
    offset: [f64; 3],
) -> Option<SpatialPlacementSpec> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone()).ok()?;
    let origin_world_point = reference_frame.basis().embed_point(placement.origin());
    Some(placement.at(reference_frame.basis().project_point([
        origin_world_point[0] + offset[0],
        origin_world_point[1] + offset[1],
        origin_world_point[2] + offset[2],
    ])))
}

fn rotate_origin_and_facing(
    placement: SpatialPlacementSpec,
    source_facing: [f64; 3],
    pivot_world_point: [f64; 3],
    axis: [f64; 3],
    angle_radians: f64,
) -> Option<SpatialPlacementSpec> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone()).ok()?;
    let origin_world_point = reference_frame.basis().embed_point(placement.origin());
    let rotated_origin_world_point =
        rotate_point_about_pivot(origin_world_point, pivot_world_point, axis, angle_radians);
    let rotated_facing = rotate_vector(source_facing, axis, angle_radians);
    Some(
        placement
            .at(reference_frame
                .basis()
                .project_point(rotated_origin_world_point))
            .facing(rotated_facing),
    )
}

fn rotate_point_about_pivot(
    point: [f64; 3],
    pivot: [f64; 3],
    axis: [f64; 3],
    angle_radians: f64,
) -> [f64; 3] {
    let rotated_offset = rotate_vector(
        [
            point[0] - pivot[0],
            point[1] - pivot[1],
            point[2] - pivot[2],
        ],
        axis,
        angle_radians,
    );
    [
        pivot[0] + rotated_offset[0],
        pivot[1] + rotated_offset[1],
        pivot[2] + rotated_offset[2],
    ]
}

fn rotate_vector(vector: [f64; 3], axis: [f64; 3], angle_radians: f64) -> [f64; 3] {
    let cos_theta = angle_radians.cos();
    let sin_theta = angle_radians.sin();
    let dot = axis[0] * vector[0] + axis[1] * vector[1] + axis[2] * vector[2];
    let cross = [
        axis[1] * vector[2] - axis[2] * vector[1],
        axis[2] * vector[0] - axis[0] * vector[2],
        axis[0] * vector[1] - axis[1] * vector[0],
    ];
    [
        vector[0] * cos_theta + cross[0] * sin_theta + axis[0] * dot * (1.0 - cos_theta),
        vector[1] * cos_theta + cross[1] * sin_theta + axis[1] * dot * (1.0 - cos_theta),
        vector[2] * cos_theta + cross[2] * sin_theta + axis[2] * dot * (1.0 - cos_theta),
    ]
}

fn rotate_vector_to_align_source(
    vector: [f64; 3],
    source: [f64; 3],
    target: [f64; 3],
) -> Option<[f64; 3]> {
    let source = UnitVector3::try_new(source).ok()?;
    let target = UnitVector3::try_new(target).ok()?;
    let dot = dot(source.as_array(), target.as_array()).clamp(-1.0, 1.0);
    if dot >= 1.0 - 1.0e-12 {
        return UnitVector3::try_new(vector).ok().map(UnitVector3::as_array);
    }
    let cross = cross(source.as_array(), target.as_array());
    let axis = if norm_sq(cross) <= f64::MIN_POSITIVE {
        canonical_perpendicular_unit_vector(source).as_array()
    } else {
        UnitVector3::try_new(cross).ok()?.as_array()
    };
    let angle = if norm_sq(cross) <= f64::MIN_POSITIVE {
        std::f64::consts::PI
    } else {
        dot.acos()
    };
    UnitVector3::try_new(rotate_vector(vector, axis, angle))
        .ok()
        .map(UnitVector3::as_array)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn norm_sq(vector: [f64; 3]) -> f64 {
    vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]
}
