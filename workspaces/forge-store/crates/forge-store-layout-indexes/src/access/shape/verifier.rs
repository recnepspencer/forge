use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, MaintenanceReadBasis};
use super::lane::AccessLaneClassification;
use super::shape::AccessShape;

#[cfg(test)]
pub(crate) fn verifier_read(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if lane != AccessLaneClassification::Verifier {
        return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::VerifierRead,
            lane,
        });
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::VerifierRead(MaintenanceReadBasis::VerifierTraversal),
        lane,
        ExpectedCounterClass::VerifierTraversal,
    ))
}
