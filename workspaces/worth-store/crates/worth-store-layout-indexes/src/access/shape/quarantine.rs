use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, MaintenanceReadBasis};
use super::kind::AccessShape;
use super::lane::AccessLaneClassification;

#[cfg(test)]
pub(crate) fn quarantine_read(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if lane != AccessLaneClassification::Verifier {
        return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::QuarantineRead,
            lane,
        });
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::QuarantineRead(MaintenanceReadBasis::Quarantine),
        lane,
        ExpectedCounterClass::QuarantineTraversal,
    ))
}
