use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::capture::{
    DiagnosticsRecord, EventRecord, EventStreamRecord, ExplanationRecord, ProvenanceRecord,
    RunRecord, ScenarioRecord, SnapshotRecord,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessCoreBundle<TargetId = String> {
    pub scenario: ScenarioRecord,
    pub pre_snapshot: Option<SnapshotRecord<TargetId>>,
    pub run: RunRecord<TargetId>,
    pub post_snapshot: Option<SnapshotRecord<TargetId>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessObservedBundle<TargetId = String> {
    pub core: HarnessCoreBundle<TargetId>,
    pub diagnostics: Option<DiagnosticsRecord>,
    pub explanations: Vec<ExplanationRecord<TargetId>>,
    pub provenance: Vec<ProvenanceRecord<TargetId>>,
    pub events: Vec<EventRecord<TargetId>>,
    pub performance: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessDiagnosedBundle<TargetId = String> {
    pub core: HarnessCoreBundle<TargetId>,
    pub diagnostics: Option<DiagnosticsRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessTimelineBundle<TargetId = String> {
    pub core: HarnessCoreBundle<TargetId>,
    pub events: Vec<EventRecord<TargetId>>,
    pub event_streams: Vec<EventStreamRecord<TargetId>>,
    pub performance: Option<Value>,
}
