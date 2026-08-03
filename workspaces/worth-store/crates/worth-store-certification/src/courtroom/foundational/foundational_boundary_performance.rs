use crate::FoundationalPerformanceEvidenceDenial;
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
) -> Result<FoundationalStoreCounterReceipt, FoundationalPerformanceEvidenceDenial> {
    let mut bundle_builder =
        performance_api::lower_lane::basis::performance_bundle(authoritative_claim())
            .attach_contract_name(contract_name(contract));
    for (name, count) in rows {
        bundle_builder = bundle_builder.attach_counter_spec(counter_spec(name, *count));
    }
    let bundle = bundle_builder
        .finish()
        .map_err(FoundationalPerformanceEvidenceDenial::PerformanceBundleDenied)?;
    let mut receipt_builder =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for (name, count) in rows {
        receipt_builder = receipt_builder.attach_counter_row(counter_row(name, *count));
    }
    receipt_builder
        .finish()
        .map_err(FoundationalPerformanceEvidenceDenial::PerformanceReceiptDenied)
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

impl From<FoundationalPerformanceBundleConstructionDenial>
    for FoundationalPerformanceEvidenceDenial
{
    fn from(denial: FoundationalPerformanceBundleConstructionDenial) -> Self {
        Self::PerformanceBundleDenied(denial)
    }
}

impl From<FoundationalCounterBackedPerformanceReceiptConstructionDenial>
    for FoundationalPerformanceEvidenceDenial
{
    fn from(denial: FoundationalCounterBackedPerformanceReceiptConstructionDenial) -> Self {
        Self::PerformanceReceiptDenied(denial)
    }
}
