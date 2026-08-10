use super::super::{classification_error, SubscriptionSupportOperationalVerdict};
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportMaintenanceWorkKind {
    Rebuild,
    Refresh,
    CompatibilityMigration,
    DegradationRecovery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMaintenanceDecision {
    evidence: SubscriptionSupportMaintenanceDecisionEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
enum SubscriptionSupportMaintenanceDecisionEvidence {
    RebuildDescriptorAdmitted {
        retained_basis_digest: String,
    },
    RefreshDescriptorAdmitted {
        refresh_reason: String,
    },
    CompatibilityMigrationDescriptorAdmitted {
        migration_digest: String,
    },
    DegradationRecoveryDescriptorAdmitted {
        recovery_reason: String,
    },
    InterruptedRestartRecovered {
        recovered_work_kind: SupportMaintenanceWorkKind,
        restart_recovery_digest: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportMaintenanceDecision {
    pub(crate) fn rebuild_descriptor_admitted(
        retained_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::RebuildDescriptorAdmitted {
                retained_basis_digest: require_non_empty(
                    "retained rebuild basis",
                    retained_basis_digest,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn refresh_descriptor_admitted(
        refresh_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::RefreshDescriptorAdmitted {
                refresh_reason: require_non_empty("refresh reason", refresh_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn compatibility_migration_descriptor_admitted(
        migration_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::CompatibilityMigrationDescriptorAdmitted {
                migration_digest: require_non_empty("compatibility migration", migration_digest)?,
            }
            .into(),
        )
    }

    pub(crate) fn degradation_recovery_descriptor_admitted(
        recovery_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::DegradationRecoveryDescriptorAdmitted {
                recovery_reason: require_non_empty("degradation recovery", recovery_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn interrupted_restart_recovered(
        recovered_work_kind: SupportMaintenanceWorkKind,
        restart_recovery_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::InterruptedRestartRecovered {
                recovered_work_kind,
                restart_recovery_digest: require_non_empty(
                    "interrupted maintenance restart recovery",
                    restart_recovery_digest,
                )?,
            }
            .into(),
        )
    }

    pub fn kind(&self) -> SubscriptionSupportMaintenanceDecisionKind {
        match &self.evidence {
            SubscriptionSupportMaintenanceDecisionEvidence::RebuildDescriptorAdmitted { .. } => {
                SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted
            }
            SubscriptionSupportMaintenanceDecisionEvidence::RefreshDescriptorAdmitted { .. } => {
                SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted
            }
            SubscriptionSupportMaintenanceDecisionEvidence::CompatibilityMigrationDescriptorAdmitted {
                ..
            } => SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted,
            SubscriptionSupportMaintenanceDecisionEvidence::DegradationRecoveryDescriptorAdmitted {
                ..
            } => SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted,
            SubscriptionSupportMaintenanceDecisionEvidence::InterruptedRestartRecovered { .. } => {
                SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
            }
        }
    }

    pub fn work_kind(&self) -> SupportMaintenanceWorkKind {
        match self.kind() {
            SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted => {
                SupportMaintenanceWorkKind::Rebuild
            }
            SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted => {
                SupportMaintenanceWorkKind::Refresh
            }
            SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted => {
                SupportMaintenanceWorkKind::CompatibilityMigration
            }
            SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted => {
                SupportMaintenanceWorkKind::DegradationRecovery
            }
            SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered => {
                match &self.evidence {
                    SubscriptionSupportMaintenanceDecisionEvidence::InterruptedRestartRecovered {
                        recovered_work_kind,
                        ..
                    } => *recovered_work_kind,
                    _ => SupportMaintenanceWorkKind::Rebuild,
                }
            }
        }
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match self.work_kind() {
            SupportMaintenanceWorkKind::Rebuild => {
                SubscriptionSupportOperationalVerdict::RebuildRequired
            }
            SupportMaintenanceWorkKind::Refresh
            | SupportMaintenanceWorkKind::CompatibilityMigration => {
                SubscriptionSupportOperationalVerdict::ExactResumePreserved
            }
            SupportMaintenanceWorkKind::DegradationRecovery => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
        }
    }

    pub(super) fn retained_basis_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportMaintenanceDecisionEvidence::RebuildDescriptorAdmitted {
                retained_basis_digest,
            } => Some(retained_basis_digest),
            _ => None,
        }
    }
}

impl From<SubscriptionSupportMaintenanceDecisionEvidence>
    for SubscriptionSupportMaintenanceDecision
{
    fn from(evidence: SubscriptionSupportMaintenanceDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportMaintenanceDecisionKind {
    RebuildDescriptorAdmitted,
    RefreshDescriptorAdmitted,
    CompatibilityMigrationDescriptorAdmitted,
    DegradationRecoveryDescriptorAdmitted,
    InterruptedRestartRecovered,
}
