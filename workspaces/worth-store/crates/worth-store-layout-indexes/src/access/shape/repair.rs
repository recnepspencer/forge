use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, MaintenanceReadBasis};
use super::kind::AccessShape;
use super::lane::AccessLaneClassification;

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
        AccessShapeDetail::RepairRead(MaintenanceReadBasis::Repair),
        lane,
        ExpectedCounterClass::RepairTraversal,
    ))
}
