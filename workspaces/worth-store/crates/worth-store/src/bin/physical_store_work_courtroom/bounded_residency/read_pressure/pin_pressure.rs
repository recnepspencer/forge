use worth_store::physical_runtime::{CertificationFrameReadFailure, ServingPhysicalRuntime};
use worth_store_buffer_pool::PhysicalResidencyDenial;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::super::BoundedResidencyConfiguration;
use super::require_bound_work;

const PIN_FRAME_BYTES: u32 = 8;

pub(in crate::bounded_residency) struct PinnedFramePressureEvidence {
    pub(in crate::bounded_residency) cold_work: u64,
    pub(in crate::bounded_residency) hot_work: u64,
    pub(in crate::bounded_residency) refault_work: u64,
    pub(in crate::bounded_residency) peak_pinned_frames: u32,
    pub(in crate::bounded_residency) peak_pin_leases: u32,
    pub(in crate::bounded_residency) dimension:
        worth_store::physical_runtime::PhysicalResidencyDimension,
    pub(in crate::bounded_residency) scope:
        worth_store::physical_runtime::PhysicalOperationAllocationScope,
    pub(in crate::bounded_residency) requested: u64,
    pub(in crate::bounded_residency) admitted: u64,
    pub(in crate::bounded_residency) limit: u64,
}

pub(in crate::bounded_residency) fn prove_pins(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
) -> Result<PinnedFramePressureEvidence, String> {
    serving
        .certification_physical_residency()
        .drain_unpinned_clean_frames();
    let residency = serving.certification_physical_residency();
    let first_coordinate = pin_coordinate(0)?;
    let first = residency
        .pin_exact(first_coordinate)
        .map_err(|failure| format!("cold C.6 pin failed: {failure:?}"))?;
    require_bound_work(serving, &first, "cold pin")?;
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
    require_bound_work(serving, &refault, "pin refault")?;
    let counters = residency.counters();
    if counters.evictions() == 0 {
        return Err("C.6 pin pressure produced no eviction".to_owned());
    }
    let evidence = PinnedFramePressureEvidence {
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

fn pin_coordinate(offset: u64) -> Result<RecordFrameCoordinate, String> {
    RecordFrameCoordinate::new(
        RecordArtifactFile::BootstrapCatalog,
        offset,
        PIN_FRAME_BYTES,
    )
    .ok_or_else(|| "C.6 pin coordinate was invalid".to_owned())
}
