use std::collections::BTreeMap;

use forge_harness::facade::{
    diagnostics_id, explanation_id, provenance_id, run_id, scenario_id, DiagnosticsHarnessAdapter,
    DiagnosticsRecord, ExecutionProfile, ExecutionRequest, ExplanationHarnessAdapter,
    ExplanationRecord, HarnessAdapter, PerformanceHarnessAdapter, ProvenanceHarnessAdapter, ProvenanceRecord,
    RecordSchemaVersion, ScenarioFixture,
};
use serde_json::{json, Value};

use super::adapter_core::SignalHarnessAdapter;

impl DiagnosticsHarnessAdapter for SignalHarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<DiagnosticsRecord, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, "diagnostics");
        let summary = runtime
            .graph
            .diagnostics_summary(Self::diagnostics_profile(profile.diagnostics_level));

        Ok(DiagnosticsRecord {
            schema_version: RecordSchemaVersion::V1,
            diagnostics_id: diagnostics_id(&run_id),
            run_id,
            adapter_name: self.adapter_name().to_string(),
            profile_name: profile.name.clone(),
            level: profile.diagnostics_level,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: serde_json::to_value(summary).unwrap_or_else(|_| json!({})),
            extensions: BTreeMap::new(),
        })
    }
}

impl ExplanationHarnessAdapter for SignalHarnessAdapter {
    fn capture_explanations(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ExplanationRecord<Self::TargetId>>, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        request
            .targets
            .iter()
            .map(|label| {
                let node = runtime.resolve(label)?;
                let explanation = runtime.graph.explain(node)?;
                Ok(ExplanationRecord {
                    schema_version: RecordSchemaVersion::V1,
                    explanation_id: explanation_id(&run_id, label),
                    run_id: run_id.clone(),
                    adapter_name: self.adapter_name().to_string(),
                    target: label.clone(),
                    time_marker: profile.time_marker.clone(),
                    attachments: Vec::new(),
                    summary: Self::explanation_summary(&explanation),
                    extensions: BTreeMap::new(),
                })
            })
            .collect()
    }
}

impl ProvenanceHarnessAdapter for SignalHarnessAdapter {
    fn capture_provenance(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ProvenanceRecord<Self::TargetId>>, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        request
            .targets
            .iter()
            .map(|label| {
                let node = runtime.resolve(label)?;
                let explanation = runtime.graph.explain(node)?;
                Ok(ProvenanceRecord {
                    schema_version: RecordSchemaVersion::V1,
                    provenance_id: provenance_id(&run_id, label),
                    run_id: run_id.clone(),
                    adapter_name: self.adapter_name().to_string(),
                    target: label.clone(),
                    time_marker: profile.time_marker.clone(),
                    attachments: Vec::new(),
                    summary: json!({
                        "execution_record_id": explanation.execution_record_id,
                        "upstream_count": explanation.upstream.len(),
                        "propagation_suppressed": explanation.propagation_suppressed,
                    }),
                    extensions: BTreeMap::new(),
                })
            })
            .collect()
    }
}

impl PerformanceHarnessAdapter for SignalHarnessAdapter {
    fn capture_performance(
        &self,
        runtime: &Self::Runtime,
        _fixture: &ScenarioFixture<Self::Fixture>,
        _profile: &ExecutionProfile,
    ) -> Result<Value, Self::Error> {
        let runtime = runtime.runtime()?;
        Ok(serde_json::to_value(runtime.graph.metrics()).unwrap_or_else(|_| json!({})))
    }
}
