fn main() {
    let claim = forge_foundational::performance()
        .claim()
        .authoritative_execution()
        .boundary(forge_foundational::FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(
            forge_foundational::FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
        )
        .breadth_locality(forge_foundational::FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(forge_foundational::FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(forge_foundational::FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(
            forge_foundational::FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
        )
        .fallback_debt(forge_foundational::FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(forge_foundational::FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(forge_foundational::FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap();
    let bundle = forge_foundational::performance_bundle(claim).finish().unwrap();

    let _ = bundle.supporting_evidence_rows();
}
