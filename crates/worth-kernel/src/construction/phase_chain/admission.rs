use crate::construction::digest::digest_owned_parts;
use crate::construction::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometry, PrimitiveConstructionPhaseError,
};
use crate::construction::scaffold::PrimitiveConstructionScaffold;
use crate::construction::scaffold_realization::build_admitted_scaffold;
use worth_spatial::facade::{
    admit_spatial_placement, AdmittedSpatialPlacement, SpatialPlacementError,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AdmittedPrimitiveConstructionGeometry {
    SimplexSolid {
        placement: AdmittedSpatialPlacement,
        scale: f64,
    },
    Orthotope {
        placement: AdmittedSpatialPlacement,
        half_extents: [f64; 3],
    },
    RegularPrism {
        placement: AdmittedSpatialPlacement,
        sides: u32,
        radius: f64,
        height: f64,
    },
    RegularPyramid {
        placement: AdmittedSpatialPlacement,
        sides: u32,
        radius: f64,
        height: f64,
    },
    WireBody {
        placement: AdmittedSpatialPlacement,
        edge_count: u32,
    },
    ShellWithHole {
        placement: AdmittedSpatialPlacement,
        outer_loop_edge_count: u32,
        hole_loop_edge_counts: Vec<u32>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedPrimitiveConstructionIntent {
    family: PrimitiveConstructionFamily,
    geometry: AdmittedPrimitiveConstructionGeometry,
    request_digest: String,
    intent_digest: String,
}

impl AdmittedPrimitiveConstructionIntent {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn intent_digest(&self) -> &str {
        &self.intent_digest
    }

    pub fn build_scaffold(
        &self,
    ) -> Result<PrimitiveConstructionScaffold, PrimitiveConstructionPhaseError> {
        build_admitted_scaffold(
            self.family,
            &self.request_digest,
            &self.intent_digest,
            &self.geometry,
        )
    }
}

pub(crate) fn admit_request(
    family: PrimitiveConstructionFamily,
    geometry: PrimitiveConstructionGeometry,
    request_digest: String,
) -> Result<AdmittedPrimitiveConstructionIntent, PrimitiveConstructionPhaseError> {
    let geometry = match geometry {
        PrimitiveConstructionGeometry::SimplexSolid { placement, scale } => {
            let placement = admit_placement(family, placement)?;
            let scale = f64::from_bits(scale);
            reject_non_positive_scalar(family, "scale", scale)?;
            AdmittedPrimitiveConstructionGeometry::SimplexSolid { placement, scale }
        }
        PrimitiveConstructionGeometry::Orthotope {
            placement,
            half_extents,
        } => {
            let placement = admit_placement(family, placement)?;
            let half_extents = decode_triplet(half_extents);
            if half_extents
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
            {
                return Err(PrimitiveConstructionPhaseError::InvalidRequest {
                    family,
                    reason: "orthotope half-extents must stay finite and positive",
                });
            }
            AdmittedPrimitiveConstructionGeometry::Orthotope {
                placement,
                half_extents,
            }
        }
        PrimitiveConstructionGeometry::RegularPrism {
            placement,
            sides,
            radius,
            height,
        } => {
            let placement = admit_placement(family, placement)?;
            reject_minimum_sides(family, sides)?;
            let radius = f64::from_bits(radius);
            let height = f64::from_bits(height);
            reject_non_positive_scalar(family, "radius", radius)?;
            reject_non_positive_scalar(family, "height", height)?;
            AdmittedPrimitiveConstructionGeometry::RegularPrism {
                placement,
                sides,
                radius,
                height,
            }
        }
        PrimitiveConstructionGeometry::RegularPyramid {
            placement,
            sides,
            radius,
            height,
        } => {
            let placement = admit_placement(family, placement)?;
            reject_minimum_sides(family, sides)?;
            let radius = f64::from_bits(radius);
            let height = f64::from_bits(height);
            reject_non_positive_scalar(family, "radius", radius)?;
            reject_non_positive_scalar(family, "height", height)?;
            AdmittedPrimitiveConstructionGeometry::RegularPyramid {
                placement,
                sides,
                radius,
                height,
            }
        }
        PrimitiveConstructionGeometry::WireBody {
            placement,
            edge_count,
        } => {
            let placement = admit_placement(family, placement)?;
            reject_minimum_sides(family, edge_count)?;
            AdmittedPrimitiveConstructionGeometry::WireBody {
                placement,
                edge_count,
            }
        }
        PrimitiveConstructionGeometry::ShellWithHole {
            placement,
            outer_loop_edge_count,
            hole_loop_edge_counts,
        } => {
            let placement = admit_placement(family, placement)?;
            reject_minimum_sides(family, outer_loop_edge_count)?;
            if hole_loop_edge_counts.is_empty() {
                return Err(PrimitiveConstructionPhaseError::InvalidRequest {
                    family,
                    reason: "shell-with-hole requires at least one inner hole loop",
                });
            }
            for hole_edge_count in hole_loop_edge_counts.iter().copied() {
                reject_minimum_sides(family, hole_edge_count)?;
            }
            AdmittedPrimitiveConstructionGeometry::ShellWithHole {
                placement,
                outer_loop_edge_count,
                hole_loop_edge_counts,
            }
        }
    };
    Ok(AdmittedPrimitiveConstructionIntent {
        family,
        geometry,
        request_digest: request_digest.clone(),
        intent_digest: digest_owned_parts(&[
            request_digest,
            family.as_str().to_string(),
            "admitted".to_string(),
        ]),
    })
}

fn reject_non_positive_scalar(
    family: PrimitiveConstructionFamily,
    name: &'static str,
    value: f64,
) -> Result<(), PrimitiveConstructionPhaseError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest {
            family,
            reason: match name {
                "scale" => "scale must stay finite and positive",
                "radius" => "radius must stay finite and positive",
                "height" => "height must stay finite and positive",
                _ => "scalar parameter must stay finite and positive",
            },
        });
    }
    Ok(())
}

