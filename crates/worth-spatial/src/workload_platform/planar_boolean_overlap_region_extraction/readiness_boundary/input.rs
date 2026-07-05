use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;

use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;

pub struct PlanarBooleanOverlapRegionExtractionRequestInput<'a> {
    readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    loop_ledger_receipt: PlanarBooleanLoopReconstructionLedgerReceipt,
}

impl<'a> PlanarBooleanOverlapRegionExtractionRequestInput<'a> {
    pub fn from_readiness_consumer_and_loop_ledger(
        readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        loop_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
    ) -> Self {
        Self {
            readiness_consumer,
            loop_ledger_receipt: loop_ledger_receipt.clone(),
        }
    }

    pub(crate) fn readiness_consumer(
        &self,
    ) -> &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer {
        self.readiness_consumer
    }

    pub(crate) fn loop_ledger_receipt(&self) -> &PlanarBooleanLoopReconstructionLedgerReceipt {
        &self.loop_ledger_receipt
    }
}
