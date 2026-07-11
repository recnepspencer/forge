use forge_foundational::performance_api::lower_lane::basis::{
    FoundationalPerformanceAttachmentConstructionDenial, FoundationalPerformanceBundle,
    FoundationalPerformanceBundleConstructionDenial, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterSpec,
};
use forge_foundational::performance_api::lower_lane::receipts::{
    counter_backed_performance_receipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceCounterRow,
};
use forge_foundational::{
    performance, performance_bundle, FoundationalAuthoritativePerformanceClaim,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use super::super::basis::S6FoundationalCounterReceipt;
use super::super::{PhysicalIsolationCounterSnapshot, S6IoQosIsolationReadinessDenial};

pub(crate) fn construct_s6_foundational_counter_receipt(
    counters: PhysicalIsolationCounterSnapshot,
) -> Result<S6FoundationalCounterReceipt, S6IoQosIsolationReadinessDenial> {
    let bundle = construct_s6_executed_performance_bundle(counters)?;
    counter_backed_performance_receipt(bundle)
        .attach_counter_row(counter_row(
            "s5.closeout.outcomes",
            counters.outcome_count(),
        )?)
        .attach_counter_row(counter_row("s5.closeout.waits", counters.wait_count())?)
        .attach_counter_row(counter_row("s5.closeout.retries", counters.retry_count())?)
        .attach_counter_row(counter_row(
            "s5.closeout.latch-counter-rows",
            counters.latch_counter_rows(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.latch-waits",
            counters.latch_wait_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.reclaim-counter-rows",
            counters.reclaim_counter_rows(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.blocked-maintenance",
            counters.blocked_maintenance_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.reclaim-blocks",
            counters.reclaim_block_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.protected-byte-footprint",
            counters.protected_byte_footprint(),
        )?)
        .finish()
        .map_err(map_receipt_denial)
}

fn construct_s6_executed_performance_bundle(
    counters: PhysicalIsolationCounterSnapshot,
) -> Result<
    FoundationalPerformanceBundle<FoundationalAuthoritativePerformanceClaim>,
    S6IoQosIsolationReadinessDenial,
> {
    let claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::TraversalLocal)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .finish()
        .map_err(|_| S6IoQosIsolationReadinessDenial::MissingExecutedCounter)?;

    performance_bundle(claim)
        .attach_contract_name(contract_name()?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.outcomes",
            counters.outcome_count(),
        )?)
        .attach_counter_spec(counter_spec("s5.closeout.waits", counters.wait_count())?)
        .attach_counter_spec(counter_spec("s5.closeout.retries", counters.retry_count())?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.latch-counter-rows",
            counters.latch_counter_rows(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.latch-waits",
            counters.latch_wait_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.reclaim-counter-rows",
            counters.reclaim_counter_rows(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.blocked-maintenance",
            counters.blocked_maintenance_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.reclaim-blocks",
            counters.reclaim_block_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.protected-byte-footprint",
            counters.protected_byte_footprint(),
        )?)
        .finish()
        .map_err(map_bundle_denial)
}

fn contract_name() -> Result<FoundationalPerformanceContractName, S6IoQosIsolationReadinessDenial> {
    FoundationalPerformanceContractName::new("forge-store.s5.executed-isolation-closeout")
        .map_err(map_attachment_denial)
}

fn counter_spec(
    name: &'static str,
    expected_exact_count: u64,
) -> Result<FoundationalPerformanceCounterSpec, S6IoQosIsolationReadinessDenial> {
    Ok(FoundationalPerformanceCounterSpec::new(
        counter_name(name)?,
        FoundationalPerformanceWorkClass::ValidationPlanning,
        expected_exact_count,
    ))
}

fn counter_row(
    name: &'static str,
    observed_count: u64,
) -> Result<FoundationalPerformanceCounterRow, S6IoQosIsolationReadinessDenial> {
    Ok(FoundationalPerformanceCounterRow::new(
        counter_name(name)?,
        observed_count,
    ))
}

fn counter_name(
    name: &'static str,
) -> Result<FoundationalPerformanceCounterName, S6IoQosIsolationReadinessDenial> {
    FoundationalPerformanceCounterName::new(name).map_err(map_attachment_denial)
}

fn map_attachment_denial(
    _: FoundationalPerformanceAttachmentConstructionDenial,
) -> S6IoQosIsolationReadinessDenial {
    S6IoQosIsolationReadinessDenial::MissingExecutedCounter
}

fn map_bundle_denial(
    _: FoundationalPerformanceBundleConstructionDenial,
) -> S6IoQosIsolationReadinessDenial {
    S6IoQosIsolationReadinessDenial::MissingExecutedCounter
}

fn map_receipt_denial(
    _: FoundationalCounterBackedPerformanceReceiptConstructionDenial,
) -> S6IoQosIsolationReadinessDenial {
    S6IoQosIsolationReadinessDenial::MissingExecutedCounter
}
