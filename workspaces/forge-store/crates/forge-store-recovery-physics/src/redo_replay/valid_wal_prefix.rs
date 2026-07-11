use crate::{
    AdmittedRecoverySource, LogSequenceNumber, WalLsnRange,
    WalSegmentGeneration,
};

use super::{
    MiddleWalCorruptionDenial, MissingAcknowledgedWalRangeDenial, RedoPlanningDenial,
    RedoPlanningDenialKind, StaleWalGenerationDenial, TornWalTailClassification,
    WalPrefixFrameObservation, WalPrefixFramePosture, WalPrefixIntegrityObservation,
    WalPrefixObservationScan,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalValidPrefix {
    source_range: WalLsnRange,
    prefix_range: WalLsnRange,
    counters: WalValidPrefixCounters,
    torn_tail: Option<TornWalTailClassification>,
}

impl WalValidPrefix {
    pub fn from_selected_source(
        source: &AdmittedRecoverySource,
        expected_generation: WalSegmentGeneration,
        acknowledged_range: WalLsnRange,
        observations: Vec<WalPrefixIntegrityObservation>,
    ) -> Result<Self, RedoPlanningDenial> {
        Self::from_observation_scan(
            source,
            expected_generation,
            acknowledged_range,
            WalPrefixObservationScan::from_observations(observations),
        )
    }

    pub fn from_observation_scan(
        source: &AdmittedRecoverySource,
        expected_generation: WalSegmentGeneration,
        acknowledged_range: WalLsnRange,
        scan: WalPrefixObservationScan,
    ) -> Result<Self, RedoPlanningDenial> {
        let replay_index = crate::layout_access::admit_recovery_source_replay_index(source)
            .map_err(|_| RedoPlanningDenial::new(RedoPlanningDenialKind::NoAdmittedWalTail))?;
        let source_range = replay_index.replay_frontier();
        crate::layout_access::lookup_recovery_tail_range(&replay_index, acknowledged_range)
            .map_err(|_| missing_acknowledged_range(acknowledged_range, acknowledged_range))?;
        Self::from_observed_frames(
            source_range,
            expected_generation,
            acknowledged_range,
            scan.into_frame_observations(),
        )
    }

    pub fn source_range(&self) -> WalLsnRange {
        self.source_range
    }

    pub fn prefix_range(&self) -> WalLsnRange {
        self.prefix_range
    }

    pub const fn counters(&self) -> WalValidPrefixCounters {
        self.counters
    }

    pub const fn torn_tail(&self) -> Option<TornWalTailClassification> {
        self.torn_tail
    }

    pub const fn admitted_frame_count(&self) -> usize {
        self.counters.admitted_prefix_frames
    }

    pub const fn contains_lsn(&self, lsn: LogSequenceNumber) -> bool {
        self.prefix_range.contains(lsn)
    }

    fn from_observed_frames(
        source_range: WalLsnRange,
        expected_generation: WalSegmentGeneration,
        acknowledged_range: WalLsnRange,
        mut observations: Vec<WalPrefixFrameObservation>,
    ) -> Result<Self, RedoPlanningDenial> {
        observations.sort_by_key(|frame| frame.lsn());
        let mut expected_lsn = source_range.start().get();
        let mut admitted_prefix_frames = 0usize;
        let mut torn_tail = None;

        for frame in observations {
            if !source_range.contains(frame.lsn()) {
                continue;
            }
            if frame.lsn().get() < expected_lsn {
                continue;
            }
            if frame.lsn().get() > expected_lsn {
                return Err(missing_from_gap(
                    expected_lsn,
                    frame.lsn(),
                    acknowledged_range,
                ));
            }
            if frame.segment_generation() != expected_generation {
                return Err(RedoPlanningDenial::new(
                    RedoPlanningDenialKind::StaleWalGeneration(StaleWalGenerationDenial::new(
                        frame.lsn(),
                        expected_generation,
                        frame.segment_generation(),
                    )),
                ));
            }
            match frame.posture() {
                WalPrefixFramePosture::IntegrityVetted => {
                    expected_lsn += 1;
                    admitted_prefix_frames += 1;
                }
                WalPrefixFramePosture::TornTail => {
                    if frame.lsn().get() < acknowledged_range.end_exclusive().get() {
                        return Err(missing_acknowledged_range(
                            range_from_values(
                                frame.lsn().get(),
                                acknowledged_range.end_exclusive().get(),
                            ),
                            acknowledged_range,
                        ));
                    }
                    let prefix_range = range_from_values(source_range.start().get(), expected_lsn);
                    torn_tail = Some(TornWalTailClassification::new(prefix_range, frame.lsn()));
                    break;
                }
                WalPrefixFramePosture::MiddleCorruption => {
                    return Err(RedoPlanningDenial::new(
                        RedoPlanningDenialKind::MiddleWalCorruption(
                            MiddleWalCorruptionDenial::new(frame.lsn(), acknowledged_range),
                        ),
                    ));
                }
            }
        }

        if expected_lsn < acknowledged_range.end_exclusive().get() {
            return Err(missing_acknowledged_range(
                range_from_values(expected_lsn, acknowledged_range.end_exclusive().get()),
                acknowledged_range,
            ));
        }

        let prefix_range = range_from_values(source_range.start().get(), expected_lsn);
        Ok(Self {
            source_range,
            prefix_range,
            counters: WalValidPrefixCounters {
                observed_frames: admitted_prefix_frames + usize::from(torn_tail.is_some()),
                admitted_prefix_frames,
            },
            torn_tail,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalValidPrefixCounters {
    observed_frames: usize,
    admitted_prefix_frames: usize,
}

impl WalValidPrefixCounters {
    pub const fn observed_frames(self) -> usize {
        self.observed_frames
    }

    pub const fn admitted_prefix_frames(self) -> usize {
        self.admitted_prefix_frames
    }
}

fn missing_from_gap(
    expected_lsn: u64,
    observed_lsn: LogSequenceNumber,
    acknowledged_range: WalLsnRange,
) -> RedoPlanningDenial {
    let gap_end = observed_lsn
        .get()
        .min(acknowledged_range.end_exclusive().get());
    missing_acknowledged_range(range_from_values(expected_lsn, gap_end), acknowledged_range)
}

fn missing_acknowledged_range(
    missing_range: WalLsnRange,
    acknowledged_range: WalLsnRange,
) -> RedoPlanningDenial {
    RedoPlanningDenial::new(RedoPlanningDenialKind::MissingAcknowledgedWalRange(
        MissingAcknowledgedWalRangeDenial::new(missing_range, acknowledged_range),
    ))
}

fn range_from_values(start: u64, end_exclusive: u64) -> WalLsnRange {
    WalLsnRange::new(
        LogSequenceNumber::new(start),
        LogSequenceNumber::new(end_exclusive.max(start + 1)),
    )
    .expect("valid prefix ranges are constructed from monotonic WAL LSN values")
}
