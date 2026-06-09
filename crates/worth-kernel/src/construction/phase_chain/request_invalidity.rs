use super::{PrimitiveConstructionGeometry, PrimitiveConstructionRequest};

pub(crate) fn primitive_construction_invalid_request_reason(
    request: &PrimitiveConstructionRequest,
) -> Option<&'static str> {
    match request.geometry() {
        PrimitiveConstructionGeometry::SimplexSolid {
            scale,
            auxiliary_altitude_component,
            ..
        } => {
            if invalid_positive_scalar(*scale) {
                Some("scale must stay finite and positive")
            } else if invalid_non_negative_scalar(*auxiliary_altitude_component) {
                Some("auxiliary altitude component must stay finite and non-negative")
            } else {
                None
            }
        }
        PrimitiveConstructionGeometry::Orthotope { half_extents, .. } => {
            if half_extents
                .iter()
                .any(|value| invalid_positive_scalar(*value))
            {
                Some("orthotope half-extents must stay finite and positive")
            } else {
                None
            }
        }
        PrimitiveConstructionGeometry::RegularPrism {
            sides,
            radius,
            height,
            ..
        } => polygon_invalidity(*sides)
            .or_else(|| {
                invalid_positive_scalar(*radius).then_some("radius must stay finite and positive")
            })
            .or_else(|| {
                invalid_positive_scalar(*height).then_some("height must stay finite and positive")
            }),
        PrimitiveConstructionGeometry::RegularPyramid {
            sides,
            radius,
            height,
            ..
        } => polygon_invalidity(*sides)
            .or_else(|| {
                invalid_non_negative_scalar(*radius)
                    .then_some("radius must stay finite and non-negative")
            })
            .or_else(|| {
                invalid_positive_scalar(*height).then_some("height must stay finite and positive")
            }),
        PrimitiveConstructionGeometry::WireBody { edge_count, .. } => {
            polygon_invalidity(*edge_count)
        }
        PrimitiveConstructionGeometry::ShellWithHole {
            outer_loop_edge_count,
            hole_loop_edge_counts,
            ..
        } => {
            if hole_loop_edge_counts.is_empty() {
                Some("shell-with-hole requires at least one inner hole loop")
            } else {
                polygon_invalidity(*outer_loop_edge_count).or_else(|| {
                    hole_loop_edge_counts
                        .iter()
                        .any(|count| *count < 3)
                        .then_some("polygonal construction families require at least three edges")
                })
            }
        }
    }
}

fn polygon_invalidity(count: u32) -> Option<&'static str> {
    (count < 3).then_some("polygonal construction families require at least three edges")
}

fn invalid_positive_scalar(bits: u64) -> bool {
    let value = f64::from_bits(bits);
    !value.is_finite() || value <= 0.0
}

fn invalid_non_negative_scalar(bits: u64) -> bool {
    let value = f64::from_bits(bits);
    !value.is_finite() || value < 0.0
}
