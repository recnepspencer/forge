use crate::LayoutCorruptionClassification;

use super::super::quarantine::LayoutQuarantineWitness;
use super::super::readmission::{
    ImportReadmissionRequirement, OfflineReadmissionRequirement, QuarantineReadmissionRequirement,
};
use super::cases::{CorruptionClassificationCaseId, CORRUPTION_CLASSIFICATION_CASES};
use super::class::LayoutCorruptionClass;

#[derive(Debug, PartialEq, Eq)]
enum LayoutCorruptionCase {
    RebuildRequired(LayoutCorruptionClassification),
    Quarantined(LayoutQuarantineWitness),
    QuarantineReadmissionRequired(QuarantineReadmissionRequirement),
    OfflineReadmissionRequired(OfflineReadmissionRequirement),
    ImportReadmissionRequired(ImportReadmissionRequirement),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutCorruptionOutcome {
    case: LayoutCorruptionCase,
    counters: super::LayoutCorruptionCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCorruptionView<'a> {
    RebuildRequired(&'a LayoutCorruptionClassification),
    Quarantined(&'a LayoutQuarantineWitness),
    QuarantineReadmissionRequired(&'a QuarantineReadmissionRequirement),
    OfflineReadmissionRequired(&'a OfflineReadmissionRequirement),
    ImportReadmissionRequired(&'a ImportReadmissionRequirement),
}

impl LayoutCorruptionOutcome {
    pub(super) fn rebuild_required(
        value: LayoutCorruptionClassification,
        counters: super::LayoutCorruptionCounterSnapshot,
    ) -> Self {
        Self::issue(LayoutCorruptionCase::RebuildRequired(value), counters)
    }

    pub(super) fn quarantined(
        value: LayoutQuarantineWitness,
        counters: super::LayoutCorruptionCounterSnapshot,
    ) -> Self {
        Self::issue(LayoutCorruptionCase::Quarantined(value), counters)
    }

    pub(super) fn quarantine_readmission_required(
        value: QuarantineReadmissionRequirement,
        counters: super::LayoutCorruptionCounterSnapshot,
    ) -> Self {
        Self::issue(
            LayoutCorruptionCase::QuarantineReadmissionRequired(value),
            counters,
        )
    }

    pub(super) fn offline_readmission_required(
        value: OfflineReadmissionRequirement,
        counters: super::LayoutCorruptionCounterSnapshot,
    ) -> Self {
        Self::issue(
            LayoutCorruptionCase::OfflineReadmissionRequired(value),
            counters,
        )
    }

    pub(super) fn import_readmission_required(
        value: ImportReadmissionRequirement,
        counters: super::LayoutCorruptionCounterSnapshot,
    ) -> Self {
        Self::issue(
            LayoutCorruptionCase::ImportReadmissionRequired(value),
            counters,
        )
    }

    fn issue(case: LayoutCorruptionCase, counters: super::LayoutCorruptionCounterSnapshot) -> Self {
        Self { case, counters }
    }

    pub const fn view(&self) -> LayoutCorruptionView<'_> {
        match &self.case {
            LayoutCorruptionCase::RebuildRequired(value) => {
                LayoutCorruptionView::RebuildRequired(value)
            }
            LayoutCorruptionCase::Quarantined(value) => LayoutCorruptionView::Quarantined(value),
            LayoutCorruptionCase::QuarantineReadmissionRequired(value) => {
                LayoutCorruptionView::QuarantineReadmissionRequired(value)
            }
            LayoutCorruptionCase::OfflineReadmissionRequired(value) => {
                LayoutCorruptionView::OfflineReadmissionRequired(value)
            }
            LayoutCorruptionCase::ImportReadmissionRequired(value) => {
                LayoutCorruptionView::ImportReadmissionRequired(value)
            }
        }
    }

    pub const fn counters(&self) -> super::LayoutCorruptionCounterSnapshot {
        self.counters
    }

    pub const fn class(&self) -> LayoutCorruptionClass {
        match self.case {
            LayoutCorruptionCase::RebuildRequired(_) => {
                LayoutCorruptionClass::DerivedProjectionCorruption
            }
            LayoutCorruptionCase::Quarantined(_) => {
                LayoutCorruptionClass::AuthoritativeArtifactCorruption
            }
            LayoutCorruptionCase::QuarantineReadmissionRequired(_)
            | LayoutCorruptionCase::OfflineReadmissionRequired(_)
            | LayoutCorruptionCase::ImportReadmissionRequired(_) => {
                LayoutCorruptionClass::ReadmissionRequired
            }
        }
    }

    pub const fn case_id(&self) -> CorruptionClassificationCaseId {
        match self.case {
            LayoutCorruptionCase::RebuildRequired(_) => CORRUPTION_CLASSIFICATION_CASES[0],
            LayoutCorruptionCase::Quarantined(_) => CORRUPTION_CLASSIFICATION_CASES[1],
            LayoutCorruptionCase::QuarantineReadmissionRequired(_) => {
                CORRUPTION_CLASSIFICATION_CASES[2]
            }
            LayoutCorruptionCase::OfflineReadmissionRequired(_) => {
                CORRUPTION_CLASSIFICATION_CASES[3]
            }
            LayoutCorruptionCase::ImportReadmissionRequired(_) => {
                CORRUPTION_CLASSIFICATION_CASES[4]
            }
        }
    }

    pub(super) fn into_quarantined(
        self,
    ) -> Result<
        (
            LayoutQuarantineWitness,
            super::LayoutCorruptionCounterSnapshot,
        ),
        Self,
    > {
        match self.case {
            LayoutCorruptionCase::Quarantined(value) => Ok((value, self.counters)),
            case => Err(Self::issue(case, self.counters)),
        }
    }

    pub fn into_quarantine_readmission_requirement(
        self,
    ) -> Result<QuarantineReadmissionRequirement, Self> {
        match self.case {
            LayoutCorruptionCase::QuarantineReadmissionRequired(value) => Ok(value),
            case => Err(Self::issue(case, self.counters)),
        }
    }

    pub fn into_offline_readmission_requirement(
        self,
    ) -> Result<OfflineReadmissionRequirement, Self> {
        match self.case {
            LayoutCorruptionCase::OfflineReadmissionRequired(value) => Ok(value),
            case => Err(Self::issue(case, self.counters)),
        }
    }

    pub fn into_import_readmission_requirement(self) -> Result<ImportReadmissionRequirement, Self> {
        match self.case {
            LayoutCorruptionCase::ImportReadmissionRequired(value) => Ok(value),
            case => Err(Self::issue(case, self.counters)),
        }
    }
}
