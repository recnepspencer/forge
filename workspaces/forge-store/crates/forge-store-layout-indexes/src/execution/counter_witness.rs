use forge_store_budgets::{CounterEvidenceStrength, S8PreExecutionPlanBinding};
use forge_store_physical_format::layout_access::baseline_btree_counter_observation::{
    BaselineBTreeExactCounterWitness, BaselineBTreeLookupExecution, BaselineBTreeReadShape,
    BaselineBTreeReplayRecoveryExecution, BaselineBTreeRootPublicationExecution,
};

use super::{
    S8AccessLoweringBasis, S8AccessPathCounterSnapshot, S8AccessPathKind,
    S8ObservedAccessPathCounters,
};
use crate::{S8AccessShapeDetail, S8PrefixBasis, S8RangeBasis};

mod private {
    pub trait Sealed {}

    impl Sealed for forge_store_physical_format::layout_access::baseline_btree_counter_observation::BaselineBTreeLookupExecution {}
    impl Sealed for forge_store_physical_format::layout_access::baseline_btree_counter_observation::BaselineBTreeRootPublicationExecution {}
    impl Sealed for forge_store_physical_format::layout_access::baseline_btree_counter_observation::BaselineBTreeReplayRecoveryExecution {}
}

pub trait S8ExecutedCounterWitness: private::Sealed {
    fn plan_binding(&self) -> S8PreExecutionPlanBinding;
    fn path_kind(&self) -> S8AccessPathKind;
    fn exact_snapshot(&self) -> S8AccessPathCounterSnapshot;
}

impl S8ExecutedCounterWitness for BaselineBTreeLookupExecution {
    fn plan_binding(&self) -> S8PreExecutionPlanBinding {
        BaselineBTreeLookupExecution::plan_binding(*self)
    }

    fn path_kind(&self) -> S8AccessPathKind {
        S8AccessPathKind::BaselineBTreeRead(match self.shape() {
            BaselineBTreeReadShape::PointLookup => S8AccessShapeDetail::PointLookup,
            BaselineBTreeReadShape::RangeLookup => {
                S8AccessShapeDetail::RangeLookup(S8RangeBasis::CanonicalRangeBounds)
            }
            BaselineBTreeReadShape::PrefixLookup => {
                S8AccessShapeDetail::PrefixLookup(S8PrefixBasis::CanonicalPrefixBounds)
            }
        })
    }

    fn exact_snapshot(&self) -> S8AccessPathCounterSnapshot {
        baseline_btree_snapshot(self.exact_counters())
    }
}

impl S8ExecutedCounterWitness for BaselineBTreeRootPublicationExecution {
    fn plan_binding(&self) -> S8PreExecutionPlanBinding {
        BaselineBTreeRootPublicationExecution::plan_binding(self)
    }

    fn path_kind(&self) -> S8AccessPathKind {
        S8AccessPathKind::BaselineBTreeRootPublication
    }

    fn exact_snapshot(&self) -> S8AccessPathCounterSnapshot {
        baseline_btree_snapshot(self.exact_counters())
    }
}

impl S8ExecutedCounterWitness for BaselineBTreeReplayRecoveryExecution {
    fn plan_binding(&self) -> S8PreExecutionPlanBinding {
        BaselineBTreeReplayRecoveryExecution::plan_binding(self)
    }

    fn path_kind(&self) -> S8AccessPathKind {
        S8AccessPathKind::BaselineBTreeReplayRecovery
    }

    fn exact_snapshot(&self) -> S8AccessPathCounterSnapshot {
        baseline_btree_snapshot(self.exact_counters())
    }
}

pub(crate) fn admit_execution_witness<W: S8ExecutedCounterWitness>(
    basis: S8AccessLoweringBasis,
    expected_plan_binding: S8PreExecutionPlanBinding,
    witness: &W,
) -> Result<S8ObservedAccessPathCounters, S8ObservedAccessPathCounters> {
    let observed = S8ObservedAccessPathCounters::admitted(
        S8AccessLoweringBasis::new(basis.fingerprint(), witness.path_kind()),
        witness.exact_snapshot(),
        CounterEvidenceStrength::Exact,
    );

    if witness.plan_binding() == expected_plan_binding && observed.basis().path_kind() == basis.path_kind() {
        Ok(observed)
    } else {
        Err(observed)
    }
}

pub(crate) fn exact_snapshot_from_witness<W: S8ExecutedCounterWitness>(
    witness: &W,
) -> S8AccessPathCounterSnapshot {
    witness.exact_snapshot()
}

const fn baseline_btree_snapshot(
    counters: BaselineBTreeExactCounterWitness,
) -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(
        counters.point_lookups(),
        counters.range_lookups(),
        counters.wal_replays(),
        counters.publications(),
        counters.maintenance_reads(),
        counters.page_touches(),
        counters.index_probes(),
        counters.key_comparisons(),
        counters.range_steps(),
        counters.prefix_steps(),
        counters.chunk_tree_node_reads(),
        counters.manifest_reads(),
        counters.bytes_read(),
        counters.bytes_written(),
        counters.write_fanout(),
        counters.read_amplification(),
        counters.write_amplification(),
    )
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TestExecutedCounterWitness {
    plan_binding: S8PreExecutionPlanBinding,
    path_kind: S8AccessPathKind,
    snapshot: S8AccessPathCounterSnapshot,
}

#[cfg(test)]
impl TestExecutedCounterWitness {
    pub(crate) const fn new(
        plan_binding: S8PreExecutionPlanBinding,
        path_kind: S8AccessPathKind,
        snapshot: S8AccessPathCounterSnapshot,
    ) -> Self {
        Self {
            plan_binding,
            path_kind,
            snapshot,
        }
    }
}

#[cfg(test)]
impl private::Sealed for TestExecutedCounterWitness {}

#[cfg(test)]
impl S8ExecutedCounterWitness for TestExecutedCounterWitness {
    fn plan_binding(&self) -> S8PreExecutionPlanBinding {
        self.plan_binding
    }

    fn path_kind(&self) -> S8AccessPathKind {
        self.path_kind
    }

    fn exact_snapshot(&self) -> S8AccessPathCounterSnapshot {
        self.snapshot
    }
}
