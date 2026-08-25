use serde::{Deserialize, Serialize};

use super::reference::{FoundationalBranchReferenceObservation, FoundationalBranchTargetBasis};

/// The complete exact source observation used to describe a fork.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FoundationalBranchForkBasis<T: FoundationalBranchTargetBasis> {
    source_observation: FoundationalBranchReferenceObservation<T>,
}

impl<T: FoundationalBranchTargetBasis> FoundationalBranchForkBasis<T> {
    pub fn new(source_observation: FoundationalBranchReferenceObservation<T>) -> Self {
        Self { source_observation }
    }

    pub fn source_observation(&self) -> &FoundationalBranchReferenceObservation<T> {
        &self.source_observation
    }
}