fn reject_minimum_sides(
    family: PrimitiveConstructionFamily,
    sides: u32,
) -> Result<(), PrimitiveConstructionPhaseError> {
    if sides < 3 {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest {
            family,
            reason: "polygonal construction families require at least three edges",
        });
    }
    Ok(())
}

fn decode_triplet(bits: [u64; 3]) -> [f64; 3] {
    bits.map(f64::from_bits)
}

fn admit_placement(
    family: PrimitiveConstructionFamily,
    placement: crate::construction::request::PrimitiveConstructionPlacement,
) -> Result<AdmittedSpatialPlacement, PrimitiveConstructionPhaseError> {
    admit_spatial_placement(placement.decode()).map_err(|error| {
        PrimitiveConstructionPhaseError::InvalidRequest {
            family,
            reason: placement_error_reason(error),
        }
    })
}

fn placement_error_reason(error: SpatialPlacementError) -> &'static str {
    match error {
        SpatialPlacementError::NonFiniteOrigin => "placement origin must stay finite",
        SpatialPlacementError::DirectionWitnessFailure(class) => match class {
            worth_spatial::facade::SpatialWitnessFailureClass::NonFinite => {
                "placement direction witness must stay finite"
            }
            worth_spatial::facade::SpatialWitnessFailureClass::Ambiguous => {
                "placement direction witness must not be ambiguous"
            }
            worth_spatial::facade::SpatialWitnessFailureClass::Undefined => {
                "placement direction witness must not collapse to zero"
            }
            worth_spatial::facade::SpatialWitnessFailureClass::Unsupported => {
                "placement direction witness role is not supported yet"
            }
            worth_spatial::facade::SpatialWitnessFailureClass::Degenerate => {
                "placement direction witness must not derive from a degenerate frame"
            }
            worth_spatial::facade::SpatialWitnessFailureClass::Coincident => {
                "placement direction witness must not be coincident with its target"
            }
            worth_spatial::facade::SpatialWitnessFailureClass::Exhausted => {
                "placement direction witness exhausted sanctioned resolution strategies"
            }
        },
        SpatialPlacementError::InvalidReferenceFrame(_) => {
            "placement reference frame must stay finite and non-degenerate"
        }
        SpatialPlacementError::InvalidEmbeddedPlane => {
            "placement embedding must keep support planes valid"
        }
    }
}
