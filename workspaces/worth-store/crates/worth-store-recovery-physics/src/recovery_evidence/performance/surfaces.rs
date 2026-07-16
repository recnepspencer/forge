use worth_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalPerformanceBoundary, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPolicyAdmissionReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPerformanceSurfaceKind {
    RecoveryOnly,
    ColdReplay,
    VerifierRead,
    Materialization,
    SupportOnly,
    PolicyAdmission,
    CounterBacked,
    FreshnessRetention,
    FallbackDebt,
    Certified,
    Readmitted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPerformanceSurface {
    kind: RecoveryPerformanceSurfaceKind,
    boundary: FoundationalPerformanceBoundary,
    evidence_strength: FoundationalPerformanceEvidenceStrength,
    execution_temperature: FoundationalPerformanceExecutionTemperature,
    freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    fallback_debt: FoundationalPerformanceFallbackDebtPosture,
    exact_counter_assertions: usize,
    counter_backed_current_truth: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecoveryPerformanceCounterEvidence {
    exact_counter_assertions: usize,
    counter_backed_current_truth: bool,
}

impl RecoveryPerformanceCounterEvidence {
    const fn current(exact_counter_assertions: usize) -> Self {
        Self {
            exact_counter_assertions,
            counter_backed_current_truth: true,
        }
    }

    const fn retained(exact_counter_assertions: usize) -> Self {
        Self {
            exact_counter_assertions,
            counter_backed_current_truth: false,
        }
    }
}

impl RecoveryPerformanceSurface {
    pub const fn kind(&self) -> RecoveryPerformanceSurfaceKind {
        self.kind
    }

    pub const fn boundary(&self) -> FoundationalPerformanceBoundary {
        self.boundary
    }

    pub const fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
        self.evidence_strength
    }

    pub const fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
        self.execution_temperature
    }

    pub const fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
        self.freshness_retention
    }

    pub const fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
        self.fallback_debt
    }

    pub const fn exact_counter_assertions(&self) -> usize {
        self.exact_counter_assertions
    }

    pub const fn counter_backed_current_truth(&self) -> bool {
        self.counter_backed_current_truth
    }
}

fn performance_surface(
    kind: RecoveryPerformanceSurfaceKind,
    boundary: FoundationalPerformanceBoundary,
    evidence_strength: FoundationalPerformanceEvidenceStrength,
    execution_temperature: FoundationalPerformanceExecutionTemperature,
    freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    fallback_debt: FoundationalPerformanceFallbackDebtPosture,
    counter_evidence: RecoveryPerformanceCounterEvidence,
) -> RecoveryPerformanceSurface {
    RecoveryPerformanceSurface {
        kind,
        boundary,
        evidence_strength,
        execution_temperature,
        freshness_retention,
        fallback_debt,
        exact_counter_assertions: counter_evidence.exact_counter_assertions,
        counter_backed_current_truth: counter_evidence.counter_backed_current_truth,
    }
}

pub(crate) fn recovery_performance_surfaces(
    counter_rows: usize,
    policy_admission: &FoundationalPolicyAdmissionReceipt,
    counter_backed: &FoundationalCounterBackedPerformanceReceipt<
        FoundationalAuthoritativePerformanceClaim,
    >,
) -> Vec<RecoveryPerformanceSurface> {
    vec![
        performance_surface(
            RecoveryPerformanceSurfaceKind::RecoveryOnly,
            FoundationalPerformanceBoundary::RestoreRecovery,
            FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            FoundationalPerformanceExecutionTemperature::RecoveryOnly,
            FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::current(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::ColdReplay,
            FoundationalPerformanceBoundary::ReplayReconstruction,
            FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            FoundationalPerformanceExecutionTemperature::ColdPath,
            FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::current(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::VerifierRead,
            FoundationalPerformanceBoundary::ReplayReconstruction,
            FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            FoundationalPerformanceExecutionTemperature::ColdPath,
            FoundationalPerformanceFreshnessRetentionPosture::HistoricalRetained,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::Materialization,
            FoundationalPerformanceBoundary::BoundaryMaterialization,
            FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim,
            FoundationalPerformanceExecutionTemperature::SupportOnly,
            FoundationalPerformanceFreshnessRetentionPosture::HistoricalRetained,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::SupportOnly,
            FoundationalPerformanceBoundary::SupportAssembly,
            FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim,
            FoundationalPerformanceExecutionTemperature::SupportOnly,
            FoundationalPerformanceFreshnessRetentionPosture::StaleSupport,
            FoundationalPerformanceFallbackDebtPosture::Deferred,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::PolicyAdmission,
            policy_admission.boundary(),
            policy_admission.evidence_strength(),
            FoundationalPerformanceExecutionTemperature::RecoveryOnly,
            FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::CounterBacked,
            counter_backed.bundle().claim().boundary(),
            counter_backed.bundle().claim().evidence_strength(),
            counter_backed.bundle().claim().execution_temperature(),
            counter_backed.bundle().claim().freshness_retention(),
            counter_backed.bundle().claim().fallback_debt(),
            RecoveryPerformanceCounterEvidence::current(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::FreshnessRetention,
            FoundationalPerformanceBoundary::RestoreRecovery,
            FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            FoundationalPerformanceExecutionTemperature::RecoveryOnly,
            FoundationalPerformanceFreshnessRetentionPosture::RestoredReadmitted,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::current(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::FallbackDebt,
            FoundationalPerformanceBoundary::SupportAssembly,
            FoundationalPerformanceEvidenceStrength::ExplicitDebtDeferredClaim,
            FoundationalPerformanceExecutionTemperature::SupportOnly,
            FoundationalPerformanceFreshnessRetentionPosture::StaleSupport,
            FoundationalPerformanceFallbackDebtPosture::FreshFreezeRebuildReadmissionRequired,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::Certified,
            FoundationalPerformanceBoundary::SupportAssembly,
            FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim,
            FoundationalPerformanceExecutionTemperature::SupportOnly,
            FoundationalPerformanceFreshnessRetentionPosture::HistoricalRetained,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
        performance_surface(
            RecoveryPerformanceSurfaceKind::Readmitted,
            FoundationalPerformanceBoundary::SupportAssembly,
            FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim,
            FoundationalPerformanceExecutionTemperature::SupportOnly,
            FoundationalPerformanceFreshnessRetentionPosture::RestoredReadmitted,
            FoundationalPerformanceFallbackDebtPosture::Verified,
            RecoveryPerformanceCounterEvidence::retained(counter_rows),
        ),
    ]
}
