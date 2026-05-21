use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::specialist::VersionSummary;
use crate::runtime::summaries::{
    FlowSurfaceSummary, LineageSummary, ObservationSurfaceSummary, ReplaySummary, RunSummary,
    WhySummary,
};
use forge_signal::facade::adapters::RuntimeProofReport;
use forge_signal::facade::diagnostics::ExecutionHistorySummary;

use super::WorkerRuntimeShell;

impl WorkerRuntimeShell {
    pub fn why(&self, id: &str) -> Result<WhySummary, ForgeSignalJsError> {
        self.core.why(id)
    }

    pub fn latest_flow(&self) -> Result<Option<FlowSurfaceSummary>, ForgeSignalJsError> {
        self.core.latest_flow()
    }

    pub fn latest_observation(
        &self,
    ) -> Result<Option<ObservationSurfaceSummary>, ForgeSignalJsError> {
        self.core.latest_observation()
    }

    pub fn recent_history(&self) -> Result<Vec<ExecutionHistorySummary>, ForgeSignalJsError> {
        self.core.recent_history()
    }

    pub fn replay_for_id(&mut self, id: &str) -> Result<ReplaySummary, ForgeSignalJsError> {
        self.core.replay_for_id(id)
    }

    pub fn lineage_for_id(&mut self, id: &str) -> Result<LineageSummary, ForgeSignalJsError> {
        self.core.lineage_for_id(id)
    }

    pub fn read_versions(
        &mut self,
        ids: Vec<String>,
    ) -> Result<Vec<VersionSummary>, ForgeSignalJsError> {
        self.core.read_versions(ids)
    }

    pub fn evaluate_dirty(&mut self) -> Result<RunSummary, ForgeSignalJsError> {
        self.core.evaluate_dirty()
    }

    pub fn export_definitions(&mut self) -> Result<RuntimeDefinitionEnvelope, ForgeSignalJsError> {
        self.core.export_definitions()
    }

    pub fn runtime_proof_report(&self) -> RuntimeProofReport {
        self.core.runtime_proof_report()
    }
}
