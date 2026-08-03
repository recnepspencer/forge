use worth_store::physical_runtime::{
    PhysicalResidencyCounterSnapshot, PhysicalWorkCounterSnapshot, PhysicalWorkCounterStage,
    PhysicalWorkOperationFamily, ServingPhysicalRuntime,
};

use super::super::super::{
    configuration::{BoundedResidencyConfiguration, STREAMING_SCRATCH_BYTES},
    read_pressure::{
        media_observation::{metadata_reads, positioned_reads},
        work_accounting::WorkCounterDelta,
    },
};
use super::{
    pressure_schedule::ReadPressureSchedule, record_copy::largest_record_bytes,
    BoundedReadPressureEvidence,
};

pub(super) struct ReadPressureBaseline {
    residency: PhysicalResidencyCounterSnapshot,
    work: PhysicalWorkCounterSnapshot,
    positioned_effects: u64,
    metadata_effects: u64,
}

struct ResidencyDeltas {
    faults: u64,
    source_loads: u64,
    hits: u64,
    evictions: u64,
}

struct CopyReconciliation {
    store_operations: u64,
    store_bytes: u64,
    store_maximum_width: u64,
    largest_record_bytes: u64,
}

struct WorkReconciliation {
    metadata_declared: u64,
    metadata_dispatched: u64,
    metadata_terminal: u64,
    range_declared: u64,
    range_dispatched: u64,
    range_terminal: u64,
}

struct ReconciledReadEvidence {
    counters: PhysicalResidencyCounterSnapshot,
    residency: ResidencyDeltas,
    copies: CopyReconciliation,
    work: WorkReconciliation,
    positioned_effects: u64,
    metadata_effects: u64,
}

impl ReadPressureBaseline {
    pub(super) fn capture(serving: &ServingPhysicalRuntime) -> Self {
        Self {
            residency: serving.residency_observation().counters(),
            work: serving.physical_work_counters(),
            positioned_effects: positioned_reads(serving),
            metadata_effects: metadata_reads(serving),
        }
    }
}

pub(super) fn reconcile(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    baseline: ReadPressureBaseline,
    schedule: ReadPressureSchedule,
) -> Result<BoundedReadPressureEvidence, String> {
    let counters = serving.residency_observation().counters();
    let residency = reconcile_residency(counters, baseline.residency, configuration)?;
    let copies = reconcile_copies(counters, baseline.residency, configuration, &schedule)?;
    let work = reconcile_work(baseline.work, serving.physical_work_counters())?;
    if schedule.work_span.count() != schedule.read_work {
        return Err("C.6 read observations omitted canonical work identities".to_owned());
    }
    let positioned_effects = delta(
        positioned_reads(serving),
        baseline.positioned_effects,
        "read-scenario positioned reads",
    )?;
    let metadata_effects = delta(
        metadata_reads(serving),
        baseline.metadata_effects,
        "read-scenario metadata reads",
    )?;
    Ok(evidence(
        schedule,
        ReconciledReadEvidence {
            counters,
            residency,
            copies,
            work,
            positioned_effects,
            metadata_effects,
        },
    ))
}

fn reconcile_residency(
    after: PhysicalResidencyCounterSnapshot,
    before: PhysicalResidencyCounterSnapshot,
    configuration: BoundedResidencyConfiguration,
) -> Result<ResidencyDeltas, String> {
    let evictions = delta(after.evictions(), before.evictions(), "residency evictions")?;
    if evictions == 0 || after.peak_resident_bytes() > configuration.resident_bytes() {
        return Err("C.6 read pressure escaped its residency bound".to_owned());
    }
    Ok(ResidencyDeltas {
        faults: delta(after.faults(), before.faults(), "residency faults")?,
        source_loads: delta(
            after.source_loads(),
            before.source_loads(),
            "residency source loads",
        )?,
        hits: delta(after.hits(), before.hits(), "residency hits")?,
        evictions,
    })
}

fn reconcile_copies(
    after: PhysicalResidencyCounterSnapshot,
    before: PhysicalResidencyCounterSnapshot,
    configuration: BoundedResidencyConfiguration,
    schedule: &ReadPressureSchedule,
) -> Result<CopyReconciliation, String> {
    let store_operations = delta(
        after.copy_operations(),
        before.copy_operations(),
        "Store copy operations",
    )?;
    let store_bytes = delta(
        after.copied_bytes(),
        before.copied_bytes(),
        "Store copied bytes",
    )?;
    let store_maximum_width = after.maximum_copy_width();
    let largest_record_bytes = largest_record_bytes(configuration);
    if store_operations != schedule.copies.operations()
        || store_bytes != schedule.copies.bytes()
        || schedule.copies.maximum_width() > STREAMING_SCRATCH_BYTES as u64
        || store_maximum_width != schedule.copies.maximum_width()
        || largest_record_bytes <= STREAMING_SCRATCH_BYTES as u64
    {
        return Err("C.6 bounded-copy observations did not reconcile".to_owned());
    }
    Ok(CopyReconciliation {
        store_operations,
        store_bytes,
        store_maximum_width,
        largest_record_bytes,
    })
}

