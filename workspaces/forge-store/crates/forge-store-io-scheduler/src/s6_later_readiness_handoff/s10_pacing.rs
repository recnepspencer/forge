use forge_store_contracts::S6LaterMilestoneDestination;

use crate::{BackgroundPacingCounterSnapshot, BackgroundPacingOutcome};

use super::S6LaterReadinessHandoffDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10CompactionPacingEvidence {
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10BackupExportPacingEvidence {
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10RepairScanPacingEvidence {
    counters: BackgroundPacingCounterSnapshot,
}

impl S10CompactionPacingEvidence {
    pub fn from_background_pacing(
        outcome: BackgroundPacingOutcome,
    ) -> Result<Self, S6LaterReadinessHandoffDenial> {
        let counters = background_pacing_counters(outcome);
        require_budget(
            counters.compaction_debt(),
            S6LaterMilestoneDestination::S10Compaction,
        )?;
        Ok(Self { counters })
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl S10BackupExportPacingEvidence {
    pub fn from_background_pacing(
        outcome: BackgroundPacingOutcome,
    ) -> Result<Self, S6LaterReadinessHandoffDenial> {
        let counters = background_pacing_counters(outcome);
        require_budget(
            counters.backup_pressure(),
            S6LaterMilestoneDestination::S10BackupExport,
        )?;
        Ok(Self { counters })
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl S10RepairScanPacingEvidence {
    pub fn from_background_pacing(
        outcome: BackgroundPacingOutcome,
    ) -> Result<Self, S6LaterReadinessHandoffDenial> {
        let counters = background_pacing_counters(outcome);
        require_budget(
            counters.repair_pressure(),
            S6LaterMilestoneDestination::S10RepairScan,
        )?;
        Ok(Self { counters })
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

const fn require_budget(
    budget: crate::BackgroundResourceBudget,
    destination: S6LaterMilestoneDestination,
) -> Result<(), S6LaterReadinessHandoffDenial> {
    if budget.is_empty() {
        Err(S6LaterReadinessHandoffDenial::MissingBackgroundPacingEvidence { destination })
    } else {
        Ok(())
    }
}

pub(super) const fn background_pacing_counters(
    outcome: BackgroundPacingOutcome,
) -> BackgroundPacingCounterSnapshot {
    match outcome {
        BackgroundPacingOutcome::Yield(outcome) => outcome.counters(),
        BackgroundPacingOutcome::Deferred(outcome) => outcome.counters(),
        BackgroundPacingOutcome::Denied(outcome) => outcome.counters(),
        BackgroundPacingOutcome::StaleRebindRequired(outcome) => outcome.counters(),
        BackgroundPacingOutcome::Throttled(outcome) => outcome.counters(),
        BackgroundPacingOutcome::AdmittedWithDebt(outcome) => outcome.counters(),
        BackgroundPacingOutcome::Violation(outcome) => outcome.counters(),
    }
}
