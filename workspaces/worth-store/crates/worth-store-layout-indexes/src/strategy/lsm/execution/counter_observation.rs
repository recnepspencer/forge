#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmCounterObservation {
    snapshot: crate::AccessPathCounterSnapshot,
}

impl BaselineLsmCounterObservation {
    pub(super) const fn lookup(comparisons: u16) -> Self {
        Self {
            snapshot: crate::AccessPathCounterSnapshot::exact(
                1,
                0,
                0,
                0,
                0,
                0,
                comparisons,
                comparisons,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                comparisons,
                0,
            ),
        }
    }

    pub(in crate::strategy::lsm) const fn replay(
        replayed_records: u16,
        cleanup_batches: u16,
    ) -> Self {
        Self {
            snapshot: crate::AccessPathCounterSnapshot::exact(
                0,
                0,
                replayed_records,
                0,
                cleanup_batches,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ),
        }
    }

    pub(in crate::strategy::lsm) const fn manifest_publication(published_runs: u16) -> Self {
        Self {
            snapshot: crate::AccessPathCounterSnapshot::exact(
                0,
                0,
                0,
                2,
                published_runs,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ),
        }
    }

    pub(in crate::strategy::lsm) const fn compaction(retired_runs: u16) -> Self {
        Self {
            snapshot: crate::AccessPathCounterSnapshot::exact(
                0,
                0,
                0,
                1,
                retired_runs,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ),
        }
    }

    pub const fn point_lookups(self) -> u16 {
        self.snapshot.point_lookups()
    }

    pub const fn range_lookups(self) -> u16 {
        self.snapshot.range_lookups()
    }

    pub const fn wal_replays(self) -> u16 {
        self.snapshot.wal_replays()
    }

    pub const fn publications(self) -> u16 {
        self.snapshot.publications()
    }

    pub const fn maintenance_reads(self) -> u16 {
        self.snapshot.maintenance_reads()
    }

    pub const fn index_probes(self) -> u16 {
        self.snapshot.index_probes()
    }

    pub const fn key_comparisons(self) -> u16 {
        self.snapshot.key_comparisons()
    }

    pub const fn read_amplification(self) -> u16 {
        self.snapshot.read_amplification()
    }

    pub(super) const fn access_path_snapshot(self) -> crate::AccessPathCounterSnapshot {
        self.snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmLookupCounterReceipt {
    plan_binding: crate::AccessPlanIdentity,
    planned: crate::AccessPathCounterSnapshot,
    observed: crate::AccessPathCounterSnapshot,
    observation: crate::PlannedCounterObservation,
}

impl BaselineLsmLookupCounterReceipt {
    pub(super) fn issue(
        plan_binding: &crate::AccessPlanIdentity,
        observed: BaselineLsmCounterObservation,
    ) -> Result<Self, crate::CounterEnvelopeViolation> {
        let planned = plan_binding.planned_counter_envelope().lookup();
        let observed = observed
            .access_path_snapshot()
            .with_selected_plan_authority_allocation();
        let observation = planned.validate_observation(observed)?;
        Ok(Self {
            plan_binding: plan_binding.clone(),
            planned,
            observed,
            observation,
        })
    }

    pub const fn plan_binding(&self) -> &crate::AccessPlanIdentity {
        &self.plan_binding
    }

    pub const fn planned(&self) -> crate::AccessPathCounterSnapshot {
        self.planned
    }

    pub const fn observed(&self) -> crate::AccessPathCounterSnapshot {
        self.observed
    }

    pub const fn observation(&self) -> crate::PlannedCounterObservation {
        self.observation
    }
}
