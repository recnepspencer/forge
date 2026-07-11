use crate::artifact_family::PhysicalArtifactFamily;
use crate::materialization::{S8LayoutCoverageWitness, S8MaterializationDenial};
use crate::production_transition::define_owner_outcome;
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

define_owner_outcome!(
    pub S8DerivedIndexRebuildOutcome,
    pub S8DerivedIndexRebuildView,
    S8DerivedIndexRebuildCase,
    DerivedRebuildParity,
    RebuildDerivedIndex,
    [
        rebuilt => Rebuilt(S8DerivedIndexRebuildReceipt): Declared => Rebuild => Rebuilt,
        quarantined => Quarantined(S8LayoutQuarantineWitness): Declared => Quarantine => Quarantined,
        denied => Denied(S8DerivedIndexRebuildDenied): Declared => Deny => Denied
    ]
);

impl S8DerivedIndexRebuildOutcome {
    pub fn into_rebuilt(self) -> Result<S8DerivedIndexRebuildReceipt, Self> {
        match self.into_owner_payload() {
            S8DerivedIndexRebuildCase::Rebuilt(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}

define_owner_outcome!(
    pub S8DerivedIndexParityOutcome,
    pub S8DerivedIndexParityView,
    S8DerivedIndexParityCase,
    DerivedRebuildParity,
    VerifyDerivedParity,
    [
        verified => Verified(S8DerivedIndexParityWitness): Rebuilt => VerifyParity => ParityVerified,
        denied => Denied(S8DerivedIndexRebuildDenied): Rebuilt => Deny => Denied
    ]
);

impl S8DerivedIndexParityOutcome {
    pub fn into_verified(self) -> Result<S8DerivedIndexParityWitness, Self> {
        match self.into_owner_payload() {
            S8DerivedIndexParityCase::Verified(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}
