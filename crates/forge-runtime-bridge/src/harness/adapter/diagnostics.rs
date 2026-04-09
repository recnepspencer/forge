use std::collections::BTreeMap;

use forge_harness::facade::{
    diagnostics_id, DiagnosticsHarnessAdapter, DiagnosticsRecord, ExecutionProfile, HarnessAdapter,
    RecordSchemaVersion, ScenarioFixture,
};
use serde_json::json;

use super::support::route_record_json;
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
            summary: json!({
                "tier": format!("{:?}", runtime_bridge.diagnostics().tier()),
                "record_count": runtime_bridge.diagnostics().route_records().len(),
                "source_materialization_record_count": runtime_bridge
                    .diagnostics()
                    .source_materialization_records()
                    .len(),
                "source_failure_record_count": runtime_bridge
                    .diagnostics()
                    .source_failure_records()
                    .len(),
                "route_records": runtime_bridge
                    .diagnostics()
                    .route_records()
                    .into_iter()
                    .map(route_record_json)
                    .collect::<Vec<_>>(),
            }),
            extensions: BTreeMap::new(),
        })
    }
}
