use worth_store_physical_format::RecordFrameCoordinate;

#[cfg(feature = "certification-test-authority")]
use crate::physical_runtime::LifecycleGeneration;
use crate::physical_runtime::PhysicalWorkIdentity;

#[cfg(feature = "certification-test-authority")]
use super::super::frame_loading::{LoadedPhysicalFrame, PhysicalFrameAccessOrigin};
use super::super::{PhysicalRecordPressureEvidence, PhysicalRecordResidencyFailure};
use super::PhysicalSpeculativeReadFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPrefetchOutcome {
    Hit {
        coordinate: RecordFrameCoordinate,
    },
    Coalesced {
        coordinate: RecordFrameCoordinate,
    },
    Loaded {
        coordinate: RecordFrameCoordinate,
        work: PhysicalWorkIdentity,
    },
    Dropped(PhysicalSpeculativeReadDrop),
    Failed(PhysicalSpeculativeReadFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReadAheadFrameOutcome {
    Hit {
        coordinate: RecordFrameCoordinate,
    },
    Coalesced {
        coordinate: RecordFrameCoordinate,
    },
    Loaded {
        coordinate: RecordFrameCoordinate,
        work: PhysicalWorkIdentity,
    },
    Failed {
        coordinate: RecordFrameCoordinate,
        failure: PhysicalSpeculativeReadFailure,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalReadAheadBatch {
    frames: Vec<PhysicalReadAheadFrameOutcome>,
    hits: u32,
    coalesced: u32,
    loaded: u32,
    failed: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PhysicalReadAheadOutcome {
    Complete(PhysicalReadAheadBatch),
    Partial(PhysicalReadAheadBatch),
    Failed(PhysicalReadAheadBatch),
    Dropped(PhysicalSpeculativeReadDrop),
    FailedBeforeFrames(PhysicalSpeculativeReadFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSpeculativeReadDrop {
    failure: PhysicalRecordResidencyFailure,
    pressure: Option<PhysicalRecordPressureEvidence>,
}

impl PhysicalSpeculativeReadDrop {
    #[cfg(feature = "certification-test-authority")]
    pub(super) fn bind(
        failure: PhysicalRecordResidencyFailure,
        generation: LifecycleGeneration,
    ) -> Self {
        Self {
            failure,
            pressure: PhysicalRecordPressureEvidence::from_store_failure(failure, generation),
        }
    }

    pub const fn failure(self) -> PhysicalRecordResidencyFailure {
        self.failure
    }

    pub const fn pressure(self) -> Option<PhysicalRecordPressureEvidence> {
        self.pressure
    }
}

impl PhysicalReadAheadFrameOutcome {
    pub const fn coordinate(self) -> RecordFrameCoordinate {
        match self {
            Self::Hit { coordinate }
            | Self::Coalesced { coordinate }
            | Self::Loaded { coordinate, .. }
            | Self::Failed { coordinate, .. } => coordinate,
        }
    }

    pub const fn work(self) -> Option<PhysicalWorkIdentity> {
        match self {
            Self::Loaded { work, .. } => Some(work),
            _ => None,
        }
    }

    pub const fn failure(self) -> Option<PhysicalSpeculativeReadFailure> {
        match self {
            Self::Failed { failure, .. } => Some(failure),
            _ => None,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) const fn failed(
        coordinate: RecordFrameCoordinate,
        failure: PhysicalSpeculativeReadFailure,
    ) -> Self {
        Self::Failed {
            coordinate,
            failure,
        }
    }
}

impl PhysicalReadAheadBatch {
    #[cfg(feature = "certification-test-authority")]
    pub(super) fn bind(frames: Vec<PhysicalReadAheadFrameOutcome>) -> Self {
        let mut hits = 0_u32;
        let mut coalesced = 0_u32;
        let mut loaded = 0_u32;
        let mut failed = 0_u32;
        for frame in &frames {
            match frame {
                PhysicalReadAheadFrameOutcome::Hit { .. } => hits += 1,
                PhysicalReadAheadFrameOutcome::Coalesced { .. } => coalesced += 1,
                PhysicalReadAheadFrameOutcome::Loaded { .. } => loaded += 1,
                PhysicalReadAheadFrameOutcome::Failed { .. } => failed += 1,
            }
        }
        Self {
            frames,
            hits,
            coalesced,
            loaded,
            failed,
        }
    }

    pub fn frames(&self) -> &[PhysicalReadAheadFrameOutcome] {
        &self.frames
    }

    pub const fn hits(&self) -> u32 {
        self.hits
    }

    pub const fn coalesced(&self) -> u32 {
        self.coalesced
    }

    pub const fn loaded(&self) -> u32 {
        self.loaded
    }

    pub const fn failed(&self) -> u32 {
        self.failed
    }

    pub fn total(&self) -> usize {
        self.frames.len()
    }
}

impl PhysicalReadAheadOutcome {
    #[cfg(feature = "certification-test-authority")]
    pub(super) fn from_batch(batch: PhysicalReadAheadBatch) -> Self {
        if batch.failed == 0 {
            Self::Complete(batch)
        } else if usize::try_from(batch.failed).ok() == Some(batch.frames.len()) {
            Self::Failed(batch)
        } else {
            Self::Partial(batch)
        }
    }
}

#[cfg(feature = "certification-test-authority")]
pub(super) fn classify_loaded_frame(
    frame: LoadedPhysicalFrame,
) -> Result<PhysicalReadAheadFrameOutcome, PhysicalSpeculativeReadFailure> {
    let coordinate = frame.coordinate();
    let work = frame.work_trace();
    match frame.origin() {
        PhysicalFrameAccessOrigin::Hit if work.count() == 0 => {
            Ok(PhysicalReadAheadFrameOutcome::Hit { coordinate })
        }
        PhysicalFrameAccessOrigin::Hit => {
            Err(PhysicalSpeculativeReadFailure::HitCreatedPhysicalWork {
                coordinate,
                observed_count: work.count(),
            })
        }
        PhysicalFrameAccessOrigin::Coalesced if work.count() == 0 => {
            Ok(PhysicalReadAheadFrameOutcome::Coalesced { coordinate })
        }
        PhysicalFrameAccessOrigin::Coalesced => Err(
            PhysicalSpeculativeReadFailure::CoalescedConsumerCreatedPhysicalWork {
                coordinate,
                observed_count: work.count(),
            },
        ),
        PhysicalFrameAccessOrigin::Fault
            if work.count() == 1 && work.first().is_some() && work.first() == work.last() =>
        {
            Ok(PhysicalReadAheadFrameOutcome::Loaded {
                coordinate,
                work: work
                    .first()
                    .expect("the guarded canonical miss trace has one identity"),
            })
        }
        PhysicalFrameAccessOrigin::Fault => Err(
            PhysicalSpeculativeReadFailure::CanonicalMissWorkIdentityMismatch {
                coordinate,
                observed_count: work.count(),
                first: work.first(),
                last: work.last(),
            },
        ),
    }
}

#[cfg(feature = "certification-test-authority")]
pub(super) fn prefetch_outcome(frame: PhysicalReadAheadFrameOutcome) -> PhysicalPrefetchOutcome {
    match frame {
        PhysicalReadAheadFrameOutcome::Hit { coordinate } => {
            PhysicalPrefetchOutcome::Hit { coordinate }
        }
        PhysicalReadAheadFrameOutcome::Coalesced { coordinate } => {
            PhysicalPrefetchOutcome::Coalesced { coordinate }
        }
        PhysicalReadAheadFrameOutcome::Loaded { coordinate, work } => {
            PhysicalPrefetchOutcome::Loaded { coordinate, work }
        }
        PhysicalReadAheadFrameOutcome::Failed { failure, .. } => {
            PhysicalPrefetchOutcome::Failed(failure)
        }
    }
}
