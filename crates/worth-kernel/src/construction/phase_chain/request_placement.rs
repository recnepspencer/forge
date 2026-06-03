use worth_spatial::facade::placement::SpatialPlacementSpec;

use super::PrimitiveConstructionGeometry;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionPlacement {
    spec: SpatialPlacementSpec,
}

impl PrimitiveConstructionPlacement {
    pub(crate) fn world() -> Self {
        Self::from_spec(SpatialPlacementSpec::world())
    }

    pub(crate) fn from_spec(spec: SpatialPlacementSpec) -> Self {
        Self { spec }
    }

    pub(crate) fn decode(self) -> SpatialPlacementSpec {
        self.spec
    }
}

pub(crate) fn placement_of(
    geometry: &PrimitiveConstructionGeometry,
) -> PrimitiveConstructionPlacement {
    match geometry {
        PrimitiveConstructionGeometry::SimplexSolid { placement, .. }
        | PrimitiveConstructionGeometry::Orthotope { placement, .. }
        | PrimitiveConstructionGeometry::RegularPrism { placement, .. }
        | PrimitiveConstructionGeometry::RegularPyramid { placement, .. }
        | PrimitiveConstructionGeometry::WireBody { placement, .. }
        | PrimitiveConstructionGeometry::ShellWithHole { placement, .. } => placement.clone(),
    }
}

pub(crate) fn map_geometry_placement(
    geometry: PrimitiveConstructionGeometry,
    placement: PrimitiveConstructionPlacement,
) -> PrimitiveConstructionGeometry {
    match geometry {
        PrimitiveConstructionGeometry::SimplexSolid {
            scale,
            auxiliary_altitude_component,
            ..
        } => PrimitiveConstructionGeometry::SimplexSolid {
            placement,
            scale,
            auxiliary_altitude_component,
        },
        PrimitiveConstructionGeometry::Orthotope { half_extents, .. } => {
            PrimitiveConstructionGeometry::Orthotope {
                placement,
                half_extents,
            }
        }
        PrimitiveConstructionGeometry::RegularPrism {
            sides,
            radius,
            height,
            ..
        } => PrimitiveConstructionGeometry::RegularPrism {
            placement,
            sides,
            radius,
            height,
        },
        PrimitiveConstructionGeometry::RegularPyramid {
            sides,
            radius,
            height,
            ..
        } => PrimitiveConstructionGeometry::RegularPyramid {
            placement,
            sides,
            radius,
            height,
        },
        PrimitiveConstructionGeometry::WireBody { edge_count, .. } => {
            PrimitiveConstructionGeometry::WireBody {
                placement,
                edge_count,
            }
        }
        PrimitiveConstructionGeometry::ShellWithHole {
            outer_loop_edge_count,
            hole_loop_edge_counts,
            ..
        } => PrimitiveConstructionGeometry::ShellWithHole {
            placement,
            outer_loop_edge_count,
            hole_loop_edge_counts,
        },
    }
}

pub(crate) fn request_digest_parts(geometry: &PrimitiveConstructionGeometry) -> Vec<String> {
    let family = geometry.family();
    let placement = placement_of(geometry).decode();
    let mut parts = vec![
        family.as_str().to_string(),
        format!("{:?}", placement.origin().map(f64::to_bits)),
        format!("{:?}", placement.direction_witness()),
        format!("{:?}", placement.reference_frame()),
    ];
    match geometry {
        PrimitiveConstructionGeometry::SimplexSolid {
            scale,
            auxiliary_altitude_component,
            ..
        } => {
            parts.push(scale.to_string());
            parts.push(auxiliary_altitude_component.to_string());
        }
        PrimitiveConstructionGeometry::Orthotope { half_extents, .. } => {
            parts.push(format!("{:?}", half_extents));
        }
        PrimitiveConstructionGeometry::RegularPrism {
            sides,
            radius,
            height,
            ..
        }
        | PrimitiveConstructionGeometry::RegularPyramid {
            sides,
            radius,
            height,
            ..
        } => {
            parts.push(sides.to_string());
            parts.push(radius.to_string());
            parts.push(height.to_string());
        }
        PrimitiveConstructionGeometry::WireBody { edge_count, .. } => {
            parts.push(edge_count.to_string());
        }
        PrimitiveConstructionGeometry::ShellWithHole {
            outer_loop_edge_count,
            hole_loop_edge_counts,
            ..
        } => {
            parts.push(outer_loop_edge_count.to_string());
            parts.push(format!("{:?}", hole_loop_edge_counts));
        }
    }
    parts
}
