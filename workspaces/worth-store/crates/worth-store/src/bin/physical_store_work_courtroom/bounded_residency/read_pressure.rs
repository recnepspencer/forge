use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalRecordId, PhysicalWorkCounterSnapshot,
    PhysicalWorkCounterStage, PhysicalWorkIdentity, PhysicalWorkOperationFamily, RecordByteLimit,
    RecordReadLimits, RecordReadObservation, ServingPhysicalRuntime,
};
use worth_store_physical_backend::MediaOperationRole;

use super::BoundedResidencyConfiguration;

mod cancellation;
mod pin_pressure;

pub(super) use cancellation::{prove_cancellation, ResidencyCancellationEvidence};
pub(super) use pin_pressure::{prove_pins, PinnedFramePressureEvidence};

pub(super) struct BoundedReadPressureEvidence {
    pub(super) cold_read_effects: u64,
    pub(super) hot_read_effects: u64,
    pub(super) refault_effects: u64,
    pub(super) cold_metadata_effects: u64,
    pub(super) hot_metadata_effects: u64,
    pub(super) refault_metadata_effects: u64,
    pub(super) cold_read_work: u64,
    pub(super) hot_read_work: u64,
    pub(super) refault_work: u64,
    pub(super) read_work: u64,
    pub(super) positioned_read_effects: u64,
    pub(super) metadata_read_effects: u64,
    pub(super) metadata_read_work_declared: u64,
    pub(super) metadata_read_work_dispatched: u64,
    pub(super) metadata_read_work_terminal: u64,
    pub(super) range_read_work_declared: u64,
    pub(super) range_read_work_dispatched: u64,
    pub(super) range_read_work_terminal: u64,
    pub(super) first_operation: u64,
    pub(super) last_operation: u64,
    pub(super) runtime_bound: bool,
    pub(super) peak_resident_bytes: u64,
    pub(super) peak_admitted_bytes: u64,
    pub(super) faults: u64,
    pub(super) source_loads: u64,
    pub(super) hits: u64,
    pub(super) evictions: u64,
}

#[derive(Default)]
struct ReadWorkIdentitySpan {
    count: u64,
    first: Option<PhysicalWorkIdentity>,
    last: Option<PhysicalWorkIdentity>,
}

struct WorkCounterDelta {
    before: PhysicalWorkCounterSnapshot,
    after: PhysicalWorkCounterSnapshot,
}

