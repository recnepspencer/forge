use std::collections::{BTreeMap, VecDeque};
use std::convert::Infallible;

use serde_json::json;

use crate::capture::{
    DiagnosticsLevel, DiagnosticsRecord, EventCategory, EventRecord, EventStreamRecord,
    ExplanationRecord, ObservationStatus, ProvenanceRecord, RecordSchemaVersion, RunOutcome,
    RunRecord, RunStatus, ScenarioRecord, SnapshotObservation, SnapshotPayload, SnapshotRecord,
    StructuredValue, TargetStatusRecord,
};
use crate::identity::{event_stream_id, replay_id, run_id, scenario_id, snapshot_id};
use crate::replay::{ReplayRecord, ReplayRequest};
use crate::runtime::{
    DiagnosticsHarnessAdapter, EventHarnessAdapter, EventStreamHarnessAdapter,
    ExplanationHarnessAdapter, HarnessAdapter, HarnessCapabilities, PerformanceHarnessAdapter,
    ProvenanceHarnessAdapter, ReplayHarnessAdapter,
};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdapterDoubleRuntime {
    pub loaded_fixture_names: Vec<String>,
    pub applied_batch_names: Vec<String>,
}

pub struct AdapterDouble {
    name: &'static str,
    capabilities: HarnessCapabilities,
    next_run: VecDeque<RunRecord<String>>,
    next_snapshot: VecDeque<SnapshotRecord<String>>,
}

impl AdapterDouble {
    pub fn new(name: &'static str, capabilities: HarnessCapabilities) -> Self {
        Self {
            name,
            capabilities,
            next_run: VecDeque::new(),
            next_snapshot: VecDeque::new(),
        }
    }

    pub fn push_run_record(&mut self, record: RunRecord<String>) {
        self.next_run.push_back(record);
    }

    pub fn push_snapshot_record(&mut self, record: SnapshotRecord<String>) {
        self.next_snapshot.push_back(record);
    }

    fn default_run_record(
        &self,
        fixture: &ScenarioFixture<serde_json::Value>,
        request: &ExecutionRequest<String>,
        profile: &ExecutionProfile,
    ) -> RunRecord<String> {
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        RunRecord {
            schema_version: RecordSchemaVersion::V1,
            run_id: run_id.clone(),
            scenario_id,
            adapter_name: self.name.to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            feed_batch: request.feed_batch.clone(),
            execution_mode: profile.execution_mode,
            diagnostics_level: profile.diagnostics_level,
            status: RunStatus::Succeeded,
            outcome: RunOutcome::Completed,
            budget_usage: None,
            requested_targets: request.targets.clone(),
            target_statuses: request
                .targets
                .iter()
                .cloned()
                .map(|target| TargetStatusRecord {
                    target,
                    status: ObservationStatus::Clean,
                    detail: None,
                })
                .collect(),
            changed_targets: request.targets.clone(),
            attachments: Vec::new(),
            summary: json!({ "double": self.name }),
            extensions: BTreeMap::new(),
        }
    }

    fn default_snapshot_record(
        &self,
        fixture: &ScenarioFixture<serde_json::Value>,
        request: &ExecutionRequest<String>,
        profile: &ExecutionProfile,
    ) -> SnapshotRecord<String> {
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        SnapshotRecord {
            schema_version: RecordSchemaVersion::V1,
            snapshot_id: snapshot_id(&run_id, "capture"),
            run_id,
            adapter_name: self.name.to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            observations: request
                .targets
                .iter()
                .cloned()
                .map(|target| SnapshotObservation {
                    target: target.clone(),
                    status: ObservationStatus::Unknown,
                    detail: None,
                    value: Some(SnapshotPayload::Structured(StructuredValue::Json(
                        json!({ "target": target }),
                    ))),
                })
                .collect(),
            attachments: Vec::new(),
            extensions: BTreeMap::new(),
        }
    }
}

impl HarnessAdapter for AdapterDouble {
    type Runtime = AdapterDoubleRuntime;
    type Fixture = serde_json::Value;
    type Mutation = serde_json::Value;
    type TargetId = String;
    type Error = Infallible;

