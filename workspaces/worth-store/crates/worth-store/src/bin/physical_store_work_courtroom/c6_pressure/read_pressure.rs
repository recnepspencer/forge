use worth_store::physical_runtime::{
    CertificationFrameReadFailure, ExternalPhysicalRecordLocator, PhysicalRecordId,
    RecordByteLimit, RecordReadLimits, RecordReadObservation, ServingPhysicalRuntime,
};
use worth_store_buffer_pool::PhysicalResidencyDenial;
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::C6PressureConfiguration;

const PIN_FRAME_BYTES: u32 = 8;

pub(super) struct C6ReadPressureEvidence {
    pub(super) cold_read_effects: u64,
    pub(super) hot_read_effects: u64,
    pub(super) refault_effects: u64,
    pub(super) read_work: u64,
    pub(super) peak_resident_bytes: u64,
    pub(super) peak_admitted_bytes: u64,
    pub(super) faults: u64,
    pub(super) hits: u64,
    pub(super) evictions: u64,
}

pub(super) struct C6PinPressureEvidence {
    pub(super) cold_work: u64,
    pub(super) hot_work: u64,
    pub(super) refault_work: u64,
    pub(super) peak_pinned_frames: u32,
    pub(super) peak_pin_leases: u32,
    pub(super) dimension: worth_store::physical_runtime::PhysicalResidencyDimension,
    pub(super) scope: worth_store::physical_runtime::PhysicalOperationAllocationScope,
    pub(super) requested: u64,
    pub(super) admitted: u64,
    pub(super) limit: u64,
}

pub(super) struct C6CancellationEvidence {
    pub(super) physical_work: u64,
    pub(super) first_operation: u64,
    pub(super) last_operation: u64,
    pub(super) handoff_bound: bool,
    pub(super) unread_payload_bytes: u64,
    pub(super) open_media_effects: u64,
    pub(super) cancellation_media_effects: u64,
}

pub(super) fn prove_pins(
    serving: &ServingPhysicalRuntime,
    configuration: C6PressureConfiguration,
) -> Result<C6PinPressureEvidence, String> {
    serving.drain_clean_residency();
    let handoff = serving.c6_physical_work_handoff();
    let residency = serving.certification_physical_residency();
    let first_coordinate = pin_coordinate(0)?;
    let first = residency
        .pin_exact(first_coordinate)
        .map_err(|failure| format!("cold C.6 pin failed: {failure:?}"))?;
    require_bound_work(&handoff, &first, "cold pin")?;
    let hot = residency
        .pin_exact(first_coordinate)
        .map_err(|failure| format!("hot C.6 pin failed: {failure:?}"))?;
    if hot.physical_work_count() != 0 {
        return Err("hot C.6 pin admitted abandoned physical work".to_owned());
    }
    let denial = match residency.pin_exact(first_coordinate) {
        Err(CertificationFrameReadFailure::Residency(denial)) => denial,
        Err(failure) => return Err(format!("over-pin failed for the wrong reason: {failure:?}")),
        Ok(_) => return Err("C.6 over-pin unexpectedly succeeded".to_owned()),
    };
    let PhysicalResidencyDenial::Pressure(pressure) = denial else {
        return Err(format!("C.6 over-pin returned bare denial {denial:?}"));
    };
    if pressure.dimension() != worth_store::physical_runtime::PhysicalResidencyDimension::PinLeases
        || pressure.scope()
            != worth_store::physical_runtime::PhysicalOperationAllocationScope::ForegroundRead
        || pressure.requested() != 1
        || pressure.current() != u64::from(configuration.pin_leases())
        || pressure.limit() != u64::from(configuration.pin_leases())
    {
        return Err(format!(
            "C.6 over-pin returned imprecise pressure {pressure:?}"
        ));
    }
    if residency.counters().pin_leases() != configuration.pin_leases() {
        return Err("C.6 pin counter did not reach its admitted lease limit".to_owned());
    }
    drop(hot);
    drop(first);

    for ordinal in 1..=configuration.frame_entries() {
        let coordinate = pin_coordinate(u64::from(ordinal) * u64::from(PIN_FRAME_BYTES))?;
        drop(
            residency
                .pin_exact(coordinate)
                .map_err(|failure| format!("C.6 pressure pin failed: {failure:?}"))?,
        );
    }
    let refault = residency
        .pin_exact(first_coordinate)
        .map_err(|failure| format!("C.6 pin refault failed: {failure:?}"))?;
    require_bound_work(&handoff, &refault, "pin refault")?;
    let counters = residency.counters();
    if counters.evictions() == 0 {
        return Err("C.6 pin pressure produced no eviction".to_owned());
    }
    let evidence = C6PinPressureEvidence {
        cold_work: 1,
        hot_work: 0,
        refault_work: refault.physical_work_count(),
        peak_pinned_frames: counters.peak_pinned_frames(),
        peak_pin_leases: counters.peak_pin_leases(),
        dimension: pressure.dimension(),
        scope: pressure.scope(),
        requested: pressure.requested(),
        admitted: pressure.current(),
        limit: pressure.limit(),
    };
    drop(refault);
    Ok(evidence)
}

