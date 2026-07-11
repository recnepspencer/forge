use crate::catalog::PhysicalArtifactFamily;
use crate::materialization::{S8LayoutCoverageWitness, S8MaterializationDenial};
use crate::strategy::{
    S8LayoutStrategyFamily, S8StrategyDenial, S8StrategyRebuildSourceRequirement,
};
use crate::S8LayoutQuarantineWitness;

use super::parity::S8DerivedIndexParityWitness;
use super::rebuild::S8DerivedIndexRebuildReceipt;
use super::source::S8DerivedIndexRebuildSourceInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8DerivedIndexRebuildDenied {
    StrategyDenied {
        denial: S8StrategyDenial,
    },
    SourceInputIsNotAuthority {
        source: S8DerivedIndexRebuildSourceInput,
    },
    CoverageDenied {
        denial: S8MaterializationDenial,
    },
    RebuildShapeRequired {
        family: S8LayoutStrategyFamily,
    },
    SourceCoverageDoesNotMatchRebuildShape {
        expected: S8LayoutCoverageWitness,
        actual: S8LayoutCoverageWitness,
    },
    SourceFamilyMismatch {
        expected: PhysicalArtifactFamily,
        actual: PhysicalArtifactFamily,
    },
    SourceArtifactDoesNotMatchStrategy {
        required: S8StrategyRebuildSourceRequirement,
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
        expected: S8LayoutCoverageWitness,
        actual: S8LayoutCoverageWitness,
    },
    ParityCostEnvelopeMismatch,
    ParityCounterShapeMismatch,
}

#[derive(Debug, PartialEq, Eq)]
enum S8DerivedIndexRebuildCase {
    Rebuilt(S8DerivedIndexRebuildReceipt),
    Quarantined(S8LayoutQuarantineWitness),
    Denied(S8DerivedIndexRebuildDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DerivedIndexRebuildOutcome {
    case: S8DerivedIndexRebuildCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexRebuildView<'a> {
    Rebuilt(&'a S8DerivedIndexRebuildReceipt),
    Quarantined(&'a S8LayoutQuarantineWitness),
    Denied(&'a S8DerivedIndexRebuildDenied),
}

impl S8DerivedIndexRebuildOutcome {
    pub(crate) fn rebuilt(value: S8DerivedIndexRebuildReceipt) -> Self {
        Self::from_owner_payload(S8DerivedIndexRebuildCase::Rebuilt(value))
    }

    pub(crate) fn quarantined(value: S8LayoutQuarantineWitness) -> Self {
        Self::from_owner_payload(S8DerivedIndexRebuildCase::Quarantined(value))
    }

    pub(crate) fn denied(value: S8DerivedIndexRebuildDenied) -> Self {
        Self::from_owner_payload(S8DerivedIndexRebuildCase::Denied(value))
    }

    fn from_owner_payload(case: S8DerivedIndexRebuildCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8DerivedIndexRebuildView<'_> {
        match &self.case {
            S8DerivedIndexRebuildCase::Rebuilt(value) => S8DerivedIndexRebuildView::Rebuilt(value),
            S8DerivedIndexRebuildCase::Quarantined(value) => {
                S8DerivedIndexRebuildView::Quarantined(value)
            }
            S8DerivedIndexRebuildCase::Denied(value) => S8DerivedIndexRebuildView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> S8DerivedIndexRebuildCase {
        self.case
    }
}

impl S8DerivedIndexRebuildOutcome {
    pub fn into_rebuilt(self) -> Result<S8DerivedIndexRebuildReceipt, Self> {
        match self.into_owner_payload() {
            S8DerivedIndexRebuildCase::Rebuilt(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8DerivedIndexParityCase {
    Verified(S8DerivedIndexParityWitness),
    Denied(S8DerivedIndexRebuildDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DerivedIndexParityOutcome {
    case: S8DerivedIndexParityCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexParityView<'a> {
    Verified(&'a S8DerivedIndexParityWitness),
    Denied(&'a S8DerivedIndexRebuildDenied),
}

impl S8DerivedIndexParityOutcome {
    pub(crate) fn verified(value: S8DerivedIndexParityWitness) -> Self {
        Self::from_owner_payload(S8DerivedIndexParityCase::Verified(value))
    }

    pub(crate) fn denied(value: S8DerivedIndexRebuildDenied) -> Self {
        Self::from_owner_payload(S8DerivedIndexParityCase::Denied(value))
    }

    fn from_owner_payload(case: S8DerivedIndexParityCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8DerivedIndexParityView<'_> {
        match &self.case {
            S8DerivedIndexParityCase::Verified(value) => S8DerivedIndexParityView::Verified(value),
            S8DerivedIndexParityCase::Denied(value) => S8DerivedIndexParityView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> S8DerivedIndexParityCase {
        self.case
    }
}

impl S8DerivedIndexParityOutcome {
    pub fn into_verified(self) -> Result<S8DerivedIndexParityWitness, Self> {
        match self.into_owner_payload() {
            S8DerivedIndexParityCase::Verified(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}
