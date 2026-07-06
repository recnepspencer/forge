use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use crate::BlobStreamingIngestCounterSnapshot;

pub type BlobStreamingCounterBackedPerformanceReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

pub(crate) fn counter_backed_streaming_performance_receipt(
    counters: BlobStreamingIngestCounterSnapshot,
) -> BlobStreamingCounterBackedPerformanceReceipt {
    let specs = counter_specs(counters);
    let mut bundle = performance_api::lower_lane::basis::performance_bundle(counter_claim());
    for spec in &specs {
        bundle = bundle.attach_counter_spec(spec.clone());
    }
    let bundle = bundle
        .finish()
        .expect("blob streaming counter bundle should build");
    let mut receipt =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for row in counter_rows(counters) {
        receipt = receipt.attach_counter_row(row);
    }
    receipt
        .finish()
        .expect("blob streaming exact counters should satisfy receipt specs")
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
        .expect("blob streaming counter claim should build")
}

fn counter_specs(
    counters: BlobStreamingIngestCounterSnapshot,
) -> [FoundationalPerformanceCounterSpec; 12] {
    counter_rows(counters).map(|row| {
        FoundationalPerformanceCounterSpec::new(
            row.name().clone(),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            row.observed_count(),
        )
    })
}

fn counter_rows(
    counters: BlobStreamingIngestCounterSnapshot,
) -> [FoundationalPerformanceCounterRow; 12] {
    [
        counter_row("source_windows_observed", counters.windows_observed()),
        counter_row("bytes_streamed", counters.bytes_streamed()),
        counter_row("chunks_read", counters.chunks_read()),
        counter_row("chunks_written", counters.chunks_written()),
        counter_row(
            "backend_write_observations",
            counters.backend_write_observations(),
        ),
        counter_row("peak_resident_bytes", counters.peak_resident_bytes()),
        counter_row("allocation_count", counters.allocation_count()),
        counter_row("scheduler_yields", counters.scheduler_yields()),
        counter_row("scheduler_waits", counters.scheduler_waits()),
        counter_row("scheduler_throttles", counters.scheduler_throttles()),
        counter_row("scheduler_admissions", counters.scheduler_admissions()),
        counter_row("denials", counters.denials()),
    ]
}

fn counter_row(name: &'static str, value: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(counter_name(name), value)
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(format!("store.s7.blob.streaming.{name}"))
        .expect("static blob streaming counter name should be valid")
}