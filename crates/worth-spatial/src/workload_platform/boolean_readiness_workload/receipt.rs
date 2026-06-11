use crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt;

use super::counters::PlanarBooleanReadinessWorkloadCounters;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanReadinessWorkloadReceipt {
    readiness_receipt: PlanarM7ReadinessReceipt,
    workload_digest: String,
    declaration: String,
    counters: PlanarBooleanReadinessWorkloadCounters,
}

impl PlanarBooleanReadinessWorkloadReceipt {
    pub(crate) fn new(
        readiness_receipt: PlanarM7ReadinessReceipt,
        workload_digest: String,
        declaration: String,
        counters: PlanarBooleanReadinessWorkloadCounters,
    ) -> Self {
        Self {
            readiness_receipt,
            workload_digest,
            declaration,
            counters,
        }
    }

    pub fn m7_readiness_receipt(&self) -> &PlanarM7ReadinessReceipt {
        &self.readiness_receipt
    }

    pub fn workload_digest(&self) -> &str {
        &self.workload_digest
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn counters(&self) -> PlanarBooleanReadinessWorkloadCounters {
        self.counters
    }
}
