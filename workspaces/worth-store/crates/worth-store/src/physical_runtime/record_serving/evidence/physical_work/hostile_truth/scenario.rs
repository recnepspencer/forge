use std::{collections::BTreeSet, num::NonZeroU32};

use super::PhysicalWorkHostileTruthEvidenceDenial;
use crate::physical_runtime::{PhysicalWorkProcessEvidence, PhysicalWorkProcessFateEvidence};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhysicalWorkHostileTruthScenario {
    BeforeBackendDispatch,
    DuringShortWrite,
    AfterExactWriteBeforeSchedulerSettlement,
    DuringRootPublication,
    DuringShutdown,
}

impl PhysicalWorkHostileTruthScenario {
    pub const ALL: [Self; 5] = [
        Self::BeforeBackendDispatch,
        Self::DuringShortWrite,
        Self::AfterExactWriteBeforeSchedulerSettlement,
        Self::DuringRootPublication,
        Self::DuringShutdown,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::BeforeBackendDispatch => "before-backend-dispatch",
            Self::DuringShortWrite => "during-short-write",
            Self::AfterExactWriteBeforeSchedulerSettlement => {
                "after-exact-write-before-scheduler-settlement"
            }
            Self::DuringRootPublication => "during-root-publication",
            Self::DuringShutdown => "during-shutdown",
        }
    }

    pub(in crate::physical_runtime::record_serving::evidence::physical_work) const fn requires_recovery_obligation(
        self,
    ) -> bool {
        matches!(
            self,
            Self::DuringShortWrite
                | Self::AfterExactWriteBeforeSchedulerSettlement
                | Self::DuringRootPublication
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkHostileProcessEvidence {
    seed: PhysicalWorkProcessEvidence,
    baseline_observer: PhysicalWorkProcessEvidence,
    writer: PhysicalWorkProcessEvidence,
    observer: PhysicalWorkProcessEvidence,
    reopener: PhysicalWorkProcessEvidence,
}

impl PhysicalWorkHostileProcessEvidence {
    pub fn new(
        seed: PhysicalWorkProcessEvidence,
        baseline_observer: PhysicalWorkProcessEvidence,
        writer: PhysicalWorkProcessEvidence,
        observer: PhysicalWorkProcessEvidence,
        reopener: PhysicalWorkProcessEvidence,
    ) -> Result<Self, PhysicalWorkHostileTruthEvidenceDenial> {
        let processes = [&seed, &baseline_observer, &writer, &observer, &reopener];
        if processes
            .iter()
            .map(|process| process.process())
            .collect::<BTreeSet<_>>()
            .len()
            != processes.len()
        {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::DuplicateProcessIdentity);
        }
        require_roles(&processes)?;
        require_fates(&processes)?;
        Ok(Self {
            seed,
            baseline_observer,
            writer,
            observer,
            reopener,
        })
    }

    pub fn ordered(&self) -> [&PhysicalWorkProcessEvidence; 5] {
        [
            &self.seed,
            &self.baseline_observer,
            &self.writer,
            &self.observer,
            &self.reopener,
        ]
    }

    pub fn ordered_ids(&self) -> [NonZeroU32; 5] {
        self.ordered().map(PhysicalWorkProcessEvidence::process)
    }
}

fn require_roles(
    processes: &[&PhysicalWorkProcessEvidence; 5],
) -> Result<(), PhysicalWorkHostileTruthEvidenceDenial> {
    let expected = [
        "seed-writer",
        "baseline-observer",
        "faulting-writer",
        "post-kill-observer",
        "fresh-reopener",
    ];
    if processes
        .iter()
        .zip(expected)
        .any(|(process, role)| process.role() != role)
    {
        return Err(PhysicalWorkHostileTruthEvidenceDenial::InvalidProcessRole);
    }
    Ok(())
}

fn require_fates(
    processes: &[&PhysicalWorkProcessEvidence; 5],
) -> Result<(), PhysicalWorkHostileTruthEvidenceDenial> {
    let successes = [0, 1, 3, 4].into_iter().all(|index| {
        matches!(
            processes[index].fate(),
            PhysicalWorkProcessFateEvidence::ExitedSuccess
        )
    });
    if !successes
        || !matches!(
            processes[2].fate(),
            PhysicalWorkProcessFateEvidence::KilledAtYieldpoint(_)
        )
    {
        return Err(PhysicalWorkHostileTruthEvidenceDenial::InvalidProcessFate);
    }
    Ok(())
}
