use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use crate::BlobStreamingReadCounterSnapshot;

pub type BlobStreamingReadCounterBackedPerformanceReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

pub(crate) fn counter_backed_streaming_read_performance_receipt(
    counters: BlobStreamingReadCounterSnapshot,
) -> BlobStreamingReadCounterBackedPerformanceReceipt {
    let specs = counter_specs(counters);
    let mut bundle = performance_api::lower_lane::basis::performance_bundle(counter_claim());
    for spec in &specs {
        bundle = bundle.attach_counter_spec(spec.clone());
    }
    let bundle = bundle
        .finish()
        .expect("blob streaming read counter bundle should build");
    let mut receipt =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for row in counter_rows(counters) {
        receipt = receipt.attach_counter_row(row);
    }
    receipt
        .finish()
        .expect("blob streaming read exact counters should satisfy receipt specs")
}

fn counter_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::ScanHeavy)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("blob streaming read counter claim should build")
}

fn counter_specs(
    counters: BlobStreamingReadCounterSnapshot,
) -> [FoundationalPerformanceCounterSpec; 23] {
    counter_rows(counters).map(|row| {
        FoundationalPerformanceCounterSpec::new(
            row.name().clone(),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            row.observed_count(),
        )
    })
}

fn counter_rows(
    counters: BlobStreamingReadCounterSnapshot,
) -> [FoundationalPerformanceCounterRow; 23] {
    [
        counter_row("read_windows_observed", counters.windows_observed()),
        counter_row("bytes_read", counters.bytes_read()),
        counter_row("chunks_read", counters.chunks_read()),
        counter_row("chunks_verified", counters.chunks_verified()),
        counter_row(
            "chunk_checksum_verifications",
            counters.chunk_checksum_verifications(),
        ),
        counter_row("digest_updates", counters.digest_updates()),
        counter_row(
            "read_amplification_bytes",
            counters.read_amplification_bytes(),
        ),
        counter_row("allocation_count", counters.allocation_count()),
        counter_row("scheduler_waits", counters.scheduler_waits()),
        counter_row("pressure_yield_denials", counters.pressure_yield_denials()),
        counter_row(
            "pressure_deferred_denials",
            counters.pressure_deferred_denials(),
        ),
        counter_row(
            "pressure_denied_denials",
            counters.pressure_denied_denials(),
        ),
        counter_row("pressure_stale_denials", counters.pressure_stale_denials()),
        counter_row("pressure_throttles", counters.pressure_throttles()),
        counter_row(
            "pressure_admitted_with_debt",
            counters.pressure_admitted_with_debt(),
        ),
        counter_row("pressure_violations", counters.pressure_violations()),
        counter_row("protected_read_denials", counters.protected_read_denials()),
        counter_row(
            "cold_unavailable_denials",
            counters.cold_unavailable_denials(),
        ),
        counter_row("stale_read_denials", counters.stale_read_denials()),
        counter_row("corrupt_chunk_denials", counters.corrupt_chunk_denials()),
        counter_row("order_denials", counters.order_denials()),
        counter_row("missing_chunk_denials", counters.missing_chunk_denials()),
        counter_row("peak_resident_bytes", counters.peak_resident_bytes()),
    ]
}

fn counter_row(name: &'static str, value: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(counter_name(name), value)
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(format!("store.s7.blob.streaming_read.{name}"))
        .expect("static blob streaming read counter name should be valid")
}