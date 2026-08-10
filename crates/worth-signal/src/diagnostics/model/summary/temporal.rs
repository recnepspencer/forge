use serde::{Deserialize, Serialize};

use crate::data::telemetry::TemporalTelemetry;
use crate::data::temporal::{RuntimeClockBasis, TemporalFrontierSnapshot, TemporalWakeSummary};
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::profile::DiagnosticsTier;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalPerformanceFailureMode {
    TemporalBroadScan,
    IntervalCatchUpExplosion,
    WakeAllocationChurn,
    BranchRestoreTemporalRebuild,
    RescheduleBreadthLeak,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalCostContractSummary {
    pub temporal_registration_lowering: String,
    pub clock_advance: String,
    pub ready_node_selection: String,
    pub interval_regeneration: String,
    pub wake_retirement_and_reschedule: String,
    pub previous_value_lookup: String,
    pub branch_restore: String,
    pub diagnostics_expansion: String,
    pub prohibited_failure_modes: Vec<TemporalPerformanceFailureMode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalDiagnosticsSummary {
    pub profile: DiagnosticsTier,
    pub clock_basis: RuntimeClockBasis,
    pub wake_summary: TemporalWakeSummary,
    pub frontier: TemporalFrontierSnapshot,
    pub artifact: crate::logic::transaction::TemporalReconstructabilityArtifact,
    pub telemetry: TemporalTelemetry,
    pub cost_contracts: TemporalCostContractSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventEpochHistorySummary {
    pub epochs: Vec<EventEpochSummary>,
}

impl Default for TemporalCostContractSummary {
    fn default() -> Self {
        Self {
            temporal_registration_lowering:
                "declared temporal nodes admitted by the current plan or explicit node set"
                    .to_owned(),
            clock_advance:
                "validated clock request only; due-wake promotion is a separate frontier operation"
                    .to_owned(),
            ready_node_selection:
                "due scheduled frontier width plus ready frontier maintenance, never total graph size"
                    .to_owned(),
            interval_regeneration:
                "due recurring wakes and missed-tick policy outcome, not total elapsed periods"
                    .to_owned(),
            wake_retirement_and_reschedule:
                "affected owner wake footprint, not total temporal registry breadth".to_owned(),
            previous_value_lookup: "committed branch lineage access for the requested node"
                .to_owned(),
            branch_restore:
                "retained branch-local temporal state and summaries, not raw condition rediscovery"
                    .to_owned(),
            diagnostics_expansion:
                "retained temporal artifact expansion only; diagnostics do not re-decide readiness"
                    .to_owned(),
            prohibited_failure_modes: vec![
                TemporalPerformanceFailureMode::TemporalBroadScan,
                TemporalPerformanceFailureMode::IntervalCatchUpExplosion,
                TemporalPerformanceFailureMode::WakeAllocationChurn,
                TemporalPerformanceFailureMode::BranchRestoreTemporalRebuild,
                TemporalPerformanceFailureMode::RescheduleBreadthLeak,
            ],
        }
    }
}

impl TemporalDiagnosticsSummary {
    pub(crate) fn from_artifact(
        profile: DiagnosticsTier,
        frontier: TemporalFrontierSnapshot,
        artifact: crate::logic::transaction::TemporalReconstructabilityArtifact,
        telemetry: TemporalTelemetry,
    ) -> Self {
        Self {
            profile,
            clock_basis: artifact.clock_basis,
            wake_summary: artifact.wake_summary,
            frontier,
            artifact,
            telemetry,
            cost_contracts: TemporalCostContractSummary::default(),
        }
    }

    pub fn with_profile(&self, profile: DiagnosticsTier) -> Self {
        let mut cloned = self.clone();
        cloned.profile = profile;
        cloned
    }
}
