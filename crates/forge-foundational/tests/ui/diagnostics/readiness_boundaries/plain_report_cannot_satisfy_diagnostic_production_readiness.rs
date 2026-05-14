fn require_readiness(
    _value: forge_foundational::FoundationalDiagnosticProductionTestReadyArtifact,
) {
}

fn main() {
    let report: forge_foundational::FoundationalDiagnosticProductionReadinessReport =
        forge_foundational::foundational_diagnostic_milestone6_readiness_report();
    require_readiness(report);
}
