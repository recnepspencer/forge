use crate::construction::request::{
    placement_of, PrimitiveConstructionPhaseError, PrimitiveConstructionRequest,
};
use worth_spatial::facade::{
    admit_spatial_placement, AdmittedSpatialPlacement, SpatialPlacementError,
};

pub(super) fn admit_request_placement(
    request: &PrimitiveConstructionRequest,
) -> Result<AdmittedSpatialPlacement, PrimitiveConstructionPhaseError> {
    admit_spatial_placement(placement_of(request.geometry()).clone().decode()).map_err(|error| {
        PrimitiveConstructionPhaseError::InvalidRequest {
            family: request.family(),
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
