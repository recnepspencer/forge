use crate::BlobStreamingReadObservation;

pub(crate) enum ObservationKind<'a> {
    Chunk(&'a crate::BlobStreamingReadObservedChunk),
    ColdUnavailable { ordinal: crate::BlobChunkOrdinal },
}

pub(crate) fn classify(observation: &BlobStreamingReadObservation) -> ObservationKind<'_> {
    match observation {
        BlobStreamingReadObservation::Chunk(chunk) => ObservationKind::Chunk(chunk),
        BlobStreamingReadObservation::ColdUnavailable { ordinal, .. } => {
            ObservationKind::ColdUnavailable { ordinal: *ordinal }
        }
    }
}

pub(crate) fn observation_ordinal(
    observation: &BlobStreamingReadObservation,
) -> Option<crate::BlobChunkOrdinal> {
    match observation {
        BlobStreamingReadObservation::Chunk(chunk) => Some(chunk.ordinal()),
        BlobStreamingReadObservation::ColdUnavailable { ordinal, .. } => Some(*ordinal),
    }
}
