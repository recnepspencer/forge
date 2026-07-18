use serde::{Deserialize, Serialize};

use crate::discovery::{CaseKind, TestCaseSurface};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProofFamily {
    OwnerBehavior,
    OwnerInvariant,
    CrossOwnerIntegration,
    CompilerBoundary,
    DependencyBoundary,
    StructuralTopology,
    DeterministicSimulation,
    FreshProcessIsolation,
    FormalConformance,
    PerformanceEnvelope,
    Soak,
    ReleaseQualification,
    HardwareQualification,
}

impl ProofFamily {
    pub(crate) fn from_case(case: &TestCaseSurface) -> Self {
        let path = case.source_path.to_ascii_lowercase();
        let identity = case.identity.stable_id.to_ascii_lowercase();
        if matches!(
            case.kind,
            CaseKind::DoctestCompileFail | CaseKind::DoctestIgnored
        ) {
            return Self::CompilerBoundary;
        }
        if case.kind == CaseKind::DoctestRunnable {
            return Self::OwnerBehavior;
        }
        if case.kind == CaseKind::UiFixture || path.contains("compile_fail") {
            return if identity.contains("dependency") || identity.contains("reverse_flow") {
                Self::DependencyBoundary
            } else {
                Self::CompilerBoundary
            };
        }
        if identity.contains("fresh_process")
            || identity.contains("crash_process")
            || identity.contains("offline_verifier")
        {
            return Self::FreshProcessIsolation;
        }
        if path.contains("formal-models") || identity.contains("protocol_model") {
            return Self::FormalConformance;
        }
        if identity.contains("performance")
            || identity.contains("scale")
            || identity.contains("pressure")
        {
            return Self::PerformanceEnvelope;
        }
        if identity.contains("simulation")
            || identity.contains("schedule")
            || identity.contains("interleaving")
        {
            return Self::DeterministicSimulation;
        }
        if path.contains("/tests/scenarios/") || path.contains("/certification/") {
            return Self::CrossOwnerIntegration;
        }
        if path.contains("/src/") {
            return Self::OwnerInvariant;
        }
        Self::OwnerBehavior
    }
}
