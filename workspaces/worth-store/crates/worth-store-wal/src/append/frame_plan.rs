use crate::{
    artifact_store::{WalArtifactStoreDenial, WalFrameAppendPlan},
    LogSequenceNumber, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};

/// Exact already-observed WAL tail supplied to the effect-free frame planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalAppendFrontier {
    segment: WalSegmentId,
    generation: WalSegmentGeneration,
    valid_prefix_bytes: u64,
    last_lsn_end: Option<LogSequenceNumber>,
}

/// Immutable frame bytes paired with the frontier they establish if appended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedWalFrameAppend {
    frame: WalFrameAppendPlan,
    resulting_frontier: WalAppendFrontier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalFramePlanningDenial {
    RangeNotContiguous,
    InvalidFrame,
    LengthOverflow,
}

impl WalAppendFrontier {
    pub const fn empty(segment: WalSegmentId, generation: WalSegmentGeneration) -> Self {
        Self {
            segment,
            generation,
            valid_prefix_bytes: 0,
            last_lsn_end: None,
        }
    }

    pub const fn observed(
        segment: WalSegmentId,
        generation: WalSegmentGeneration,
        valid_prefix_bytes: u64,
        last_lsn_end: LogSequenceNumber,
    ) -> Self {
        Self {
            segment,
            generation,
            valid_prefix_bytes,
            last_lsn_end: Some(last_lsn_end),
        }
    }

    pub const fn segment(self) -> WalSegmentId {
        self.segment
    }

    pub const fn generation(self) -> WalSegmentGeneration {
        self.generation
    }

    pub const fn valid_prefix_bytes(self) -> u64 {
        self.valid_prefix_bytes
    }

    pub const fn last_lsn_end(self) -> Option<LogSequenceNumber> {
        self.last_lsn_end
    }
}

impl PlannedWalFrameAppend {
    pub const fn frame(&self) -> &WalFrameAppendPlan {
        &self.frame
    }

    pub const fn resulting_frontier(&self) -> WalAppendFrontier {
        self.resulting_frontier
    }

    pub fn into_parts(self) -> (WalFrameAppendPlan, WalAppendFrontier) {
        (self.frame, self.resulting_frontier)
    }
}

/// Plans immutable WAL bytes without reading a path or executing an effect.
pub fn plan_wal_frame_append(
    frontier: WalAppendFrontier,
    range: WalLsnRange,
    declared_identity: &str,
    payload: &[u8],
) -> Result<PlannedWalFrameAppend, WalFramePlanningDenial> {
    if frontier
        .last_lsn_end
        .is_some_and(|last| last != range.start())
    {
        return Err(WalFramePlanningDenial::RangeNotContiguous);
    }
    let frame = crate::artifact_store::encode_wal_frame_at_frontier(
        frontier.segment.get(),
        frontier.generation.get(),
        range.start().get(),
        range.end_exclusive().get(),
        declared_identity,
        payload,
        frontier.valid_prefix_bytes,
        frontier.last_lsn_end.map(LogSequenceNumber::get),
    )
    .map_err(map_frame_denial)?;
    let encoded_bytes = u64::try_from(frame.encoded_frame().len())
        .map_err(|_| WalFramePlanningDenial::LengthOverflow)?;
    let next_bytes = frontier
        .valid_prefix_bytes
        .checked_add(encoded_bytes)
        .ok_or(WalFramePlanningDenial::LengthOverflow)?;
    Ok(PlannedWalFrameAppend {
        frame,
        resulting_frontier: WalAppendFrontier::observed(
            frontier.segment,
            frontier.generation,
            next_bytes,
            range.end_exclusive(),
        ),
    })
}

fn map_frame_denial(denial: WalArtifactStoreDenial) -> WalFramePlanningDenial {
    match denial {
        WalArtifactStoreDenial::NonContiguousLsn => WalFramePlanningDenial::RangeNotContiguous,
        _ => WalFramePlanningDenial::InvalidFrame,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planning_is_path_free_and_advances_only_the_returned_frontier() {
        let segment = WalSegmentId::new(7).unwrap();
        let generation = WalSegmentGeneration::new(3).unwrap();
        let initial = WalAppendFrontier::empty(segment, generation);
        let first_range =
            WalLsnRange::new(LogSequenceNumber::new(11), LogSequenceNumber::new(13)).unwrap();
        let first = plan_wal_frame_append(initial, first_range, "member-a", b"redo-a").unwrap();
        assert_eq!(initial.valid_prefix_bytes(), 0);
        assert_eq!(
            first.resulting_frontier().last_lsn_end(),
            Some(LogSequenceNumber::new(13))
        );
        assert_eq!(first.frame().prefix_bytes_scanned(), 0);

        let second_range =
            WalLsnRange::new(LogSequenceNumber::new(13), LogSequenceNumber::new(14)).unwrap();
        let second = plan_wal_frame_append(
            first.resulting_frontier(),
            second_range,
            "member-b",
            b"redo-b",
        )
        .unwrap();
        assert_eq!(
            second.frame().valid_prefix_bytes(),
            first.resulting_frontier().valid_prefix_bytes()
        );
        assert_eq!(second.frame().prefix_bytes_scanned(), 0);
    }

    #[test]
    fn a_gap_cannot_be_planned_from_an_observed_frontier() {
        let frontier = WalAppendFrontier::observed(
            WalSegmentId::new(1).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
            88,
            LogSequenceNumber::new(9),
        );
        let range =
            WalLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(11)).unwrap();
        assert_eq!(
            plan_wal_frame_append(frontier, range, "member", b"redo"),
            Err(WalFramePlanningDenial::RangeNotContiguous)
        );
    }
}
