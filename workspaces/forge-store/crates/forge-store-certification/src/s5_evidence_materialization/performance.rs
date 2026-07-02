use forge_foundational::FoundationalAuthoritativePerformanceClaim;
use forge_foundational::FoundationalCounterBackedPerformanceReceipt;

use crate::foundational_boundary_performance::counter_receipt;

use super::{S5ExecutedIsolationFinding, S5ExecutedIsolationRequiredCounters};

type Receipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5FoundationalPerformanceReceipts {
    required: Receipt,
}

impl S5FoundationalPerformanceReceipts {
    pub(crate) fn from_finding(
        finding: &S5ExecutedIsolationFinding,
    ) -> Result<Self, crate::FoundationalBoundaryEvidenceDenial> {
        let counters = finding.counters();
        let rows = [
            ("store.s5.isolation.outcome", counters.outcome_count()),
            ("store.s5.isolation.retry", counters.retry_count()),
            ("store.s5.isolation.latch", counters.latch_count()),
            ("store.s5.isolation.reclaim", counters.reclaim_count()),
        ];
        Ok(Self {
            required: counter_receipt("store.s5.executed_isolation", &rows)?,
        })
    }

    pub const fn required_counter_receipt(&self) -> &Receipt {
        &self.required
    }

    pub fn preserves_required_counters(&self) -> bool {
        let names = self
            .required
            .counter_rows()
            .iter()
            .map(|row| row.name().as_str())
            .collect::<Vec<_>>();
        [
            "store.s5.isolation.outcome",
            "store.s5.isolation.retry",
            "store.s5.isolation.latch",
            "store.s5.isolation.reclaim",
        ]
        .into_iter()
        .all(|required| names.contains(&required))
    }

    pub fn matches_required_counters(&self, required: S5ExecutedIsolationRequiredCounters) -> bool {
        [
            ("store.s5.isolation.outcome", required.outcome_count()),
            ("store.s5.isolation.retry", required.retry_count()),
            ("store.s5.isolation.latch", required.latch_count()),
            ("store.s5.isolation.reclaim", required.reclaim_count()),
        ]
        .into_iter()
        .all(|(name, count)| self.counter_value(name) == Some(count))
    }

    fn counter_value(&self, name: &str) -> Option<u64> {
        self.required
            .counter_rows()
            .iter()
            .find(|row| row.name().as_str() == name)
            .map(|row| row.observed_count())
    }
}
