use super::ConsumedProjectionAuthorityCounters;
use crate::projection_consumption::ProjectionSourceFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedProjectionAuthorityEvidence {
    source_family: ProjectionSourceFamily,
    source_identity_projection: String,
    receipt_identity_projection: String,
    counters: ConsumedProjectionAuthorityCounters,
}

impl ConsumedProjectionAuthorityEvidence {
    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn source_identity_projection(&self) -> &str {
        &self.source_identity_projection
    }

    pub fn receipt_identity_projection(&self) -> &str {
        &self.receipt_identity_projection
    }

    pub fn counters(&self) -> &ConsumedProjectionAuthorityCounters {
        &self.counters
    }

    pub(super) fn new(
        source_family: ProjectionSourceFamily,
        source_identity_projection: String,
        receipt_identity_projection: String,
        counters: ConsumedProjectionAuthorityCounters,
    ) -> Self {
        Self {
            source_family,
            source_identity_projection,
            receipt_identity_projection,
            counters,
        }
    }
}
