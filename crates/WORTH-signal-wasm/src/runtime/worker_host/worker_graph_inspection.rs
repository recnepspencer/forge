use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::specialist::VersionSummary;
use crate::runtime::summaries::{
    FlowSurfaceSummary, LineageSummary, ObservationSurfaceSummary, ReplaySummary, RunSummary,
    WhySummary,
};
use worth_signal::facade::adapters::RuntimeProofReport;
use worth_signal::facade::diagnostics::ExecutionHistorySummary;

use super::WorkerRuntimeShell;

impl WorkerRuntimeShell {
    pub fn why(&self, id: &str) -> Result<WhySummary, WORTHSignalJsError> {
        self.core.why(id)
    }

    pub fn latest_flow(&self) -> Result<Option<FlowSurfaceSummary>, WORTHSignalJsError> {
        self.core.latest_flow()
    }

    pub fn latest_observation(
        &self,
    ) -> Result<Option<ObservationSurfaceSummary>, WORTHSignalJsError> {
        self.core.latest_observation()
    }

    pub fn recent_history(&self) -> Result<Vec<ExecutionHistorySummary>, WORTHSignalJsError> {
        self.core.recent_history()
    }

    pub fn replay_for_id(&mut self, id: &str) -> Result<ReplaySummary, WORTHSignalJsError> {
        self.core.replay_for_id(id)
    }

    pub fn lineage_for_id(&mut self, id: &str) -> Result<LineageSummary, WORTHSignalJsError> {
        self.core.lineage_for_id(id)
    }

    pub fn read_versions(
        &mut self,
        ids: Vec<String>,
    ) -> Result<Vec<VersionSummary>, WORTHSignalJsError> {
        self.core.read_versions(ids)
    }

    pub fn evaluate_dirty(&mut self) -> Result<RunSummary, WORTHSignalJsError> {
        self.core.evaluate_dirty()
    }

    pub fn export_definitions(&mut self) -> Result<RuntimeDefinitionEnvelope, WORTHSignalJsError> {
        self.core.export_definitions()
    }

    pub fn runtime_proof_report(&self) -> RuntimeProofReport {
        self.core.runtime_proof_report()
    }
}
