use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, MaintenanceReadBasis};
use super::lane::AccessLaneClassification;
use super::shape::AccessShape;

#[cfg(test)]
pub(crate) fn repair_read(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if lane != AccessLaneClassification::Maintenance {
        return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::RepairRead,
            lane,
        });
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::RepairRead(MaintenanceReadBasis::RepairTraversal),
        lane,
        ExpectedCounterClass::RepairTraversal,
    ))
}
