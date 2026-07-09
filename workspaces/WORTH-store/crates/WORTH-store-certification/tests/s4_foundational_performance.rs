#[path = "s4_foundational_evidence_support.rs"]
mod evidence_support;

use worth_foundational::{
    FoundationalCertifiedPerformanceClass, FoundationalPerformanceBoundary,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceReportMaterializationBoundary,
};
use worth_store_recovery_physics::{
    RecoveryCounterPerformanceReceipt, RecoveryPerformanceSurfaceKind,
};

#[test]
fn recovery_performance_surfaces_distinguish_counter_truth_from_support_claims() {
    let source = evidence_support::verified_source();
    let performance = RecoveryCounterPerformanceReceipt::from_source(&source);
    let counters = source.counters();

    assert_eq!(
        performance.exact_counter_assertions(),
        performance.rows().len()
    );
    assert_counter(
        &performance,
        "recovery.allocation_bytes",
        counters.allocation_bytes(),
    );
    assert_counter(
        &performance,
        "recovery.memory_envelope_bytes",
        counters.memory_envelope_bytes(),
    );
    assert_counter(
        &performance,
        "recovery.memory_envelope_frames",
        counters.memory_envelope_frames() as u64,
    );
    assert_counter(
        &performance,
        "recovery.page_redos",
        counters.page_redos() as u64,
    );
    assert_counter(
        &performance,
        "recovery.replayed_frames",
        counters.replayed_frames() as u64,
    );
    assert_counter(
        &performance,
        "recovery.residue_rejections",
        counters.residue_rejections() as u64,
    );
    assert_counter(
        &performance,
        "recovery.skipped_frames",
        counters.skipped_frames() as u64,
    );
    assert_counter(
        &performance,
        "recovery.validated_checkpoints",
        counters.validated_checkpoints(),
    );
    assert_counter(
        &performance,
        "recovery.verifier_forbidden_full_store_scans",
        counters.forbidden_full_store_scans(),
    );
    assert_eq!(
        performance.counter_backed().counter_rows(),
        performance.rows()
    );
    assert_eq!(
        performance
            .policy_admission()
            .stronger_evidence_still_required(),
        FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
    );

    let surfaces = performance.surfaces();
    for kind in [
        RecoveryPerformanceSurfaceKind::RecoveryOnly,
        RecoveryPerformanceSurfaceKind::ColdReplay,
        RecoveryPerformanceSurfaceKind::VerifierRead,
        RecoveryPerformanceSurfaceKind::Materialization,
        RecoveryPerformanceSurfaceKind::SupportOnly,
        RecoveryPerformanceSurfaceKind::PolicyAdmission,
        RecoveryPerformanceSurfaceKind::CounterBacked,
        RecoveryPerformanceSurfaceKind::FreshnessRetention,
        RecoveryPerformanceSurfaceKind::FallbackDebt,
        RecoveryPerformanceSurfaceKind::Certified,
        RecoveryPerformanceSurfaceKind::Readmitted,
    ] {
        assert!(surfaces.iter().any(|surface| surface.kind() == kind));
    }
    assert!(surfaces
        .iter()
        .filter(|surface| surface.counter_backed_current_truth())
        .all(|surface| surface.evidence_strength()
            == FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt));
    for kind in [
        RecoveryPerformanceSurfaceKind::Materialization,
        RecoveryPerformanceSurfaceKind::SupportOnly,
        RecoveryPerformanceSurfaceKind::PolicyAdmission,
        RecoveryPerformanceSurfaceKind::FallbackDebt,
        RecoveryPerformanceSurfaceKind::Certified,
        RecoveryPerformanceSurfaceKind::Readmitted,
    ] {
        assert!(!surface(&surfaces, kind).counter_backed_current_truth());
        assert_ne!(
            surface(&surfaces, kind).evidence_strength(),
            FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
        );
    }
    assert_eq!(
        surface(&surfaces, RecoveryPerformanceSurfaceKind::ColdReplay).execution_temperature(),
        FoundationalPerformanceExecutionTemperature::ColdPath
    );
    assert_eq!(
        surface(&surfaces, RecoveryPerformanceSurfaceKind::SupportOnly).boundary(),
        FoundationalPerformanceBoundary::SupportAssembly
    );
    assert_eq!(
        surface(&surfaces, RecoveryPerformanceSurfaceKind::Readmitted).freshness_retention(),
        FoundationalPerformanceFreshnessRetentionPosture::RestoredReadmitted
    );
    assert_eq!(
        surface(&surfaces, RecoveryPerformanceSurfaceKind::FallbackDebt).fallback_debt(),
        FoundationalPerformanceFallbackDebtPosture::FreshFreezeRebuildReadmissionRequired
    );

    let support_expansion = performance.support_expansion_report();
    assert_eq!(
        support_expansion.materialization_boundary(),
        FoundationalPerformanceReportMaterializationBoundary::SupportExpansion
    );
    assert_eq!(support_expansion.counter_rows(), performance.rows());
    assert_eq!(
        performance
            .readmitted_support_expansion()
            .unwrap()
            .certified_class(),
        FoundationalCertifiedPerformanceClass::SupportExpansionCompatibility
    );
}

fn assert_counter(performance: &RecoveryCounterPerformanceReceipt, name: &str, expected: u64) {
    let observed = performance
        .rows()
        .iter()
        .find(|row| row.name().as_str() == name)
        .unwrap_or_else(|| panic!("missing recovery performance counter {name}"))
        .observed_count();
    assert_eq!(observed, expected);
}

fn surface<'a>(
    surfaces: &'a [worth_store_recovery_physics::RecoveryPerformanceSurface],
    kind: RecoveryPerformanceSurfaceKind,
) -> &'a worth_store_recovery_physics::RecoveryPerformanceSurface {
    surfaces
        .iter()
        .find(|surface| surface.kind() == kind)
        .expect("surface exists")
}
