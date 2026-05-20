use forge_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalPerformanceReportPlan,
};

fn require_counter_backed_receipt(
    _receipt: &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
) {
}

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
    let attached = forge_foundational::attach_performance_bundle(
        forge_foundational::FoundationalPerformanceAttachmentTargetKind::BoundaryArtifact,
        bundle,
    )
    .unwrap();
    let plan: FoundationalPerformanceReportPlan<_> =
        forge_foundational::plan_performance_report(forge_foundational::FoundationalPerformanceReportRequest {
            source: attached,
            profile: forge_foundational::profiles()
                .set()
                .diagnostic_richness(forge_foundational::DiagnosticRichnessProfile::Standard)
                .support_posture(forge_foundational::SupportPostureProfile::SupportReady)
                .compatibility_posture(forge_foundational::CompatibilityPostureProfile::NativeOnly)
                .admission_readiness(forge_foundational::AdmissionReadinessProfile::Admitted)
                .retention_delivery(forge_foundational::RetentionDeliveryProfile::Retained)
                .certification_posture(forge_foundational::CertificationPostureProfile::Uncertified)
                .compose()
                .unwrap(),
            include_layout_intent: false,
            include_contract_names: false,
            include_counter_specs: false,
            include_counter_rows: false,
            include_supporting_evidence_rows: false,
            include_budget_decisions: false,
            include_denied_work: false,
            include_widened_work: false,
        });

    require_counter_backed_receipt(&plan);
}
