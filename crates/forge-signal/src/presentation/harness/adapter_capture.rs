use std::collections::BTreeMap;

use forge_harness::facade::{
    diagnostics_id, explanation_id, provenance_id, run_id, scenario_id, DiagnosticsHarnessAdapter,
    DiagnosticsRecord, ExecutionProfile, ExecutionRequest, ExplanationHarnessAdapter,
    ExplanationRecord, HarnessAdapter, PerformanceHarnessAdapter, ProvenanceHarnessAdapter,
    ProvenanceRecord, RecordSchemaVersion, ReplayHarnessAdapter, ScenarioFixture,
};
use forge_harness::facade::{replay_id, ReplayRecord, ReplayRequest};
use serde_json::{json, Value};

use super::adapter_core::SignalHarnessAdapter;
use crate::facade::CORE_STORAGE_PROFILE_ID;

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
        let runtime_policy = Self::runtime_policy(profile.diagnostics_level);

        Ok(DiagnosticsRecord {
            schema_version: RecordSchemaVersion::V2,
            diagnostics_id: diagnostics_id(&run_id),
            run_id,
            adapter_name: self.adapter_name().to_string(),
            profile_name: profile.name.clone(),
            level: profile.diagnostics_level,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: serde_json::to_value(summary).unwrap_or_else(|_| json!({})),
            extensions: BTreeMap::from([
                (
                    "runtime_policy".to_string(),
                    SignalHarnessAdapter::runtime_policy_summary(runtime_policy),
                ),
                (
                    "core_storage_profile".to_string(),
                    json!(CORE_STORAGE_PROFILE_ID),
                ),
            ]),
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
                let (explanation, materialization_mode) = runtime.graph.explain_artifact(node)?;
                let summary = if let Some(explanation) = explanation {
                    if let Some(fact) = runtime.graph.explanation_fact(node) {
                        json!({
                            "node": fact.node.to_string(),
                            "state": fact.state,
                            "execution_record_id": fact.execution_record_id,
                            "semantic_segment_id": fact.semantic_segment_id,
                            "upstream_count": fact.upstream_count,
                            "propagation_suppressed": fact.propagation_suppressed,
                            "changed_region_count": fact.changed_region_count,
                            "output_change": fact.output_change,
                            "artifact_materialization": SignalHarnessAdapter::artifact_materialization_label(materialization_mode),
                            "artifact_materialization_message": materialization_mode.message(),
                        })
                    } else {
                        let mut summary = Self::explanation_summary(&explanation);
                        if let Some(map) = summary.as_object_mut() {
                            map.insert(
                                "artifact_materialization".to_string(),
                                json!(SignalHarnessAdapter::artifact_materialization_label(
                                    materialization_mode
                                )),
                            );
                            map.insert(
                                "artifact_materialization_message".to_string(),
                                json!(materialization_mode.message()),
                            );
                        }
                        summary
                    }
                } else {
                    json!({
                        "artifact_materialization": SignalHarnessAdapter::artifact_materialization_label(materialization_mode),
                        "artifact_materialization_message": materialization_mode.message(),
                        "available": false,
                    })
                };
                Ok(ExplanationRecord {
                    schema_version: RecordSchemaVersion::V2,
                    explanation_id: explanation_id(&run_id, label),
                    run_id: run_id.clone(),
                    adapter_name: self.adapter_name().to_string(),
                    target: label.clone(),
                    time_marker: profile.time_marker.clone(),
                    attachments: Vec::new(),
                    summary,
                    extensions: BTreeMap::from([
                        (
                            "runtime_policy".to_string(),
                            SignalHarnessAdapter::runtime_policy_summary(Self::runtime_policy(
                                profile.diagnostics_level,
                            )),
                        ),
                        (
                            "artifact_materialization".to_string(),
                            json!(SignalHarnessAdapter::artifact_materialization_label(
                                materialization_mode
                            )),
                        ),
                        (
                            "artifact_materialization_message".to_string(),
                            json!(materialization_mode.message()),
                        ),
                        (
                            "core_storage_profile".to_string(),
                            json!(CORE_STORAGE_PROFILE_ID),
                        ),
                    ]),
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
                let (provenance, materialization_mode) = runtime.graph.provenance_artifact(node)?;
                let summary = if let Some(fact) = provenance {
                    json!({
                        "execution_record_id": fact.execution_record_id,
                        "semantic_segment_id": fact.semantic_segment_id,
                        "vertex_count": fact.vertices.len(),
                        "edge_count": fact.edges.len(),
                        "propagation_suppressed": fact.propagation_suppressed,
                        "vertices": fact.vertices,
                        "edges": fact.edges,
                        "causality_kind": fact.causality_kind,
                        "artifact_materialization": SignalHarnessAdapter::artifact_materialization_label(materialization_mode),
                        "artifact_materialization_message": materialization_mode.message(),
                    })
                } else {
                    json!({
                        "artifact_materialization": SignalHarnessAdapter::artifact_materialization_label(materialization_mode),
                        "artifact_materialization_message": materialization_mode.message(),
                        "available": false,
                    })
                };
                Ok(ProvenanceRecord {
                    schema_version: RecordSchemaVersion::V2,
                    provenance_id: provenance_id(&run_id, label),
                    run_id: run_id.clone(),
                    adapter_name: self.adapter_name().to_string(),
                    target: label.clone(),
                    time_marker: profile.time_marker.clone(),
                    attachments: Vec::new(),
                    summary,
                    extensions: BTreeMap::from([
                        (
                            "runtime_policy".to_string(),
                            SignalHarnessAdapter::runtime_policy_summary(Self::runtime_policy(
                                profile.diagnostics_level,
                            )),
                        ),
                        (
                            "artifact_materialization".to_string(),
                            json!(SignalHarnessAdapter::artifact_materialization_label(
                                materialization_mode
                            )),
                        ),
                        (
                            "artifact_materialization_message".to_string(),
                            json!(materialization_mode.message()),
                        ),
                        (
                            "core_storage_profile".to_string(),
                            json!(CORE_STORAGE_PROFILE_ID),
                        ),
                    ]),
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

impl ReplayHarnessAdapter for SignalHarnessAdapter {
    fn capture_replay(
        &self,
        _runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        replay: &ReplayRequest<Self::TargetId>,
    ) -> Result<ReplayRecord<Self::TargetId>, Self::Error> {
        let scenario_id = scenario_id(&fixture.name);
        let source_run_id = replay.source_run.run_id.clone();
        let runtime = _runtime.runtime()?;
        let events = runtime
            .graph
            .replay_events()
            .iter()
            .map(|event| {
                json!({
                    "sequence": event.sequence,
                    "kind": format!("{:?}", event.kind),
                    "node": event.node.map(|node| node.to_string()),
                    "execution_record_id": event.execution_record_id,
                    "semantic_segment_id": event.semantic_segment_id,
                    "detail": event.detail,
                })
            })
            .collect::<Vec<_>>();
        Ok(ReplayRecord {
            schema_version: RecordSchemaVersion::V2,
            replay_id: replay_id(&source_run_id, &replay.name),
            source_run_id,
            scenario_id,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            replay_name: replay.name.clone(),
            requested_targets: replay.request.targets.clone(),
            summary: json!({
                "profile": replay.profile.name,
                "runtime_policy": SignalHarnessAdapter::runtime_policy_summary(
                    Self::runtime_policy(replay.profile.diagnostics_level)
                ),
                "core_storage_profile": CORE_STORAGE_PROFILE_ID,
                "source_status": format!("{:?}", replay.source_run.status),
                "requested_targets": replay.request.targets,
                "events": events,
            }),
            attachments: Vec::new(),
        })
    }
}
