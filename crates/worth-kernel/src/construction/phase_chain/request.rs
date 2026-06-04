use crate::construction::digest::digest_owned_parts;
use crate::construction::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};
mod request_placement;
pub(crate) use request_placement::placement_of;
pub(crate) use request_placement::PrimitiveConstructionPlacement;
use request_placement::{map_geometry_placement, request_digest_parts};
use topology::facade::TopologyConstructionQueryAdmittedHandoffError;
use worth_geom::facade::{PrimitiveRealizationError, PrimitiveRealizationExhaustionReport};
use worth_spatial::facade::placement::SpatialPlacementSpec;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveConstructionFamily {
    SimplexSolid,
    Orthotope,
    RegularPrism,
    RegularPyramid,
    WireBody,
    ShellWithHole,
}

impl PrimitiveConstructionFamily {
    pub const ALL: [Self; 6] = [
        Self::SimplexSolid,
        Self::Orthotope,
        Self::RegularPrism,
        Self::RegularPyramid,
        Self::WireBody,
        Self::ShellWithHole,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SimplexSolid => "simplex_solid",
            Self::Orthotope => "orthotope",
            Self::RegularPrism => "regular_prism",
            Self::RegularPyramid => "regular_pyramid",
            Self::WireBody => "wire_body",
            Self::ShellWithHole => "shell_with_hole",
        }
    }

    pub fn topology_birth_class(self) -> &'static str {
        match self {
            Self::SimplexSolid => "closed_simplex_body",
            Self::Orthotope => "closed_orthotope_body",
            Self::RegularPrism => "closed_regular_prism_body",
            Self::RegularPyramid => "closed_regular_pyramid_body",
            Self::WireBody => "planar_wire_body",
            Self::ShellWithHole => "planar_shell_with_hole_body",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PrimitiveConstructionGeometry {
    SimplexSolid {
        placement: PrimitiveConstructionPlacement,
        scale: u64,
        auxiliary_altitude_component: u64,
    },
    Orthotope {
        placement: PrimitiveConstructionPlacement,
        half_extents: [u64; 3],
    },
    RegularPrism {
        placement: PrimitiveConstructionPlacement,
        sides: u32,
        radius: u64,
        height: u64,
    },
    RegularPyramid {
        placement: PrimitiveConstructionPlacement,
        sides: u32,
        radius: u64,
        height: u64,
    },
    WireBody {
        placement: PrimitiveConstructionPlacement,
        edge_count: u32,
    },
    ShellWithHole {
        placement: PrimitiveConstructionPlacement,
        outer_loop_edge_count: u32,
        hole_loop_edge_counts: Vec<u32>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionRequest {
    geometry: PrimitiveConstructionGeometry,
    request_digest: String,
}

impl PrimitiveConstructionRequest {
    pub fn simplex_solid(center: [f64; 3], scale: f64) -> Self {
        Self::simplex_solid_spec(SimplexSolidSpec::new(scale)).with_origin(center)
    }

    pub fn simplex_solid_spec(spec: SimplexSolidSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::SimplexSolid {
            placement: PrimitiveConstructionPlacement::world(),
            scale: spec.scale.to_bits(),
            auxiliary_altitude_component: spec.auxiliary_altitude_component.to_bits(),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn orthotope(center: [f64; 3], half_extents: [f64; 3]) -> Self {
        Self::orthotope_spec(OrthotopeSpec { half_extents }).with_origin(center)
    }

    pub fn orthotope_spec(spec: OrthotopeSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::Orthotope {
            placement: PrimitiveConstructionPlacement::world(),
            half_extents: spec.half_extents.map(f64::to_bits),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn regular_prism(center: [f64; 3], sides: u32, radius: f64, height: f64) -> Self {
        Self::regular_prism_spec(RegularPrismSpec {
            sides,
            radius,
            height,
        })
        .with_origin(center)
    }

    pub fn regular_prism_spec(spec: RegularPrismSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::RegularPrism {
            placement: PrimitiveConstructionPlacement::world(),
            sides: spec.sides,
            radius: spec.radius.to_bits(),
            height: spec.height.to_bits(),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn regular_pyramid(center: [f64; 3], sides: u32, radius: f64, height: f64) -> Self {
        Self::regular_pyramid_spec(RegularPyramidSpec {
            sides,
            radius,
            height,
        })
        .with_origin(center)
    }

    pub fn regular_pyramid_spec(spec: RegularPyramidSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::RegularPyramid {
            placement: PrimitiveConstructionPlacement::world(),
            sides: spec.sides,
            radius: spec.radius.to_bits(),
            height: spec.height.to_bits(),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn wire_body(edge_count: u32) -> Self {
        Self::wire_body_spec(WireBodySpec { edge_count })
    }

    pub fn wire_body_spec(spec: WireBodySpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::WireBody {
            placement: PrimitiveConstructionPlacement::world(),
            edge_count: spec.edge_count,
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn shell_with_hole(outer_loop_edge_count: u32, hole_loop_edge_counts: Vec<u32>) -> Self {
        Self::shell_with_hole_spec(ShellWithHoleSpec {
            outer_loop_edge_count,
            hole_loop_edge_counts,
        })
    }

    pub fn shell_with_hole_spec(spec: ShellWithHoleSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::ShellWithHole {
            placement: PrimitiveConstructionPlacement::world(),
            outer_loop_edge_count: spec.outer_loop_edge_count,
            hole_loop_edge_counts: spec.hole_loop_edge_counts.clone(),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    fn new(geometry: PrimitiveConstructionGeometry, parts: Vec<String>) -> Self {
        Self {
            geometry,
            request_digest: digest_owned_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.geometry.family()
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn placement_spec(&self) -> SpatialPlacementSpec {
        placement_of(&self.geometry).decode()
    }

    pub fn with_origin(self, origin: [f64; 3]) -> Self {
        let placement = self.placement_spec().at(origin);
        self.with_placement_spec(placement)
    }

    pub fn with_facing(self, facing: [f64; 3]) -> Self {
        let placement = self.placement_spec().facing(facing);
        self.with_placement_spec(placement)
    }
    pub(crate) fn with_placement_spec(self, placement: SpatialPlacementSpec) -> Self {
        let geometry = map_geometry_placement(
            self.geometry,
            PrimitiveConstructionPlacement::from_spec(placement),
        );
        let parts = request_digest_parts(&geometry);
        Self::new(geometry, parts)
    }

    pub(crate) fn geometry(&self) -> &PrimitiveConstructionGeometry {
        &self.geometry
    }
}

impl PrimitiveConstructionGeometry {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        match self {
            Self::SimplexSolid { .. } => PrimitiveConstructionFamily::SimplexSolid,
            Self::Orthotope { .. } => PrimitiveConstructionFamily::Orthotope,
            Self::RegularPrism { .. } => PrimitiveConstructionFamily::RegularPrism,
            Self::RegularPyramid { .. } => PrimitiveConstructionFamily::RegularPyramid,
            Self::WireBody { .. } => PrimitiveConstructionFamily::WireBody,
            Self::ShellWithHole { .. } => PrimitiveConstructionFamily::ShellWithHole,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionGeometryError {
    RealizationExhausted(PrimitiveRealizationExhaustionReport),
    GeometryFailure(String),
}

impl PrimitiveConstructionGeometryError {
    pub fn from_realization_error(error: PrimitiveRealizationError) -> Self {
        match error {
            PrimitiveRealizationError::Exhausted(report) => Self::RealizationExhausted(report),
            PrimitiveRealizationError::Geometry(error) => Self::GeometryFailure(error.to_string()),
        }
    }
}

impl std::fmt::Display for PrimitiveConstructionGeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RealizationExhausted(report) => write!(f, "{report}"),
            Self::GeometryFailure(reason) => write!(f, "{reason}"),
        }
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPhaseError {
    InvalidRequest {
        family: PrimitiveConstructionFamily,
        reason: &'static str,
    },
    Geometry(PrimitiveConstructionGeometryError),
    TopologyQueryAdmittedHandoff(TopologyConstructionQueryAdmittedHandoffError),
}

impl std::fmt::Display for PrimitiveConstructionPhaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest { family, reason } => {
                write!(f, "invalid {} request: {reason}", family.as_str())
            }
            Self::Geometry(error) => write!(f, "geometry scaffold failed: {error}"),
            Self::TopologyQueryAdmittedHandoff(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPhaseError {}
