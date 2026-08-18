use serde::{Deserialize, Serialize};

use super::reference::{
    FoundationalBranchReferenceMismatch, FoundationalBranchReferenceObservation,
    FoundationalBranchTargetBasis,
};

/// The exact expected observation a conditional owner movement must compare.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FoundationalBranchComparisonBasis<T: FoundationalBranchTargetBasis> {
    expected_observation: FoundationalBranchReferenceObservation<T>,
}

impl<T: FoundationalBranchTargetBasis> FoundationalBranchComparisonBasis<T> {
    pub fn new(expected_observation: FoundationalBranchReferenceObservation<T>) -> Self {
        Self {
            expected_observation,
        }
    }

    pub fn expected_observation(&self) -> &FoundationalBranchReferenceObservation<T> {
        &self.expected_observation
    }

    pub fn compare(
        &self,
        observed: &FoundationalBranchReferenceObservation<T>,
    ) -> Result<(), FoundationalBranchReferenceMismatch<T>> {
        self.expected_observation.compare(observed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FoundationalBranchReferenceMovementKind {
    Fork,
    Truth,
    Metadata,
    Lifecycle,
}

/// A descriptive before/after movement. Construction carries no owner effect
/// or currentness claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FoundationalBranchReferenceMovement<T: FoundationalBranchTargetBasis> {
    before: FoundationalBranchReferenceObservation<T>,
    after: FoundationalBranchReferenceObservation<T>,
    kind: FoundationalBranchReferenceMovementKind,
}

impl<T: FoundationalBranchTargetBasis> FoundationalBranchReferenceMovement<T> {
    pub fn new(
        before: FoundationalBranchReferenceObservation<T>,
        after: FoundationalBranchReferenceObservation<T>,
        kind: FoundationalBranchReferenceMovementKind,
    ) -> Self {
        Self {
            before,
            after,
            kind,
        }
    }

    pub fn before(&self) -> &FoundationalBranchReferenceObservation<T> {
        &self.before
    }

    pub fn after(&self) -> &FoundationalBranchReferenceObservation<T> {
        &self.after
    }

    pub const fn kind(&self) -> FoundationalBranchReferenceMovementKind {
        self.kind
    }
}
