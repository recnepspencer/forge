use crate::construction::request::{
    PrimitiveConstructionGeometryError, PrimitiveConstructionPhaseError,
};
use worth_geom::facade::PrimitiveRealizationError;
use worth_spatial::facade::placement::SpatialPlacementError;

pub(super) fn map_geometry(error: impl ToString) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(PrimitiveConstructionGeometryError::GeometryFailure(
        error.to_string(),
    ))
}

pub(super) fn map_realization_geometry(
    error: PrimitiveRealizationError,
) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(
        PrimitiveConstructionGeometryError::from_realization_error(error),
    )
}

pub(super) fn map_support_plane(error: String) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(PrimitiveConstructionGeometryError::GeometryFailure(
        error,
    ))
}

pub(super) fn map_placement_geometry(
    error: SpatialPlacementError,
) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(PrimitiveConstructionGeometryError::GeometryFailure(
        error.to_string(),
    ))
}
