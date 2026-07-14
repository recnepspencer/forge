use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, MaintenanceReadBasis};
use super::kind::AccessShape;
use super::lane::AccessLaneClassification;

pub(crate) fn rebuild_read_declaration(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if lane != AccessLaneClassification::Maintenance {
        return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::RebuildRead,
            lane,
        });
    }
    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::RebuildRead(MaintenanceReadBasis::Rebuild),
        lane,
        ExpectedCounterClass::RebuildTraversal,
    ))
}

#[cfg(test)]
pub(crate) fn rebuild_read(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    rebuild_read_declaration(lane)
}