pub(super) fn prove_reads(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    oracle: &[u8],
    configuration: BoundedResidencyConfiguration,
) -> Result<BoundedReadPressureEvidence, String> {
    serving
        .certification_physical_residency()
        .drain_unpinned_clean_frames();
    let before_residency = serving.residency_observation().counters();
    let before_work = serving.physical_work_counters();
    let before_read_effects = positioned_reads(serving);
    let before_metadata_effects = metadata_reads(serving);
    let mut work_span = ReadWorkIdentitySpan::default();
    let store = serving.store_identity();
    let first = ExternalPhysicalRecordLocator::new(store, records[0]);
    let before_cold = positioned_reads(serving);
    let before_cold_metadata = metadata_reads(serving);
    let cold = read_record(serving, first, expected(oracle, configuration, 0)?)?;
    work_span.observe(serving, cold)?;
    let cold_read_effects = counter_delta(
        positioned_reads(serving),
        before_cold,
        "cold positioned reads",
    )?;
    let cold_metadata_effects = counter_delta(
        metadata_reads(serving),
        before_cold_metadata,
        "cold metadata reads",
    )?;
    if cold_read_effects == 0 || cold.physical_work_count() == 0 {
        return Err("cold C.6 read did not enter canonical physical work".to_owned());
    }
    let before_hot = positioned_reads(serving);
    let before_hot_metadata = metadata_reads(serving);
    let hot = read_record(serving, first, expected(oracle, configuration, 0)?)?;
    work_span.observe(serving, hot)?;
    let hot_read_effects = counter_delta(
        positioned_reads(serving),
        before_hot,
        "hot positioned reads",
    )?;
    let hot_metadata_effects = counter_delta(
        metadata_reads(serving),
        before_hot_metadata,
        "hot metadata reads",
    )?;
    if hot_read_effects != 0 || hot_metadata_effects != 0 || hot.physical_work_count() != 0 {
        return Err("hot C.6 read repeated physical work or a media effect".to_owned());
    }

    let mut read_work = cold
        .physical_work_count()
        .saturating_add(hot.physical_work_count());
    for (ordinal, record) in records.iter().copied().enumerate().skip(1) {
        let locator = ExternalPhysicalRecordLocator::new(store, record);
        let observation = read_record(serving, locator, expected(oracle, configuration, ordinal)?)?;
        work_span.observe(serving, observation)?;
        read_work = read_work.saturating_add(observation.physical_work_count());
    }
    let before_refault = positioned_reads(serving);
    let before_refault_metadata = metadata_reads(serving);
    let refault = read_record(serving, first, expected(oracle, configuration, 0)?)?;
    work_span.observe(serving, refault)?;
    let refault_effects = counter_delta(
        positioned_reads(serving),
        before_refault,
        "refault positioned reads",
    )?;
    let refault_metadata_effects = counter_delta(
        metadata_reads(serving),
        before_refault_metadata,
        "refault metadata reads",
    )?;
    if refault_effects == 0 || refault.physical_work_count() == 0 {
        return Err("C.6 pressure did not refault the first record".to_owned());
    }
    read_work = read_work.saturating_add(refault.physical_work_count());
    let counters = serving.residency_observation().counters();
    let work = WorkCounterDelta {
        before: before_work,
        after: serving.physical_work_counters(),
    };
    let faults = counter_delta(
        counters.faults(),
        before_residency.faults(),
        "residency faults",
    )?;
    let source_loads = counter_delta(
        counters.source_loads(),
        before_residency.source_loads(),
        "residency source loads",
    )?;
    let hits = counter_delta(counters.hits(), before_residency.hits(), "residency hits")?;
    let evictions = counter_delta(
        counters.evictions(),
        before_residency.evictions(),
        "residency evictions",
    )?;
    if evictions == 0 || counters.peak_resident_bytes() > configuration.resident_bytes() {
        return Err("C.6 read pressure escaped its residency bound".to_owned());
    }
    if work_span.count != read_work {
        return Err("C.6 read observations omitted canonical work identities".to_owned());
    }
    Ok(BoundedReadPressureEvidence {
        cold_read_effects,
        hot_read_effects,
        refault_effects,
        cold_metadata_effects,
        hot_metadata_effects,
        refault_metadata_effects,
        cold_read_work: cold.physical_work_count(),
        hot_read_work: hot.physical_work_count(),
        refault_work: refault.physical_work_count(),
        read_work,
        positioned_read_effects: counter_delta(
            positioned_reads(serving),
            before_read_effects,
            "read-scenario positioned reads",
        )?,
        metadata_read_effects: counter_delta(
            metadata_reads(serving),
            before_metadata_effects,
            "read-scenario metadata reads",
        )?,
        metadata_read_work_declared: work.count(
            PhysicalWorkOperationFamily::ArtifactMetadataRead,
            PhysicalWorkCounterStage::Declared,
        )?,
        metadata_read_work_dispatched: work.count(
            PhysicalWorkOperationFamily::ArtifactMetadataRead,
            PhysicalWorkCounterStage::Dispatched,
        )?,
        metadata_read_work_terminal: work.count(
            PhysicalWorkOperationFamily::ArtifactMetadataRead,
            PhysicalWorkCounterStage::Terminal,
        )?,
        range_read_work_declared: work.count(
            PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkCounterStage::Declared,
        )?,
        range_read_work_dispatched: work.count(
            PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkCounterStage::Dispatched,
        )?,
        range_read_work_terminal: work.count(
            PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkCounterStage::Terminal,
        )?,
        first_operation: work_span
            .first
            .map_or(0, |identity| identity.operation().get()),
        last_operation: work_span
            .last
            .map_or(0, |identity| identity.operation().get()),
        runtime_bound: work_span.first.is_some() && work_span.last.is_some(),
        peak_resident_bytes: counters.peak_resident_bytes(),
        peak_admitted_bytes: counters.peak_admitted_bytes(),
        faults,
        source_loads,
        hits,
        evictions,
    })
}

