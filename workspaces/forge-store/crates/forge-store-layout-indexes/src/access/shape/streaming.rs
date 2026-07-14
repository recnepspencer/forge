use super::contract::{AccessShapeContract, ExpectedCounterClass};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{
    AccessShapeDetail, ChunkTreeWalkBasis, CoalescedPageReadBasis, StreamingContinuationBasis,
    StreamingReadBasis,
};
use super::kind::AccessShape;
use super::lane::AccessLaneClassification;

#[cfg(test)]
pub(crate) fn coalesced_page_read() -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::CoalescedPageRead(CoalescedPageReadBasis::AdjacentPageWindow),
        AccessLaneClassification::Foreground,
        ExpectedCounterClass::CoalescedPageRead,
    ))
}

#[cfg(test)]
pub(crate) fn chunk_tree_walk(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    match lane {
        AccessLaneClassification::Foreground | AccessLaneClassification::Maintenance => {}
        _ => {
            return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
                shape: AccessShape::ChunkTreeWalk,
                lane,
            });
        }
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::ChunkTreeWalk(ChunkTreeWalkBasis::RootedChunkTraversal),
        lane,
        ExpectedCounterClass::ChunkTreeWalk,
    ))
}

#[cfg(test)]
pub(crate) fn streaming_read(
    lane: AccessLaneClassification,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    match lane {
        AccessLaneClassification::Foreground | AccessLaneClassification::Maintenance => {}
        _ => {
            return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
                shape: AccessShape::StreamingRead,
                lane,
            });
        }
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::StreamingRead(StreamingReadBasis::SequentialStreamTraversal),
        lane,
        ExpectedCounterClass::StreamingRead,
    ))
}

#[cfg(test)]
pub(crate) fn streaming_continuation_read(
    lane: AccessLaneClassification,
    basis: StreamingContinuationBasis,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    match lane {
        AccessLaneClassification::Foreground | AccessLaneClassification::Maintenance => {}
        _ => {
            return Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
                shape: AccessShape::StreamingContinuationRead,
                lane,
            });
        }
    }

    Ok(AccessShapeContract::exact_read_declaration(
        AccessShapeDetail::StreamingContinuationRead(basis),
        lane,
        ExpectedCounterClass::StreamingContinuationRead,
    ))
}
