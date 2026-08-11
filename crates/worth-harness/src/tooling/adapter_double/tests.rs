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
