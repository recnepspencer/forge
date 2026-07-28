use crate::checkpoint_interlock::ReadDuringCheckpointVerdict;
use crate::compaction_interlock::ReadDuringCompactionVerdict;
use crate::epoch::PhysicalEpochComparisonEvidence;
use crate::latch::LatchOrderProof;
use crate::publication::PhysicalPublicationReceipt;
use crate::reclaim_reachability::ReclaimEligibilityProof;
use crate::stable_read_execution::StablePhysicalReadReceipt;

use super::basis::ExecutedIsolationBasis;
use crate::{ExecutedIsolationEvidenceDenial, PhysicalIsolationCounterSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedIsolationEvidence {
    basis: ExecutedIsolationBasis,
    counters: PhysicalIsolationCounterSnapshot,
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
    ) -> Result<Self, ExecutedIsolationEvidenceDenial> {
        assemble_executed_physical_isolation_closeout(receipts)
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn from_foreground_reservation_test_counts(
        wait_count: u64,
        retry_count: u64,
    ) -> Result<Self, ExecutedIsolationEvidenceDenial> {
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
        let proof_progression_identity =
            super::project_counters::foreground_reservation_test_progression_identity(counters);
        let basis =
            ExecutedIsolationBasis::from_executed_isolation(proof_progression_identity, counters);
        Ok(Self { basis, counters })
    }

    pub const fn basis(&self) -> ExecutedIsolationBasis {
        self.basis
    }

    pub const fn counters(&self) -> PhysicalIsolationCounterSnapshot {
        self.counters
    }
}

fn assemble_executed_physical_isolation_closeout(
    receipts: ExecutedIsolationReceipts<'_>,
) -> Result<ExecutedIsolationEvidence, ExecutedIsolationEvidenceDenial> {
    let _latch_order_proof = receipts.latch_order_proof;
    let counters = super::project_counters::project_closeout_counters(receipts)?;
    let identity = super::project_counters::proof_progression_identity(receipts, counters);
    let basis = ExecutedIsolationBasis::from_executed_isolation(identity, counters);
    Ok(ExecutedIsolationEvidence { basis, counters })
}
