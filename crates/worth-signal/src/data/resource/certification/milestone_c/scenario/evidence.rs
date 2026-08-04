use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneCPolicyCertificationFamily, ResourceMilestoneCPolicyScenarioEvidenceKind,
    ResourceMilestoneCPolicyScenarioId,
};
use super::super::digest_basis::{
    ResourceMilestoneCDiagnosticsDenialEvidenceBasis,
    ResourceMilestoneCPolicyRegistryFreezeEvidenceBasis,
    ResourceMilestoneCRestoreDenialEvidenceBasis, ResourceMilestoneCRestoreProofEvidenceBasis,
    ResourceMilestoneCRetentionCompactionEvidenceBasis, ResourceMilestoneCRetryDenialEvidenceBasis,
    ResourceMilestoneCTimeoutHeartbeatDenialEvidenceBasis,
};
use super::contract::ResourceMilestoneCPolicyScenarioRow;
use crate::data::error::SignalError;
use crate::data::resource::DeniedResourcePolicyRestoreCompatibility;
use crate::data::resource::ResourceBoundaryKind;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceDiagnosticsExpansionDenial;
use crate::data::resource::ResourceLifecycleRetentionCompactionReport;
use crate::data::resource::ResourcePolicyRegistryFreezeReport;
use crate::data::resource::ResourcePolicyRestoreCompatibilityDenialClass;
use crate::data::resource::ResourcePolicyRestoreCompatibilityProof;
use crate::data::resource::ResourceRetryDenialClass;
use crate::data::resource::ResourceRetryScheduleReport;
use crate::data::resource::ResourceTimeoutHeartbeatExtensionDenialClass;
use crate::data::resource::ResourceTimeoutHeartbeatExtensionReport;

