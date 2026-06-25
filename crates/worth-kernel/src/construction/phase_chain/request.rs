pub use super::family::{
    primitive_construction_topology_birth_class, PrimitiveConstructionFamily,
    PRIMITIVE_CONSTRUCTION_FAMILIES,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};
mod request_invalidity;
mod request_placement;
#[cfg(test)]
pub(crate) use request_invalidity::primitive_construction_invalid_request_reason;
pub(crate) use request_placement::placement_of;
pub(crate) use request_placement::PrimitiveConstructionPlacement;
use request_placement::{map_geometry_placement, request_digest_parts};
use topology::facade::{
    TopologyConstructionQueryAdmittedHandoffError,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationError, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_spatial::facade::placement::SpatialPlacementSpec;

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
    pub fn simplex_solid_spec(spec: SimplexSolidSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::SimplexSolid {
            placement: PrimitiveConstructionPlacement::world(),
            scale: spec.scale.to_bits(),
            auxiliary_altitude_component: spec.auxiliary_altitude_component.to_bits(),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn orthotope_spec(spec: OrthotopeSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::Orthotope {
            placement: PrimitiveConstructionPlacement::world(),
            half_extents: spec.half_extents.map(f64::to_bits),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
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

    pub fn regular_pyramid_spec(spec: RegularPyramidSpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::RegularPyramid {
            placement: PrimitiveConstructionPlacement::world(),
            sides: spec.sides,
            radius: spec.radius.to_bits(),
            height: spec.height.to_bits(),
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
    }

    pub fn wire_body_spec(spec: WireBodySpec) -> Self {
        let geometry = PrimitiveConstructionGeometry::WireBody {
            placement: PrimitiveConstructionPlacement::world(),
            edge_count: spec.edge_count,
        };
        Self::new(geometry.clone(), request_digest_parts(&geometry))
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
pub struct PrimitiveConstructionRealizationExhaustionFact {
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: PrimitiveConditioningWitness,
    stability_class: PrimitiveStabilityClass,
    exhaustion_reason: PrimitiveRealizationExhaustionReason,
    fact_digest: String,
}

impl PrimitiveConstructionRealizationExhaustionFact {
    fn from_realization_exhaustion_report(report: PrimitiveRealizationExhaustionReport) -> Self {
        Self {
            attempted_strategies: report.attempted_strategies().to_vec(),
            conditioning_witness: report.conditioning_witness().clone(),
            stability_class: report.stability_class(),
            exhaustion_reason: report.exhaustion_reason(),
            fact_digest: report.report_digest().to_string(),
        }
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn exhaustion_reason(&self) -> PrimitiveRealizationExhaustionReason {
        self.exhaustion_reason
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }
}

impl std::fmt::Display for PrimitiveConstructionRealizationExhaustionFact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "realization exhausted after {}: {}",
            self.attempted_strategies
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join(" -> "),
            self.exhaustion_reason.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionGeometryError {
    RealizationExhausted(PrimitiveConstructionRealizationExhaustionFact),
    GeometryFailure(String),
}

impl PrimitiveConstructionGeometryError {
    pub fn from_realization_error(error: PrimitiveRealizationError) -> Self {
        match error {
            PrimitiveRealizationError::Exhausted(report) => Self::RealizationExhausted(
                PrimitiveConstructionRealizationExhaustionFact::from_realization_exhaustion_report(
                    report,
                ),
            ),
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
    TopologyBirthCompose(TopologyPrimitiveConstructionBirthComposeExecutionError),
}

impl std::fmt::Display for PrimitiveConstructionPhaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest { family, reason } => {
                write!(f, "invalid {} request: {reason}", family.as_str())
            }
            Self::Geometry(error) => write!(f, "geometry scaffold failed: {error}"),
            Self::TopologyQueryAdmittedHandoff(error) => write!(f, "{error}"),
            Self::TopologyBirthCompose(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPhaseError {}
