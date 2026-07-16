use super::{UiAllocationReceiptGeneration, UiAllocationReceiptIdentity, UiAllocationReuseVerdict};

/// Whether the prior committed receipt remains usable while this lineage advances.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationReceiptFreshnessPosture {
    Current,
    RecomputePending,
}

/// Immutable commitment lineage, including ordinary freshness posture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptReport {
    receipt_identity: UiAllocationReceiptIdentity,
    receipt_generation: UiAllocationReceiptGeneration,
    reuse_verdict: UiAllocationReuseVerdict,
    freshness: UiAllocationReceiptFreshnessPosture,
}
impl UiAllocationReceiptReport {
    pub(crate) fn new(
        receipt_identity: UiAllocationReceiptIdentity,
        receipt_generation: UiAllocationReceiptGeneration,
        reuse_verdict: UiAllocationReuseVerdict,
    ) -> Self {
        let freshness = match reuse_verdict {
            UiAllocationReuseVerdict::StructureReuseLeafRemeasure(_) => {
                UiAllocationReceiptFreshnessPosture::RecomputePending
            }
            UiAllocationReuseVerdict::NewCommit
            | UiAllocationReuseVerdict::FullReuse
            | UiAllocationReuseVerdict::Denied(_) => UiAllocationReceiptFreshnessPosture::Current,
        };
        Self {
            receipt_identity,
            receipt_generation,
            reuse_verdict,
            freshness,
        }
    }
    pub fn receipt_identity(&self) -> &UiAllocationReceiptIdentity {
        &self.receipt_identity
    }
    pub fn receipt_generation(&self) -> UiAllocationReceiptGeneration {
        self.receipt_generation
    }
    pub fn reuse_verdict(&self) -> &UiAllocationReuseVerdict {
        &self.reuse_verdict
    }
    pub fn freshness(&self) -> UiAllocationReceiptFreshnessPosture {
        self.freshness
    }
}
