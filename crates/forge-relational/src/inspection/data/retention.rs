use serde::{Deserialize, Serialize};

use crate::identity::data::VersionId;
use crate::snapshots::data::SnapshotInspectionSummary;
use crate::storage::data::{RecordLifecycleState, RetentionPassOutcome};
use crate::transactions::data::RecordRef;

use super::{
    HistoricalAvailabilityObservation, InspectionAccessPath, InspectionAvailability,
    InspectionDegradation, InspectionOrigin,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionStateObservation {
    pub target: RecordRef,
    pub lifecycle: RecordLifecycleState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinStateObservation {
    pub target: RecordRef,
    pub snapshot_pins: u32,
    pub branch_pins: u32,
    pub replay_pins: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReclaimEligibility {
    EligibleNow,
    BlockedBySnapshotPins,
    BlockedByBranchPins,
    BlockedByReplayPins,
    BlockedByRetentionFence,
    BlockedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct RecordRetentionInspection {
    pub state: RetentionStateObservation,
    pub pins: PinStateObservation,
    pub reclaim_eligibility: ReclaimEligibility,
    pub historical_availability: HistoricalAvailabilityObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct RetentionInspectionSummary {
    pub current_version_id: VersionId,
    pub active_snapshot_count: u64,
    pub branch_pinned_entities: u64,
    pub replay_pinned_entities: u64,
    pub snapshot_pinned_entities: u64,
    pub branch_pinned_relations: u64,
    pub replay_pinned_relations: u64,
    pub snapshot_pinned_relations: u64,
    pub reclaimable_entities: u64,
    pub reclaimable_relations: u64,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionInspectionRequest {
    pub max_entity_slots_scanned: u64,
    pub max_relation_slots_scanned: u64,
    pub max_work_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct SnapshotPinInspection {
    pub snapshot: SnapshotInspectionSummary,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct RetentionExecutionInspection {
    pub outcome: RetentionPassOutcome,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}
