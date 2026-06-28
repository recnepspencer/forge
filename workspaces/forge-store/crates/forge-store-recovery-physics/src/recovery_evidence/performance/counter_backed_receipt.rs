use forge_foundational::{
    performance_bundle, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptBuilder,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceClaimAuthoringFrontDoor,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceSupportingEvidenceCode, FoundationalPerformanceSupportingEvidenceRow,
    FoundationalPerformanceWorkClass,
};

pub(crate) fn counter_backed_receipt(
    rows: Vec<FoundationalPerformanceCounterRow>,
) -> FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim> {
    let claim = FoundationalPerformanceClaimAuthoringFrontDoor
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::RebuildCapable)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::RecoveryOnly)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("recovery counter-backed performance claim is legal");
    let mut bundle = performance_bundle(claim);
    for row in &rows {
        bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            row.name().clone(),
            FoundationalPerformanceWorkClass::ReplayReconstruction,
            row.observed_count(),
        ));
    }
    bundle =
        bundle.attach_supporting_evidence_row(FoundationalPerformanceSupportingEvidenceRow::new(
            FoundationalPerformanceSupportingEvidenceCode::new("recovery.counter-backed-support")
                .expect("static support evidence code"),
            FoundationalPerformanceWorkClass::SupportReportAssembly,
        ));
    let bundle = bundle
        .finish()
        .expect("recovery performance bundle has unique counters");
    let mut receipt = FoundationalCounterBackedPerformanceReceiptBuilder::new(bundle);
    for row in rows {
        receipt = receipt.attach_counter_row(row);
    }
    receipt
        .finish()
        .expect("counter rows match recovery performance specs")
}
