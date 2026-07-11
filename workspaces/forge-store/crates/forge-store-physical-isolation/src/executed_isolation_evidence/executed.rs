use crate::checkpoint_interlock::ReadDuringCheckpointVerdict;
use crate::compaction_interlock::ReadDuringCompactionVerdict;
use crate::epoch::PhysicalEpochComparisonEvidence;
use crate::latch::LatchOrderProof;
use crate::publication::PhysicalPublicationReceipt;
use crate::reclaim_reachability::ReclaimEligibilityProof;
use crate::stable_read_execution::StablePhysicalReadReceipt;

use crate::readiness::isolation_evidence::basis::{
    ExecutedIsolationBasis, FoundationalIsolationCounterReceipt,
};
use crate::{IsolationReadinessDenial, PhysicalIsolationCounterSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedIsolationEvidence {
    basis: ExecutedIsolationBasis,
    counters: PhysicalIsolationCounterSnapshot,
    foundational_counter_receipt: FoundationalIsolationCounterReceipt,
    proof_progression_identity: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutedIsolationReceipts<'a> {
    pub stable_read: StablePhysicalReadReceipt,
    pub latch_order_proof: &'a LatchOrderProof,
    pub epoch_freshness: &'a PhysicalEpochComparisonEvidence,
    pub publication: &'a PhysicalPublicationReceipt,
    pub reclaim: &'a ReclaimEligibilityProof,
    pub compaction: &'a ReadDuringCompactionVerdict,
    pub checkpoint: &'a ReadDuringCheckpointVerdict,
}

impl ExecutedIsolationEvidence {
    pub fn from_physical_isolation_receipts(
        receipts: ExecutedIsolationReceipts<'_>,
    ) -> Result<Self, IsolationReadinessDenial> {
        assemble_executed_s5_closeout(receipts)
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn from_foreground_reservation_test_counts(
        wait_count: u64,
        retry_count: u64,
    ) -> Result<Self, IsolationReadinessDenial> {
        let counters = PhysicalIsolationCounterSnapshot::from_store_executed_counts(
            1,
            wait_count,
            retry_count,
            1,
            1,
            1,
            1,
            1,
            4096,
        )?;
        let foundational_counter_receipt =
            super::performance_receipt::construct_s6_foundational_counter_receipt(counters)?;
        let proof_progression_identity =
            super::project_counters::foreground_reservation_test_progression_identity(counters);
        let basis =
            ExecutedIsolationBasis::from_executed_isolation(proof_progression_identity, counters);
        Ok(Self {
            basis,
            counters,
            foundational_counter_receipt,
            proof_progression_identity,
        })
    }

    pub const fn basis(&self) -> ExecutedIsolationBasis {
        self.basis
    }

    pub const fn counters(&self) -> PhysicalIsolationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn foundational_counter_receipt(
        &self,
    ) -> &FoundationalIsolationCounterReceipt {
        &self.foundational_counter_receipt
    }

    pub(crate) const fn proof_progression_identity(&self) -> u64 {
        self.proof_progression_identity
    }
}

fn assemble_executed_s5_closeout(
    receipts: ExecutedIsolationReceipts<'_>,
) -> Result<ExecutedIsolationEvidence, IsolationReadinessDenial> {
    let _latch_order_proof = receipts.latch_order_proof;
    let counters = super::project_counters::project_closeout_counters(receipts)?;
    let foundational_counter_receipt =
        super::performance_receipt::construct_s6_foundational_counter_receipt(counters)?;
    let identity = super::project_counters::proof_progression_identity(receipts, counters);
    let basis = ExecutedIsolationBasis::from_executed_isolation(identity, counters);
    Ok(ExecutedIsolationEvidence {
        basis,
        counters,
        foundational_counter_receipt,
        proof_progression_identity: identity,
    })
}
