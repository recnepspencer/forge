use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{S8AccessShapeDetail, S8MaintenanceReadBasis};
use super::lane::S8AccessLaneClassification;
use super::shape::S8AccessShape;
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) fn quarantine_read(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    if lane != S8AccessLaneClassification::Verifier {
        return Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::QuarantineRead,
            lane,
        });
    }

    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::QuarantineRead(S8MaintenanceReadBasis::QuarantineTraversal),
        lane,
        S8ExpectedCounterClass::QuarantineTraversal,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}
