use super::affected_set::SupportAffectedSet;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactedSupportBasis {
    affected_set: SupportAffectedSet,
    compacted_basis_digest: String,
}

impl CompactedSupportBasis {
    pub(crate) fn new(
        affected_set: SupportAffectedSet,
        compacted_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            compacted_basis_digest: require_non_empty(
                "compacted support basis",
                compacted_basis_digest,
            )?,
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn compacted_basis_digest(&self) -> &str {
        &self.compacted_basis_digest
    }
}