    fn adapter_name(&self) -> &'static str {
        self.name
    }

    fn capabilities(&self) -> HarnessCapabilities {
        self.capabilities.clone()
    }

    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
        Ok(AdapterDoubleRuntime::default())
    }

    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error> {
        runtime.loaded_fixture_names.push(fixture.name.clone());
        Ok(())
    }

    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error> {
        runtime.applied_batch_names.push(batch.name.clone());
        Ok(())
    }

    fn execute(
        &self,
        _runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<Self::TargetId>, Self::Error> {
        Ok(self
            .next_run
            .front()
            .cloned()
            .unwrap_or_else(|| self.default_run_record(fixture, request, profile)))
    }

    fn capture_snapshot(
        &self,
        _runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<SnapshotRecord<Self::TargetId>, Self::Error> {
        Ok(self
            .next_snapshot
            .front()
            .cloned()
            .unwrap_or_else(|| self.default_snapshot_record(fixture, request, profile)))
    }

    fn scenario_record(&self, fixture: &ScenarioFixture<Self::Fixture>) -> ScenarioRecord {
        ScenarioRecord::new(
            scenario_id(&fixture.name),
            fixture.name.clone(),
            fixture.declared_inputs.clone(),
            fixture.declared_observations.clone(),
            fixture.metadata.clone(),
        )
    }
}

impl EventHarnessAdapter for AdapterDouble {
    fn capture_events(
        &self,
        _runtime: &Self::Runtime,
        _fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<EventRecord<Self::TargetId>>, Self::Error> {
        Ok(request
            .targets
            .iter()
            .cloned()
            .map(|target| EventRecord {
                schema_version: RecordSchemaVersion::V1,
                adapter_name: self.name.to_string(),
                category: EventCategory::ExecutionFinished,
                target: Some(target),
                detail: Some("double event".to_string()),
                time_marker: profile.time_marker.clone(),
                feed_batch: request.feed_batch.clone(),
                fields: BTreeMap::new(),
            })
            .collect())
    }
}

impl DiagnosticsHarnessAdapter for AdapterDouble {
    fn capture_diagnostics(
        &self,
        _runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<DiagnosticsRecord, Self::Error> {
        let scenario_id = scenario_id(&fixture.name);
        Ok(DiagnosticsRecord {
            schema_version: RecordSchemaVersion::V1,
            diagnostics_id: crate::identity::diagnostics_id(&run_id(
                &scenario_id,
                &profile.name,
                "diagnostics",
            )),
            run_id: run_id(&scenario_id, &profile.name, "diagnostics"),
            adapter_name: self.name.to_string(),
            profile_name: profile.name.clone(),
            level: DiagnosticsLevel::Operational,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: json!({"double": true}),
            extensions: BTreeMap::new(),
        })
    }
}

impl ExplanationHarnessAdapter for AdapterDouble {
    fn capture_explanations(
        &self,
        _runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ExplanationRecord<Self::TargetId>>, Self::Error> {
        let scenario_id = scenario_id(&fixture.name);
        let current_run_id = run_id(&scenario_id, &profile.name, &request.name);
        Ok(request
            .targets
            .iter()
            .cloned()
            .map(|target| ExplanationRecord {
                schema_version: RecordSchemaVersion::V1,
                explanation_id: crate::identity::explanation_id(&current_run_id, &target),
                run_id: current_run_id.clone(),
                adapter_name: self.name.to_string(),
                target,
                time_marker: profile.time_marker.clone(),
                attachments: Vec::new(),
                summary: json!({"kind": "explanation"}),
                extensions: BTreeMap::new(),
            })
            .collect())
    }
}

impl ProvenanceHarnessAdapter for AdapterDouble {
    fn capture_provenance(
        &self,
        _runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ProvenanceRecord<Self::TargetId>>, Self::Error> {
        let scenario_id = scenario_id(&fixture.name);
        let current_run_id = run_id(&scenario_id, &profile.name, &request.name);
        Ok(request
            .targets
            .iter()
            .cloned()
            .map(|target| ProvenanceRecord {
                schema_version: RecordSchemaVersion::V1,
                provenance_id: crate::identity::provenance_id(&current_run_id, &target),
                run_id: current_run_id.clone(),
                adapter_name: self.name.to_string(),
                target,
                time_marker: profile.time_marker.clone(),
                attachments: Vec::new(),
                summary: json!({"kind": "provenance"}),
                extensions: BTreeMap::new(),
            })
            .collect())
    }
}

impl EventStreamHarnessAdapter for AdapterDouble {
    fn capture_event_streams(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<EventStreamRecord<Self::TargetId>>, Self::Error> {
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        Ok(vec![EventStreamRecord {
            schema_version: RecordSchemaVersion::V1,
            event_stream_id: event_stream_id(&run_id, &fixture.name),
            run_id,
            adapter_name: self.name.to_string(),
            stream_name: format!("{}-stream", fixture.name),
            time_marker: profile.time_marker.clone(),
            feed_batch: request.feed_batch.clone(),
            events: self.capture_events(runtime, fixture, request, profile)?,
            attachments: Vec::new(),
        }])
    }
}

impl PerformanceHarnessAdapter for AdapterDouble {
    fn capture_performance(
        &self,
        runtime: &Self::Runtime,
        _fixture: &ScenarioFixture<Self::Fixture>,
        _profile: &ExecutionProfile,
    ) -> Result<serde_json::Value, Self::Error> {
        Ok(json!({
            "loaded_fixture_count": runtime.loaded_fixture_names.len(),
            "applied_batch_count": runtime.applied_batch_names.len(),
        }))
    }
}

impl ReplayHarnessAdapter for AdapterDouble {
    fn capture_replay(
        &self,
        _runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        replay: &ReplayRequest<Self::TargetId>,
    ) -> Result<ReplayRecord<Self::TargetId>, Self::Error> {
        let scenario_id = scenario_id(&fixture.name);
        let source_run_id = replay.source_run.run_id.clone();
        Ok(ReplayRecord {
            schema_version: RecordSchemaVersion::V1,
            replay_id: replay_id(&source_run_id, &replay.name),
            source_run_id,
            scenario_id,
            adapter_name: self.name.to_string(),
            scenario_name: fixture.name.clone(),
            replay_name: replay.name.clone(),
            requested_targets: replay.request.targets.clone(),
            summary: json!({
                "source_status": format!("{:?}", replay.source_run.status),
                "profile": replay.profile.name,
            }),
            attachments: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capture::{DiagnosticsLevel, ExecutionMode};
    use crate::comparison::{ComparisonMode, ComparisonProfile};
    use crate::replay::ReplayRequest;
    use crate::runtime::{AdapterSupport, CaptureDepth, HarnessCapabilities, HarnessRunner};
    use crate::scenario::{
        CaptureMask, ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioPlan,
    };
    use crate::timeline::{ClockDomain, ExecutionPhase, FeedBatch, TimeMarker};
    use crate::workload::{WorkBudget, WorkloadProfile};

    use super::AdapterDouble;

    #[test]
    fn harness_runner_executes_with_adapter_double() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.clock_domains.insert(ClockDomain::Logical);

        let adapter = AdapterDouble::new("double", capabilities);
        let runner = HarnessRunner::new(adapter);
        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::new("request", vec!["a".to_string()]);
        let profile = ExecutionProfile::serial("serial");
        let batch = MutationBatch::new("mutation").push(json!({ "bump": 1 }));

        let bundle = runner
            .execute_core(&fixture, Some(&batch), &request, &profile)
            .unwrap();

        assert_eq!(bundle.scenario.scenario_name, "fixture");
        assert!(bundle.scenario.scenario_id.0.contains("scenario:fixture"));
        assert_eq!(bundle.run.profile_name, "serial");
        assert!(bundle.pre_snapshot.is_some());
        assert!(bundle.post_snapshot.is_some());
    }

    #[test]
    fn compare_runs_uses_harness_comparison_profile() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.comparison_modes.insert(ComparisonMode::Exact);

        let adapter = AdapterDouble::new("double", capabilities);
        let runner = HarnessRunner::new(adapter);
        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::new("request", vec!["a".to_string()]);
        let profile = ExecutionProfile::serial("serial");

        let left = runner
            .execute_core(&fixture, None, &request, &profile)
            .unwrap()
            .run;
        let right = runner
            .execute_core(&fixture, None, &request, &profile)
            .unwrap()
            .run;

        let comparison = runner
            .compare_runs(&left, &right, &ComparisonProfile::default())
            .unwrap();
        assert!(comparison.matched);
    }

    #[test]
    fn execute_streamed_captures_event_streams_and_performance() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.clock_domains.insert(ClockDomain::Logical);
        capabilities
            .execution_phases
            .insert(ExecutionPhase::Evaluate);
        capabilities.workload_budget_support = AdapterSupport::Supported;

        let adapter = AdapterDouble::new("double", capabilities);
        let runner = HarnessRunner::new(adapter);
        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::new("request", vec!["a".to_string()])
            .with_feed_batch(FeedBatch::new("feed", 1, 1).with_phase(ExecutionPhase::Evaluate));
        let profile = ExecutionProfile::serial("serial")
            .with_time_marker(TimeMarker {
                clock_domain: ClockDomain::Logical,
                sequence: 1,
                tick: Some(1),
                phase: Some(ExecutionPhase::Evaluate),
                wall_time_rfc3339: None,
            })
            .with_workload(
                WorkloadProfile::new("frame")
                    .with_phase(ExecutionPhase::Evaluate)
                    .with_budget(WorkBudget {
                        max_operations: Some(10),
                        max_duration_millis: Some(5),
                        frame_budget_micros: Some(1_000),
                    }),
            )
            .with_work_budget(WorkBudget {
                max_operations: Some(10),
                max_duration_millis: Some(5),
                frame_budget_micros: Some(1_000),
            });

        let bundle = runner
            .execute_streamed(&fixture, None, &request, &profile)
            .unwrap();
        assert_eq!(bundle.event_streams.len(), 1);
        assert!(bundle.performance.is_some());
    }

    #[test]
    fn execute_replay_returns_replay_record() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.clock_domains.insert(ClockDomain::Logical);
        capabilities.replay_support = AdapterSupport::Supported;

        let adapter = AdapterDouble::new("double", capabilities);
        let runner = HarnessRunner::new(adapter);
        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::new("request", vec!["a".to_string()]);
        let profile = ExecutionProfile::serial("serial");
        let source_run = runner
            .execute_core(&fixture, None, &request, &profile)
            .unwrap()
            .run;
        let replay = ReplayRequest {
            name: "replay".to_string(),
            source_run,
            request,
            profile,
        };

        let record = runner.execute_replay(&fixture, None, &replay).unwrap();
        assert_eq!(record.replay_name, "replay");
    }

    #[test]
    fn capture_policy_filters_snapshots_and_observed_artifacts() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.clock_domains.insert(ClockDomain::Logical);

        let adapter = AdapterDouble::new("double", capabilities);
        let runner = HarnessRunner::new(adapter);
        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::new("request", vec!["a".to_string(), "b".to_string()])
            .capture_only_targets(vec!["a".to_string()])
            .with_capture_mask(CaptureMask {
                diagnostics: false,
                explanations: false,
                provenance: false,
                ..CaptureMask::default()
            });
        let profile = ExecutionProfile::serial("serial");

        let core = runner
            .execute_core(&fixture, None, &request, &profile)
            .unwrap();
        let observed = runner
            .execute_observed(&fixture, None, &request, &profile)
            .unwrap();

        assert_eq!(core.pre_snapshot.as_ref().unwrap().observations.len(), 1);
        assert!(observed.diagnostics.is_none());
        assert!(observed.explanations.is_empty());
        assert!(observed.provenance.is_empty());
    }
}
