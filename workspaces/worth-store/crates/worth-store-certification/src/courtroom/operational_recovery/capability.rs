use super::{S10OperationalScenarioKind, S10ScenarioSuiteEvidence, ScenarioScaleProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalRecoveryCapability {
    Backup,
    Restore,
    PointInTimeRecovery,
    Rollback,
    Repair,
    ReplicaBootstrap,
    ReplicaPromotion,
    ForensicAcquisition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalRecoveryCapabilityRow {
    capability: OperationalRecoveryCapability,
    source_scenario: S10OperationalScenarioKind,
    ci_evidence_identity: [u8; 32],
    release_evidence_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryCapabilityMatrix {
    rows: Vec<OperationalRecoveryCapabilityRow>,
}

impl OperationalRecoveryCapabilityMatrix {
    pub(super) fn from_closed_suites(
        ci: &S10ScenarioSuiteEvidence,
        release: &S10ScenarioSuiteEvidence,
    ) -> Self {
        debug_assert_eq!(ci.profile(), ScenarioScaleProfile::Ci);
        debug_assert_eq!(release.profile(), ScenarioScaleProfile::Release);
        let rows = [
            (
                OperationalRecoveryCapability::Backup,
                S10OperationalScenarioKind::BurningPrimary,
            ),
            (
                OperationalRecoveryCapability::Restore,
                S10OperationalScenarioKind::AuthorityRepairRollback,
            ),
            (
                OperationalRecoveryCapability::PointInTimeRecovery,
                S10OperationalScenarioKind::BurningPrimary,
            ),
            (
                OperationalRecoveryCapability::Rollback,
                S10OperationalScenarioKind::AuthorityRepairRollback,
            ),
            (
                OperationalRecoveryCapability::Repair,
                S10OperationalScenarioKind::AuthorityRepairRollback,
            ),
            (
                OperationalRecoveryCapability::ReplicaBootstrap,
                S10OperationalScenarioKind::SplitBrainPromotion,
            ),
            (
                OperationalRecoveryCapability::ReplicaPromotion,
                S10OperationalScenarioKind::SplitBrainPromotion,
            ),
            (
                OperationalRecoveryCapability::ForensicAcquisition,
                S10OperationalScenarioKind::BurningPrimary,
            ),
        ]
        .into_iter()
        .map(
            |(capability, source_scenario)| OperationalRecoveryCapabilityRow {
                capability,
                source_scenario,
                ci_evidence_identity: ci.scenario(source_scenario).evidence_identity(),
                release_evidence_identity: release.scenario(source_scenario).evidence_identity(),
            },
        )
        .collect();
        Self { rows }
    }

    pub fn rows(&self) -> &[OperationalRecoveryCapabilityRow] {
        &self.rows
    }
}

impl OperationalRecoveryCapabilityRow {
    pub const fn capability(self) -> OperationalRecoveryCapability {
        self.capability
    }
    pub const fn source_scenario(self) -> S10OperationalScenarioKind {
        self.source_scenario
    }
    pub const fn ci_evidence_identity(self) -> [u8; 32] {
        self.ci_evidence_identity
    }
    pub const fn release_evidence_identity(self) -> [u8; 32] {
        self.release_evidence_identity
    }
}