impl ResourceMilestoneCPolicyScenarioRow {
    pub(super) fn from_registry_freeze(
        report: &ResourcePolicyRegistryFreezeReport,
    ) -> Result<Self, SignalError> {
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::RegistryOrderCanonicalization,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::RegistryFreeze,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncResourcePolicyFamilyCertification,
            ),
            policy_provenance_digest: Some(report.registry_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCPolicyRegistryFreezeEvidenceBasis {
                    descriptor_count: report.descriptor_count(),
                    id_index_width: report.id_index_width(),
                    kind_name_index_width: report.kind_name_index_width(),
                    registry_digest: report.registry_digest().as_str(),
                },
            ),
            performance: ResourceBoundaryPerformanceEnvelope::policy_compatibility(
                report.descriptor_count() as u32,
                0,
            ),
            passed: true,
        })
    }

    pub(super) fn from_retry_denial(
        report: &ResourceRetryScheduleReport,
    ) -> Result<Self, SignalError> {
        let denied = report.denied_retry().ok_or_else(|| {
            SignalError::invalid_input(
                "resource milestone C policy scenario retry-budget-exhaustion-rejected requires denied retry evidence",
            )
        })?;
        if denied.class() != ResourceRetryDenialClass::RetryBudgetExhausted {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario retry-budget-exhaustion-rejected requires RetryBudgetExhausted denial evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::RetryBudgetExhaustionRejected,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::RetryDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetryBudgetAndBackoffCertification,
            ),
            policy_provenance_digest: Some(denied.policy_decision_digest().as_str().to_owned()),
            retry_denial_class: Some(denied.class()),
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(&ResourceMilestoneCRetryDenialEvidenceBasis {
                class: denied.class(),
                retry_budget_scope: denied.retry_budget_scope(),
                retry_budget_limit: denied.retry_budget_limit(),
                retry_budget_usage: denied.retry_budget_usage(),
                performance: report.performance(),
            }),
            performance: report.performance(),
            passed: true,
        })
    }

    pub(super) fn from_timeout_heartbeat_denial(
        report: &ResourceTimeoutHeartbeatExtensionReport,
    ) -> Result<Self, SignalError> {
        let denied = report.denied_extension().ok_or_else(|| {
            SignalError::invalid_input(
                "resource milestone C policy scenario heartbeat-extension-terminal-denied requires denied heartbeat extension evidence",
            )
        })?;
        if denied.class() != ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario heartbeat-extension-terminal-denied requires NonActiveRequest denial evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::HeartbeatExtensionTerminalDenied,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::TimeoutHeartbeatDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncTimeoutDeadlineCertification,
            ),
            policy_provenance_digest: None,
            retry_denial_class: None,
            timeout_heartbeat_denial_class: Some(denied.class()),
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCTimeoutHeartbeatDenialEvidenceBasis {
                    class: denied.class(),
                    performance: report.performance(),
                },
            ),
            performance: report.performance(),
            passed: true,
        })
    }

    pub(super) fn from_retention_compaction(
        report: &ResourceLifecycleRetentionCompactionReport,
    ) -> Result<Self, SignalError> {
        if report.retained_history_unavailable_count() == 0
            && report.retained_denied_completion_pruned_count() == 0
            && report.retained_retry_lineage_pruned_count() == 0
        {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario retention-compaction-reports-unavailable-history requires unavailable or pruned history evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::RetentionCompactionReportsUnavailableHistory,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::RetentionCompaction,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(report.policy_provenance_digest().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCRetentionCompactionEvidenceBasis {
                    retained_history_pruned_count: report.retained_history_pruned_count(),
                    retained_history_unavailable_count: report
                        .retained_history_unavailable_count(),
                    retained_denied_completion_pruned_count: report
                        .retained_denied_completion_pruned_count(),
                    retained_retry_lineage_pruned_count: report
                        .retained_retry_lineage_pruned_count(),
                    compacted_terminal_summary_count: report.compacted_terminal_summary_count(),
                    performance: report.performance(),
                },
            ),
            performance: report.performance(),
            passed: true,
        })
    }

    pub(super) fn from_diagnostics_denial(
        denial: &ResourceDiagnosticsExpansionDenial,
    ) -> Result<Self, SignalError> {
        if denial.performance().boundary() != ResourceBoundaryKind::DiagnosticsExpansion {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario diagnostics-expansion-budget-denied-zero-cold requires diagnostics expansion denial evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::DiagnosticsExpansionBudgetDeniedZeroCold,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::DiagnosticsExpansionDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(denial.policy_decision_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCDiagnosticsDenialEvidenceBasis {
                    class: denial.class(),
                    policy_decision_class: denial.policy_decision_class(),
                    replay_reconstruction_width: denial.replay_reconstruction_width(),
                    forensic_reconstruction_width: denial.forensic_reconstruction_width(),
                    performance: denial.performance(),
                    policy_decision_digest: denial.policy_decision_digest().as_str(),
                },
            ),
            performance: denial.performance(),
            passed: true,
        })
    }

    pub(super) fn from_restore_proof(
        proof: &ResourcePolicyRestoreCompatibilityProof,
    ) -> Result<Self, SignalError> {
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::CompatibleDescriptorRestoreAdmitted,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::ReplayCompatibilityProof,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(proof.replay_decision_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCRestoreProofEvidenceBasis {
                    compatibility_digest: proof.compatibility_digest().as_str(),
                    replay_decision_digest: proof.replay_decision_digest().as_str(),
                    performance: proof.performance(),
                },
            ),
            performance: proof.performance(),
            passed: true,
        })
    }

    pub(super) fn from_restore_denial(
        id: ResourceMilestoneCPolicyScenarioId,
        denial: &DeniedResourcePolicyRestoreCompatibility,
    ) -> Result<Self, SignalError> {
        let expected_class = match id {
            ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied => {
                ResourcePolicyRestoreCompatibilityDenialClass::VersionIncompatible
            }
            ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied => {
                ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor
            }
            _ => {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy scenario {} is not a restore-denial scenario",
                    id.label()
                )))
            }
        };
        if denial.class() != expected_class {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy scenario {} requires {:?} denial evidence",
                id.label(),
                expected_class
            )));
        }
        Ok(Self {
            id,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::ReplayCompatibilityDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(denial.replay_decision_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: Some(denial.class()),
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCRestoreDenialEvidenceBasis {
                    class: denial.class(),
                    primary_incompatible_kind: denial.primary_incompatible_kind(),
                    compatibility_digest: denial.compatibility_digest().as_str(),
                    replay_decision_digest: denial.replay_decision_digest().as_str(),
                    performance: denial.performance(),
                },
            ),
            performance: denial.performance(),
            passed: true,
        })
    }
}
