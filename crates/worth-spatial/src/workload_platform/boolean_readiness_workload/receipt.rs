use crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt;

use super::counters::PlanarBooleanReadinessWorkloadCounters;
use super::stage_coverage::PlanarBooleanReadinessStageCoverage;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanReadinessWorkloadReceipt {
    readiness_receipt: PlanarM7ReadinessReceipt,
    workload_digest: String,
    declaration: String,
    stage_coverage: PlanarBooleanReadinessStageCoverage,
    counters: PlanarBooleanReadinessWorkloadCounters,
}

impl PlanarBooleanReadinessWorkloadReceipt {
    pub(crate) fn new(
        readiness_receipt: PlanarM7ReadinessReceipt,
        workload_digest: String,
        declaration: String,
        stage_coverage: PlanarBooleanReadinessStageCoverage,
        counters: PlanarBooleanReadinessWorkloadCounters,
    ) -> Self {
        Self {
            readiness_receipt,
            workload_digest,
            declaration,
            stage_coverage,
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

    pub fn stage_coverage(&self) -> &PlanarBooleanReadinessStageCoverage {
        &self.stage_coverage
    }

    pub fn counters(&self) -> PlanarBooleanReadinessWorkloadCounters {
        self.counters
    }
}
