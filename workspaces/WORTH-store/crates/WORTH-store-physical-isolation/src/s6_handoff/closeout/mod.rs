mod performance_receipt;
mod project_counters;

use crate::checkpoint_interlock::ReadDuringCheckpointVerdict;
use crate::compaction_interlock::ReadDuringCompactionVerdict;
use crate::epoch::PhysicalEpochComparisonEvidence;
use crate::latch::LatchOrderProof;
use crate::publication::PhysicalPublicationReceipt;
use crate::reclaim_reachability::ReclaimEligibilityProof;
use crate::stable_read_execution::StablePhysicalReadReceipt;

use super::basis::S6FoundationalCounterReceipt;
use super::{PhysicalIsolationCounterSnapshot, S5PhysicalIsolationCloseoutBasis, S6IoQosIsolationReadinessDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedS5IsolationCloseout {
    basis: S5PhysicalIsolationCloseoutBasis,
    counters: PhysicalIsolationCounterSnapshot,
    foundational_counter_receipt: S6FoundationalCounterReceipt,
    proof_progression_identity: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutedS5IsolationCloseoutReceipts<'a> {
    pub stable_read: StablePhysicalReadReceipt,
    pub latch_order_proof: &'a LatchOrderProof,
    pub epoch_freshness: &'a PhysicalEpochComparisonEvidence,
    pub publication: &'a PhysicalPublicationReceipt,
    pub reclaim: &'a ReclaimEligibilityProof,
    pub compaction: &'a ReadDuringCompactionVerdict,
    pub checkpoint: &'a ReadDuringCheckpointVerdict,
}

impl ExecutedS5IsolationCloseout {
    pub fn from_physical_isolation_receipts(
        receipts: ExecutedS5IsolationCloseoutReceipts<'_>,
    ) -> Result<Self, S6IoQosIsolationReadinessDenial> {
        assemble_executed_s5_closeout(receipts)
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn from_foreground_reservation_test_counts(
        wait_count: u64,
        retry_count: u64,
    ) -> Result<Self, S6IoQosIsolationReadinessDenial> {
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
            performance_receipt::construct_s6_foundational_counter_receipt(counters)?;
        let proof_progression_identity =
            project_counters::foreground_reservation_test_progression_identity(counters);
        let basis = S5PhysicalIsolationCloseoutBasis::from_executed_isolation(
            proof_progression_identity,
            counters,
        );
        Ok(Self {
            basis,
            counters,
            foundational_counter_receipt,
            proof_progression_identity,
        })
    }

    pub const fn basis(&self) -> S5PhysicalIsolationCloseoutBasis {
        self.basis
    }

    pub const fn counters(&self) -> PhysicalIsolationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn foundational_counter_receipt(&self) -> &S6FoundationalCounterReceipt {
        &self.foundational_counter_receipt
    }

    pub(crate) const fn proof_progression_identity(&self) -> u64 {
        self.proof_progression_identity
    }
}

fn assemble_executed_s5_closeout(
    receipts: ExecutedS5IsolationCloseoutReceipts<'_>,
) -> Result<ExecutedS5IsolationCloseout, S6IoQosIsolationReadinessDenial> {
    let _latch_order_proof = receipts.latch_order_proof;
    let counters = project_counters::project_closeout_counters(receipts)?;
    let foundational_counter_receipt =
        performance_receipt::construct_s6_foundational_counter_receipt(counters)?;
    let identity = project_counters::proof_progression_identity(receipts, counters);
    let basis = S5PhysicalIsolationCloseoutBasis::from_executed_isolation(identity, counters);
    Ok(ExecutedS5IsolationCloseout {
        basis,
        counters,
        foundational_counter_receipt,
        proof_progression_identity: identity,
    })
}