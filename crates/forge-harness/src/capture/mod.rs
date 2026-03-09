use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::artifact::{AttachmentRecord, BlobDescriptor};
use crate::identity::{
    DiagnosticsId, EventStreamId, ExplanationId, ProvenanceId, RunId, ScenarioId, SnapshotId,
};
use crate::timeline::{FeedBatch, TimeMarker};
use crate::workload::BudgetUsage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecordSchemaVersion {
    V1,
    V2,
}

impl Default for RecordSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExecutionMode {
    RuntimeDefault,
    Serial,
    StagedParallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiagnosticsLevel {
    Off,
    Operational,
    Development,
    Forensic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RunStatus {
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RunOutcome {
    Completed,
    Deferred,
    BudgetExhausted,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ObservationStatus {
    Clean,
    MaybeStale,
    Dirty,
    Deferred,
    Validated,
    Recomputed,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventCategory {
    MutationApplied,
    ExecutionRequested,
    ExecutionStarted,
    ExecutionFinished,
    TargetValidated,
    TargetDeferred,
    TargetRecomputed,
    FailureEmitted,
    RollbackOccurred,
    DiagnosticsEmitted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioRecord {
    pub schema_version: RecordSchemaVersion,
    pub scenario_id: ScenarioId,
    pub scenario_name: String,
    pub declared_inputs: Vec<String>,
    pub declared_observations: Vec<String>,
    pub metadata: BTreeMap<String, String>,
}

impl ScenarioRecord {
    pub fn new(
        scenario_id: ScenarioId,
        scenario_name: impl Into<String>,
        declared_inputs: Vec<String>,
        declared_observations: Vec<String>,
        metadata: BTreeMap<String, String>,
    ) -> Self {
        Self {
            schema_version: RecordSchemaVersion::V1,
            scenario_id,
            scenario_name: scenario_name.into(),
            declared_inputs,
            declared_observations,
            metadata,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetStatusRecord<TargetId = String> {
    pub target: TargetId,
    pub status: ObservationStatus,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub run_id: RunId,
    pub scenario_id: ScenarioId,
    pub adapter_name: String,
    pub scenario_name: String,
    pub profile_name: String,
    pub time_marker: Option<TimeMarker>,
    pub feed_batch: Option<FeedBatch>,
    pub execution_mode: ExecutionMode,
    pub diagnostics_level: DiagnosticsLevel,
    pub status: RunStatus,
    pub outcome: RunOutcome,
    pub budget_usage: Option<BudgetUsage>,
    pub requested_targets: Vec<TargetId>,
    pub target_statuses: Vec<TargetStatusRecord<TargetId>>,
    pub changed_targets: Vec<TargetId>,
    pub attachments: Vec<AttachmentRecord>,
    pub summary: Value,
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructuredValue {
    Json(Value),
    Text(String),
    Fields(BTreeMap<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryValue {
    pub media_type: String,
    pub bytes: Vec<u8>,
    pub content_hash: Option<String>,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SnapshotPayload {
    Structured(StructuredValue),
    Binary(BinaryValue),
    External(BlobDescriptor),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotObservation<TargetId = String> {
    pub target: TargetId,
    pub status: ObservationStatus,
    pub detail: Option<String>,
    pub value: Option<SnapshotPayload>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub snapshot_id: SnapshotId,
    pub run_id: RunId,
    pub adapter_name: String,
    pub scenario_name: String,
    pub profile_name: String,
    pub time_marker: Option<TimeMarker>,
    pub observations: Vec<SnapshotObservation<TargetId>>,
    pub attachments: Vec<AttachmentRecord>,
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub adapter_name: String,
    pub category: EventCategory,
    pub target: Option<TargetId>,
    pub detail: Option<String>,
    pub time_marker: Option<TimeMarker>,
    pub feed_batch: Option<FeedBatch>,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventStreamRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub event_stream_id: EventStreamId,
    pub run_id: RunId,
    pub adapter_name: String,
    pub stream_name: String,
    pub time_marker: Option<TimeMarker>,
    pub feed_batch: Option<FeedBatch>,
    pub events: Vec<EventRecord<TargetId>>,
    pub attachments: Vec<AttachmentRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsRecord {
    pub schema_version: RecordSchemaVersion,
    pub diagnostics_id: DiagnosticsId,
    pub run_id: RunId,
    pub adapter_name: String,
    pub profile_name: String,
    pub level: DiagnosticsLevel,
    pub time_marker: Option<TimeMarker>,
    pub attachments: Vec<AttachmentRecord>,
    pub summary: Value,
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplanationRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub explanation_id: ExplanationId,
    pub run_id: RunId,
    pub adapter_name: String,
    pub target: TargetId,
    pub time_marker: Option<TimeMarker>,
    pub attachments: Vec<AttachmentRecord>,
    pub summary: Value,
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub provenance_id: ProvenanceId,
    pub run_id: RunId,
    pub adapter_name: String,
    pub target: TargetId,
    pub time_marker: Option<TimeMarker>,
    pub attachments: Vec<AttachmentRecord>,
    pub summary: Value,
    pub extensions: BTreeMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::artifact::BlobDescriptor;

    use super::{BinaryValue, SnapshotPayload, StructuredValue};

    #[test]
    fn snapshot_payload_structured_round_trips() {
        let payload = SnapshotPayload::Structured(StructuredValue::Fields(
            [("value".to_string(), json!(42))].into_iter().collect(),
        ));
        let value = serde_json::to_value(&payload).unwrap();
        let round_trip: SnapshotPayload = serde_json::from_value(value).unwrap();
        match round_trip {
            SnapshotPayload::Structured(StructuredValue::Fields(fields)) => {
                assert_eq!(fields.get("value"), Some(&Value::from(42)));
            }
            _ => panic!("expected structured fields payload"),
        }
    }

    #[test]
    fn snapshot_payload_binary_round_trips() {
        let payload = SnapshotPayload::Binary(BinaryValue {
            media_type: "application/octet-stream".to_string(),
            bytes: vec![1, 2, 3],
            content_hash: Some("hash".to_string()),
            size_bytes: Some(3),
        });
        let value = serde_json::to_value(&payload).unwrap();
        let round_trip: SnapshotPayload = serde_json::from_value(value).unwrap();
        match round_trip {
            SnapshotPayload::Binary(binary) => assert_eq!(binary.bytes, vec![1, 2, 3]),
            _ => panic!("expected binary payload"),
        }
    }

    #[test]
    fn snapshot_payload_external_round_trips() {
        let payload = SnapshotPayload::External(BlobDescriptor {
            logical_name: "mesh".to_string(),
            media_type: "application/octet-stream".to_string(),
            content_hash: Some("hash".to_string()),
            dedup_key: Some("dedup".to_string()),
            size_bytes: Some(128),
            content_reference: "blob://mesh".to_string(),
            metadata: Default::default(),
        });
        let value = serde_json::to_value(&payload).unwrap();
        let round_trip: SnapshotPayload = serde_json::from_value(value).unwrap();
        match round_trip {
            SnapshotPayload::External(blob) => {
                assert_eq!(blob.content_reference, "blob://mesh");
                assert_eq!(blob.dedup_key.as_deref(), Some("dedup"));
            }
            _ => panic!("expected external payload"),
        }
    }
}
