use super::super::{
    canonical_milestone2_production_readiness_report,
    certify_canonical_milestone2_production_readiness, require_canonical_production_test_readiness,
    CanonicalMilestone2PhaseGate, CanonicalPhaseGateEvidence, CanonicalProductionReadinessReport,
    CanonicalProductionTestReadyArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalReadinessFrontDoor;

impl CanonicalReadinessFrontDoor {
    pub fn report(self) -> CanonicalProductionReadinessReport {
        canonical_milestone2_production_readiness_report()
    }

    pub fn certify(self) -> CanonicalProductionTestReadyArtifact {
        certify_canonical_milestone2_production_readiness()
    }

    pub fn require(
        self,
        readiness: &CanonicalProductionTestReadyArtifact,
    ) -> &CanonicalProductionReadinessReport {
        require_canonical_production_test_readiness(readiness)
    }

    pub fn passes(self, report: &CanonicalProductionReadinessReport) -> bool {
        report.passes_readiness_checklist()
    }

    pub fn phase_gate(
        self,
        report: &CanonicalProductionReadinessReport,
        gate: CanonicalMilestone2PhaseGate,
    ) -> Option<&CanonicalPhaseGateEvidence> {
        report
            .phase_gates()
            .iter()
            .find(|evidence| evidence.gate() == gate)
    }
}
