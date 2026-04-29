use serde::{Deserialize, Serialize};

use super::diagnostics::{ResourceDiagnosticsExpansionDenial, ResourceDiagnosticsSummary};
use super::policy::{
    DeniedResourcePolicyRestoreCompatibility, ResourcePolicyRestoreCompatibilityProof,
};
use super::summary::{ResourceBoundaryPerformanceEnvelope, ResourceRuntimeSummaryReadReport};

pub const RESOURCE_REPLAY_AVAILABILITY_SCHEMA_VERSION: &str =
    "forge.resource.replay-availability.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceReplayAvailabilityClass {
    Retained,
    Reconstructed,
    Omitted,
    Unavailable,
    Denied,
}

impl ResourceReplayAvailabilityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::Reconstructed => "reconstructed",
            Self::Omitted => "omitted",
            Self::Unavailable => "unavailable",
            Self::Denied => "denied",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceReplayAvailabilityDenialClass {
    RestoreCompatibilityDenied,
    BudgetHistoryUnavailable,
}

impl ResourceReplayAvailabilityDenialClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RestoreCompatibilityDenied => "restore-compatibility-denied",
            Self::BudgetHistoryUnavailable => "budget-history-unavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceReplayAvailabilityReport {
    class: ResourceReplayAvailabilityClass,
    denial_class: Option<ResourceReplayAvailabilityDenialClass>,
    retained_history_unavailable_count: u32,
    denied_completion_unavailable_count: u32,
    retry_lineage_unavailable_count: u32,
    summary_read: ResourceRuntimeSummaryReadReport,
    restore_compatibility: Option<ResourcePolicyRestoreCompatibilityProof>,
    restore_compatibility_denial: Option<DeniedResourcePolicyRestoreCompatibility>,
    diagnostics_summary: Option<ResourceDiagnosticsSummary>,
    diagnostics_denial: Option<ResourceDiagnosticsExpansionDenial>,
    availability_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceReplayAvailabilityReport {
    pub(crate) fn new(
        class: ResourceReplayAvailabilityClass,
        denial_class: Option<ResourceReplayAvailabilityDenialClass>,
        summary_read: ResourceRuntimeSummaryReadReport,
        restore_compatibility: Option<ResourcePolicyRestoreCompatibilityProof>,
        restore_compatibility_denial: Option<DeniedResourcePolicyRestoreCompatibility>,
        diagnostics_summary: Option<ResourceDiagnosticsSummary>,
        diagnostics_denial: Option<ResourceDiagnosticsExpansionDenial>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let retained_history_unavailable_count =
            summary_read.summary().retained_history_unavailable_count() as u32;
        let denied_completion_unavailable_count = summary_read
            .summary()
            .retained_denied_completion_unavailable_count()
            as u32;
        let retry_lineage_unavailable_count = summary_read
            .summary()
            .retained_retry_lineage_unavailable_count()
            as u32;
        let availability_digest = resource_replay_availability_digest(
            class,
            denial_class,
            retained_history_unavailable_count,
            denied_completion_unavailable_count,
            retry_lineage_unavailable_count,
            restore_compatibility.as_ref().map(|proof| {
                (
                    proof.compatibility_digest().as_str(),
                    proof.replay_decision_digest().as_str(),
                )
            }),
            restore_compatibility_denial.as_ref().map(|denial| {
                (
                    denial.compatibility_digest().as_str(),
                    denial.replay_decision_digest().as_str(),
                )
            }),
            diagnostics_summary
                .as_ref()
                .map(|summary| summary.provenance_digest()),
            diagnostics_denial
                .as_ref()
                .map(|denial| denial.policy_decision_digest().as_str()),
        );
        Self {
            class,
            denial_class,
            retained_history_unavailable_count,
            denied_completion_unavailable_count,
            retry_lineage_unavailable_count,
            summary_read,
            restore_compatibility,
            restore_compatibility_denial,
            diagnostics_summary,
            diagnostics_denial,
            availability_digest,
            performance,
        }
    }

    pub fn class(&self) -> ResourceReplayAvailabilityClass {
        self.class
    }

    pub fn denial_class(&self) -> Option<ResourceReplayAvailabilityDenialClass> {
        self.denial_class
    }

    pub fn retained_history_unavailable_count(&self) -> u32 {
        self.retained_history_unavailable_count
    }

    pub fn denied_completion_unavailable_count(&self) -> u32 {
        self.denied_completion_unavailable_count
    }

    pub fn retry_lineage_unavailable_count(&self) -> u32 {
        self.retry_lineage_unavailable_count
    }

    pub fn summary_read(&self) -> &ResourceRuntimeSummaryReadReport {
        &self.summary_read
    }

    pub fn restore_compatibility(&self) -> Option<&ResourcePolicyRestoreCompatibilityProof> {
        self.restore_compatibility.as_ref()
    }

    pub fn restore_compatibility_denial(
        &self,
    ) -> Option<&DeniedResourcePolicyRestoreCompatibility> {
        self.restore_compatibility_denial.as_ref()
    }

    pub fn diagnostics_summary(&self) -> Option<&ResourceDiagnosticsSummary> {
        self.diagnostics_summary.as_ref()
    }

    pub fn diagnostics_denial(&self) -> Option<&ResourceDiagnosticsExpansionDenial> {
        self.diagnostics_denial.as_ref()
    }

    pub fn availability_digest(&self) -> &str {
        &self.availability_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

fn resource_replay_availability_digest(
    class: ResourceReplayAvailabilityClass,
    denial_class: Option<ResourceReplayAvailabilityDenialClass>,
    retained_history_unavailable_count: u32,
    denied_completion_unavailable_count: u32,
    retry_lineage_unavailable_count: u32,
    restore_compatibility_provenance: Option<(&str, &str)>,
    restore_compatibility_denial_provenance: Option<(&str, &str)>,
    diagnostics_summary_digest: Option<&str>,
    diagnostics_denial_policy_digest: Option<&str>,
) -> String {
    let restore_compatibility_digest = restore_compatibility_provenance
        .map(|(compatibility, replay)| format!("{compatibility}:{replay}"))
        .unwrap_or_else(|| "none".to_owned());
    let restore_compatibility_denial_digest = restore_compatibility_denial_provenance
        .map(|(compatibility, replay)| format!("{compatibility}:{replay}"))
        .unwrap_or_else(|| "none".to_owned());
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        RESOURCE_REPLAY_AVAILABILITY_SCHEMA_VERSION,
        class.as_str(),
        denial_class.map(|class| class.as_str()).unwrap_or("none"),
        retained_history_unavailable_count,
        denied_completion_unavailable_count,
        retry_lineage_unavailable_count,
        restore_compatibility_digest,
        restore_compatibility_denial_digest,
        diagnostics_summary_digest.unwrap_or("none"),
        diagnostics_denial_policy_digest.unwrap_or("none")
    )
}
