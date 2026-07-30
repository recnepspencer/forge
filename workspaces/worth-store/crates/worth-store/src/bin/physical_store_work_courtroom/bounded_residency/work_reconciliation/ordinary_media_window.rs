use worth_store::physical_runtime::ServingPhysicalRuntime;
use worth_store_physical_backend::{MediaCounterSnapshot, MediaOperationRole};

use super::PhysicalWorkReconciliationBasis;

pub(in crate::bounded_residency) struct PhysicalWorkReconciliationWindow {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    runtime: worth_store::physical_runtime::RuntimeIdentity,
    generation: worth_store::physical_runtime::LifecycleGeneration,
    faults: u64,
    source_loads: u64,
    exact_writebacks: u64,
    media: MediaCounterSnapshot,
}

impl PhysicalWorkReconciliationWindow {
    pub(in crate::bounded_residency) fn begin(
        serving: &ServingPhysicalRuntime,
    ) -> Result<Self, String> {
        let residency = serving.residency_observation();
        let media = serving.media_counters();
        require_unsaturated(media)?;
        Ok(Self {
            store: serving.store_identity(),
            runtime: serving.runtime_identity(),
            generation: residency.store_generation(),
            faults: residency.counters().faults(),
            source_loads: residency.counters().source_loads(),
            exact_writebacks: residency.writebacks().exact_receipts(),
            media,
        })
    }

    pub(in crate::bounded_residency) fn finish(
        self,
        serving: &ServingPhysicalRuntime,
    ) -> Result<PhysicalWorkReconciliationBasis, String> {
        let residency = serving.residency_observation();
        if serving.store_identity() != self.store
            || serving.runtime_identity() != self.runtime
            || residency.store_generation() != self.generation
        {
            return Err("physical work reconciliation window changed runtime identity".to_owned());
        }
        let media = serving.media_counters();
        require_unsaturated(media)?;
        Ok(PhysicalWorkReconciliationBasis {
            store: self.store,
            runtime: self.runtime,
            generation: self.generation,
            faults: delta(residency.counters().faults(), self.faults, "faults")?,
            source_loads: delta(
                residency.counters().source_loads(),
                self.source_loads,
                "source loads",
            )?,
            exact_writebacks: delta(
                residency.writebacks().exact_receipts(),
                self.exact_writebacks,
                "exact writebacks",
            )?,
            identified_metadata_reads: delta(
                media.identified_operation_attempts_for(MediaOperationRole::ReadMetadata),
                self.media
                    .identified_operation_attempts_for(MediaOperationRole::ReadMetadata),
                "identified metadata reads",
            )?,
            identified_positioned_reads: delta(
                media.identified_operation_attempts_for(MediaOperationRole::PositionedRead),
                self.media
                    .identified_operation_attempts_for(MediaOperationRole::PositionedRead),
                "identified positioned reads",
            )?,
            identified_positioned_writes: delta(
                media.identified_operation_attempts_for(MediaOperationRole::PositionedWrite),
                self.media
                    .identified_operation_attempts_for(MediaOperationRole::PositionedWrite),
                "identified positioned writes",
            )?,
            signal_bindings: super::signal_basis::observe(serving),
        })
    }
}

fn delta(after: u64, before: u64, label: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("physical work reconciliation {label} counter regressed"))
}

fn require_unsaturated(media: MediaCounterSnapshot) -> Result<(), String> {
    if media.saturated() {
        return Err("physical work reconciliation media counters saturated".to_owned());
    }
    Ok(())
}
