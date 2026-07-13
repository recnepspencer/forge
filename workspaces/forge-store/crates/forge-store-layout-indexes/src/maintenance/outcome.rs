use crate::catalog::PhysicalArtifactFamily;
use crate::materialization::{LayoutCoverageWitness, MaterializationDenial};
use crate::strategy::{LayoutStrategyFamily, StrategyDenial, StrategyRebuildSourceRequirement};
use crate::LayoutQuarantineWitness;

use super::parity::DerivedIndexParityWitness;
use super::rebuild::DerivedIndexRebuildReceipt;
use super::source::DerivedIndexRebuildSourceInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedIndexRebuildDenied {
    StrategyDenied {
        denial: StrategyDenial,
    },
    SourceInputIsNotAuthority {
        source: DerivedIndexRebuildSourceInput,
    },
    CoverageDenied {
        denial: MaterializationDenial,
    },
    RebuildShapeRequired {
        family: LayoutStrategyFamily,
    },
    SourceCoverageDoesNotMatchRebuildShape {
        expected: LayoutCoverageWitness,
        actual: LayoutCoverageWitness,
    },
    SourceFamilyMismatch {
        expected: PhysicalArtifactFamily,
        actual: PhysicalArtifactFamily,
    },
    SourceArtifactDoesNotMatchStrategy {
        required: StrategyRebuildSourceRequirement,
        source: &'static str,
    },
    SourceParityBasisDoesNotMatchAuthorityArtifact {
        expected_rows: usize,
        actual_rows: usize,
    },
    SourceParityBasisKeysDoNotMatchAuthorityArtifact,
    ParityRowsMustBeCanonical,
    ParityKeysMustBeUnique,
    ParityCounterShapeMustBeCanonical,
    ParityKeyIdentityMismatch,
    ParityValueIdentityMismatch,
    ParityOrderingMismatch,
    ParityCoverageMismatch {
        expected: LayoutCoverageWitness,
        actual: LayoutCoverageWitness,
    },
    ParityCostEnvelopeMismatch,
    ParityCounterShapeMismatch,
}

#[derive(Debug, PartialEq, Eq)]
enum DerivedIndexRebuildCase {
    Rebuilt(DerivedIndexRebuildReceipt),
    Quarantined(LayoutQuarantineWitness),
    Denied(DerivedIndexRebuildDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildOutcome {
    case: DerivedIndexRebuildCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexRebuildView<'a> {
    Rebuilt(&'a DerivedIndexRebuildReceipt),
    Quarantined(&'a LayoutQuarantineWitness),
    Denied(&'a DerivedIndexRebuildDenied),
}

impl DerivedIndexRebuildOutcome {
    pub(crate) fn rebuilt(value: DerivedIndexRebuildReceipt) -> Self {
        Self::from_owner_payload(DerivedIndexRebuildCase::Rebuilt(value))
    }

    pub(crate) fn quarantined(value: LayoutQuarantineWitness) -> Self {
        Self::from_owner_payload(DerivedIndexRebuildCase::Quarantined(value))
    }

    pub(crate) fn denied(value: DerivedIndexRebuildDenied) -> Self {
        Self::from_owner_payload(DerivedIndexRebuildCase::Denied(value))
    }

    fn from_owner_payload(case: DerivedIndexRebuildCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> DerivedIndexRebuildView<'_> {
        match &self.case {
            DerivedIndexRebuildCase::Rebuilt(value) => DerivedIndexRebuildView::Rebuilt(value),
            DerivedIndexRebuildCase::Quarantined(value) => {
                DerivedIndexRebuildView::Quarantined(value)
            }
            DerivedIndexRebuildCase::Denied(value) => DerivedIndexRebuildView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> DerivedIndexRebuildCase {
        self.case
    }
}

impl DerivedIndexRebuildOutcome {
    pub fn into_rebuilt(self) -> Result<DerivedIndexRebuildReceipt, Self> {
        match self.into_owner_payload() {
            DerivedIndexRebuildCase::Rebuilt(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DerivedIndexParityCase {
    Verified(DerivedIndexParityWitness),
    Denied(DerivedIndexRebuildDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexParityOutcome {
    case: DerivedIndexParityCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexParityView<'a> {
    Verified(&'a DerivedIndexParityWitness),
    Denied(&'a DerivedIndexRebuildDenied),
}

impl DerivedIndexParityOutcome {
    pub(crate) fn verified(value: DerivedIndexParityWitness) -> Self {
        Self::from_owner_payload(DerivedIndexParityCase::Verified(value))
    }

    pub(crate) fn denied(value: DerivedIndexRebuildDenied) -> Self {
        Self::from_owner_payload(DerivedIndexParityCase::Denied(value))
    }

    fn from_owner_payload(case: DerivedIndexParityCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> DerivedIndexParityView<'_> {
        match &self.case {
            DerivedIndexParityCase::Verified(value) => DerivedIndexParityView::Verified(value),
            DerivedIndexParityCase::Denied(value) => DerivedIndexParityView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> DerivedIndexParityCase {
        self.case
    }
}

impl DerivedIndexParityOutcome {
    pub fn into_verified(self) -> Result<DerivedIndexParityWitness, Self> {
        match self.into_owner_payload() {
            DerivedIndexParityCase::Verified(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}