pub(super) fn prove_reads(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    oracle: &[u8],
    configuration: C6PressureConfiguration,
) -> Result<C6ReadPressureEvidence, String> {
    serving.drain_clean_residency();
    let store = serving.store_identity();
    let first = ExternalPhysicalRecordLocator::new(store, records[0]);
    let before_cold = positioned_reads(serving);
    let cold = read_record(serving, first, expected(oracle, configuration, 0)?)?;
    let cold_read_effects = positioned_reads(serving).saturating_sub(before_cold);
    if cold_read_effects == 0 || cold.physical_work_count() == 0 {
        return Err("cold C.6 read did not enter canonical physical work".to_owned());
    }
    let before_hot = positioned_reads(serving);
    let hot = read_record(serving, first, expected(oracle, configuration, 0)?)?;
    let hot_read_effects = positioned_reads(serving).saturating_sub(before_hot);
    if hot_read_effects != 0 {
        return Err("hot C.6 read repeated a media effect".to_owned());
    }

    let mut read_work = cold
        .physical_work_count()
        .saturating_add(hot.physical_work_count());
    for (ordinal, record) in records.iter().copied().enumerate().skip(1) {
        let locator = ExternalPhysicalRecordLocator::new(store, record);
        let observation = read_record(serving, locator, expected(oracle, configuration, ordinal)?)?;
        read_work = read_work.saturating_add(observation.physical_work_count());
    }
    let before_refault = positioned_reads(serving);
    let refault = read_record(serving, first, expected(oracle, configuration, 0)?)?;
    let refault_effects = positioned_reads(serving).saturating_sub(before_refault);
    if refault_effects == 0 || refault.physical_work_count() == 0 {
        return Err("C.6 pressure did not refault the first record".to_owned());
    }
    read_work = read_work.saturating_add(refault.physical_work_count());
    let counters = serving.residency_observation().counters();
    if counters.evictions() == 0 || counters.peak_resident_bytes() > configuration.resident_bytes()
    {
        return Err("C.6 read pressure escaped its residency bound".to_owned());
    }
    Ok(C6ReadPressureEvidence {
        cold_read_effects,
        hot_read_effects,
        refault_effects,
        read_work,
        peak_resident_bytes: counters.peak_resident_bytes(),
        peak_admitted_bytes: counters.peak_admitted_bytes(),
        faults: counters.faults(),
        hits: counters.hits(),
        evictions: counters.evictions(),
    })
}

pub(super) fn prove_cancellation(
    serving: &ServingPhysicalRuntime,
    record: PhysicalRecordId,
    configuration: C6PressureConfiguration,
) -> Result<C6CancellationEvidence, String> {
    serving.drain_clean_residency();
    let locator = ExternalPhysicalRecordLocator::new(serving.store_identity(), record);
    let before_open = positioned_reads(serving);
    let session = serving
        .records()
        .open_external(locator, read_limits(configuration))
        .map_err(|failure| format!("cancellable C.6 read open failed: {failure:?}"))?;
    let after_open = positioned_reads(serving);
    let cancelled = session.cancel();
    let observation = cancelled.observation();
    let first = observation.first_physical_work();
    let last = observation.last_physical_work();
    let handoff = serving.c6_physical_work_handoff().identity();
    Ok(C6CancellationEvidence {
        physical_work: observation.physical_work_count(),
        first_operation: first.map_or(0, |identity| identity.operation().get()),
        last_operation: last.map_or(0, |identity| identity.operation().get()),
        handoff_bound: first.is_some_and(|identity| handoff.admits(identity))
            && last.is_some_and(|identity| handoff.admits(identity)),
        unread_payload_bytes: cancelled.unread_payload_bytes(),
        open_media_effects: after_open.saturating_sub(before_open),
        cancellation_media_effects: positioned_reads(serving).saturating_sub(after_open),
    })
}

fn require_bound_work(
    handoff: &worth_store::physical_runtime::C6PhysicalWorkHandoff,
    lease: &worth_store::physical_runtime::CertificationResidentFrame,
    label: &str,
) -> Result<(), String> {
    if lease.physical_work_count() != 1
        || lease
            .first_physical_work()
            .is_none_or(|identity| !handoff.identity().admits(identity))
        || lease.first_physical_work() != lease.last_physical_work()
    {
        return Err(format!(
            "{label} did not retain one handoff-bound work identity"
        ));
    }
    Ok(())
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
    configuration: C6PressureConfiguration,
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

fn read_limits(configuration: C6PressureConfiguration) -> RecordReadLimits {
    RecordReadLimits::new(
        RecordByteLimit::new(configuration.record_bytes() as u32)
            .expect("validated C.6 record bytes are nonzero"),
    )
}

fn pin_coordinate(offset: u64) -> Result<RecordFrameCoordinate, String> {
    RecordFrameCoordinate::new(
        RecordArtifactFile::BootstrapCatalog,
        offset,
        PIN_FRAME_BYTES,
    )
    .ok_or_else(|| "C.6 pin coordinate was invalid".to_owned())
}

fn positioned_reads(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedRead)
}
