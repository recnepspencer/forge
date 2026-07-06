use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use super::BlobPlacementMovementCounterSnapshot;

pub type BlobPlacementMovementCounterBackedPerformanceReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

pub(crate) fn counter_backed_placement_movement_performance_receipt(
    counters: BlobPlacementMovementCounterSnapshot,
) -> BlobPlacementMovementCounterBackedPerformanceReceipt {
    let specs = counter_specs(counters);
    let mut bundle = performance_api::lower_lane::basis::performance_bundle(counter_claim());
    for spec in &specs {
        bundle = bundle.attach_counter_spec(spec.clone());
    }
    let bundle = bundle
        .finish()
        .expect("blob placement movement counter bundle should build");
    let mut receipt =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for row in counter_rows(counters) {
        receipt = receipt.attach_counter_row(row);
    }
    receipt
        .finish()
        .expect("blob placement movement exact counters should satisfy receipt specs")
}

fn counter_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("blob placement movement counter claim should build")
}

fn counter_specs(
    counters: BlobPlacementMovementCounterSnapshot,
) -> [FoundationalPerformanceCounterSpec; 10] {
    counter_rows(counters).map(|row| {
        FoundationalPerformanceCounterSpec::new(
            row.name().clone(),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            row.observed_count(),
        )
    })
}

fn counter_rows(
    counters: BlobPlacementMovementCounterSnapshot,
) -> [FoundationalPerformanceCounterRow; 10] {
    [
        counter_row("placement_moves", counters.placement_moves()),
        counter_row("inline_reads", counters.inline_reads()),
        counter_row("external_reads", counters.external_reads()),
        counter_row("cold_fetches", counters.cold_fetches()),
        counter_row(
            "unavailable_cold_chunks",
            counters.unavailable_cold_chunks(),
        ),
        counter_row("tier_move_retries", counters.tier_move_retries()),
        counter_row("protected_denials", counters.protected_denials()),
        counter_row("execution_receipts", counters.execution_receipts()),
        counter_row("published_observations", counters.published_observations()),
        counter_row("exact_counter_strength", 1),
    ]
}

fn counter_row(name: &'static str, value: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(counter_name(name), value)
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(format!("store.s7.blob.placement_movement.{name}"))
        .expect("static blob placement movement counter name should be valid")
}
