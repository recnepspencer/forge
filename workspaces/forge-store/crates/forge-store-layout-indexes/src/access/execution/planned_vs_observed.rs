use super::{S8CostEnvelopeViolationOutcome, S8ObservedCounterMetric};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PlannedVsObservedCounterReceipt {
    fingerprint: crate::access::planning::S8PlanFingerprint,
    path_kind: crate::access::execution::S8AccessPathKind,
    planned: crate::access::execution::S8AccessPathCounterSnapshot,
    observed: crate::access::execution::S8AccessPathCounterSnapshot,
}

impl S8PlannedVsObservedCounterReceipt {
    pub fn from_executed(executed: &crate::access::execution::S8ExecutedAccessReceipt) -> Self {
        Self::new(
            executed.selected().fingerprint(),
            executed.path_kind(),
            executed.selected().planned_counter_envelope().lookup(),
            executed.observed(),
        )
    }

    pub(crate) const fn new(
        fingerprint: crate::access::planning::S8PlanFingerprint,
        path_kind: crate::access::execution::S8AccessPathKind,
        planned: crate::access::execution::S8AccessPathCounterSnapshot,
        observed: crate::access::execution::S8AccessPathCounterSnapshot,
    ) -> Self {
        Self {
            fingerprint,
            path_kind,
            planned,
            observed,
        }
    }

    pub const fn fingerprint(self) -> crate::access::planning::S8PlanFingerprint {
        self.fingerprint
    }

    pub const fn planned(self) -> crate::access::execution::S8AccessPathCounterSnapshot {
        self.planned
    }

    pub const fn observed(self) -> crate::access::execution::S8AccessPathCounterSnapshot {
        self.observed
    }

    pub const fn path_kind(self) -> crate::access::execution::S8AccessPathKind {
        self.path_kind
    }

    pub fn parity_holds(self) -> bool {
        self.planned() == self.observed() && self.violation_outcome().is_none()
    }

    pub fn violation_outcome(self) -> Option<S8CostEnvelopeViolationOutcome> {
        [
            (
                S8ObservedCounterMetric::PointLookups,
                self.planned.point_lookups() as u64,
                self.observed.point_lookups() as u64,
            ),
            (
                S8ObservedCounterMetric::RangeLookups,
                self.planned.range_lookups() as u64,
                self.observed.range_lookups() as u64,
            ),
            (
                S8ObservedCounterMetric::WalReplays,
                self.planned.wal_replays() as u64,
                self.observed.wal_replays() as u64,
            ),
            (
                S8ObservedCounterMetric::Publications,
                self.planned.publications() as u64,
                self.observed.publications() as u64,
            ),
            (
                S8ObservedCounterMetric::MaintenanceReads,
                self.planned.maintenance_reads() as u64,
                self.observed.maintenance_reads() as u64,
            ),
            (
                S8ObservedCounterMetric::PageTouches,
                self.planned.page_touches() as u64,
                self.observed.page_touches() as u64,
            ),
            (
                S8ObservedCounterMetric::IndexProbes,
                self.planned.index_probes() as u64,
                self.observed.index_probes() as u64,
            ),
            (
                S8ObservedCounterMetric::KeyComparisons,
                self.planned.key_comparisons() as u64,
                self.observed.key_comparisons() as u64,
            ),
            (
                S8ObservedCounterMetric::RangeSteps,
                self.planned.range_steps() as u64,
                self.observed.range_steps() as u64,
            ),
            (
                S8ObservedCounterMetric::PrefixSteps,
                self.planned.prefix_steps() as u64,
                self.observed.prefix_steps() as u64,
            ),
            (
                S8ObservedCounterMetric::ChunkTreeNodeReads,
                self.planned.chunk_tree_node_reads() as u64,
                self.observed.chunk_tree_node_reads() as u64,
            ),
            (
                S8ObservedCounterMetric::ManifestReads,
                self.planned.manifest_reads() as u64,
                self.observed.manifest_reads() as u64,
            ),
            (
                S8ObservedCounterMetric::BytesRead,
                self.planned.bytes_read(),
                self.observed.bytes_read(),
            ),
            (
                S8ObservedCounterMetric::BytesWritten,
                self.planned.bytes_written(),
                self.observed.bytes_written(),
            ),
            (
                S8ObservedCounterMetric::WriteFanout,
                self.planned.write_fanout() as u64,
                self.observed.write_fanout() as u64,
            ),
            (
                S8ObservedCounterMetric::ReadAmplification,
                self.planned.read_amplification() as u64,
                self.observed.read_amplification() as u64,
            ),
            (
                S8ObservedCounterMetric::WriteAmplification,
                self.planned.write_amplification() as u64,
                self.observed.write_amplification() as u64,
            ),
        ]
        .into_iter()
        .find_map(|(metric, planned, observed)| {
            (observed > planned).then_some(
                S8CostEnvelopeViolationOutcome::ObservedExceededPlanned {
                    metric,
                    planned,
                    observed,
                },
            )
        })
    }
}