fn reconcile_work(
    before: PhysicalWorkCounterSnapshot,
    after: PhysicalWorkCounterSnapshot,
) -> Result<WorkReconciliation, String> {
    let work = WorkCounterDelta::new(before, after);
    Ok(WorkReconciliation {
        metadata_declared: count(
            &work,
            PhysicalWorkOperationFamily::ArtifactMetadataRead,
            PhysicalWorkCounterStage::Declared,
        )?,
        metadata_dispatched: count(
            &work,
            PhysicalWorkOperationFamily::ArtifactMetadataRead,
            PhysicalWorkCounterStage::Dispatched,
        )?,
        metadata_terminal: count(
            &work,
            PhysicalWorkOperationFamily::ArtifactMetadataRead,
            PhysicalWorkCounterStage::Terminal,
        )?,
        range_declared: count(
            &work,
            PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkCounterStage::Declared,
        )?,
        range_dispatched: count(
            &work,
            PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkCounterStage::Dispatched,
        )?,
        range_terminal: count(
            &work,
            PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkCounterStage::Terminal,
        )?,
    })
}

fn count(
    work: &WorkCounterDelta,
    family: PhysicalWorkOperationFamily,
    stage: PhysicalWorkCounterStage,
) -> Result<u64, String> {
    work.count(family, stage)
}

fn evidence(
    schedule: ReadPressureSchedule,
    reconciled: ReconciledReadEvidence,
) -> BoundedReadPressureEvidence {
    BoundedReadPressureEvidence {
        cold_read_effects: schedule.cold.positioned_effects,
        hot_read_effects: schedule.hot.positioned_effects,
        refault_effects: schedule.refault.positioned_effects,
        cold_metadata_effects: schedule.cold.metadata_effects,
        hot_metadata_effects: schedule.hot.metadata_effects,
        refault_metadata_effects: schedule.refault.metadata_effects,
        cold_read_work: schedule.cold.copy.observation.physical_work_count(),
        hot_read_work: schedule.hot.copy.observation.physical_work_count(),
        refault_work: schedule.refault.copy.observation.physical_work_count(),
        read_work: schedule.read_work,
        positioned_read_effects: reconciled.positioned_effects,
        metadata_read_effects: reconciled.metadata_effects,
        metadata_read_work_declared: reconciled.work.metadata_declared,
        metadata_read_work_dispatched: reconciled.work.metadata_dispatched,
        metadata_read_work_terminal: reconciled.work.metadata_terminal,
        range_read_work_declared: reconciled.work.range_declared,
        range_read_work_dispatched: reconciled.work.range_dispatched,
        range_read_work_terminal: reconciled.work.range_terminal,
        first_operation: schedule
            .work_span
            .first()
            .map_or(0, |identity| identity.operation().get()),
        last_operation: schedule
            .work_span
            .last()
            .map_or(0, |identity| identity.operation().get()),
        runtime_bound: schedule.work_span.first().is_some() && schedule.work_span.last().is_some(),
        peak_resident_bytes: reconciled.counters.peak_resident_bytes(),
        peak_admitted_bytes: reconciled.counters.peak_admitted_bytes(),
        faults: reconciled.residency.faults,
        source_loads: reconciled.residency.source_loads,
        hits: reconciled.residency.hits,
        evictions: reconciled.residency.evictions,
        caller_copy_operations: schedule.copies.operations(),
        caller_copied_bytes: schedule.copies.bytes(),
        store_copy_operations: reconciled.copies.store_operations,
        store_copied_bytes: reconciled.copies.store_bytes,
        peak_copy_width: schedule.copies.maximum_width(),
        store_maximum_copy_width: reconciled.copies.store_maximum_width,
        streaming_scratch_bytes: STREAMING_SCRATCH_BYTES as u64,
        largest_record_bytes: reconciled.copies.largest_record_bytes,
    }
}

fn delta(after: u64, before: u64, label: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("C.6 {label} counter regressed"))
}
