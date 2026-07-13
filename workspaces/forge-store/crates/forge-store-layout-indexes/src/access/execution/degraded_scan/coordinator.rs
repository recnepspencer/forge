use super::{
    DegradedScanExecution, DegradedScanReadinessOutcome, DegradedScanReadmission,
    DegradedScanReady, LoweredDegradedExactScan, StaleDegradedExactScan,
};

pub(in crate::access::execution) fn lower(
    selected: crate::planning::SelectedDegradedExactScan,
) -> LoweredDegradedExactScan {
    LoweredDegradedExactScan::issue(selected)
}

pub(in crate::access::execution) fn classify_readiness(
    lowered: LoweredDegradedExactScan,
    frontier: crate::CurrentMaterializationFrontier,
) -> DegradedScanReadinessOutcome {
    let materialization = lowered
        .selected()
        .materialization()
        .expect("degraded selection retains admitted materialization")
        .clone();
    match materialization.classify_freshness_at(frontier) {
        Ok(crate::MaterializationFreshness::Current(current)) => {
            DegradedScanReadinessOutcome::ready(DegradedScanReady::issue(lowered, current))
        }
        Ok(crate::MaterializationFreshness::Stale(stale)) => {
            DegradedScanReadinessOutcome::stale(lowered.stale(stale))
        }
        Err(_) => unreachable!("degraded selection retains exact admitted materialization"),
    }
}

pub(in crate::access::execution) fn executed(
    ready: DegradedScanReady,
    physical: forge_store_physical_format::PlatformPhysicalDegradedExecutionObservation,
) -> DegradedScanExecution {
    let (recipe, current) = ready.into_parts();
    DegradedScanExecution::observe(recipe, current, physical)
}

pub(in crate::access::execution) fn admit_stale_readmission(
    stale: &StaleDegradedExactScan,
    current: crate::CurrentLayoutMaterialization,
) -> Result<DegradedScanReadmission, crate::access::execution::DegradedScanAdmissionDenied> {
    super::readmission::admit_stale(stale, current)
}

pub(in crate::access::execution) fn readmit_stale(
    stale: StaleDegradedExactScan,
    admission: DegradedScanReadmission,
) -> Result<DegradedScanReady, crate::access::execution::DegradedScanAdmissionDenied> {
    super::readmission::readmit(stale, admission)
}
