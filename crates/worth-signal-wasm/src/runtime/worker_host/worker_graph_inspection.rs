use crate::boundary::errors::WorthSignalJsError;
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
    pub fn why(&self, id: &str) -> Result<WhySummary, WorthSignalJsError> {
        self.core.why(id)
    }

    pub fn latest_flow(&self) -> Result<Option<FlowSurfaceSummary>, WorthSignalJsError> {
        self.core.latest_flow()
    }

    pub fn latest_observation(
        &self,
    ) -> Result<Option<ObservationSurfaceSummary>, WorthSignalJsError> {
        self.core.latest_observation()
    }

    pub fn recent_history(&self) -> Result<Vec<ExecutionHistorySummary>, WorthSignalJsError> {
        self.core.recent_history()
    }

    pub fn replay_for_id(&mut self, id: &str) -> Result<ReplaySummary, WorthSignalJsError> {
        self.core.replay_for_id(id)
    }

    pub fn lineage_for_id(&mut self, id: &str) -> Result<LineageSummary, WorthSignalJsError> {
        self.core.lineage_for_id(id)
    }

    pub fn read_versions(
        &mut self,
        ids: Vec<String>,
    ) -> Result<Vec<VersionSummary>, WorthSignalJsError> {
        self.core.read_versions(ids)
    }

    pub fn evaluate_dirty(&mut self) -> Result<RunSummary, WorthSignalJsError> {
        self.core.evaluate_dirty()
    }

    pub fn export_definitions(&mut self) -> Result<RuntimeDefinitionEnvelope, WorthSignalJsError> {
        self.core.export_definitions()
    }

    pub fn runtime_proof_report(&self) -> RuntimeProofReport {
        self.core.runtime_proof_report()
    }
}
