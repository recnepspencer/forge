use crate::spatial_intent::lowering::SpatialPlacementSpec;
use crate::spatial_intent::refs::admit_spatial_frame;

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
    let translated_origin = reference_frame.basis().project_point([
        reference_frame.basis().embed_point(placement.origin())[0] + offset[0],
        reference_frame.basis().embed_point(placement.origin())[1] + offset[1],
        reference_frame.basis().embed_point(placement.origin())[2] + offset[2],
    ]);
    Some(placement.at(translated_origin))
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