fn require_bound_work(
    serving: &ServingPhysicalRuntime,
    lease: &worth_store::physical_runtime::CertificationResidentFrame,
    label: &str,
) -> Result<(), String> {
    if lease.physical_work_count() != 1
        || lease
            .first_physical_work()
            .is_none_or(|identity| !work_belongs_to_runtime(serving, identity))
        || lease.first_physical_work() != lease.last_physical_work()
    {
        return Err(format!(
            "{label} did not retain one runtime-bound work identity"
        ));
    }
    Ok(())
}

fn work_belongs_to_runtime(
    serving: &ServingPhysicalRuntime,
    identity: worth_store::physical_runtime::PhysicalWorkIdentity,
) -> bool {
    identity.store() == serving.store_identity()
        && identity.runtime() == serving.runtime_identity()
        && identity.generation().lifecycle() == serving.residency_observation().store_generation()
}

impl ReadWorkIdentitySpan {
    fn observe(
        &mut self,
        serving: &ServingPhysicalRuntime,
        observation: RecordReadObservation,
    ) -> Result<(), String> {
        let count = observation.physical_work_count();
        let endpoints = (
            observation.first_physical_work(),
            observation.last_physical_work(),
        );
        let (Some(first), Some(last)) = endpoints else {
            return if count == 0 {
                Ok(())
            } else {
                Err("C.6 read work omitted an identity endpoint".to_owned())
            };
        };
        if count == 0
            || !work_belongs_to_runtime(serving, first)
            || !work_belongs_to_runtime(serving, last)
            || operation_span(first, last) != Some(count)
            || self.last.is_some_and(|previous| {
                previous.operation().get().checked_add(1) != Some(first.operation().get())
            })
        {
            return Err("C.6 read work identities were foreign or discontinuous".to_owned());
        }
        self.count = self.count.saturating_add(count);
        self.first = self.first.or(Some(first));
        self.last = Some(last);
        Ok(())
    }
}

impl WorkCounterDelta {
    fn count(
        &self,
        family: PhysicalWorkOperationFamily,
        stage: PhysicalWorkCounterStage,
    ) -> Result<u64, String> {
        self.after
            .count(family, stage)
            .checked_sub(self.before.count(family, stage))
            .ok_or_else(|| format!("C.6 {family:?} {stage:?} counter regressed"))
    }
}

fn operation_span(first: PhysicalWorkIdentity, last: PhysicalWorkIdentity) -> Option<u64> {
    last.operation()
        .get()
        .checked_sub(first.operation().get())
        .and_then(|difference| difference.checked_add(1))
}

fn counter_delta(after: u64, before: u64, label: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("C.6 {label} counter regressed"))
}

fn read_record(
    serving: &ServingPhysicalRuntime,
    locator: ExternalPhysicalRecordLocator,
    expected: &[u8],
) -> Result<RecordReadObservation, String> {
    let mut session = serving
        .records()
        .open_external(
            locator,
            RecordReadLimits::new(
                RecordByteLimit::new(expected.len() as u32)
                    .ok_or_else(|| "C.6 record limit was zero".to_owned())?,
            ),
        )
        .map_err(|failure| format!("C.6 record open failed: {failure:?}"))?;
    let mut observed = vec![0_u8; expected.len()];
    let mut offset = 0;
    while offset < observed.len() {
        let count = session
            .read_next(&mut observed[offset..])
            .map_err(|failure| format!("C.6 record read failed: {failure:?}"))?;
        if count == 0 {
            return Err("C.6 record ended before its declared payload".to_owned());
        }
        offset += count;
    }
    if observed != expected {
        return Err("C.6 record bytes diverged from the parent oracle".to_owned());
    }
    Ok(session.observation())
}

fn expected(
    oracle: &[u8],
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
) -> Result<&[u8], String> {
    let start = ordinal
        .checked_mul(configuration.record_bytes())
        .ok_or_else(|| "C.6 oracle offset overflowed".to_owned())?;
    let end = start
        .checked_add(configuration.record_bytes())
        .ok_or_else(|| "C.6 oracle range overflowed".to_owned())?;
    oracle
        .get(start..end)
        .ok_or_else(|| "C.6 oracle omitted a configured record".to_owned())
}

fn read_limits(configuration: BoundedResidencyConfiguration) -> RecordReadLimits {
    RecordReadLimits::new(
        RecordByteLimit::new(configuration.record_bytes() as u32)
            .expect("validated C.6 record bytes are nonzero"),
    )
}

fn positioned_reads(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedRead)
}

fn metadata_reads(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::ReadMetadata)
}
