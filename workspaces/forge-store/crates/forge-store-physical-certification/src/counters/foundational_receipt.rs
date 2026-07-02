use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceContractName, FoundationalPerformanceCounterName,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};

use super::{counter_contract_kind_token, CounterContractKind, CounterMismatchEvidence};
use crate::PhysicalCounterEvidenceRow;

pub(crate) fn build_foundational_receipt(
    rows: &[PhysicalCounterEvidenceRow],
) -> Result<
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
    CounterMismatchEvidence,
> {
    let claim = performance()
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
        .map_err(|_| CounterMismatchEvidence::FoundationalReceiptDenied)?;
    let mut bundle = performance_api::lower_lane::basis::performance_bundle(claim)
        .attach_contract_name(
            FoundationalPerformanceContractName::new("store.physical.counter-contracts")
                .map_err(|_| CounterMismatchEvidence::FoundationalReceiptDenied)?,
        );
    for row in rows {
        let name = foundational_counter_name(row.kind())?;
        bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            name,
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            row.observed_count(),
        ));
    }
    let bundle = bundle
        .finish()
        .map_err(|_| CounterMismatchEvidence::FoundationalReceiptDenied)?;
    let mut receipt =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for row in rows {
        receipt = receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
            foundational_counter_name(row.kind())?,
            row.observed_count(),
        ));
    }
    receipt
        .finish()
        .map_err(|_| CounterMismatchEvidence::FoundationalReceiptDenied)
}

fn foundational_counter_name(
    kind: CounterContractKind,
) -> Result<FoundationalPerformanceCounterName, CounterMismatchEvidence> {
    FoundationalPerformanceCounterName::new(format!(
        "store.physical.{}",
        counter_contract_kind_token(kind)
    ))
    .map_err(|_| CounterMismatchEvidence::FoundationalReceiptDenied)
}
