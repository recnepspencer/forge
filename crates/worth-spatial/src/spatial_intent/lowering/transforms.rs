use crate::spatial_intent::lowering::{admit_spatial_placement, SpatialPlacementSpec};
use crate::spatial_intent::resolution::admit_spatial_frame;
use worth_math::{canonical_perpendicular_unit_vector, UnitVector3};

pub(crate) fn translate_anchor_to_world_point(
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

pub(crate) fn translate_placement_world_offset(
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

pub(crate) fn rotate_point_about_pivot(
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

pub(crate) fn rotate_vector(vector: [f64; 3], axis: [f64; 3], angle_radians: f64) -> [f64; 3] {
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

pub(crate) fn rotate_facing_to_align_source(
    placement: &SpatialPlacementSpec,
    source_world_direction: [f64; 3],
    target_world_direction: [f64; 3],
) -> Option<[f64; 3]> {
    let admitted = admit_spatial_placement(placement.clone()).ok()?;
    rotate_vector_to_align_source(
        admitted.facing_vector(),
        source_world_direction,
        target_world_direction,
    )
}

pub(crate) fn rotate_vector_to_align_source(
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

pub(crate) fn project_subject_anchor_onto_frame_plane(
    placement: SpatialPlacementSpec,
    target_frame: &crate::spatial_intent::resolution::AdmittedSpatialFrameRef,
    anchor_world_point: [f64; 3],
) -> Option<SpatialPlacementSpec> {
    let current_reference_frame = admit_spatial_frame(placement.reference_frame().clone()).ok()?;
    let current_origin_world_point = current_reference_frame
        .basis()
        .embed_point(placement.origin());
    let target_anchor_local = target_frame.basis().project_point(anchor_world_point);
    let projected_anchor_world_point =
        target_frame
            .basis()
            .embed_point([target_anchor_local[0], target_anchor_local[1], 0.0]);
    let translated_origin_world_point = [
        current_origin_world_point[0] + projected_anchor_world_point[0] - anchor_world_point[0],
        current_origin_world_point[1] + projected_anchor_world_point[1] - anchor_world_point[1],
        current_origin_world_point[2] + projected_anchor_world_point[2] - anchor_world_point[2],
    ];
    Some(
        placement
            .relative_to(target_frame.spec().clone())
            .at(target_frame
                .basis()
                .project_point(translated_origin_world_point)),
    )
}

pub(crate) fn rotate_origin_and_facing(
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
