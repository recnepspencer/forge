use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::{Mutex, RwLock};

use crate::durability::data::{DurableCheckpoint, DurableCommitEnvelope, DurableStore};
use crate::history::data::{BranchId, VersionNode};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::publication::data::PublicationBundle;
use crate::publication::data::diff::PatchStreamPosition;
use crate::replay::data::{CanonicalCommitEnvelope, RelationalReplayRecord};
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};
use crate::storage::overlay::{PartitionState, SnapshotState};
use crate::symbols::data::StringInterner;

use super::{
    CompiledExecutionArtifact, RelationalRuntimeConfig, RuntimeComplexityCounters,
};

#[derive(Debug, Default)]
pub(crate) struct SnapshotRegistry {
    pub(crate) active: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    pub(crate) published_handles: BTreeMap<SnapshotId, crate::identity::data::VersionId>,
    pub(crate) visibility_states:
        RwLock<BTreeMap<crate::identity::data::VersionId, SnapshotState>>,
    pub(crate) visibility_residency:
        RwLock<BTreeMap<crate::identity::data::VersionId, VisibilityResidency>>,
    pub(crate) recent_policy: Mutex<DeterministicVersionWindowPolicy>,
    pub(crate) replay_retained: BTreeMap<crate::identity::data::VersionId, ReplayRetentionState>,
    pub(crate) next_snapshot_id: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotHandleBinding {
    pub(crate) version_id: crate::identity::data::VersionId,
    pub(crate) read_policy: SnapshotReadPolicy,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VisibilityResidency {
    pub(crate) branch_head_refs: u32,
    pub(crate) replay_refs: u32,
    pub(crate) active_snapshot_refs: u32,
    pub(crate) recent_resident: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DeterministicVersionWindowPolicy {
    pub(crate) recent_version_window: usize,
    pub(crate) order: VecDeque<crate::identity::data::VersionId>,
    pub(crate) resident_count: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct ReplayRetentionState {
    pub(crate) ref_count: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PublicationState {
    pub(crate) diagnostics: Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    pub(crate) latest_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
}

#[derive(Debug, Clone)]
pub(crate) struct HistoryState {
    pub(crate) branch_heads: BTreeMap<BranchId, Option<crate::history::data::CommitReference>>,
    pub(crate) commit_graph: BTreeMap<crate::history::data::CommitId, VersionNode>,
    pub(crate) commit_envelopes: BTreeMap<crate::history::data::CommitId, CanonicalCommitEnvelope>,
    pub(crate) patch_stream_index: BTreeMap<PatchStreamPosition, crate::history::data::CommitId>,
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct IndexState {
    pub(crate) definitions: BTreeMap<DerivedIndexId, DerivedIndexDefinition>,
    pub(crate) generations: BTreeMap<DerivedIndexId, Vec<DerivedIndexGeneration>>,
    pub(crate) entity_unique_field_index:
        BTreeMap<String, BTreeMap<String, BTreeSet<crate::identity::data::EntityId>>>,
    pub(crate) next_index_id: u64,
    pub(crate) next_generation_id: u64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LineageState {
    pub(crate) nodes: BTreeMap<crate::identity::data::LineageId, LineageNode>,
    pub(crate) events: Vec<LineageEventRecord>,
    pub(crate) correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub(crate) next_lineage_id: u64,
    pub(crate) next_event_id: u64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DurabilityState {
    pub(crate) log: Vec<DurableCommitEnvelope>,
    pub(crate) checkpoints: Vec<DurableCheckpoint>,
    pub(crate) store: Option<DurableStore>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuntimeSequenceState {
    pub(crate) next_transaction_id: u64,
    pub(crate) next_savepoint_id: u64,
}

#[derive(Debug, Default)]
pub(crate) struct RuntimeInstrumentation {
    pub(crate) complexity_counters: Mutex<RuntimeComplexityCounters>,
}

impl RuntimeInstrumentation {
    pub(crate) fn count(&self, update: impl FnOnce(&mut RuntimeComplexityCounters)) {
        update(
            &mut self
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned"),
        );
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SimulationState {
    pub(crate) compiled_artifacts: BTreeMap<u64, CompiledExecutionArtifact>,
    pub(crate) next_compiled_artifact_id: u64,
}

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) snapshots: SnapshotRegistry,
    pub(crate) publication: PublicationState,
    pub(crate) history: HistoryState,
    pub(crate) indexes: IndexState,
    pub(crate) lineage: LineageState,
    pub(crate) durability: DurabilityState,
    pub(crate) sequence: RuntimeSequenceState,
    pub(crate) symbols: StringInterner,
    pub(crate) instrumentation: RuntimeInstrumentation,
    pub(crate) simulation: SimulationState,
}
