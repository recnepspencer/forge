use super::contract::{S8AccessShapeContract, S8ExpectedCounterClass};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{
    S8AccessShapeDetail, S8ChunkTreeWalkBasis, S8CoalescedPageReadBasis,
    S8StreamingContinuationBasis, S8StreamingReadBasis,
};
use super::lane::S8AccessLaneClassification;
use super::shape::S8AccessShape;
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) fn coalesced_page_read(
    coverage: S8LayoutCoverageWitness,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::CoalescedPageRead(S8CoalescedPageReadBasis::AdjacentPageWindow),
        S8AccessLaneClassification::Foreground,
        S8ExpectedCounterClass::CoalescedPageRead,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn chunk_tree_walk(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    match lane {
        S8AccessLaneClassification::Foreground | S8AccessLaneClassification::Maintenance => {}
        _ => {
            return Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
                shape: S8AccessShape::ChunkTreeWalk,
                lane,
            });
        }
    }

    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::ChunkTreeWalk(S8ChunkTreeWalkBasis::RootedChunkTraversal),
        lane,
        S8ExpectedCounterClass::ChunkTreeWalk,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn streaming_read(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    match lane {
        S8AccessLaneClassification::Foreground | S8AccessLaneClassification::Maintenance => {}
        _ => {
            return Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
                shape: S8AccessShape::StreamingRead,
                lane,
            });
        }
    }

    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::StreamingRead(S8StreamingReadBasis::SequentialStreamTraversal),
        lane,
        S8ExpectedCounterClass::StreamingRead,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}

pub(crate) fn streaming_continuation_read(
    coverage: S8LayoutCoverageWitness,
    lane: S8AccessLaneClassification,
    basis: S8StreamingContinuationBasis,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    match lane {
        S8AccessLaneClassification::Foreground | S8AccessLaneClassification::Maintenance => {}
        _ => {
            return Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
                shape: S8AccessShape::StreamingContinuationRead,
                lane,
            });
        }
    }

    Ok(S8AccessShapeContract::exact_read(
        S8AccessShapeDetail::StreamingContinuationRead(basis),
        lane,
        S8ExpectedCounterClass::StreamingContinuationRead,
        coverage
            .require_exact()
            .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
    ))
}
