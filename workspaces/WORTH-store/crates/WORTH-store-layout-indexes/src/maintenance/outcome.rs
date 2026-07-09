use crate::artifact_family::PhysicalArtifactFamily;
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
pub enum S8DerivedIndexRebuildOutcome {
    Rebuilt(S8DerivedIndexRebuildReceipt),
    Quarantined(S8LayoutQuarantineWitness),
    Denied(S8DerivedIndexRebuildDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub enum S8DerivedIndexParityOutcome {
    Verified(S8DerivedIndexParityWitness),
    Denied(S8DerivedIndexRebuildDenied),
}
