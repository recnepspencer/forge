use crate::FoundationalBoundaryEvidenceDenial;
use worth_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceContractName, FoundationalPerformanceCounterName,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};

pub(crate) type FoundationalStoreCounterReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

pub(crate) fn counter_receipt(
    contract: &'static str,
    rows: &[(&'static str, u64)],
) -> Result<FoundationalStoreCounterReceipt, FoundationalBoundaryEvidenceDenial> {
    let mut bundle_builder =
        performance_api::lower_lane::basis::performance_bundle(authoritative_claim())
            .attach_contract_name(contract_name(contract));
    for (name, count) in rows {
        bundle_builder = bundle_builder.attach_counter_spec(counter_spec(name, *count));
    }
    let bundle = bundle_builder
        .finish()
        .map_err(FoundationalBoundaryEvidenceDenial::PerformanceBundleDenied)?;
    let mut receipt_builder =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for (name, count) in rows {
        receipt_builder = receipt_builder.attach_counter_row(counter_row(name, *count));
    }
    receipt_builder
        .finish()
        .map_err(FoundationalBoundaryEvidenceDenial::PerformanceReceiptDenied)
}

pub(crate) fn resident_rows(
    counters: worth_store_buffer_pool::ResidentFrameCounterSnapshot,
) -> [(&'static str, u64); 4] {
    [
        (
            "store.resident_memory.resident_bytes",
            counters.resident_bytes().as_bytes(),
        ),
        ("store.resident_memory.hit", counters.hit_count()),
        ("store.resident_memory.miss", counters.miss_count()),
        (
            "store.resident_memory.lookup",
            counters.frame_table_lookup_count(),
        ),
    ]
}

pub(crate) fn allocation_rows(
    counters: worth_store_buffer_pool::AllocationCounterSnapshot,
) -> Vec<(&'static str, u64)> {
    let mut rows = Vec::with_capacity(19);
    for scope in worth_store_buffer_pool::AllocationScope::ALL {
        let prefix = allocation_scope_prefix(scope);
        let scope_counters = counters.scope(scope);
        rows.push((prefix.requested, scope_counters.requested_bytes()));
        rows.push((prefix.admitted, scope_counters.admitted_bytes()));
        rows.push((prefix.allocated, scope_counters.allocated_bytes()));
    }
    rows.push((
        "store.allocation.fixed_metadata_bytes",
        counters.fixed_metadata_bytes(),
    ));
    rows
}

pub(crate) fn copy_rows(
    counters: worth_store_buffer_pool::RecordCopyCounterSnapshot,
) -> [(&'static str, u64); 5] {
    [
        (
            "store.copy.zero_copy_attempt",
            counters.zero_copy_admission_attempt_count(),
        ),
        (
            "store.copy.zero_copy_admitted",
            counters.zero_copy_admission_count(),
        ),
        (
            "store.copy.bounded_copy_attempt",
            counters.bounded_copy_attempt_count(),
        ),
        ("store.copy.bounded_copy", counters.bounded_copy_count()),
        ("store.copy.copied_bytes", counters.copied_bytes()),
    ]
}

#[derive(Debug, Clone, Copy)]
struct AllocationScopePrefix {
    requested: &'static str,
    admitted: &'static str,
    allocated: &'static str,
}

fn allocation_scope_prefix(
    scope: worth_store_buffer_pool::AllocationScope,
) -> AllocationScopePrefix {
    match scope {
        worth_store_buffer_pool::AllocationScope::Foreground => AllocationScopePrefix {
            requested: "store.allocation.foreground.requested",
            admitted: "store.allocation.foreground.admitted",
            allocated: "store.allocation.foreground.allocated",
        },
        worth_store_buffer_pool::AllocationScope::Maintenance => AllocationScopePrefix {
            requested: "store.allocation.maintenance.requested",
            admitted: "store.allocation.maintenance.admitted",
            allocated: "store.allocation.maintenance.allocated",
        },
        worth_store_buffer_pool::AllocationScope::Recovery => AllocationScopePrefix {
            requested: "store.allocation.recovery.requested",
            admitted: "store.allocation.recovery.admitted",
            allocated: "store.allocation.recovery.allocated",
        },
        worth_store_buffer_pool::AllocationScope::Scrub => AllocationScopePrefix {
            requested: "store.allocation.scrub.requested",
            admitted: "store.allocation.scrub.admitted",
            allocated: "store.allocation.scrub.allocated",
        },
        worth_store_buffer_pool::AllocationScope::ImportExport => AllocationScopePrefix {
            requested: "store.allocation.import_export.requested",
            admitted: "store.allocation.import_export.admitted",
            allocated: "store.allocation.import_export.allocated",
        },
        worth_store_buffer_pool::AllocationScope::Streaming => AllocationScopePrefix {
            requested: "store.allocation.streaming.requested",
            admitted: "store.allocation.streaming.admitted",
            allocated: "store.allocation.streaming.allocated",
        },
    }
}

fn authoritative_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("static phase 11 performance claim is valid")
}

fn contract_name(name: &'static str) -> FoundationalPerformanceContractName {
    FoundationalPerformanceContractName::new(name).expect("static contract name is valid")
}

fn counter_spec(name: &'static str, expected: u64) -> FoundationalPerformanceCounterSpec {
    FoundationalPerformanceCounterSpec::new(
        counter_name(name),
        FoundationalPerformanceWorkClass::AuthoritativeMutation,
        expected,
    )
}

fn counter_row(name: &'static str, observed: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(counter_name(name), observed)
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).expect("static counter name is valid")
}

impl From<FoundationalPerformanceBundleConstructionDenial> for FoundationalBoundaryEvidenceDenial {
    fn from(denial: FoundationalPerformanceBundleConstructionDenial) -> Self {
        Self::PerformanceBundleDenied(denial)
    }
}

impl From<FoundationalCounterBackedPerformanceReceiptConstructionDenial>
    for FoundationalBoundaryEvidenceDenial
{
    fn from(denial: FoundationalCounterBackedPerformanceReceiptConstructionDenial) -> Self {
        Self::PerformanceReceiptDenied(denial)
    }
}
