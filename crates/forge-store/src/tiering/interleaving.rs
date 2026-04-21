use crate::live_query::ContinuationStrategy;
use crate::ForegroundIsolationOutcome;
use serde::Serialize;

use super::{PlacementExecutionOrigin, RetainedReadPlacementPath, TierMissOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PlacementRaceOutcome {
    NoRace,
    MovePrepareObserved,
    TransferObserved,
    CutoverObserved,
    RecallObserved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierInterleavingObservation {
    race_outcome: PlacementRaceOutcome,
    observed_artifact_keys: Vec<String>,
}

impl TierInterleavingObservation {
    pub(crate) fn new(
        race_outcome: PlacementRaceOutcome,
        mut observed_artifact_keys: Vec<String>,
    ) -> Self {
        observed_artifact_keys.sort();
        observed_artifact_keys.dedup();
        Self {
            race_outcome,
            observed_artifact_keys,
        }
    }

    pub fn race_outcome(&self) -> PlacementRaceOutcome {
        self.race_outcome
    }

    pub fn observed_artifact_keys(&self) -> &[String] {
        &self.observed_artifact_keys
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InterleavedReadParityReport {
    observation: TierInterleavingObservation,
    execution_origin: PlacementExecutionOrigin,
    placement_path: RetainedReadPlacementPath,
    tier_miss_outcome: TierMissOutcome,
    foreground_isolation: Option<ForegroundIsolationOutcome>,
    parity_preserved: bool,
}

impl InterleavedReadParityReport {
    pub(crate) fn new(
        observation: TierInterleavingObservation,
        execution_origin: PlacementExecutionOrigin,
        placement_path: RetainedReadPlacementPath,
        tier_miss_outcome: TierMissOutcome,
        foreground_isolation: Option<ForegroundIsolationOutcome>,
        parity_preserved: bool,
    ) -> Self {
        Self {
            observation,
            execution_origin,
            placement_path,
            tier_miss_outcome,
            foreground_isolation,
            parity_preserved,
        }
    }

    pub fn observation(&self) -> &TierInterleavingObservation {
        &self.observation
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }

    pub fn placement_path(&self) -> RetainedReadPlacementPath {
        self.placement_path
    }

    pub fn tier_miss_outcome(&self) -> TierMissOutcome {
        self.tier_miss_outcome
    }

    pub fn foreground_isolation(&self) -> Option<&ForegroundIsolationOutcome> {
        self.foreground_isolation.as_ref()
    }

    pub fn parity_preserved(&self) -> bool {
        self.parity_preserved
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InterleavedContinuationParityReport {
    observation: TierInterleavingObservation,
    strategy: ContinuationStrategy,
    foreground_isolation: ForegroundIsolationOutcome,
    parity_preserved: bool,
}

impl InterleavedContinuationParityReport {
    pub(crate) fn new(
        observation: TierInterleavingObservation,
        strategy: ContinuationStrategy,
        foreground_isolation: ForegroundIsolationOutcome,
        parity_preserved: bool,
    ) -> Self {
        Self {
            observation,
            strategy,
            foreground_isolation,
            parity_preserved,
        }
    }

    pub fn observation(&self) -> &TierInterleavingObservation {
        &self.observation
    }

    pub fn strategy(&self) -> ContinuationStrategy {
        self.strategy
    }

    pub fn foreground_isolation(&self) -> &ForegroundIsolationOutcome {
        &self.foreground_isolation
    }

    pub fn parity_preserved(&self) -> bool {
        self.parity_preserved
    }
}
