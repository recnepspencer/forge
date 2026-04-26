use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::summary::{
    ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
    ResourceReplayReconstructionReport, ResourceRuntimeSummary,
};

pub const RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION: &str =
    "forge-signal-resource-diagnostics-summary-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDiagnosticsExpansionBudget {
    allow_cold_reconstruction: bool,
    max_replay_reconstruction_width: u32,
}

impl ResourceDiagnosticsExpansionBudget {
    pub fn retained_summary_only() -> Self {
        Self {
            allow_cold_reconstruction: false,
            max_replay_reconstruction_width: 0,
        }
    }

    pub fn allow_cold_reconstruction(max_replay_reconstruction_width: u32) -> Self {
        Self {
            allow_cold_reconstruction: true,
            max_replay_reconstruction_width,
        }
    }

    pub fn allow_cold_reconstruction_flag(self) -> bool {
        self.allow_cold_reconstruction
    }

    pub fn max_replay_reconstruction_width(self) -> u32 {
        self.max_replay_reconstruction_width
    }

    pub fn admits_replay_width(self, replay_reconstruction_width: u32) -> bool {
        self.allow_cold_reconstruction
            && replay_reconstruction_width <= self.max_replay_reconstruction_width
    }

    pub fn denial_class(
        self,
        replay_reconstruction_width: u32,
    ) -> Option<ResourceDiagnosticsExpansionDenialClass> {
        if !self.allow_cold_reconstruction {
            Some(ResourceDiagnosticsExpansionDenialClass::ColdReconstructionDisabled)
        } else if replay_reconstruction_width > self.max_replay_reconstruction_width {
            Some(ResourceDiagnosticsExpansionDenialClass::ReplayReconstructionBudgetExceeded)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceDiagnosticsExpansionDenialClass {
    ColdReconstructionDisabled,
    ReplayReconstructionBudgetExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDiagnosticsExpansionDenial {
    class: ResourceDiagnosticsExpansionDenialClass,
    budget: ResourceDiagnosticsExpansionBudget,
    replay_reconstruction_width: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceDiagnosticsExpansionDenial {
    pub(crate) fn new(
        class: ResourceDiagnosticsExpansionDenialClass,
        budget: ResourceDiagnosticsExpansionBudget,
        replay_reconstruction_width: u32,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            class,
            budget,
            replay_reconstruction_width,
            performance,
        }
    }

    pub fn class(self) -> ResourceDiagnosticsExpansionDenialClass {
        self.class
    }

    pub fn budget(self) -> ResourceDiagnosticsExpansionBudget {
        self.budget
    }

    pub fn replay_reconstruction_width(self) -> u32 {
        self.replay_reconstruction_width
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDiagnosticsSummary {
    schema_version: String,
    runtime_summary: ResourceRuntimeSummary,
    latest_branch_restore_report: Option<ResourceBranchRestoreReport>,
    replay_reconstruction: ResourceReplayReconstructionReport,
    performance: ResourceBoundaryPerformanceEnvelope,
    provenance_digest: String,
}

impl ResourceDiagnosticsSummary {
    pub(crate) fn new(
        runtime_summary: ResourceRuntimeSummary,
        latest_branch_restore_report: Option<ResourceBranchRestoreReport>,
        replay_reconstruction: ResourceReplayReconstructionReport,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let provenance_digest =
            resource_diagnostics_digest(&ResourceDiagnosticsSummaryDigestBasis {
                schema_version: RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION,
                runtime_summary,
                latest_branch_restore_report,
                replay_reconstruction: &replay_reconstruction,
                performance,
            });
        Self {
            schema_version: RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION.to_owned(),
            runtime_summary,
            latest_branch_restore_report,
            replay_reconstruction,
            performance,
            provenance_digest,
        }
    }

    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn runtime_summary(&self) -> ResourceRuntimeSummary {
        self.runtime_summary
    }

    pub fn latest_branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.latest_branch_restore_report
    }

    pub fn replay_reconstruction(&self) -> &ResourceReplayReconstructionReport {
        &self.replay_reconstruction
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }
}

#[derive(Debug, Serialize)]
struct ResourceDiagnosticsSummaryDigestBasis<'a> {
    schema_version: &'static str,
    runtime_summary: ResourceRuntimeSummary,
    latest_branch_restore_report: Option<ResourceBranchRestoreReport>,
    replay_reconstruction: &'a ResourceReplayReconstructionReport,
    performance: ResourceBoundaryPerformanceEnvelope,
}

fn resource_diagnostics_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("resource diagnostics serialization");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}
