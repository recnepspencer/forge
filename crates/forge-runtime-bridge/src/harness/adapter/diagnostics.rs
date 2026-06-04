use std::collections::BTreeMap;

use forge_harness::facade::{
    diagnostics_id, DiagnosticsHarnessAdapter, DiagnosticsRecord, ExecutionProfile, HarnessAdapter,
    RecordSchemaVersion, ScenarioFixture,
};

use super::terminal_report_export::diagnostics_summary_json;
use super::types::{BridgeHarnessAdapter, BridgeHarnessError, BridgeHarnessSession};

impl DiagnosticsHarnessAdapter for BridgeHarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &BridgeHarnessSession,
        _fixture: &ScenarioFixture<crate::harness::fixtures::BridgeHarnessFixture>,
        profile: &ExecutionProfile,
    ) -> Result<DiagnosticsRecord, BridgeHarnessError> {
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let run_id_value =
            forge_harness::facade::RunId::new(format!("bridge-diagnostics:{}", profile.name));
        Ok(DiagnosticsRecord {
            schema_version: RecordSchemaVersion::V1,
            diagnostics_id: diagnostics_id(&run_id_value),
            run_id: run_id_value,
            adapter_name: self.adapter_name().to_string(),
            profile_name: profile.name.clone(),
            level: profile.diagnostics_level,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: diagnostics_summary_json(runtime_bridge),
            extensions: BTreeMap::new(),
        })
    }
}
